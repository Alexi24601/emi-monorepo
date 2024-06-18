//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedSample {
    pub inner: Rc<crate::database::flat_variants::Sample>,
    pub container: Rc<crate::database::nested_variants::NestedSampleContainer>,
    pub project: Rc<crate::database::nested_variants::NestedProject>,
    pub created_by: Rc<crate::database::nested_variants::NestedUser>,
    pub sampled_by: Rc<crate::database::nested_variants::NestedUser>,
    pub updated_by: Rc<crate::database::nested_variants::NestedUser>,
    pub state: Rc<crate::database::nested_variants::NestedSampleState>,
}

unsafe impl Send for NestedSample {}
unsafe impl Sync for NestedSample {}
impl crate::database::Tabular for NestedSample {
    const TABLE: crate::database::Table = crate::database::Table::Samples;
}
impl crate::database::Filtrable for NestedSample {
    type Filter = crate::database::filter_variants::SampleFilter;
}
impl crate::database::Describable for NestedSample {
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }
}
impl crate::database::Colorable for NestedSample {
    fn color(&self) -> Option<&str> {
        None
    }
}
#[cfg(feature = "frontend")]
impl crate::database::AllRecords for NestedSample {
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
impl NestedSample {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) async fn from_flat(
        flat_variant: crate::database::flat_variants::Sample,
        connection: &mut gluesql::prelude::Glue<
            impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
        >,
    ) -> Result<Self, crate::api::ApiError> {
        Ok(Self {
            container: Rc::from(
                crate::database::nested_variants::NestedSampleContainer::get(
                    flat_variant.container_id,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            project: Rc::from(
                crate::database::nested_variants::NestedProject::get(
                    flat_variant.project_id,
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
            sampled_by: Rc::from(
                crate::database::nested_variants::NestedUser::get(
                    flat_variant.sampled_by,
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
            state: Rc::from(
                crate::database::nested_variants::NestedSampleState::get(
                    flat_variant.state_id,
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
        uuid::Uuid: AsRef<E>,
    {
        self.inner.get_id()
    }
    /// Get the container_id attribute.
    pub fn get_container_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_container_id()
    }
    /// Get the notes attribute.
    pub fn get_notes<E>(&self) -> Option<&E>
    where
        String: AsRef<E>,
    {
        self.inner.get_notes()
    }
    /// Get the project_id attribute.
    pub fn get_project_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_project_id()
    }
    /// Get the created_by attribute.
    pub fn get_created_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_created_by()
    }
    /// Get the sampled_by attribute.
    pub fn get_sampled_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_sampled_by()
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
    /// Get the state_id attribute.
    pub fn get_state_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_state_id()
    }
    /// Insert the Sample into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().insert(connection).await
    }
    /// Get the Sample from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        if let Some(flat_variant) =
            crate::database::flat_variants::Sample::get(id, connection).await?
        {
            Ok(Some(Self::from_flat(flat_variant, connection).await?))
        } else {
            Ok(None)
        }
    }
    /// Delete the Sample from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().delete(connection).await
    }
    /// Delete the Sample from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::flat_variants::Sample::delete_from_id(id, connection).await
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
    /// Get all Sample from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::SampleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        let mut samples = Vec::new();
        for flat_variant in
            crate::database::flat_variants::Sample::all(filter, limit, offset, connection)
                .await?
                .into_iter()
        {
            samples.push(Self::from_flat(flat_variant, connection).await?);
        }
        Ok(samples)
    }
}
