//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedDocumentFormat {
    pub inner: Rc<crate::database::flat_variants::DocumentFormat>,
    pub icon: Rc<crate::database::flat_variants::FontAwesomeIcon>,
    pub color: Rc<crate::database::flat_variants::Color>,
}

unsafe impl Send for NestedDocumentFormat {}
unsafe impl Sync for NestedDocumentFormat {}
impl crate::database::Tabular for NestedDocumentFormat {
    const TABLE: crate::database::Table = crate::database::Table::DocumentFormats;
}
impl crate::database::Filtrable for NestedDocumentFormat {
    type Filter = crate::database::filter_variants::DocumentFormatFilter;
}
impl crate::database::Describable for NestedDocumentFormat {
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }
}
impl crate::database::Colorable for NestedDocumentFormat {
    fn color(&self) -> Option<&str> {
        Some(self.color.name.as_str())
    }
}
#[cfg(feature = "frontend")]
impl crate::database::AllRecords for NestedDocumentFormat {
    fn all_records<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&<Self as crate::database::Filtrable>::Filter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, crate::api::ApiError>> {
        Self::all(filter, limit, offset, connection)
    }
}
#[cfg(feature = "frontend")]
impl NestedDocumentFormat {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) async fn from_flat(
        flat_variant: crate::database::flat_variants::DocumentFormat,
        connection: &mut gluesql::prelude::Glue<
            impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
        >,
    ) -> Result<Self, crate::api::ApiError> {
        Ok(Self {
            icon: Rc::from(
                crate::database::flat_variants::FontAwesomeIcon::get(
                    flat_variant.icon_id,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            color: Rc::from(
                crate::database::flat_variants::Color::get(flat_variant.color_id, connection)
                    .await?
                    .unwrap(),
            ),
            inner: Rc::from(flat_variant),
        })
    }
    /// Get the id attribute.
    pub fn get_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_id()
    }
    /// Get the extension attribute.
    pub fn get_extension<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_extension()
    }
    /// Get the mime_type attribute.
    pub fn get_mime_type<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_mime_type()
    }
    /// Get the description attribute.
    pub fn get_description<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_description()
    }
    /// Get the icon_id attribute.
    pub fn get_icon_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_icon_id()
    }
    /// Get the color_id attribute.
    pub fn get_color_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_color_id()
    }
    /// Get the DocumentFormat from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        if let Some(flat_variant) =
            crate::database::flat_variants::DocumentFormat::get(id, connection).await?
        {
            Ok(Some(Self::from_flat(flat_variant, connection).await?))
        } else {
            Ok(None)
        }
    }
    /// Delete the DocumentFormat from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().delete(connection).await
    }
    /// Delete the DocumentFormat from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::flat_variants::DocumentFormat::delete_from_id(id, connection).await
    }
    /// Get all DocumentFormat from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::DocumentFormatFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        let mut document_formats = Vec::new();
        for flat_variant in
            crate::database::flat_variants::DocumentFormat::all(filter, limit, offset, connection)
                .await?
                .into_iter()
        {
            document_formats.push(Self::from_flat(flat_variant, connection).await?);
        }
        Ok(document_formats)
    }
    /// Update or insert the record in the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update_or_insert<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        &self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::flat_variants::FontAwesomeIcon::update_or_insert(
            self.icon.as_ref(),
            connection,
        )
        .await?;
        crate::database::flat_variants::Color::update_or_insert(self.color.as_ref(), connection)
            .await?;
        crate::database::flat_variants::DocumentFormat::update_or_insert(
            self.inner.as_ref(),
            connection,
        )
        .await
    }
}
