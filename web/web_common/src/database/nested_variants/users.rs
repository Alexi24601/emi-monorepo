//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedUser {
    pub inner: Rc<crate::database::flat_variants::User>,
    pub organization: Option<Rc<crate::database::nested_variants::NestedOrganization>>,
}

unsafe impl Send for NestedUser {}
unsafe impl Sync for NestedUser {}
impl crate::database::Tabular for NestedUser {
    const TABLE: crate::database::Table = crate::database::Table::Users;
}
impl crate::database::Filtrable for NestedUser {
    type Filter = crate::database::filter_variants::UserFilter;
}
impl crate::database::Describable for NestedUser {
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }
}
impl crate::database::Colorable for NestedUser {
    fn color(&self) -> Option<&str> {
        None
    }
}
#[cfg(feature = "frontend")]
impl crate::database::AllRecords for NestedUser {
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
impl NestedUser {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) async fn from_flat(
        flat_variant: crate::database::flat_variants::User,
        connection: &mut gluesql::prelude::Glue<
            impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
        >,
    ) -> Result<Self, crate::api::ApiError> {
        Ok(Self {
            organization: if let Some(organization_id) = flat_variant.organization_id {
                crate::database::nested_variants::NestedOrganization::get(
                    organization_id,
                    connection,
                )
                .await?
                .map(Rc::from)
            } else {
                None
            },
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
    /// Get the first_name attribute.
    pub fn get_first_name<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_first_name()
    }
    /// Get the middle_name attribute.
    pub fn get_middle_name<E>(&self) -> Option<&E>
    where
        String: AsRef<E>,
    {
        self.inner.get_middle_name()
    }
    /// Get the last_name attribute.
    pub fn get_last_name<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_last_name()
    }
    /// Get the description attribute.
    pub fn get_description<E>(&self) -> Option<&E>
    where
        String: AsRef<E>,
    {
        self.inner.get_description()
    }
    /// Get the organization_id attribute.
    pub fn get_organization_id<E>(&self) -> Option<&E>
    where
        i32: AsRef<E>,
    {
        self.inner.get_organization_id()
    }
    /// Get the created_at attribute.
    pub fn get_created_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.inner.get_created_at()
    }
    /// Get the updated_at attribute.
    pub fn get_updated_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.inner.get_updated_at()
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
    /// Insert the User into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().insert(connection).await
    }
    /// Get the User from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        if let Some(flat_variant) =
            crate::database::flat_variants::User::get(id, connection).await?
        {
            Ok(Some(Self::from_flat(flat_variant, connection).await?))
        } else {
            Ok(None)
        }
    }
    /// Delete the User from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().delete(connection).await
    }
    /// Delete the User from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::flat_variants::User::delete_from_id(id, connection).await
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
    /// Get all User from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::UserFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        let mut users = Vec::new();
        for flat_variant in
            crate::database::flat_variants::User::all(filter, limit, offset, connection)
                .await?
                .into_iter()
        {
            users.push(Self::from_flat(flat_variant, connection).await?);
        }
        Ok(users)
    }
}
