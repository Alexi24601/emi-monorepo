//! Webcommon traits generation
use std::path::Path;
mod deletable;
mod foreign;
mod attribute_traits;

use diesel::PgConnection;

use proc_macro2::TokenStream;

use crate::Codegen;
use crate::Table;
use syn::Ident;

impl Codegen<'_> {
    /// Code relative to generating all of the diesel code.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(crate) fn generate_table_traits(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        let submodule_file = root.with_extension("rs");

        std::fs::create_dir_all(root)?;

        let mut submodule_file_content = TokenStream::new();

        if self.enable_deletable_trait {
            self.generate_deletable_impls(
                root.join("deletable").as_path(),
                tables,
                conn,
            )?;

            let deletable_module_ident =
                Ident::new("deletable", proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                pub mod #deletable_module_ident;
            });
        }

        if self.enable_attribute_trait {
            self.generate_attribute_impls(
                root.join("attributes").as_path(),
                tables,
                conn,
            )?;

            let attribute_module_ident =
                Ident::new("attributes", proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                pub mod #attribute_module_ident;
            });
        }

        if self.enable_foreign_trait {
            self.generate_foreign_impls(
                root.join("foreign").as_path(),
                tables,
                conn,
            )?;

            let foreign_module_ident =
                Ident::new("foreign", proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                pub mod #foreign_module_ident;
            });
        }

        std::fs::write(&submodule_file, submodule_file_content.to_string())?;

        Ok(())
    }
}
