//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedUserEmail {
    pub inner: Rc<crate::database::flat_variants::UserEmail>,
    pub created_by: Rc<crate::database::nested_variants::NestedUser>,
    pub login_provider: Rc<crate::database::nested_variants::NestedLoginProvider>,
}

unsafe impl Send for NestedUserEmail {}
unsafe impl Sync for NestedUserEmail {}
impl crate::database::Tabular for NestedUserEmail {
    const TABLE: crate::database::Table = crate::database::Table::UserEmails;
}
impl crate::database::Filtrable for NestedUserEmail {
    type Filter = crate::database::filter_variants::UserEmailFilter;
}
impl crate::database::Describable for NestedUserEmail {
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }
}
impl crate::database::Colorable for NestedUserEmail {
    fn color(&self) -> Option<&str> {
        None
    }
}
#[cfg(feature = "frontend")]
impl crate::database::AllRecords for NestedUserEmail {
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
impl NestedUserEmail {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) async fn from_flat(
        flat_variant: crate::database::flat_variants::UserEmail,
        connection: &mut gluesql::prelude::Glue<
            impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
        >,
    ) -> Result<Self, crate::api::ApiError> {
        Ok(Self {
            created_by: Rc::from(
                crate::database::nested_variants::NestedUser::get(
                    flat_variant.created_by,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            login_provider: Rc::from(
                crate::database::nested_variants::NestedLoginProvider::get(
                    flat_variant.login_provider_id,
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
    /// Get the email attribute.
    pub fn get_email<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.inner.get_email()
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
    /// Get the login_provider_id attribute.
    pub fn get_login_provider_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_login_provider_id()
    }
    /// Whether the UserEmail is primary_email.
    pub fn is_primary_email<E>(&self) -> &E
    where
        bool: AsRef<E>,
    {
        self.inner.is_primary_email()
    }
    /// Get the UserEmail from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        if let Some(flat_variant) =
            crate::database::flat_variants::UserEmail::get(id, connection).await?
        {
            Ok(Some(Self::from_flat(flat_variant, connection).await?))
        } else {
            Ok(None)
        }
    }
    /// Delete the UserEmail from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.as_ref().clone().delete(connection).await
    }
    /// Delete the UserEmail from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::flat_variants::UserEmail::delete_from_id(id, connection).await
    }
    /// Get all UserEmail from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::UserEmailFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        let mut user_emails = Vec::new();
        for flat_variant in
            crate::database::flat_variants::UserEmail::all(filter, limit, offset, connection)
                .await?
                .into_iter()
        {
            user_emails.push(Self::from_flat(flat_variant, connection).await?);
        }
        Ok(user_emails)
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
        crate::database::nested_variants::NestedUser::update_or_insert(
            self.created_by.as_ref(),
            connection,
        )
        .await?;
        crate::database::nested_variants::NestedLoginProvider::update_or_insert(
            self.login_provider.as_ref(),
            connection,
        )
        .await?;
        crate::database::flat_variants::UserEmail::update_or_insert(self.inner.as_ref(), connection)
            .await
    }
}
