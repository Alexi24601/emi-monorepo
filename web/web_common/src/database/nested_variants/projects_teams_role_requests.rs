//! This module contains the nested structs for the database tables.
//!
//! This file is automatically generated. Do not write anything here.

use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedProjectsTeamsRoleRequest {
    pub inner: crate::database::flat_variants::ProjectsTeamsRoleRequest,
    pub table: Rc<crate::database::nested_variants::NestedProject>,
    pub team: Rc<crate::database::nested_variants::NestedTeam>,
    pub role: Rc<crate::database::nested_variants::NestedRole>,
    pub created_by: Rc<crate::database::nested_variants::NestedUser>,
}

unsafe impl Send for NestedProjectsTeamsRoleRequest {}
unsafe impl Sync for NestedProjectsTeamsRoleRequest {}
impl crate::database::Tabular for NestedProjectsTeamsRoleRequest {
    const TABLE: crate::database::Table = crate::database::Table::ProjectsTeamsRoleRequests;
}
impl crate::database::Filtrable for NestedProjectsTeamsRoleRequest {
    type Filter = crate::database::filter_variants::ProjectsTeamsRoleRequestFilter;
}
impl crate::database::Describable for NestedProjectsTeamsRoleRequest {
    fn description(&self) -> Option<&str> {
        self.inner.description()
    }
}
impl crate::database::Colorable for NestedProjectsTeamsRoleRequest {
    fn color(&self) -> Option<&str> {
        None
    }
}
#[cfg(feature = "frontend")]
impl crate::database::AllRecords for NestedProjectsTeamsRoleRequest {
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
impl NestedProjectsTeamsRoleRequest {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) async fn from_flat(
        flat_variant: crate::database::flat_variants::ProjectsTeamsRoleRequest,
        connection: &mut gluesql::prelude::Glue<
            impl gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
        >,
    ) -> Result<Self, crate::api::ApiError> {
        Ok(Self {
            table: Rc::from(
                crate::database::nested_variants::NestedProject::get(
                    flat_variant.table_id,
                    connection,
                )
                .await?
                .unwrap(),
            ),
            team: Rc::from(
                crate::database::nested_variants::NestedTeam::get(flat_variant.team_id, connection)
                    .await?
                    .unwrap(),
            ),
            role: Rc::from(
                crate::database::nested_variants::NestedRole::get(flat_variant.role_id, connection)
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
            inner: flat_variant,
        })
    }
    /// Get the table_id attribute.
    pub fn get_table_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_table_id()
    }
    /// Get the team_id attribute.
    pub fn get_team_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_team_id()
    }
    /// Get the role_id attribute.
    pub fn get_role_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.inner.get_role_id()
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
    /// Get the ProjectsTeamsRoleRequest from the database by its ID.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        (table_id, team_id): (i32, i32),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        if let Some(flat_variant) = crate::database::flat_variants::ProjectsTeamsRoleRequest::get(
            (table_id, team_id),
            connection,
        )
        .await?
        {
            Ok(Some(Self::from_flat(flat_variant, connection).await?))
        } else {
            Ok(None)
        }
    }
    /// Delete the ProjectsTeamsRoleRequest from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        self.inner.delete(connection).await
    }
    /// Delete the ProjectsTeamsRoleRequest from the database by its ID.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        (table_id, team_id): (i32, i32),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        crate::database::flat_variants::ProjectsTeamsRoleRequest::delete_from_id(
            (table_id, team_id),
            connection,
        )
        .await
    }
    /// Get all ProjectsTeamsRoleRequest from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::ProjectsTeamsRoleRequestFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        let mut projects_teams_role_requests = Vec::new();
        for flat_variant in crate::database::flat_variants::ProjectsTeamsRoleRequest::all(
            filter, limit, offset, connection,
        )
        .await?
        .into_iter()
        {
            projects_teams_role_requests.push(Self::from_flat(flat_variant, connection).await?);
        }
        Ok(projects_teams_role_requests)
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
        crate::database::nested_variants::NestedProject::update_or_insert(
            self.table.as_ref(),
            connection,
        )
        .await?;
        crate::database::nested_variants::NestedTeam::update_or_insert(
            self.team.as_ref(),
            connection,
        )
        .await?;
        crate::database::nested_variants::NestedRole::update_or_insert(
            self.role.as_ref(),
            connection,
        )
        .await?;
        crate::database::nested_variants::NestedUser::update_or_insert(
            self.created_by.as_ref(),
            connection,
        )
        .await?;
        crate::database::flat_variants::ProjectsTeamsRoleRequest::update_or_insert(
            &self.inner,
            connection,
        )
        .await
    }
}
