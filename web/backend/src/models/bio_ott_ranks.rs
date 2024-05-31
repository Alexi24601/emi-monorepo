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
    Associations,
    Insertable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = bio_ott_ranks)]
#[diesel(belongs_to(crate::models::font_awesome_icons::FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(crate::models::colors::Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct BioOttRank {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<BioOttRank> for web_common::database::tables::BioOttRank {
    fn from(item: BioOttRank) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::BioOttRank> for BioOttRank {
    fn from(item: web_common::database::tables::BioOttRank) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl BioOttRank {
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
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        filter: Option<&BioOttRankFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::bio_ott_ranks;
        let mut query = bio_ott_ranks::dsl::bio_ott_ranks.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_ranks::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_ranks::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable_sorted(
        filter: Option<&BioOttRankFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::bio_ott_ranks;
        let mut query = bio_ott_ranks::dsl::bio_ott_ranks.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_ranks::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_ranks::dsl::color_id.eq(color_id));
        }
        query
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
        use crate::schema::bio_ott_ranks;
        bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::id.eq(id))
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
        use crate::schema::bio_ott_ranks;
        let flat_variant = bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::name.eq(name))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn similarity_search_viewable(
        filter: Option<&BioOttRankFilter>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::bio_ott_ranks;
        if filter
            .map(|f| f.icon_id.is_some() && f.color_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            return bio_ott_ranks::dsl::bio_ott_ranks
                .filter(bio_ott_ranks::dsl::icon_id.eq(icon_id))
                .filter(similarity_op(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .order(similarity_dist(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            return bio_ott_ranks::dsl::bio_ott_ranks
                .filter(bio_ott_ranks::dsl::color_id.eq(color_id))
                .filter(similarity_op(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .order(similarity_dist(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        bio_ott_ranks::dsl::bio_ott_ranks
            .filter(similarity_op(
                concat_bio_ott_ranks_name_description(
                    bio_ott_ranks::dsl::name,
                    bio_ott_ranks::dsl::description,
                ),
                query,
            ))
            .order(similarity_dist(
                concat_bio_ott_ranks_name_description(
                    bio_ott_ranks::dsl::name,
                    bio_ott_ranks::dsl::description,
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
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn word_similarity_search_viewable(
        filter: Option<&BioOttRankFilter>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::bio_ott_ranks;
        if filter
            .map(|f| f.icon_id.is_some() && f.color_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            return bio_ott_ranks::dsl::bio_ott_ranks
                .filter(bio_ott_ranks::dsl::icon_id.eq(icon_id))
                .filter(word_similarity_op(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .order(word_similarity_dist_op(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            return bio_ott_ranks::dsl::bio_ott_ranks
                .filter(bio_ott_ranks::dsl::color_id.eq(color_id))
                .filter(word_similarity_op(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .order(word_similarity_dist_op(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        bio_ott_ranks::dsl::bio_ott_ranks
            .filter(word_similarity_op(
                concat_bio_ott_ranks_name_description(
                    bio_ott_ranks::dsl::name,
                    bio_ott_ranks::dsl::description,
                ),
                query,
            ))
            .order(word_similarity_dist_op(
                concat_bio_ott_ranks_name_description(
                    bio_ott_ranks::dsl::name,
                    bio_ott_ranks::dsl::description,
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
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_viewable(
        filter: Option<&BioOttRankFilter>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::bio_ott_ranks;
        if filter
            .map(|f| f.icon_id.is_some() && f.color_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            return bio_ott_ranks::dsl::bio_ott_ranks
                .filter(bio_ott_ranks::dsl::icon_id.eq(icon_id))
                .filter(strict_word_similarity_op(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .order(strict_word_similarity_dist_op(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            return bio_ott_ranks::dsl::bio_ott_ranks
                .filter(bio_ott_ranks::dsl::color_id.eq(color_id))
                .filter(strict_word_similarity_op(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .order(strict_word_similarity_dist_op(
                    concat_bio_ott_ranks_name_description(
                        bio_ott_ranks::dsl::name,
                        bio_ott_ranks::dsl::description,
                    ),
                    query,
                ))
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        bio_ott_ranks::dsl::bio_ott_ranks
            .filter(strict_word_similarity_op(
                concat_bio_ott_ranks_name_description(
                    bio_ott_ranks::dsl::name,
                    bio_ott_ranks::dsl::description,
                ),
                query,
            ))
            .order(strict_word_similarity_dist_op(
                concat_bio_ott_ranks_name_description(
                    bio_ott_ranks::dsl::name,
                    bio_ott_ranks::dsl::description,
                ),
                query,
            ))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
}
