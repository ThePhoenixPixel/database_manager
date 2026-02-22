use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Table, attributes(table_name, primary_key, auto_increment, unique, nullable))]
pub fn derive_table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    // Get table name from attribute or use struct name in lowercase
    let table_name = input.attrs.iter()
        .find(|attr| attr.path().is_ident("table_name"))
        .and_then(|attr| {
            attr.parse_args::<syn::LitStr>().ok().map(|lit| lit.value())
        })
        .unwrap_or_else(|| struct_name.to_string().to_lowercase());

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Table derive only supports structs with named fields"),
        },
        _ => panic!("Table derive only supports structs"),
    };

    let mut columns = Vec::new();
    let mut to_row_fields = Vec::new();
    let mut from_row_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_type = &field.ty;

        // Check for attributes
        let is_primary = field.attrs.iter().any(|attr| attr.path().is_ident("primary_key"));
        let is_auto_increment = field.attrs.iter().any(|attr| attr.path().is_ident("auto_increment"));
        let is_unique = field.attrs.iter().any(|attr| attr.path().is_ident("unique"));
        let is_nullable = field.attrs.iter().any(|attr| attr.path().is_ident("nullable"));

        // Build column modifications based on attributes
        let mut col_mods = Vec::new();

        if is_primary {
            col_mods.push(quote! { col = col.primary_key(); });
        }
        if is_auto_increment {
            col_mods.push(quote! { col = col.auto_increment(); });
        }
        if is_unique {
            col_mods.push(quote! { col = col.unique(); });
        }
        if is_nullable {
            col_mods.push(quote! { col = col.nullable(); });
        }

        columns.push(quote! {
            {
                let mut col = database_manager::types::Column::new(
                    #field_name_str,
                    <#field_type as database_manager::types::DbType>::column_type()
                );

                #(#col_mods)*

                col
            }
        });

        // Build to_row() conversion - skip auto_increment fields
        if !is_auto_increment {
            to_row_fields.push(quote! {
                row.insert(
                    #field_name_str.to_string(),
                    database_manager::types::DbType::to_value(self.#field_name.clone())
                );
            });
        }

        // Build from_row() conversion - all fields
        from_row_fields.push(quote! {
            #field_name: {
                let value = row.get(#field_name_str)
                    .ok_or_else(|| database_manager::types::DbError::InvalidData(
                        format!("Missing field: {}", #field_name_str)
                    ))?;
                <#field_type as database_manager::types::DbType>::from_value(value)?
            }
        });
    }

    let expanded = quote! {
        impl database_manager::Table for #struct_name {
            fn table_name() -> &'static str {
                #table_name
            }

            fn table_schema() -> database_manager::types::TableSchema {
                let mut schema = database_manager::types::TableSchema::new(Self::table_name());

                #(
                    schema = schema.add_column(#columns);
                )*

                schema
            }

            fn to_row(&self) -> database_manager::types::Row {
                let mut row = std::collections::HashMap::new();

                #(#to_row_fields)*

                row
            }

            fn from_row(row: &database_manager::types::Row) -> database_manager::types::DbResult<Self> {
                Ok(Self {
                    #(#from_row_fields),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
