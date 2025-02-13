//! Submodule providing methods to write out the schema of a table.

use crate::errors::WebCodeGenError;
use crate::Table;
use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

impl Table {
    /// Returns the Syn `TokenStream` for the diesel schema definition for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the diesel schema definition for the table.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    /// * If the columns cannot be loaded from the database.
    /// * If the diesel type for a column cannot be loaded.
    /// * If the primary key columns cannot be loaded from the database.
    /// * If the diesel feature flag name cannot be generated.
    ///
    pub fn to_schema(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        let table_schema = Ident::new(&self.table_schema, proc_macro2::Span::call_site());
        let original_table_name = &self.table_name;
        let sanitized_table_name_ident =
            Ident::new(&self.snake_case_name()?, proc_macro2::Span::call_site());
        let columns = self
            .columns(conn)?
            .into_iter()
            .map(|column| {
                let original_column_name = &column.column_name;
                let column_attribute: Ident = column.sanitized_snake_case_ident()?;
                let column_type = column.diesel_type(conn)?;
                Ok(if original_column_name == &column_attribute.to_string() {
                    quote! {
                        #column_attribute -> #column_type
                    }
                } else {
                    quote! {
                        #[sql_name = #original_column_name]
                        #column_attribute -> #column_type
                    }
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;
        let primary_key_names = self
            .primary_key_columns(conn)?
            .into_iter()
            .map(|column| Ident::new(&column.column_name, proc_macro2::Span::call_site()))
            .collect::<Vec<Ident>>();

        let primary_key_names = if primary_key_names.is_empty() {
            TokenStream::new()
        } else {
            quote! {
                (#(#primary_key_names),*)
            }
        };

        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        Ok(if self.has_snake_case_name()? {
            quote! {
                #[cfg(feature = #columns_feature_flag_name)]
                diesel::table! {
                    #[sql_name = #original_table_name]
                    #table_schema.#sanitized_table_name_ident #primary_key_names {
                        #(#columns),*
                    }
                }
            }
        } else {
            quote! {
                #[cfg(feature = #columns_feature_flag_name)]
                diesel::table! {
                    #table_schema.#sanitized_table_name_ident #primary_key_names {
                        #(#columns),*
                    }
                }
            }
        })
    }
}
