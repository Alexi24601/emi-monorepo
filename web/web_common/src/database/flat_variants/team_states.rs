//! This file is automatically generated, do not modify it directly.
use super::*;
#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

unsafe impl Send for TeamState {}
unsafe impl Sync for TeamState {}
impl Tabular for TeamState {
    const TABLE: crate::database::Table = crate::database::Table::TeamStates;
}
impl Describable for TeamState {
    fn description(&self) -> Option<&str> {
        Some(self.description.as_str())
    }
}
impl Colorable for TeamState {
    fn color(&self) -> Option<&str> {
        None
    }
}

impl Filtrable for crate::database::flat_variants::TeamState {
    type Filter = crate::database::filter_variants::TeamStateFilter;
}
#[cfg(feature = "frontend")]
impl AllRecords for TeamState {
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
impl TeamState {
    /// Get the id attribute.
    pub fn get_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.id.as_ref()
    }

    /// Get the name attribute.
    pub fn get_name<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.name.as_ref()
    }

    /// Get the description attribute.
    pub fn get_description<E>(&self) -> &E
    where
        String: AsRef<E>,
    {
        self.description.as_ref()
    }

    /// Get the icon_id attribute.
    pub fn get_icon_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.icon_id.as_ref()
    }

    /// Get the color_id attribute.
    pub fn get_color_id<E>(&self) -> &E
    where
        i32: AsRef<E>,
    {
        self.color_id.as_ref()
    }

    pub fn into_row(self) -> Vec<gluesql::core::ast_builder::ExprNode<'static>> {
        vec![
            gluesql::core::ast_builder::num(self.id),
            gluesql::core::ast_builder::text(self.name),
            gluesql::core::ast_builder::text(self.description),
            gluesql::core::ast_builder::num(self.icon_id),
            gluesql::core::ast_builder::num(self.color_id),
        ]
    }

    /// Insert the TeamState into the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn insert<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        Ok(table("team_states")
            .insert()
            .columns("id, name, description, icon_id, color_id")
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

    /// Get the TeamState from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `connection` - The connection to the database.
    pub async fn get<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("team_states")
            .select()
            .filter(col("id").eq(id.to_string()))
            .project("id, name, description, icon_id, color_id")
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

    /// Delete the TeamState from the database.
    ///
    /// * `connection` - The connection to the database.
    pub async fn delete<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        Self::delete_from_id(self.id, connection).await
    }

    /// Delete the TeamState from the database by its ID.
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
        Ok(table("team_states")
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
    pub async fn update<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        self,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        table("team_states")
            .update()
            .set("id", gluesql::core::ast_builder::num(self.id))
            .set("name", gluesql::core::ast_builder::text(self.name))
            .set(
                "description",
                gluesql::core::ast_builder::text(self.description),
            )
            .set("icon_id", gluesql::core::ast_builder::num(self.icon_id))
            .set("color_id", gluesql::core::ast_builder::num(self.color_id))
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
    /// Get all TeamState from the database.
    ///
    /// * `filter` - The filter to apply to the results.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `offset` - The offset of the results, by default `0`.
    /// * `connection` - The connection to the database.
    pub async fn all<C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut>(
        filter: Option<&crate::database::filter_variants::TeamStateFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Self>, crate::api::ApiError> {
        use gluesql::core::ast_builder::*;
        let select_row = table("team_states")
            .select()
            .filter(filter.map_or_else(
                || {
                    gluesql::core::ast::Expr::Literal(gluesql::core::ast::AstLiteral::Boolean(true))
                        .into()
                },
                |filter| filter.as_filter_expression(),
            ))
            .project("id, name, description, icon_id, color_id")
            .order_by("color_id DESC")
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
            name: match row.get("name").unwrap() {
                gluesql::prelude::Value::Str(name) => name.clone(),
                _ => unreachable!("Expected Str"),
            },
            description: match row.get("description").unwrap() {
                gluesql::prelude::Value::Str(description) => description.clone(),
                _ => unreachable!("Expected Str"),
            },
            icon_id: match row.get("icon_id").unwrap() {
                gluesql::prelude::Value::I32(icon_id) => icon_id.clone(),
                _ => unreachable!("Expected I32"),
            },
            color_id: match row.get("color_id").unwrap() {
                gluesql::prelude::Value::I32(color_id) => color_id.clone(),
                _ => unreachable!("Expected I32"),
            },
        }
    }
}
