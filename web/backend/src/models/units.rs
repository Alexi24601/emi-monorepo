//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use crate::schema::*;
use crate::sql_function_bindings::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::Identifiable;
use diesel::Insertable;
use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use web_common::database::filter_structs::*;

#[derive(
    Queryable,
    Debug,
    Identifiable,
    Eq,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
    Default,
    QueryableByName,
    Insertable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = units)]
#[diesel(primary_key(id))]
pub struct Unit {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub symbol: String,
}

impl From<Unit> for web_common::database::tables::Unit {
    fn from(item: Unit) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            symbol: item.symbol,
        }
    }
}

impl From<web_common::database::tables::Unit> for Unit {
    fn from(item: web_common::database::tables::Unit) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            symbol: item.symbol,
        }
    }
}

impl Unit {
    /// Check whether the user can view the struct.
    pub fn can_view(&self) -> Result<bool, web_common::api::ApiError> {
        Ok(true)
    }
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id() -> Result<bool, web_common::api::ApiError> {
        Ok(true)
    }
    /// Get all of the viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::units;
        units::dsl::units
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable_sorted(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::units;
        units::dsl::units
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::schema::units;
        units::dsl::units
            .filter(units::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::schema::units;
        let flat_variant = units::dsl::units
            .filter(units::dsl::name.eq(name))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn similarity_search_viewable(
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::units;
        units::dsl::units
            .filter(similarity_op(
                concat_units_name_description_symbol(
                    units::dsl::name,
                    units::dsl::description,
                    units::dsl::symbol,
                ),
                query,
            ))
            .order(similarity_dist(
                concat_units_name_description_symbol(
                    units::dsl::name,
                    units::dsl::description,
                    units::dsl::symbol,
                ),
                query,
            ))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn word_similarity_search_viewable(
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::units;
        units::dsl::units
            .filter(word_similarity_op(
                concat_units_name_description_symbol(
                    units::dsl::name,
                    units::dsl::description,
                    units::dsl::symbol,
                ),
                query,
            ))
            .order(word_similarity_dist_op(
                concat_units_name_description_symbol(
                    units::dsl::name,
                    units::dsl::description,
                    units::dsl::symbol,
                ),
                query,
            ))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_viewable(
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::units;
        units::dsl::units
            .filter(strict_word_similarity_op(
                concat_units_name_description_symbol(
                    units::dsl::name,
                    units::dsl::description,
                    units::dsl::symbol,
                ),
                query,
            ))
            .order(strict_word_similarity_dist_op(
                concat_units_name_description_symbol(
                    units::dsl::name,
                    units::dsl::description,
                    units::dsl::symbol,
                ),
                query,
            ))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
}
