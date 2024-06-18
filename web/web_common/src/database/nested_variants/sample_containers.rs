//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedSampleContainer {
    pub inner: Rc<crate::database::flat_variants::SampleContainer>,
    pub project: Rc<crate::database::nested_variants::NestedProject>,
    pub category: Rc<crate::database::nested_variants::NestedSampleContainerCategory>,
    pub created_by: Rc<crate::database::nested_variants::NestedUser>,
    pub updated_by: Rc<crate::database::nested_variants::NestedUser>,
}

unsafe impl Send for NestedSampleContainer {}
unsafe impl Sync for NestedSampleContainer {}
impl crate::database::Tabular for NestedSampleContainer {
    const TABLE: crate::database::Table = crate::database::Table::SampleContainers;
}
impl crate::database::Filtrable for NestedSampleContainer {
    type Filter = crate::database::filter_variants::SampleContainerFilter;
}
impl crate::database::Describable for NestedSampleContainer {
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }
}
impl crate::database::Colorable for NestedSampleContainer {
    fn color(&self) -> Option<&str> {
        None
    }
}
#[cfg(feature = "frontend")]
impl crate::database::AllRecords for NestedSampleContainer {
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
impl NestedSampleContainer {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) async fn from_flat(
        flat_variant: crate::database::flat_variants::SampleContainer,
        connection: &mut gluesql::prelude::Glue<
            impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
        >,
    ) -> Result<Self, crate::api::ApiError> {
        Ok(Self {
            project: Rc::from(
                crate::database::nested_variants::NestedProject::get(
                    flat_variant.project_id,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            category: Rc::from(
                crate::database::nested_variants::NestedSampleContainerCategory::get(
                    flat_variant.category_id,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            created_by: Rc::from(
                crate::database::nested_variants::NestedUser::get(
                    flat_variant.created_by,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            updated_by: Rc::from(
                crate::database::nested_variants::NestedUser::get(
                    flat_variant.updated_by,
                    connection,
                )
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
    /// Get the barcode attribute.
    pub fn get_barcode<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_barcode()
    }
    /// Get the project_id attribute.
    pub fn get_project_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_project_id()
    }
    /// Get the category_id attribute.
    pub fn get_category_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_category_id()
    }
    /// Get the created_by attribute.
    pub fn get_created_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_created_by()
    }
    /// Get the created_at attribute.
    pub fn get_created_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.inner.get_created_at()
    }
    /// Get the updated_by attribute.
    pub fn get_updated_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_updated_by()
    }
    /// Get the updated_at attribute.
    pub fn get_updated_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.inner.get_updated_at()
    }
    /// Insert the SampleContainer into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().insert(connection).await
    }
    /// Get the SampleContainer from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        if let Some(flat_variant) =
            crate::database::flat_variants::SampleContainer::get(id, connection).await?
        {
            Ok(Some(Self::from_flat(flat_variant, connection).await?))
        } else {
            Ok(None)
        }
    }
    /// Delete the SampleContainer from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().delete(connection).await
    }
    /// Delete the SampleContainer from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::flat_variants::SampleContainer::delete_from_id(id, connection).await
    }
    /// Update the struct in the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().update(connection).await
    }
    /// Update the struct in the database if it exists, otherwise insert it.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update_or_insert<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner
            .as_ref()
            .clone()
            .update_or_insert(connection)
            .await
    }
    /// Get all SampleContainer from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::SampleContainerFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        let mut sample_containers = Vec::new();
        for flat_variant in
            crate::database::flat_variants::SampleContainer::all(filter, limit, offset, connection)
                .await?
                .into_iter()
        {
            sample_containers.push(Self::from_flat(flat_variant, connection).await?);
        }
        Ok(sample_containers)
    }
}
