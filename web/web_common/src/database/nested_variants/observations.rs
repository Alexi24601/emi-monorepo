//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedObservation {
    pub inner: Rc<crate::database::flat_variants::Observation>,
    pub parent_observation: Option<Rc<crate::database::flat_variants::Observation>>,
    pub created_by: Rc<crate::database::nested_variants::NestedUser>,
    pub updated_by: Rc<crate::database::nested_variants::NestedUser>,
    pub project: Rc<crate::database::nested_variants::NestedProject>,
    pub organism: Option<Rc<crate::database::nested_variants::NestedOrganism>>,
    pub sample: Option<Rc<crate::database::nested_variants::NestedSample>>,
    pub subject: Rc<crate::database::nested_variants::NestedObservationSubject>,
}

unsafe impl Send for NestedObservation {}
unsafe impl Sync for NestedObservation {}
impl crate::database::Tabular for NestedObservation {
    const TABLE: crate::database::Table = crate::database::Table::Observations;
}
impl crate::database::Filtrable for NestedObservation {
    type Filter = crate::database::filter_variants::ObservationFilter;
}
impl crate::database::Describable for NestedObservation {
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }
}
impl crate::database::Colorable for NestedObservation {
    fn color(&self) -> Option<&str> {
        None
    }
}
#[cfg(feature = "frontend")]
impl crate::database::AllRecords for NestedObservation {
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
impl NestedObservation {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) async fn from_flat(
        flat_variant: crate::database::flat_variants::Observation,
        connection: &mut gluesql::prelude::Glue<
            impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
        >,
    ) -> Result<Self, crate::api::ApiError> {
        Ok(Self {
            parent_observation: if let Some(parent_observation_id) =
                flat_variant.parent_observation_id
            {
                crate::database::flat_variants::Observation::get(parent_observation_id, connection)
                    .await?
                    .map(Rc::from)
            } else {
                None
            },
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
            project: Rc::from(
                crate::database::nested_variants::NestedProject::get(
                    flat_variant.project_id,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            organism: if let Some(organism_id) = flat_variant.organism_id {
                crate::database::nested_variants::NestedOrganism::get(organism_id, connection)
                    .await?
                    .map(Rc::from)
            } else {
                None
            },
            sample: if let Some(sample_id) = flat_variant.sample_id {
                crate::database::nested_variants::NestedSample::get(sample_id, connection)
                    .await?
                    .map(Rc::from)
            } else {
                None
            },
            subject: Rc::from(
                crate::database::nested_variants::NestedObservationSubject::get(
                    flat_variant.subject_id,
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
    /// Get the parent_observation_id attribute.
    pub fn get_parent_observation_id<E>(&self) -> Option<&E>
    where
        uuid::Uuid: AsRef<E>,
    {
        self.inner.get_parent_observation_id()
    }
    /// Get the created_by attribute.
    pub fn get_created_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_created_by()
    }
    /// Get the created_at attribute.
    pub fn get_created_at<E>(&self) -> Option<&E>
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
    pub fn get_updated_at<E>(&self) -> Option<&E>
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.inner.get_updated_at()
    }
    /// Get the project_id attribute.
    pub fn get_project_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_project_id()
    }
    /// Get the organism_id attribute.
    pub fn get_organism_id<E>(&self) -> Option<&E>
    where
        uuid::Uuid: AsRef<E>,
    {
        self.inner.get_organism_id()
    }
    /// Get the sample_id attribute.
    pub fn get_sample_id<E>(&self) -> Option<&E>
    where
        uuid::Uuid: AsRef<E>,
    {
        self.inner.get_sample_id()
    }
    /// Get the subject_id attribute.
    pub fn get_subject_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_subject_id()
    }
    /// Get the notes attribute.
    pub fn get_notes<E>(&self) -> Option<&E>
    where
        String: AsRef<E>,
    {
        self.inner.get_notes()
    }
    /// Get the URL of the picture image.
    pub fn get_picture_as_url(&self) -> String {
        self.inner.get_picture_as_url()
    }
    /// Get the picture attribute.
    pub fn get_picture<E>(&self) -> &E
    where
        crate::types::JPEG: AsRef<E>,
    {
        self.inner.get_picture()
    }
    /// Get the Observation from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        if let Some(flat_variant) =
            crate::database::flat_variants::Observation::get(id, connection).await?
        {
            Ok(Some(Self::from_flat(flat_variant, connection).await?))
        } else {
            Ok(None)
        }
    }
    /// Delete the Observation from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().delete(connection).await
    }
    /// Delete the Observation from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::flat_variants::Observation::delete_from_id(id, connection).await
    }
    /// Get all Observation from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::ObservationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        let mut observations = Vec::new();
        for flat_variant in
            crate::database::flat_variants::Observation::all(filter, limit, offset, connection)
                .await?
                .into_iter()
        {
            observations.push(Self::from_flat(flat_variant, connection).await?);
        }
        Ok(observations)
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
        if let Some(parent_observation) = self.parent_observation.as_ref() {
            crate::database::flat_variants::Observation::update_or_insert(
                parent_observation,
                connection,
            )
            .await?;
        }
        crate::database::nested_variants::NestedUser::update_or_insert(
            self.created_by.as_ref(),
            connection,
        )
        .await?;
        crate::database::nested_variants::NestedUser::update_or_insert(
            self.updated_by.as_ref(),
            connection,
        )
        .await?;
        crate::database::nested_variants::NestedProject::update_or_insert(
            self.project.as_ref(),
            connection,
        )
        .await?;
        if let Some(organism) = self.organism.as_ref() {
            crate::database::nested_variants::NestedOrganism::update_or_insert(
                organism, connection,
            )
            .await?;
        }
        if let Some(sample) = self.sample.as_ref() {
            crate::database::nested_variants::NestedSample::update_or_insert(sample, connection)
                .await?;
        }
        crate::database::nested_variants::NestedObservationSubject::update_or_insert(
            self.subject.as_ref(),
            connection,
        )
        .await?;
        crate::database::flat_variants::Observation::update_or_insert(
            self.inner.as_ref(),
            connection,
        )
        .await
    }
}
