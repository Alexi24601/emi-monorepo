//! This file is automatically generated, do not modify it directly.
use super::*;
#[derive(
    Eq,
    PartialEq,
    PartialOrd,
    Debug,
    Clone,
    Copy,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    Default,
)]
pub struct UsersUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for UsersUsersRoleRequest {}
unsafe impl Sync for UsersUsersRoleRequest {}
impl Tabular for UsersUsersRoleRequest {
    const TABLE: crate::database::Table = crate::database::Table::UsersUsersRoleRequests;
}
impl Describable for UsersUsersRoleRequest {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for UsersUsersRoleRequest {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for crate::database::flat_variants::UsersUsersRoleRequest {
    type Filter = crate::database::filter_variants::UsersUsersRoleRequestFilter;
}
#[cfg(feature = "frontend")]
impl AllRecords for UsersUsersRoleRequest {
    fn all_records<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&<Self as Filtrable>::Filter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> impl std::future::Future<Output = Result<Vec<Self>, crate::api::ApiError>> {
        Self::all(filter, limit, offset, connection)
    }
}
#[cfg(feature = "frontend")]
impl UsersUsersRoleRequest {
    /// Get the table_id attribute.
    pub fn get_table_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.table_id.as_ref()
    }

    /// Get the user_id attribute.
    pub fn get_user_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.user_id.as_ref()
    }

    /// Get the role_id attribute.
    pub fn get_role_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.role_id.as_ref()
    }

    /// Get the created_by attribute.
    pub fn get_created_by<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.created_by.as_ref()
    }

    /// Get the created_at attribute.
    pub fn get_created_at<E>(&self) -> &E
    where
        chrono::NaiveDateTime: AsRef<E>,
    {
        self.created_at.as_ref()
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.table_id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::num(self.role_id),
            gluesql::core::ast_builder::num(self.created_by),
            gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
        ]
    }

    /// Insert the UsersUsersRoleRequest into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("users_users_role_requests")
            .insert()
            .columns("table_id, user_id, role_id, created_by, created_at")
            .values(vec![self.into_row()])
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Insert(number_of_inserted_rows) => {
                    number_of_inserted_rows
                }
                _ => unreachable!("Payload must be an Insert"),
            })?)
    }

    /// Get the UsersUsersRoleRequest from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        (table_id, user_id): (i32, i32),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("users_users_role_requests")
            .select()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .project("table_id, user_id, role_id, created_by, created_at")
            .limit(1)
            .execute(connection)
            .await?;
        Ok(select_row
            .select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>()
            .pop())
    }

    /// Delete the UsersUsersRoleRequest from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Self::delete_from_id((self.table_id, self.user_id), connection).await
    }

    /// Delete the UsersUsersRoleRequest from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        (table_id, user_id): (i32, i32),
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("users_users_role_requests")
            .delete()
            .filter(col("table_id").eq(table_id.to_string()))
            .filter(col("user_id").eq(user_id.to_string()))
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Delete(number_of_deleted_rows) => number_of_deleted_rows,
                _ => unreachable!("Payload must be a Delete"),
            })?)
    }

    /// Update the struct in the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn update<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        table("users_users_role_requests")
            .update()
            .set("table_id", gluesql::core::ast_builder::num(self.table_id))
            .set("user_id", gluesql::core::ast_builder::num(self.user_id))
            .set("role_id", gluesql::core::ast_builder::num(self.role_id))
            .set(
                "created_by",
                gluesql::core::ast_builder::num(self.created_by),
            )
            .set(
                "created_at",
                gluesql::core::ast_builder::timestamp(self.created_at.to_string()),
            )
            .execute(connection)
            .await
            .map(|payload| match payload {
                gluesql::prelude::Payload::Update(number_of_updated_rows) => number_of_updated_rows,
                _ => unreachable!("Expected Payload::Update"),
            })
            .map_err(crate::api::ApiError::from)
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
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all UsersUsersRoleRequest from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::UsersUsersRoleRequestFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("users_users_role_requests")
            .select()
            .filter(filter.map_or_else(
                || {
                    gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true))
                        .into()
                },
                |filter| filter.as_filter_expression(),
            ))
            .project("table_id, user_id, role_id, created_by, created_at")
            .order_by("created_at DESC, created_at DESC")
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .execute(connection)
            .await?;
        Ok(select_row
            .select()
            .unwrap()
            .map(Self::from_row)
            .collect::<Vec<_>>())
    }
    fn from_row(row: std::collections::HashMap<&str, &gluesql::prelude::Value>) -> Self {
        Self {
            table_id: match row.get("table_id").unwrap() {
                gluesql::prelude::Value::I32(table_id) => table_id.clone(),
                _ => unreachable!("Expected I32"),
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32"),
            },
            role_id: match row.get("role_id").unwrap() {
                gluesql::prelude::Value::I32(role_id) => role_id.clone(),
                _ => unreachable!("Expected I32"),
            },
            created_by: match row.get("created_by").unwrap() {
                gluesql::prelude::Value::I32(created_by) => created_by.clone(),
                _ => unreachable!("Expected I32"),
            },
            created_at: match row.get("created_at").unwrap() {
                gluesql::prelude::Value::Timestamp(created_at) => created_at.clone(),
                _ => unreachable!("Expected Timestamp"),
            },
        }
    }
}
