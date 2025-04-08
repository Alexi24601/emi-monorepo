//! Submodule providing the code to generate the implementation of the
//! [`InsertableVariant`] trait for all required tables.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;

use crate::{Codegen, Table};

impl Codegen<'_> {
    /// Generates the [`InsertableVariant`] trait implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(super) fn generate_insertable_variant_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut ifvb_main_module = TokenStream::new();
        let syntax_flag = self.syntax.as_feature_flag();
        let connection_type = self.syntax.as_connection_type();
        let Some(user_table) = self.users_table else {
            return Err(crate::errors::CodeGenerationError::UserTableNotProvided.into());
        };
        let user_id_type = user_table.primary_key_type(conn)?;

        for table in tables {
            if !self.is_table_insertable(table, conn)? {
                continue;
            }
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_path = table.import_struct_path()?;
            let table_ident = table.snake_case_ident()?;
            let insertable_frontend_variant = table.insertable_variant_ty()?;
            let insertable_builder = table.insertable_builder_ty()?;

            // We build a check to see whether the user is authorized to update
            // the parent tables.
            let parent_check = table
                .parent_keys(conn)?
                .iter()
                .map(|parent_key| {
                    let parent_key_method = parent_key.getter_ident()?;
                    Ok(if parent_key.is_nullable() {
                        quote::quote! {
                            if let Some(parent) = self.#parent_key_method(conn).await? {
                                if !parent.can_update(user_id, conn).await? {
                                    return Err(server_errors::Error::Unauthorized.into());
                                }
                            }
                        }
                    } else {
                        quote::quote! {
                            if !self.#parent_key_method(conn).await?.can_update(user_id, conn).await? {
                                return Err(server_errors::Error::Unauthorized.into());
                            }
                        }
                    })
                })
                .collect::<Result<TokenStream, crate::errors::WebCodeGenError>>()?;

            let (user_id, additional_imports) = if parent_check.is_empty() {
                (quote::quote! { _user_id }, TokenStream::new())
            } else {
                (quote::quote! { user_id }, quote::quote! {
                    use web_common_traits::database::Updatable;
                })
            };

            std::fs::write(&table_file, self.beautify_code(&quote::quote!{
                #syntax_flag
				impl web_common_traits::database::InsertableVariant for #insertable_frontend_variant {
					type Row = #table_path;
                    type InsertableBuilder = #insertable_builder;
                    type Conn = #connection_type;
                    type UserId = #user_id_type;

                    async fn insert(
                        self,
                        #user_id: &Self::UserId,
                        conn: &mut Self::Conn,
                    ) -> Result<
                        Self::Row,
                        web_common_traits::database::InsertError<<Self::InsertableBuilder as common_traits::prelude::Builder>::Attribute>,
                    > {
                        use diesel_async::RunQueryDsl;
                        use diesel::associations::HasTable;
                        #additional_imports

                        #parent_check

                        Ok(diesel::insert_into(Self::Row::table())
                            .values(self)
                            .get_result(conn).await?)
                    }
				}
			})?)?;

            ifvb_main_module.extend(quote::quote! {
                mod #table_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&ifvb_main_module)?)?;

        Ok(())
    }
}
