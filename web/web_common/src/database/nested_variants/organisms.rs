//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedOrganism {
    pub inner: Rc<crate::database::flat_variants::Organism>,
    pub host_organism: Option<Rc<crate::database::flat_variants::Organism>>,
    pub sample: Option<Rc<crate::database::nested_variants::NestedSample>>,
    pub nameplate: Rc<crate::database::nested_variants::NestedNameplate>,
    pub project: Rc<crate::database::nested_variants::NestedProject>,
    pub created_by: Rc<crate::database::nested_variants::NestedUser>,
    pub updated_by: Rc<crate::database::nested_variants::NestedUser>,
}

unsafe impl Send for NestedOrganism {}
unsafe impl Sync for NestedOrganism {}
impl crate::database::Tabular for NestedOrganism {
    const TABLE: crate::database::Table = crate::database::Table::Organisms;
}
impl crate::database::Filtrable for NestedOrganism {
    type Filter = crate::database::filter_variants::OrganismFilter;
}
impl crate::database::Describable for NestedOrganism {
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }
}
impl crate::database::Colorable for NestedOrganism {
    fn color(&self) -> Option<&str> {
        None
    }
}
#[cfg(feature = "frontend")]
impl crate::database::AllRecords for NestedOrganism {
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
impl NestedOrganism {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) async fn from_flat(
        flat_variant: crate::database::flat_variants::Organism,
        connection: &mut gluesql::prelude::Glue<
            impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
        >,
    ) -> Result<Self, crate::api::ApiError> {
        Ok(Self {
            host_organism: if let Some(host_organism_id) = flat_variant.host_organism_id {
                crate::database::flat_variants::Organism::get(host_organism_id, connection)
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
            nameplate: Rc::from(
                crate::database::nested_variants::NestedNameplate::get(
                    flat_variant.nameplate_id,
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
        uuid::Uuid: AsRef<E>,
    {
        self.inner.get_id()
    }
    /// Get the host_organism_id attribute.
    pub fn get_host_organism_id<E>(&self) -> Option<&E>
    where
        uuid::Uuid: AsRef<E>,
    {
        self.inner.get_host_organism_id()
    }
    /// Get the sample_id attribute.
    pub fn get_sample_id<E>(&self) -> Option<&E>
    where
        uuid::Uuid: AsRef<E>,
    {
        self.inner.get_sample_id()
    }
    /// Get the notes attribute.
    pub fn get_notes<E>(&self) -> Option<&E>
    where
        String: AsRef<E>,
    {
        self.inner.get_notes()
    }
    /// Whether the Organism is wild.
    pub fn is_wild<E>(&self) -> &E
    where
        bool: AsRef<E>,
    {
        self.inner.is_wild()
    }
    /// Get the nameplate_id attribute.
    pub fn get_nameplate_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_nameplate_id()
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
    /// Get the Organism from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        if let Some(flat_variant) =
            crate::database::flat_variants::Organism::get(id, connection).await?
        {
            Ok(Some(Self::from_flat(flat_variant, connection).await?))
        } else {
            Ok(None)
        }
    }
    /// Delete the Organism from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().delete(connection).await
    }
    /// Delete the Organism from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: uuid::Uuid,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::flat_variants::Organism::delete_from_id(id, connection).await
    }
    /// Get all Organism from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::OrganismFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        let mut organisms = Vec::new();
        for flat_variant in
            crate::database::flat_variants::Organism::all(filter, limit, offset, connection)
                .await?
                .into_iter()
        {
            organisms.push(Self::from_flat(flat_variant, connection).await?);
        }
        Ok(organisms)
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
        if let Some(host_organism) = self.host_organism.as_ref() {
            crate::database::flat_variants::Organism::update_or_insert(host_organism, connection)
                .await?;
        }
        if let Some(sample) = self.sample.as_ref() {
            crate::database::nested_variants::NestedSample::update_or_insert(sample, connection)
                .await?;
        }
        crate::database::nested_variants::NestedNameplate::update_or_insert(
            self.nameplate.as_ref(),
            connection,
        )
        .await?;
        crate::database::nested_variants::NestedProject::update_or_insert(
            self.project.as_ref(),
            connection,
        )
        .await?;
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
        crate::database::flat_variants::Organism::update_or_insert(self.inner.as_ref(), connection)
            .await
    }
}
