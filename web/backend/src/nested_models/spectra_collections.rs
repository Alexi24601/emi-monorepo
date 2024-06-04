use super::*;
use crate::models::*;
use serde::Deserialize;
use serde::Serialize;
use std::rc::Rc;
use web_common::database::filter_structs::*;

#[derive(PartialEq, PartialOrd, Debug, Clone, Serialize, Deserialize, Default)]
pub struct NestedSpectraCollection {
    pub inner: SpectraCollection,
    pub sample: NestedSample,
    pub created_by: User,
    pub updated_by: User,
}

impl NestedSpectraCollection {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `author_user_id` - The author user id.
    /// * `connection` - The database connection.
    pub fn from_flat(
        flat_variant: SpectraCollection,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        Ok(Self {
            sample: NestedSample::get(flat_variant.sample_id, author_user_id, connection)?,
            created_by: User::get(flat_variant.created_by, connection)?,
            updated_by: User::get(flat_variant.updated_by, connection)?,
            inner: flat_variant,
        })
    }
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view(
        &self,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_view(author_user_id, connection)
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view_by_id(
        id: i32,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        SpectraCollection::can_view_by_id(id, author_user_id, connection)
    }
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SpectraCollection::all_viewable(filter, author_user_id, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, author_user_id, connection))
            .collect()
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable_sorted(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SpectraCollection::all_viewable_sorted(filter, author_user_id, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, author_user_id, connection))
            .collect()
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn get(
        id: i32,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        SpectraCollection::get(id, author_user_id, connection)
            .and_then(|flat_variant| Self::from_flat(flat_variant, author_user_id, connection))
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_viewable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: Option<i32>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SpectraCollection::strict_word_similarity_search_viewable(
            filter,
            author_user_id,
            query,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, author_user_id, connection))
        .collect()
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_with_score_viewable(
        author_user_id: Option<i32>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<(Self, f32)>, web_common::api::ApiError> {
        SpectraCollection::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|(flat_variant, score)| {
            Ok((
                Self::from_flat(flat_variant, author_user_id, connection)?,
                score,
            ))
        })
        .collect()
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update(
        &self,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_update(author_user_id, connection)
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        SpectraCollection::can_update_by_id(id, author_user_id, connection)
    }
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_updatable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SpectraCollection::all_updatable(filter, author_user_id, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, Some(author_user_id), connection))
            .collect()
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_updatable_sorted(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SpectraCollection::all_updatable_sorted(filter, author_user_id, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, Some(author_user_id), connection))
            .collect()
    }
    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_updatable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SpectraCollection::strict_word_similarity_search_updatable(
            filter,
            author_user_id,
            query,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, Some(author_user_id), connection))
        .collect()
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin(
        &self,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_admin(author_user_id, connection)
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        SpectraCollection::can_admin_by_id(id, author_user_id, connection)
    }
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_administrable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SpectraCollection::all_administrable(filter, author_user_id, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, Some(author_user_id), connection))
            .collect()
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_administrable_sorted(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SpectraCollection::all_administrable_sorted(
            filter,
            author_user_id,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, Some(author_user_id), connection))
        .collect()
    }
    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_administrable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        SpectraCollection::strict_word_similarity_search_administrable(
            filter,
            author_user_id,
            query,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, Some(author_user_id), connection))
        .collect()
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete(
        &self,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        self.inner.delete(author_user_id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        SpectraCollection::delete_by_id(id, author_user_id, connection)
    }
}
impl From<web_common::database::nested_models::NestedSpectraCollection>
    for NestedSpectraCollection
{
    fn from(item: web_common::database::nested_models::NestedSpectraCollection) -> Self {
        Self {
            inner: SpectraCollection::from(item.inner.as_ref().clone()),
            sample: NestedSample::from(item.sample.as_ref().clone()),
            created_by: User::from(item.created_by.as_ref().clone()),
            updated_by: User::from(item.updated_by.as_ref().clone()),
        }
    }
}
impl From<NestedSpectraCollection>
    for web_common::database::nested_models::NestedSpectraCollection
{
    fn from(item: NestedSpectraCollection) -> Self {
        Self {
            inner: Rc::from(web_common::database::SpectraCollection::from(item.inner)),
            sample: Rc::from(web_common::database::NestedSample::from(item.sample)),
            created_by: Rc::from(web_common::database::User::from(item.created_by)),
            updated_by: Rc::from(web_common::database::User::from(item.updated_by)),
        }
    }
}
