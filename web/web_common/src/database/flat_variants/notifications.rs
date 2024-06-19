//! This file is automatically generated, do not modify it directly.
use super::*;
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub operation: String,
    pub table_name: String,
    pub record: String,
    pub read: bool,
}

unsafe impl Send for Notification {}
unsafe impl Sync for Notification {}
impl Tabular for Notification {
    const TABLE: crate::database::Table = crate::database::Table::Notifications;
}
impl Describable for Notification {
    fn description(&self) -> Option<&str> {
        None
    }
}
impl Colorable for Notification {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for crate::database::flat_variants::Notification {
    type Filter = crate::database::filter_variants::NotificationFilter;
}
#[cfg(feature = "frontend")]
impl AllRecords for Notification {
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
impl Notification {
    /// Get the id attribute.
    pub fn get_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.id.as_ref()
    }

    /// Get the user_id attribute.
    pub fn get_user_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.user_id.as_ref()
    }

    /// Get the operation attribute.
    pub fn get_operation<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.operation.as_ref()
    }

    /// Get the table_name attribute.
    pub fn get_table_name<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.table_name.as_ref()
    }

    /// Get the record attribute.
    pub fn get_record<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.record.as_ref()
    }

    /// Whether the Notification is read.
    pub fn is_read<E>(&self) -> &E
    where
        bool: AsRef<E>,
    {
        self.read.as_ref()
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::num(self.user_id),
            gluesql::core::ast_builder::text(self.operation),
            gluesql::core::ast_builder::text(self.table_name),
            gluesql::core::ast_builder::text(self.record),
            (self.read.into()),
        ]
    }

    /// Insert the Notification into the database.
    ///
    /// * `connection` - The connection to the database.
    async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("notifications")
            .insert()
            .columns("id, user_id, operation, table_name, record, read")
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

    /// Get the Notification from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("notifications")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, user_id, operation, table_name, record, read")
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

    /// Delete the Notification from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Self::delete_from_id(self.id, connection).await
    }

    /// Delete the Notification from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    pub async fn delete_from_id<
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    >(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("notifications")
            .delete()
            .filter(col("id").eq(id.to_string()))
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
    async fn update<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        table("notifications")
            .update()
            .set("id", gluesql::core::ast_builder::num(self.id))
            .set("user_id", gluesql::core::ast_builder::num(self.user_id))
            .set(
                "operation",
                gluesql::core::ast_builder::text(self.operation),
            )
            .set(
                "table_name",
                gluesql::core::ast_builder::text(self.table_name),
            )
            .set("record", gluesql::core::ast_builder::text(self.record))
            .set("read", self.read)
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
        &self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        let number_of_rows = self.clone().update(connection).await?;
        if number_of_rows == 0 {
            self.clone().insert(connection).await
        } else {
            Ok(number_of_rows)
        }
    }
    /// Get all Notification from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::NotificationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("notifications")
            .select()
            .filter(filter.map_or_else(
                || {
                    gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true))
                        .into()
                },
                |filter| filter.as_filter_expression(),
            ))
            .project("id, user_id, operation, table_name, record, read")
            .order_by("read DESC")
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
            id: match row.get("id").unwrap() {
                gluesql::prelude::Value::I32(id) => id.clone(),
                _ => unreachable!("Expected I32"),
            },
            user_id: match row.get("user_id").unwrap() {
                gluesql::prelude::Value::I32(user_id) => user_id.clone(),
                _ => unreachable!("Expected I32"),
            },
            operation: match row.get("operation").unwrap() {
                gluesql::prelude::Value::Str(operation) => operation.clone(),
                _ => unreachable!("Expected Str"),
            },
            table_name: match row.get("table_name").unwrap() {
                gluesql::prelude::Value::Str(table_name) => table_name.clone(),
                _ => unreachable!("Expected Str"),
            },
            record: match row.get("record").unwrap() {
                gluesql::prelude::Value::Str(record) => record.clone(),
                _ => unreachable!("Expected Str"),
            },
            read: match row.get("read").unwrap() {
                gluesql::prelude::Value::Bool(read) => read.clone(),
                _ => unreachable!("Expected Bool"),
            },
        }
    }
}
