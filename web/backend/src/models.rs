//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

#![allow(unused)]
#![allow(clippy::all)]

use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Identifiable;
use diesel::Insertable;
use crate::schema::*;
use crate::sql_function_bindings::*;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::prelude::*;
use web_common::database::filter_structs::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = bio_ott_ranks)]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
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
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&BioOttRankFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::bio_ott_ranks;
        let mut query = bio_ott_ranks::dsl::bio_ott_ranks
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_ranks::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_ranks::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&BioOttRankFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::bio_ott_ranks;
        let mut query = bio_ott_ranks::dsl::bio_ott_ranks
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_ranks::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_ranks::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::bio_ott_ranks;
        bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
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
    ///
    pub fn similarity_search_viewable(
filter: Option<&BioOttRankFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::bio_ott_ranks;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::icon_id.eq(icon_id))
            .filter(similarity_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .order_by(similarity_dist(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::color_id.eq(color_id))
            .filter(similarity_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .order_by(similarity_dist(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        bio_ott_ranks::dsl::bio_ott_ranks
            .filter(similarity_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .order_by(similarity_dist(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&BioOttRankFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::bio_ott_ranks;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::icon_id.eq(icon_id))
            .filter(word_similarity_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::color_id.eq(color_id))
            .filter(word_similarity_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        bio_ott_ranks::dsl::bio_ott_ranks
            .filter(word_similarity_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&BioOttRankFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::bio_ott_ranks;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::icon_id.eq(icon_id))
            .filter(strict_word_similarity_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::color_id.eq(color_id))
            .filter(strict_word_similarity_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        bio_ott_ranks::dsl::bio_ott_ranks
            .filter(strict_word_similarity_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_bio_ott_ranks_name_description(bio_ott_ranks::dsl::name, bio_ott_ranks::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = bio_ott_taxon_items)]
#[diesel(belongs_to(BioOttRank, foreign_key = ott_rank_id))]
#[diesel(belongs_to(BioOttTaxonItem, foreign_key = domain_id))]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct BioOttTaxonItem {
    pub id: i32,
    pub name: String,
    pub ott_id: i32,
    pub ott_rank_id: i32,
    pub wikidata_id: Option<i32>,
    pub ncbi_id: Option<i32>,
    pub gbif_id: Option<i32>,
    pub irmng_id: Option<i32>,
    pub worms_id: Option<i32>,
    pub domain_id: Option<i32>,
    pub kingdom_id: Option<i32>,
    pub phylum_id: Option<i32>,
    pub class_id: Option<i32>,
    pub order_id: Option<i32>,
    pub family_id: Option<i32>,
    pub genus_id: Option<i32>,
    pub parent_id: i32,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<BioOttTaxonItem> for web_common::database::tables::BioOttTaxonItem {
    fn from(item: BioOttTaxonItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            ott_id: item.ott_id,
            ott_rank_id: item.ott_rank_id,
            wikidata_id: item.wikidata_id,
            ncbi_id: item.ncbi_id,
            gbif_id: item.gbif_id,
            irmng_id: item.irmng_id,
            worms_id: item.worms_id,
            domain_id: item.domain_id,
            kingdom_id: item.kingdom_id,
            phylum_id: item.phylum_id,
            class_id: item.class_id,
            order_id: item.order_id,
            family_id: item.family_id,
            genus_id: item.genus_id,
            parent_id: item.parent_id,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::BioOttTaxonItem> for BioOttTaxonItem {
    fn from(item: web_common::database::tables::BioOttTaxonItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            ott_id: item.ott_id,
            ott_rank_id: item.ott_rank_id,
            wikidata_id: item.wikidata_id,
            ncbi_id: item.ncbi_id,
            gbif_id: item.gbif_id,
            irmng_id: item.irmng_id,
            worms_id: item.worms_id,
            domain_id: item.domain_id,
            kingdom_id: item.kingdom_id,
            phylum_id: item.phylum_id,
            class_id: item.class_id,
            order_id: item.order_id,
            family_id: item.family_id,
            genus_id: item.genus_id,
            parent_id: item.parent_id,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl BioOttTaxonItem {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&BioOttTaxonItemFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::bio_ott_taxon_items;
        let mut query = bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .into_boxed();
        if let Some(ott_rank_id) = filter.and_then(|f| f.ott_rank_id) {
            query = query.filter(bio_ott_taxon_items::dsl::ott_rank_id.eq(ott_rank_id));
        }
        if let Some(domain_id) = filter.and_then(|f| f.domain_id) {
            query = query.filter(bio_ott_taxon_items::dsl::domain_id.eq(domain_id));
        }
        if let Some(kingdom_id) = filter.and_then(|f| f.kingdom_id) {
            query = query.filter(bio_ott_taxon_items::dsl::kingdom_id.eq(kingdom_id));
        }
        if let Some(phylum_id) = filter.and_then(|f| f.phylum_id) {
            query = query.filter(bio_ott_taxon_items::dsl::phylum_id.eq(phylum_id));
        }
        if let Some(class_id) = filter.and_then(|f| f.class_id) {
            query = query.filter(bio_ott_taxon_items::dsl::class_id.eq(class_id));
        }
        if let Some(order_id) = filter.and_then(|f| f.order_id) {
            query = query.filter(bio_ott_taxon_items::dsl::order_id.eq(order_id));
        }
        if let Some(family_id) = filter.and_then(|f| f.family_id) {
            query = query.filter(bio_ott_taxon_items::dsl::family_id.eq(family_id));
        }
        if let Some(genus_id) = filter.and_then(|f| f.genus_id) {
            query = query.filter(bio_ott_taxon_items::dsl::genus_id.eq(genus_id));
        }
        if let Some(parent_id) = filter.and_then(|f| f.parent_id) {
            query = query.filter(bio_ott_taxon_items::dsl::parent_id.eq(parent_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_taxon_items::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_taxon_items::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&BioOttTaxonItemFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::bio_ott_taxon_items;
        let mut query = bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .into_boxed();
        if let Some(ott_rank_id) = filter.and_then(|f| f.ott_rank_id) {
            query = query.filter(bio_ott_taxon_items::dsl::ott_rank_id.eq(ott_rank_id));
        }
        if let Some(domain_id) = filter.and_then(|f| f.domain_id) {
            query = query.filter(bio_ott_taxon_items::dsl::domain_id.eq(domain_id));
        }
        if let Some(kingdom_id) = filter.and_then(|f| f.kingdom_id) {
            query = query.filter(bio_ott_taxon_items::dsl::kingdom_id.eq(kingdom_id));
        }
        if let Some(phylum_id) = filter.and_then(|f| f.phylum_id) {
            query = query.filter(bio_ott_taxon_items::dsl::phylum_id.eq(phylum_id));
        }
        if let Some(class_id) = filter.and_then(|f| f.class_id) {
            query = query.filter(bio_ott_taxon_items::dsl::class_id.eq(class_id));
        }
        if let Some(order_id) = filter.and_then(|f| f.order_id) {
            query = query.filter(bio_ott_taxon_items::dsl::order_id.eq(order_id));
        }
        if let Some(family_id) = filter.and_then(|f| f.family_id) {
            query = query.filter(bio_ott_taxon_items::dsl::family_id.eq(family_id));
        }
        if let Some(genus_id) = filter.and_then(|f| f.genus_id) {
            query = query.filter(bio_ott_taxon_items::dsl::genus_id.eq(genus_id));
        }
        if let Some(parent_id) = filter.and_then(|f| f.parent_id) {
            query = query.filter(bio_ott_taxon_items::dsl::parent_id.eq(parent_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_taxon_items::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_taxon_items::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::bio_ott_taxon_items;
        bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ott_id.
    ///
    /// * `ott_id` - The ott_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_ott_id(
ott_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::bio_ott_taxon_items;
        let flat_variant = bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::ott_id.eq(ott_id))
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
    ///
    pub fn similarity_search_viewable(
filter: Option<&BioOttTaxonItemFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::bio_ott_taxon_items;
 if filter.map(|f| f.ott_rank_id.is_some()&&f.parent_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(ott_rank_id) = filter.and_then(|f| f.ott_rank_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::ott_rank_id.eq(ott_rank_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(similarity_dist(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(parent_id) = filter.and_then(|f| f.parent_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::parent_id.eq(parent_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(similarity_dist(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::icon_id.eq(icon_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(similarity_dist(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::color_id.eq(color_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(similarity_dist(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(similarity_dist(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&BioOttTaxonItemFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::bio_ott_taxon_items;
 if filter.map(|f| f.ott_rank_id.is_some()&&f.parent_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(ott_rank_id) = filter.and_then(|f| f.ott_rank_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::ott_rank_id.eq(ott_rank_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(word_similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(parent_id) = filter.and_then(|f| f.parent_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::parent_id.eq(parent_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(word_similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::icon_id.eq(icon_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(word_similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::color_id.eq(color_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(word_similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(word_similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&BioOttTaxonItemFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::bio_ott_taxon_items;
 if filter.map(|f| f.ott_rank_id.is_some()&&f.parent_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(ott_rank_id) = filter.and_then(|f| f.ott_rank_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::ott_rank_id.eq(ott_rank_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(parent_id) = filter.and_then(|f| f.parent_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::parent_id.eq(parent_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::icon_id.eq(icon_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::color_id.eq(color_id))
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query))
            .order_by(strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = colors)]
#[diesel(primary_key(id))]
pub struct Color {
    pub id: i32,
    pub name: String,
    pub hexadecimal_value: String,
    pub description: String,
}

impl From<Color> for web_common::database::tables::Color {
    fn from(item: Color) -> Self {
        Self {
            id: item.id,
            name: item.name,
            hexadecimal_value: item.hexadecimal_value,
            description: item.description,
        }
    }
}

impl From<web_common::database::tables::Color> for Color {
    fn from(item: web_common::database::tables::Color) -> Self {
        Self {
            id: item.id,
            name: item.name,
            hexadecimal_value: item.hexadecimal_value,
            description: item.description,
        }
    }
}

impl Color {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::colors;
        colors::dsl::colors
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::colors;
        colors::dsl::colors
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::colors;
        colors::dsl::colors
            .filter(colors::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its hexadecimal_value.
    ///
    /// * `hexadecimal_value` - The hexadecimal_value of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_hexadecimal_value(
hexadecimal_value: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::colors;
        let flat_variant = colors::dsl::colors
            .filter(colors::dsl::hexadecimal_value.eq(hexadecimal_value))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::colors;
        let flat_variant = colors::dsl::colors
            .filter(colors::dsl::name.eq(name))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::colors;
        colors::dsl::colors
            .filter(similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(similarity_dist(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::colors;
        colors::dsl::colors
            .filter(word_similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::colors;
        colors::dsl::colors
            .filter(strict_word_similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = countries)]
#[diesel(primary_key(id))]
pub struct Country {
    pub id: i32,
    pub iso: String,
    pub emoji: String,
    pub unicode: String,
    pub name: String,
}

impl From<Country> for web_common::database::tables::Country {
    fn from(item: Country) -> Self {
        Self {
            id: item.id,
            iso: item.iso,
            emoji: item.emoji,
            unicode: item.unicode,
            name: item.name,
        }
    }
}

impl From<web_common::database::tables::Country> for Country {
    fn from(item: web_common::database::tables::Country) -> Self {
        Self {
            id: item.id,
            iso: item.iso,
            emoji: item.emoji,
            unicode: item.unicode,
            name: item.name,
        }
    }
}

impl Country {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::countries;
        countries::dsl::countries
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::countries;
        countries::dsl::countries
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::countries;
        countries::dsl::countries
            .filter(countries::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its emoji.
    ///
    /// * `emoji` - The emoji of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_emoji(
emoji: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::countries;
        let flat_variant = countries::dsl::countries
            .filter(countries::dsl::emoji.eq(emoji))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its iso.
    ///
    /// * `iso` - The iso of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_iso(
iso: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::countries;
        let flat_variant = countries::dsl::countries
            .filter(countries::dsl::iso.eq(iso))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::countries;
        let flat_variant = countries::dsl::countries
            .filter(countries::dsl::name.eq(name))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its unicode.
    ///
    /// * `unicode` - The unicode of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_unicode(
unicode: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::countries;
        let flat_variant = countries::dsl::countries
            .filter(countries::dsl::unicode.eq(unicode))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::countries;
        countries::dsl::countries
            .filter(similarity_op(countries::dsl::name, query))
            .order_by(similarity_dist(countries::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::countries;
        countries::dsl::countries
            .filter(word_similarity_op(countries::dsl::name, query))
            .order_by(word_similarity_dist_op(countries::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::countries;
        countries::dsl::countries
            .filter(strict_word_similarity_op(countries::dsl::name, query))
            .order_by(strict_word_similarity_dist_op(countries::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = derived_samples)]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(belongs_to(Sample, foreign_key = parent_sample_id))]
#[diesel(primary_key(parent_sample_id, child_sample_id))]
pub struct DerivedSample {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub parent_sample_id: Uuid,
    pub child_sample_id: Uuid,
}

impl From<DerivedSample> for web_common::database::tables::DerivedSample {
    fn from(item: DerivedSample) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            parent_sample_id: item.parent_sample_id,
            child_sample_id: item.child_sample_id,
        }
    }
}

impl From<web_common::database::tables::DerivedSample> for DerivedSample {
    fn from(item: web_common::database::tables::DerivedSample) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            parent_sample_id: item.parent_sample_id,
            child_sample_id: item.child_sample_id,
        }
    }
}

impl DerivedSample {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.parent_sample_id, self.child_sample_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( parent_sample_id, child_sample_id ): ( Uuid, Uuid ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_derived_samples(author_user_id, parent_sample_id, child_sample_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&DerivedSampleFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::derived_samples;
        let mut query = derived_samples::dsl::derived_samples
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        query
            .filter(can_view_derived_samples(author_user_id, derived_samples::dsl::parent_sample_id, derived_samples::dsl::child_sample_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&DerivedSampleFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::derived_samples;
        let mut query = derived_samples::dsl::derived_samples
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        query
            .filter(can_view_derived_samples(author_user_id, derived_samples::dsl::parent_sample_id, derived_samples::dsl::child_sample_id))
            .order_by(derived_samples::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( parent_sample_id, child_sample_id ): ( Uuid, Uuid ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( parent_sample_id, child_sample_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::derived_samples;
        derived_samples::dsl::derived_samples
            .filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id))
            .filter(derived_samples::dsl::child_sample_id.eq(child_sample_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.parent_sample_id, self.child_sample_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( parent_sample_id, child_sample_id ): ( Uuid, Uuid ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_derived_samples(author_user_id, parent_sample_id, child_sample_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&DerivedSampleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::derived_samples;
        let mut query = derived_samples::dsl::derived_samples
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        query
            .filter(can_update_derived_samples(author_user_id, derived_samples::dsl::parent_sample_id, derived_samples::dsl::child_sample_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&DerivedSampleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::derived_samples;
        let mut query = derived_samples::dsl::derived_samples
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        query
            .filter(can_update_derived_samples(author_user_id, derived_samples::dsl::parent_sample_id, derived_samples::dsl::child_sample_id))
            .order_by(derived_samples::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.parent_sample_id, self.child_sample_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( parent_sample_id, child_sample_id ): ( Uuid, Uuid ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_derived_samples(author_user_id, parent_sample_id, child_sample_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&DerivedSampleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::derived_samples;
        let mut query = derived_samples::dsl::derived_samples
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        query
            .filter(can_admin_derived_samples(author_user_id, derived_samples::dsl::parent_sample_id, derived_samples::dsl::child_sample_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&DerivedSampleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::derived_samples;
        let mut query = derived_samples::dsl::derived_samples
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        query
            .filter(can_admin_derived_samples(author_user_id, derived_samples::dsl::parent_sample_id, derived_samples::dsl::child_sample_id))
            .order_by(derived_samples::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.parent_sample_id, self.child_sample_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( parent_sample_id, child_sample_id ): ( Uuid, Uuid ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( parent_sample_id, child_sample_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(derived_samples::dsl::derived_samples
            .filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id))
            .filter(derived_samples::dsl::child_sample_id.eq(child_sample_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = document_formats)]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct DocumentFormat {
    pub id: i32,
    pub extension: String,
    pub mime_type: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<DocumentFormat> for web_common::database::tables::DocumentFormat {
    fn from(item: DocumentFormat) -> Self {
        Self {
            id: item.id,
            extension: item.extension,
            mime_type: item.mime_type,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::DocumentFormat> for DocumentFormat {
    fn from(item: web_common::database::tables::DocumentFormat) -> Self {
        Self {
            id: item.id,
            extension: item.extension,
            mime_type: item.mime_type,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl DocumentFormat {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&DocumentFormatFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::document_formats;
        let mut query = document_formats::dsl::document_formats
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(document_formats::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(document_formats::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&DocumentFormatFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::document_formats;
        let mut query = document_formats::dsl::document_formats
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(document_formats::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(document_formats::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::document_formats;
        document_formats::dsl::document_formats
            .filter(document_formats::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its extension.
    ///
    /// * `extension` - The extension of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_extension(
extension: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::document_formats;
        let flat_variant = document_formats::dsl::document_formats
            .filter(document_formats::dsl::extension.eq(extension))
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
    ///
    pub fn similarity_search_viewable(
filter: Option<&DocumentFormatFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::document_formats;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return document_formats::dsl::document_formats
            .filter(document_formats::dsl::icon_id.eq(icon_id))
            .filter(similarity_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .order_by(similarity_dist(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return document_formats::dsl::document_formats
            .filter(document_formats::dsl::color_id.eq(color_id))
            .filter(similarity_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .order_by(similarity_dist(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        document_formats::dsl::document_formats
            .filter(similarity_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .order_by(similarity_dist(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&DocumentFormatFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::document_formats;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return document_formats::dsl::document_formats
            .filter(document_formats::dsl::icon_id.eq(icon_id))
            .filter(word_similarity_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .order_by(word_similarity_dist_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return document_formats::dsl::document_formats
            .filter(document_formats::dsl::color_id.eq(color_id))
            .filter(word_similarity_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .order_by(word_similarity_dist_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        document_formats::dsl::document_formats
            .filter(word_similarity_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .order_by(word_similarity_dist_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&DocumentFormatFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::document_formats;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return document_formats::dsl::document_formats
            .filter(document_formats::dsl::icon_id.eq(icon_id))
            .filter(strict_word_similarity_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .order_by(strict_word_similarity_dist_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return document_formats::dsl::document_formats
            .filter(document_formats::dsl::color_id.eq(color_id))
            .filter(strict_word_similarity_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .order_by(strict_word_similarity_dist_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        document_formats::dsl::document_formats
            .filter(strict_word_similarity_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .order_by(strict_word_similarity_dist_op(concat_document_formats_extension_mime_type(document_formats::dsl::extension, document_formats::dsl::mime_type), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = font_awesome_icons)]
#[diesel(primary_key(id))]
pub struct FontAwesomeIcon {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl From<FontAwesomeIcon> for web_common::database::tables::FontAwesomeIcon {
    fn from(item: FontAwesomeIcon) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
        }
    }
}

impl From<web_common::database::tables::FontAwesomeIcon> for FontAwesomeIcon {
    fn from(item: web_common::database::tables::FontAwesomeIcon) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
        }
    }
}

impl FontAwesomeIcon {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::font_awesome_icons;
        font_awesome_icons::dsl::font_awesome_icons
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::font_awesome_icons;
        font_awesome_icons::dsl::font_awesome_icons
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::font_awesome_icons;
        font_awesome_icons::dsl::font_awesome_icons
            .filter(font_awesome_icons::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::font_awesome_icons;
        let flat_variant = font_awesome_icons::dsl::font_awesome_icons
            .filter(font_awesome_icons::dsl::name.eq(name))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::font_awesome_icons;
        font_awesome_icons::dsl::font_awesome_icons
            .filter(similarity_op(concat_font_awesome_icons_name(font_awesome_icons::dsl::name, font_awesome_icons::dsl::description), query))
            .order_by(similarity_dist(concat_font_awesome_icons_name(font_awesome_icons::dsl::name, font_awesome_icons::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::font_awesome_icons;
        font_awesome_icons::dsl::font_awesome_icons
            .filter(word_similarity_op(concat_font_awesome_icons_name(font_awesome_icons::dsl::name, font_awesome_icons::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_font_awesome_icons_name(font_awesome_icons::dsl::name, font_awesome_icons::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::font_awesome_icons;
        font_awesome_icons::dsl::font_awesome_icons
            .filter(strict_word_similarity_op(concat_font_awesome_icons_name(font_awesome_icons::dsl::name, font_awesome_icons::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_font_awesome_icons_name(font_awesome_icons::dsl::name, font_awesome_icons::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = login_providers)]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct LoginProvider {
    pub id: i32,
    pub name: String,
    pub icon_id: i32,
    pub color_id: i32,
    pub client_id_var_name: String,
    pub redirect_uri_var_name: String,
    pub oauth_url: String,
    pub scope: String,
}

impl From<LoginProvider> for web_common::database::tables::LoginProvider {
    fn from(item: LoginProvider) -> Self {
        Self {
            id: item.id,
            name: item.name,
            icon_id: item.icon_id,
            color_id: item.color_id,
            client_id_var_name: item.client_id_var_name,
            redirect_uri_var_name: item.redirect_uri_var_name,
            oauth_url: item.oauth_url,
            scope: item.scope,
        }
    }
}

impl From<web_common::database::tables::LoginProvider> for LoginProvider {
    fn from(item: web_common::database::tables::LoginProvider) -> Self {
        Self {
            id: item.id,
            name: item.name,
            icon_id: item.icon_id,
            color_id: item.color_id,
            client_id_var_name: item.client_id_var_name,
            redirect_uri_var_name: item.redirect_uri_var_name,
            oauth_url: item.oauth_url,
            scope: item.scope,
        }
    }
}

impl LoginProvider {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&LoginProviderFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::login_providers;
        let mut query = login_providers::dsl::login_providers
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(login_providers::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(login_providers::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&LoginProviderFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::login_providers;
        let mut query = login_providers::dsl::login_providers
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(login_providers::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(login_providers::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::login_providers;
        login_providers::dsl::login_providers
            .filter(login_providers::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_color_id(
color_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::login_providers;
        let flat_variant = login_providers::dsl::login_providers
            .filter(login_providers::dsl::color_id.eq(color_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_icon_id(
icon_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::login_providers;
        let flat_variant = login_providers::dsl::login_providers
            .filter(login_providers::dsl::icon_id.eq(icon_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::login_providers;
        let flat_variant = login_providers::dsl::login_providers
            .filter(login_providers::dsl::name.eq(name))
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
    ///
    pub fn similarity_search_viewable(
filter: Option<&LoginProviderFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::login_providers;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return login_providers::dsl::login_providers
            .filter(login_providers::dsl::icon_id.eq(icon_id))
            .select(LoginProvider::as_select())
            .inner_join(colors::dsl::colors.on(login_providers::dsl::color_id.eq(colors::dsl::id)))
            .filter(similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(similarity_dist(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return login_providers::dsl::login_providers
            .filter(login_providers::dsl::color_id.eq(color_id))
            .select(LoginProvider::as_select())
            .inner_join(colors::dsl::colors.on(login_providers::dsl::color_id.eq(colors::dsl::id)))
            .filter(similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(similarity_dist(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        login_providers::dsl::login_providers
            .select(LoginProvider::as_select())
            .inner_join(colors::dsl::colors.on(login_providers::dsl::color_id.eq(colors::dsl::id)))
            .filter(similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(similarity_dist(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&LoginProviderFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::login_providers;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return login_providers::dsl::login_providers
            .filter(login_providers::dsl::icon_id.eq(icon_id))
            .select(LoginProvider::as_select())
            .inner_join(colors::dsl::colors.on(login_providers::dsl::color_id.eq(colors::dsl::id)))
            .filter(word_similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return login_providers::dsl::login_providers
            .filter(login_providers::dsl::color_id.eq(color_id))
            .select(LoginProvider::as_select())
            .inner_join(colors::dsl::colors.on(login_providers::dsl::color_id.eq(colors::dsl::id)))
            .filter(word_similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        login_providers::dsl::login_providers
            .select(LoginProvider::as_select())
            .inner_join(colors::dsl::colors.on(login_providers::dsl::color_id.eq(colors::dsl::id)))
            .filter(word_similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&LoginProviderFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::login_providers;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return login_providers::dsl::login_providers
            .filter(login_providers::dsl::icon_id.eq(icon_id))
            .select(LoginProvider::as_select())
            .inner_join(colors::dsl::colors.on(login_providers::dsl::color_id.eq(colors::dsl::id)))
            .filter(strict_word_similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return login_providers::dsl::login_providers
            .filter(login_providers::dsl::color_id.eq(color_id))
            .select(LoginProvider::as_select())
            .inner_join(colors::dsl::colors.on(login_providers::dsl::color_id.eq(colors::dsl::id)))
            .filter(strict_word_similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        login_providers::dsl::login_providers
            .select(LoginProvider::as_select())
            .inner_join(colors::dsl::colors.on(login_providers::dsl::color_id.eq(colors::dsl::id)))
            .filter(strict_word_similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = materials)]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct Material {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

impl From<Material> for web_common::database::tables::Material {
    fn from(item: Material) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::Material> for Material {
    fn from(item: web_common::database::tables::Material) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl Material {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&MaterialFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::materials;
        let mut query = materials::dsl::materials
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(materials::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(materials::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&MaterialFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::materials;
        let mut query = materials::dsl::materials
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(materials::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(materials::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::materials;
        materials::dsl::materials
            .filter(materials::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::materials;
        let flat_variant = materials::dsl::materials
            .filter(materials::dsl::name.eq(name))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = notifications)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(primary_key(id))]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub operation: String,
    pub table_name: String,
    pub record: String,
    pub read: bool,
}

impl From<Notification> for web_common::database::tables::Notification {
    fn from(item: Notification) -> Self {
        Self {
            id: item.id,
            user_id: item.user_id,
            operation: item.operation,
            table_name: item.table_name,
            record: item.record,
            read: item.read,
        }
    }
}

impl From<web_common::database::tables::Notification> for Notification {
    fn from(item: web_common::database::tables::Notification) -> Self {
        Self {
            id: item.id,
            user_id: item.user_id,
            operation: item.operation,
            table_name: item.table_name,
            record: item.record,
            read: item.read,
        }
    }
}

impl Notification {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&NotificationFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::notifications;
        let mut query = notifications::dsl::notifications
            .into_boxed();
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(notifications::dsl::user_id.eq(user_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&NotificationFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::notifications;
        let mut query = notifications::dsl::notifications
            .into_boxed();
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(notifications::dsl::user_id.eq(user_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::notifications;
        notifications::dsl::notifications
            .filter(notifications::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = observations)]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(belongs_to(SampledIndividual, foreign_key = individual_id))]
#[diesel(primary_key(id))]
pub struct Observation {
    pub id: Uuid,
    pub created_by: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_by: i32,
    pub updated_at: Option<NaiveDateTime>,
    pub project_id: i32,
    pub individual_id: Option<Uuid>,
    pub notes: Option<String>,
    pub picture: Vec<u8>,
}

impl From<Observation> for web_common::database::tables::Observation {
    fn from(item: Observation) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            project_id: item.project_id,
            individual_id: item.individual_id,
            notes: item.notes,
            picture: item.picture,
        }
    }
}

impl From<web_common::database::tables::Observation> for Observation {
    fn from(item: web_common::database::tables::Observation) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            project_id: item.project_id,
            individual_id: item.individual_id,
            notes: item.notes,
            picture: item.picture,
        }
    }
}

impl Observation {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
id: Uuid,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_observations(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&ObservationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::observations;
        let mut query = observations::dsl::observations
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(individual_id) = filter.and_then(|f| f.individual_id) {
            query = query.filter(observations::dsl::individual_id.eq(individual_id));
        }
        query
            .filter(can_view_observations(author_user_id, observations::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&ObservationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::observations;
        let mut query = observations::dsl::observations
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(individual_id) = filter.and_then(|f| f.individual_id) {
            query = query.filter(observations::dsl::individual_id.eq(individual_id));
        }
        query
            .filter(can_view_observations(author_user_id, observations::dsl::id))
            .order_by(observations::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: Uuid,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::observations;
        observations::dsl::observations
            .filter(observations::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
id: Uuid,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_observations(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&ObservationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::observations;
        let mut query = observations::dsl::observations
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(individual_id) = filter.and_then(|f| f.individual_id) {
            query = query.filter(observations::dsl::individual_id.eq(individual_id));
        }
        query
            .filter(can_update_observations(author_user_id, observations::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&ObservationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::observations;
        let mut query = observations::dsl::observations
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(individual_id) = filter.and_then(|f| f.individual_id) {
            query = query.filter(observations::dsl::individual_id.eq(individual_id));
        }
        query
            .filter(can_update_observations(author_user_id, observations::dsl::id))
            .order_by(observations::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
id: Uuid,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_observations(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&ObservationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::observations;
        let mut query = observations::dsl::observations
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(individual_id) = filter.and_then(|f| f.individual_id) {
            query = query.filter(observations::dsl::individual_id.eq(individual_id));
        }
        query
            .filter(can_admin_observations(author_user_id, observations::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&ObservationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::observations;
        let mut query = observations::dsl::observations
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(individual_id) = filter.and_then(|f| f.individual_id) {
            query = query.filter(observations::dsl::individual_id.eq(individual_id));
        }
        query
            .filter(can_admin_observations(author_user_id, observations::dsl::id))
            .order_by(observations::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(self.id, author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
id: Uuid,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(observations::dsl::observations
            .filter(observations::dsl::id.eq(id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = organizations)]
#[diesel(belongs_to(Country, foreign_key = country_id))]
#[diesel(primary_key(id))]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub country_id: i32,
    pub state_province: Option<String>,
    pub domain: String,
}

impl From<Organization> for web_common::database::tables::Organization {
    fn from(item: Organization) -> Self {
        Self {
            id: item.id,
            name: item.name,
            url: item.url,
            country_id: item.country_id,
            state_province: item.state_province,
            domain: item.domain,
        }
    }
}

impl From<web_common::database::tables::Organization> for Organization {
    fn from(item: web_common::database::tables::Organization) -> Self {
        Self {
            id: item.id,
            name: item.name,
            url: item.url,
            country_id: item.country_id,
            state_province: item.state_province,
            domain: item.domain,
        }
    }
}

impl Organization {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&OrganizationFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::organizations;
        let mut query = organizations::dsl::organizations
            .into_boxed();
        if let Some(country_id) = filter.and_then(|f| f.country_id) {
            query = query.filter(organizations::dsl::country_id.eq(country_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&OrganizationFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::organizations;
        let mut query = organizations::dsl::organizations
            .into_boxed();
        if let Some(country_id) = filter.and_then(|f| f.country_id) {
            query = query.filter(organizations::dsl::country_id.eq(country_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::organizations;
        organizations::dsl::organizations
            .filter(organizations::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its domain.
    ///
    /// * `domain` - The domain of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_domain(
domain: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::organizations;
        let flat_variant = organizations::dsl::organizations
            .filter(organizations::dsl::domain.eq(domain))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its name, country_id and state_province.
    ///
    /// * `name` - The name of the struct to get.
    /// * `country_id` - The country_id of the struct to get.
    /// * `state_province` - The state_province of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name_and_country_id_and_state_province(
name: &str,
country_id: &i32,
state_province: Option<&str>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::organizations;
        let flat_variant = organizations::dsl::organizations
            .filter(organizations::dsl::name.eq(name))
            .filter(organizations::dsl::country_id.eq(country_id))
            .filter(organizations::dsl::state_province.eq(state_province))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its url.
    ///
    /// * `url` - The url of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_url(
url: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::organizations;
        let flat_variant = organizations::dsl::organizations
            .filter(organizations::dsl::url.eq(url))
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
    ///
    pub fn similarity_search_viewable(
filter: Option<&OrganizationFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::organizations;
if let Some(country_id) = filter.and_then(|f| f.country_id) {
        return organizations::dsl::organizations
            .filter(organizations::dsl::country_id.eq(country_id))
            .filter(similarity_op(organizations::dsl::name, query))
            .order_by(similarity_dist(organizations::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        organizations::dsl::organizations
            .filter(similarity_op(organizations::dsl::name, query))
            .order_by(similarity_dist(organizations::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&OrganizationFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::organizations;
if let Some(country_id) = filter.and_then(|f| f.country_id) {
        return organizations::dsl::organizations
            .filter(organizations::dsl::country_id.eq(country_id))
            .filter(word_similarity_op(organizations::dsl::name, query))
            .order_by(word_similarity_dist_op(organizations::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        organizations::dsl::organizations
            .filter(word_similarity_op(organizations::dsl::name, query))
            .order_by(word_similarity_dist_op(organizations::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&OrganizationFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::organizations;
if let Some(country_id) = filter.and_then(|f| f.country_id) {
        return organizations::dsl::organizations
            .filter(organizations::dsl::country_id.eq(country_id))
            .filter(strict_word_similarity_op(organizations::dsl::name, query))
            .order_by(strict_word_similarity_dist_op(organizations::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        organizations::dsl::organizations
            .filter(strict_word_similarity_op(organizations::dsl::name, query))
            .order_by(strict_word_similarity_dist_op(organizations::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = project_states)]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct ProjectState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<ProjectState> for web_common::database::tables::ProjectState {
    fn from(item: ProjectState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::ProjectState> for ProjectState {
    fn from(item: web_common::database::tables::ProjectState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl ProjectState {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&ProjectStateFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::project_states;
        let mut query = project_states::dsl::project_states
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(project_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(project_states::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&ProjectStateFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::project_states;
        let mut query = project_states::dsl::project_states
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(project_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(project_states::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::project_states;
        project_states::dsl::project_states
            .filter(project_states::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_color_id(
color_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::project_states;
        let flat_variant = project_states::dsl::project_states
            .filter(project_states::dsl::color_id.eq(color_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_icon_id(
icon_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::project_states;
        let flat_variant = project_states::dsl::project_states
            .filter(project_states::dsl::icon_id.eq(icon_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::project_states;
        let flat_variant = project_states::dsl::project_states
            .filter(project_states::dsl::name.eq(name))
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
    ///
    pub fn similarity_search_viewable(
filter: Option<&ProjectStateFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::project_states;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return project_states::dsl::project_states
            .filter(project_states::dsl::icon_id.eq(icon_id))
            .filter(similarity_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .order_by(similarity_dist(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return project_states::dsl::project_states
            .filter(project_states::dsl::color_id.eq(color_id))
            .filter(similarity_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .order_by(similarity_dist(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        project_states::dsl::project_states
            .filter(similarity_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .order_by(similarity_dist(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&ProjectStateFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::project_states;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return project_states::dsl::project_states
            .filter(project_states::dsl::icon_id.eq(icon_id))
            .filter(word_similarity_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return project_states::dsl::project_states
            .filter(project_states::dsl::color_id.eq(color_id))
            .filter(word_similarity_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        project_states::dsl::project_states
            .filter(word_similarity_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&ProjectStateFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::project_states;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return project_states::dsl::project_states
            .filter(project_states::dsl::icon_id.eq(icon_id))
            .filter(strict_word_similarity_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return project_states::dsl::project_states
            .filter(project_states::dsl::color_id.eq(color_id))
            .filter(strict_word_similarity_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        project_states::dsl::project_states
            .filter(strict_word_similarity_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_project_states_name_description(project_states::dsl::name, project_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects)]
#[diesel(belongs_to(ProjectState, foreign_key = state_id))]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(belongs_to(Project, foreign_key = parent_project_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
    pub icon_id: i32,
    pub color_id: i32,
    pub parent_project_id: Option<i32>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

impl From<Project> for web_common::database::tables::Project {
    fn from(item: Project) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            public: item.public,
            state_id: item.state_id,
            icon_id: item.icon_id,
            color_id: item.color_id,
            parent_project_id: item.parent_project_id,
            budget: item.budget,
            expenses: item.expenses,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            expected_end_date: item.expected_end_date,
            end_date: item.end_date,
        }
    }
}

impl From<web_common::database::tables::Project> for Project {
    fn from(item: web_common::database::tables::Project) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            public: item.public,
            state_id: item.state_id,
            icon_id: item.icon_id,
            color_id: item.color_id,
            parent_project_id: item.parent_project_id,
            budget: item.budget,
            expenses: item.expenses,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            expected_end_date: item.expected_end_date,
            end_date: item.end_date,
        }
    }
}

impl Project {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
id: i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_projects(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&ProjectFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects;
        let mut query = projects::dsl::projects
            .into_boxed();
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(projects::dsl::state_id.eq(state_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(projects::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(projects::dsl::color_id.eq(color_id));
        }
        if let Some(parent_project_id) = filter.and_then(|f| f.parent_project_id) {
            query = query.filter(projects::dsl::parent_project_id.eq(parent_project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(projects::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&ProjectFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects;
        let mut query = projects::dsl::projects
            .into_boxed();
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(projects::dsl::state_id.eq(state_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(projects::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(projects::dsl::color_id.eq(color_id));
        }
        if let Some(parent_project_id) = filter.and_then(|f| f.parent_project_id) {
            query = query.filter(projects::dsl::parent_project_id.eq(parent_project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(projects::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .order_by(projects::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::projects;
        projects::dsl::projects
            .filter(projects::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::projects;
        let flat_variant = projects::dsl::projects
            .filter(projects::dsl::name.eq(name))
            .first::<Self>(connection)?;
        if !flat_variant.can_view(author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
filter: Option<&ProjectFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::projects;
 if filter.map(|f| f.state_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return projects::dsl::projects
            .filter(projects::dsl::state_id.eq(state_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return projects::dsl::projects
            .filter(projects::dsl::icon_id.eq(icon_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return projects::dsl::projects
            .filter(projects::dsl::color_id.eq(color_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return projects::dsl::projects
            .filter(projects::dsl::created_by.eq(created_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return projects::dsl::projects
            .filter(projects::dsl::updated_by.eq(updated_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        projects::dsl::projects
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&ProjectFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::projects;
 if filter.map(|f| f.state_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return projects::dsl::projects
            .filter(projects::dsl::state_id.eq(state_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return projects::dsl::projects
            .filter(projects::dsl::icon_id.eq(icon_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return projects::dsl::projects
            .filter(projects::dsl::color_id.eq(color_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return projects::dsl::projects
            .filter(projects::dsl::created_by.eq(created_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return projects::dsl::projects
            .filter(projects::dsl::updated_by.eq(updated_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        projects::dsl::projects
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&ProjectFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::projects;
 if filter.map(|f| f.state_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return projects::dsl::projects
            .filter(projects::dsl::state_id.eq(state_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return projects::dsl::projects
            .filter(projects::dsl::icon_id.eq(icon_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return projects::dsl::projects
            .filter(projects::dsl::color_id.eq(color_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return projects::dsl::projects
            .filter(projects::dsl::created_by.eq(created_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return projects::dsl::projects
            .filter(projects::dsl::updated_by.eq(updated_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        projects::dsl::projects
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_view_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_projects(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&ProjectFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects;
        let mut query = projects::dsl::projects
            .into_boxed();
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(projects::dsl::state_id.eq(state_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(projects::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(projects::dsl::color_id.eq(color_id));
        }
        if let Some(parent_project_id) = filter.and_then(|f| f.parent_project_id) {
            query = query.filter(projects::dsl::parent_project_id.eq(parent_project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(projects::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&ProjectFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects;
        let mut query = projects::dsl::projects
            .into_boxed();
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(projects::dsl::state_id.eq(state_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(projects::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(projects::dsl::color_id.eq(color_id));
        }
        if let Some(parent_project_id) = filter.and_then(|f| f.parent_project_id) {
            query = query.filter(projects::dsl::parent_project_id.eq(parent_project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(projects::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .order_by(projects::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_updatable(
filter: Option<&ProjectFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::projects;
 if filter.map(|f| f.state_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return projects::dsl::projects
            .filter(projects::dsl::state_id.eq(state_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return projects::dsl::projects
            .filter(projects::dsl::icon_id.eq(icon_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return projects::dsl::projects
            .filter(projects::dsl::color_id.eq(color_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return projects::dsl::projects
            .filter(projects::dsl::created_by.eq(created_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return projects::dsl::projects
            .filter(projects::dsl::updated_by.eq(updated_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        projects::dsl::projects
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_updatable(
filter: Option<&ProjectFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::projects;
 if filter.map(|f| f.state_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return projects::dsl::projects
            .filter(projects::dsl::state_id.eq(state_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return projects::dsl::projects
            .filter(projects::dsl::icon_id.eq(icon_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return projects::dsl::projects
            .filter(projects::dsl::color_id.eq(color_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return projects::dsl::projects
            .filter(projects::dsl::created_by.eq(created_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return projects::dsl::projects
            .filter(projects::dsl::updated_by.eq(updated_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        projects::dsl::projects
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_updatable(
filter: Option<&ProjectFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::projects;
 if filter.map(|f| f.state_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return projects::dsl::projects
            .filter(projects::dsl::state_id.eq(state_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return projects::dsl::projects
            .filter(projects::dsl::icon_id.eq(icon_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return projects::dsl::projects
            .filter(projects::dsl::color_id.eq(color_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return projects::dsl::projects
            .filter(projects::dsl::created_by.eq(created_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return projects::dsl::projects
            .filter(projects::dsl::updated_by.eq(updated_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        projects::dsl::projects
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_update_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_projects(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&ProjectFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects;
        let mut query = projects::dsl::projects
            .into_boxed();
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(projects::dsl::state_id.eq(state_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(projects::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(projects::dsl::color_id.eq(color_id));
        }
        if let Some(parent_project_id) = filter.and_then(|f| f.parent_project_id) {
            query = query.filter(projects::dsl::parent_project_id.eq(parent_project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(projects::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&ProjectFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects;
        let mut query = projects::dsl::projects
            .into_boxed();
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(projects::dsl::state_id.eq(state_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(projects::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(projects::dsl::color_id.eq(color_id));
        }
        if let Some(parent_project_id) = filter.and_then(|f| f.parent_project_id) {
            query = query.filter(projects::dsl::parent_project_id.eq(parent_project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(projects::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .order_by(projects::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_administrable(
filter: Option<&ProjectFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::projects;
 if filter.map(|f| f.state_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return projects::dsl::projects
            .filter(projects::dsl::state_id.eq(state_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return projects::dsl::projects
            .filter(projects::dsl::icon_id.eq(icon_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return projects::dsl::projects
            .filter(projects::dsl::color_id.eq(color_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return projects::dsl::projects
            .filter(projects::dsl::created_by.eq(created_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return projects::dsl::projects
            .filter(projects::dsl::updated_by.eq(updated_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        projects::dsl::projects
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_administrable(
filter: Option<&ProjectFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::projects;
 if filter.map(|f| f.state_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return projects::dsl::projects
            .filter(projects::dsl::state_id.eq(state_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return projects::dsl::projects
            .filter(projects::dsl::icon_id.eq(icon_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return projects::dsl::projects
            .filter(projects::dsl::color_id.eq(color_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return projects::dsl::projects
            .filter(projects::dsl::created_by.eq(created_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return projects::dsl::projects
            .filter(projects::dsl::updated_by.eq(updated_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        projects::dsl::projects
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_administrable(
filter: Option<&ProjectFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::projects;
 if filter.map(|f| f.state_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return projects::dsl::projects
            .filter(projects::dsl::state_id.eq(state_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return projects::dsl::projects
            .filter(projects::dsl::icon_id.eq(icon_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return projects::dsl::projects
            .filter(projects::dsl::color_id.eq(color_id))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return projects::dsl::projects
            .filter(projects::dsl::created_by.eq(created_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return projects::dsl::projects
            .filter(projects::dsl::updated_by.eq(updated_by))
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        projects::dsl::projects
            .filter(projects::dsl::parent_project_id.eq(filter.and_then(|f| f.parent_project_id)))
            .filter(can_admin_projects(author_user_id, projects::dsl::id))
            .filter(strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(self.id, author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(projects::dsl::projects
            .filter(projects::dsl::id.eq(id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_teams_role_invitations)]
#[diesel(belongs_to(Project, foreign_key = table_id))]
#[diesel(belongs_to(Team, foreign_key = team_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(table_id, team_id))]
pub struct ProjectsTeamsRoleInvitation {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsTeamsRoleInvitation> for web_common::database::tables::ProjectsTeamsRoleInvitation {
    fn from(item: ProjectsTeamsRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsTeamsRoleInvitation> for ProjectsTeamsRoleInvitation {
    fn from(item: web_common::database::tables::ProjectsTeamsRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsTeamsRoleInvitation {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_projects_teams_role_invitations(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&ProjectsTeamsRoleInvitationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_invitations;
        let mut query = projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_teams_role_invitations(author_user_id, projects_teams_role_invitations::dsl::table_id, projects_teams_role_invitations::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&ProjectsTeamsRoleInvitationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_invitations;
        let mut query = projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_teams_role_invitations(author_user_id, projects_teams_role_invitations::dsl::table_id, projects_teams_role_invitations::dsl::team_id))
            .order_by(projects_teams_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, team_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, team_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::projects_teams_role_invitations;
        projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .filter(projects_teams_role_invitations::dsl::table_id.eq(table_id))
            .filter(projects_teams_role_invitations::dsl::team_id.eq(team_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_projects_teams_role_invitations(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&ProjectsTeamsRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_invitations;
        let mut query = projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_teams_role_invitations(author_user_id, projects_teams_role_invitations::dsl::table_id, projects_teams_role_invitations::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&ProjectsTeamsRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_invitations;
        let mut query = projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_teams_role_invitations(author_user_id, projects_teams_role_invitations::dsl::table_id, projects_teams_role_invitations::dsl::team_id))
            .order_by(projects_teams_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_projects_teams_role_invitations(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&ProjectsTeamsRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_invitations;
        let mut query = projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_teams_role_invitations(author_user_id, projects_teams_role_invitations::dsl::table_id, projects_teams_role_invitations::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&ProjectsTeamsRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_invitations;
        let mut query = projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_teams_role_invitations(author_user_id, projects_teams_role_invitations::dsl::table_id, projects_teams_role_invitations::dsl::team_id))
            .order_by(projects_teams_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.team_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, team_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .filter(projects_teams_role_invitations::dsl::table_id.eq(table_id))
            .filter(projects_teams_role_invitations::dsl::team_id.eq(team_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_teams_role_requests)]
#[diesel(belongs_to(Project, foreign_key = table_id))]
#[diesel(belongs_to(Team, foreign_key = team_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(table_id, team_id))]
pub struct ProjectsTeamsRoleRequest {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsTeamsRoleRequest> for web_common::database::tables::ProjectsTeamsRoleRequest {
    fn from(item: ProjectsTeamsRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsTeamsRoleRequest> for ProjectsTeamsRoleRequest {
    fn from(item: web_common::database::tables::ProjectsTeamsRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsTeamsRoleRequest {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_projects_teams_role_requests(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&ProjectsTeamsRoleRequestFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_requests;
        let mut query = projects_teams_role_requests::dsl::projects_teams_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_requests::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_teams_role_requests(author_user_id, projects_teams_role_requests::dsl::table_id, projects_teams_role_requests::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&ProjectsTeamsRoleRequestFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_requests;
        let mut query = projects_teams_role_requests::dsl::projects_teams_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_requests::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_teams_role_requests(author_user_id, projects_teams_role_requests::dsl::table_id, projects_teams_role_requests::dsl::team_id))
            .order_by(projects_teams_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, team_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, team_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::projects_teams_role_requests;
        projects_teams_role_requests::dsl::projects_teams_role_requests
            .filter(projects_teams_role_requests::dsl::table_id.eq(table_id))
            .filter(projects_teams_role_requests::dsl::team_id.eq(team_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_projects_teams_role_requests(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&ProjectsTeamsRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_requests;
        let mut query = projects_teams_role_requests::dsl::projects_teams_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_requests::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_teams_role_requests(author_user_id, projects_teams_role_requests::dsl::table_id, projects_teams_role_requests::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&ProjectsTeamsRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_requests;
        let mut query = projects_teams_role_requests::dsl::projects_teams_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_requests::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_teams_role_requests(author_user_id, projects_teams_role_requests::dsl::table_id, projects_teams_role_requests::dsl::team_id))
            .order_by(projects_teams_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_projects_teams_role_requests(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&ProjectsTeamsRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_requests;
        let mut query = projects_teams_role_requests::dsl::projects_teams_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_requests::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_teams_role_requests(author_user_id, projects_teams_role_requests::dsl::table_id, projects_teams_role_requests::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&ProjectsTeamsRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_role_requests;
        let mut query = projects_teams_role_requests::dsl::projects_teams_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_requests::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_teams_role_requests(author_user_id, projects_teams_role_requests::dsl::table_id, projects_teams_role_requests::dsl::team_id))
            .order_by(projects_teams_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.team_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, team_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(projects_teams_role_requests::dsl::projects_teams_role_requests
            .filter(projects_teams_role_requests::dsl::table_id.eq(table_id))
            .filter(projects_teams_role_requests::dsl::team_id.eq(team_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_teams_roles)]
#[diesel(belongs_to(Project, foreign_key = table_id))]
#[diesel(belongs_to(Team, foreign_key = team_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(table_id, team_id))]
pub struct ProjectsTeamsRole {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsTeamsRole> for web_common::database::tables::ProjectsTeamsRole {
    fn from(item: ProjectsTeamsRole) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsTeamsRole> for ProjectsTeamsRole {
    fn from(item: web_common::database::tables::ProjectsTeamsRole) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsTeamsRole {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_projects_teams_roles(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&ProjectsTeamsRoleFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_roles;
        let mut query = projects_teams_roles::dsl::projects_teams_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_roles::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_roles::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_teams_roles(author_user_id, projects_teams_roles::dsl::table_id, projects_teams_roles::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&ProjectsTeamsRoleFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_roles;
        let mut query = projects_teams_roles::dsl::projects_teams_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_roles::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_roles::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_teams_roles(author_user_id, projects_teams_roles::dsl::table_id, projects_teams_roles::dsl::team_id))
            .order_by(projects_teams_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, team_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, team_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::projects_teams_roles;
        projects_teams_roles::dsl::projects_teams_roles
            .filter(projects_teams_roles::dsl::table_id.eq(table_id))
            .filter(projects_teams_roles::dsl::team_id.eq(team_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_projects_teams_roles(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&ProjectsTeamsRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_roles;
        let mut query = projects_teams_roles::dsl::projects_teams_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_roles::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_roles::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_teams_roles(author_user_id, projects_teams_roles::dsl::table_id, projects_teams_roles::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&ProjectsTeamsRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_roles;
        let mut query = projects_teams_roles::dsl::projects_teams_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_roles::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_roles::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_teams_roles(author_user_id, projects_teams_roles::dsl::table_id, projects_teams_roles::dsl::team_id))
            .order_by(projects_teams_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_projects_teams_roles(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&ProjectsTeamsRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_roles;
        let mut query = projects_teams_roles::dsl::projects_teams_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_roles::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_roles::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_teams_roles(author_user_id, projects_teams_roles::dsl::table_id, projects_teams_roles::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&ProjectsTeamsRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_teams_roles;
        let mut query = projects_teams_roles::dsl::projects_teams_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_roles::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_roles::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_teams_roles(author_user_id, projects_teams_roles::dsl::table_id, projects_teams_roles::dsl::team_id))
            .order_by(projects_teams_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.team_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, team_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(projects_teams_roles::dsl::projects_teams_roles
            .filter(projects_teams_roles::dsl::table_id.eq(table_id))
            .filter(projects_teams_roles::dsl::team_id.eq(team_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_users_role_invitations)]
#[diesel(belongs_to(Project, foreign_key = table_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct ProjectsUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsUsersRoleInvitation> for web_common::database::tables::ProjectsUsersRoleInvitation {
    fn from(item: ProjectsUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsUsersRoleInvitation> for ProjectsUsersRoleInvitation {
    fn from(item: web_common::database::tables::ProjectsUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsUsersRoleInvitation {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_projects_users_role_invitations(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&ProjectsUsersRoleInvitationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_invitations;
        let mut query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_users_role_invitations(author_user_id, projects_users_role_invitations::dsl::table_id, projects_users_role_invitations::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&ProjectsUsersRoleInvitationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_invitations;
        let mut query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_users_role_invitations(author_user_id, projects_users_role_invitations::dsl::table_id, projects_users_role_invitations::dsl::user_id))
            .order_by(projects_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::projects_users_role_invitations;
        projects_users_role_invitations::dsl::projects_users_role_invitations
            .filter(projects_users_role_invitations::dsl::table_id.eq(table_id))
            .filter(projects_users_role_invitations::dsl::user_id.eq(user_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_projects_users_role_invitations(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&ProjectsUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_invitations;
        let mut query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_users_role_invitations(author_user_id, projects_users_role_invitations::dsl::table_id, projects_users_role_invitations::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&ProjectsUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_invitations;
        let mut query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_users_role_invitations(author_user_id, projects_users_role_invitations::dsl::table_id, projects_users_role_invitations::dsl::user_id))
            .order_by(projects_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_projects_users_role_invitations(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&ProjectsUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_invitations;
        let mut query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_users_role_invitations(author_user_id, projects_users_role_invitations::dsl::table_id, projects_users_role_invitations::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&ProjectsUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_invitations;
        let mut query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_users_role_invitations(author_user_id, projects_users_role_invitations::dsl::table_id, projects_users_role_invitations::dsl::user_id))
            .order_by(projects_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.user_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(projects_users_role_invitations::dsl::projects_users_role_invitations
            .filter(projects_users_role_invitations::dsl::table_id.eq(table_id))
            .filter(projects_users_role_invitations::dsl::user_id.eq(user_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_users_role_requests)]
#[diesel(belongs_to(Project, foreign_key = table_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct ProjectsUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsUsersRoleRequest> for web_common::database::tables::ProjectsUsersRoleRequest {
    fn from(item: ProjectsUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsUsersRoleRequest> for ProjectsUsersRoleRequest {
    fn from(item: web_common::database::tables::ProjectsUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsUsersRoleRequest {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_projects_users_role_requests(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&ProjectsUsersRoleRequestFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_requests;
        let mut query = projects_users_role_requests::dsl::projects_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_users_role_requests(author_user_id, projects_users_role_requests::dsl::table_id, projects_users_role_requests::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&ProjectsUsersRoleRequestFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_requests;
        let mut query = projects_users_role_requests::dsl::projects_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_users_role_requests(author_user_id, projects_users_role_requests::dsl::table_id, projects_users_role_requests::dsl::user_id))
            .order_by(projects_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::projects_users_role_requests;
        projects_users_role_requests::dsl::projects_users_role_requests
            .filter(projects_users_role_requests::dsl::table_id.eq(table_id))
            .filter(projects_users_role_requests::dsl::user_id.eq(user_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_projects_users_role_requests(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&ProjectsUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_requests;
        let mut query = projects_users_role_requests::dsl::projects_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_users_role_requests(author_user_id, projects_users_role_requests::dsl::table_id, projects_users_role_requests::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&ProjectsUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_requests;
        let mut query = projects_users_role_requests::dsl::projects_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_users_role_requests(author_user_id, projects_users_role_requests::dsl::table_id, projects_users_role_requests::dsl::user_id))
            .order_by(projects_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_projects_users_role_requests(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&ProjectsUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_requests;
        let mut query = projects_users_role_requests::dsl::projects_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_users_role_requests(author_user_id, projects_users_role_requests::dsl::table_id, projects_users_role_requests::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&ProjectsUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_role_requests;
        let mut query = projects_users_role_requests::dsl::projects_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_users_role_requests(author_user_id, projects_users_role_requests::dsl::table_id, projects_users_role_requests::dsl::user_id))
            .order_by(projects_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.user_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(projects_users_role_requests::dsl::projects_users_role_requests
            .filter(projects_users_role_requests::dsl::table_id.eq(table_id))
            .filter(projects_users_role_requests::dsl::user_id.eq(user_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_users_roles)]
#[diesel(belongs_to(Project, foreign_key = table_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct ProjectsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsUsersRole> for web_common::database::tables::ProjectsUsersRole {
    fn from(item: ProjectsUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsUsersRole> for ProjectsUsersRole {
    fn from(item: web_common::database::tables::ProjectsUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsUsersRole {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_projects_users_roles(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&ProjectsUsersRoleFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_roles;
        let mut query = projects_users_roles::dsl::projects_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_users_roles(author_user_id, projects_users_roles::dsl::table_id, projects_users_roles::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&ProjectsUsersRoleFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_roles;
        let mut query = projects_users_roles::dsl::projects_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_projects_users_roles(author_user_id, projects_users_roles::dsl::table_id, projects_users_roles::dsl::user_id))
            .order_by(projects_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::projects_users_roles;
        projects_users_roles::dsl::projects_users_roles
            .filter(projects_users_roles::dsl::table_id.eq(table_id))
            .filter(projects_users_roles::dsl::user_id.eq(user_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_projects_users_roles(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&ProjectsUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_roles;
        let mut query = projects_users_roles::dsl::projects_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_users_roles(author_user_id, projects_users_roles::dsl::table_id, projects_users_roles::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&ProjectsUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_roles;
        let mut query = projects_users_roles::dsl::projects_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_projects_users_roles(author_user_id, projects_users_roles::dsl::table_id, projects_users_roles::dsl::user_id))
            .order_by(projects_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_projects_users_roles(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&ProjectsUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_roles;
        let mut query = projects_users_roles::dsl::projects_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_users_roles(author_user_id, projects_users_roles::dsl::table_id, projects_users_roles::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&ProjectsUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::projects_users_roles;
        let mut query = projects_users_roles::dsl::projects_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_projects_users_roles(author_user_id, projects_users_roles::dsl::table_id, projects_users_roles::dsl::user_id))
            .order_by(projects_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.user_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(projects_users_roles::dsl::projects_users_roles
            .filter(projects_users_roles::dsl::table_id.eq(table_id))
            .filter(projects_users_roles::dsl::user_id.eq(user_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = roles)]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<Role> for web_common::database::tables::Role {
    fn from(item: Role) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::Role> for Role {
    fn from(item: web_common::database::tables::Role) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl Role {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&RoleFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::roles;
        let mut query = roles::dsl::roles
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(roles::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(roles::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&RoleFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::roles;
        let mut query = roles::dsl::roles
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(roles::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(roles::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::roles;
        roles::dsl::roles
            .filter(roles::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_color_id(
color_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::roles;
        let flat_variant = roles::dsl::roles
            .filter(roles::dsl::color_id.eq(color_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its description.
    ///
    /// * `description` - The description of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_description(
description: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::roles;
        let flat_variant = roles::dsl::roles
            .filter(roles::dsl::description.eq(description))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_icon_id(
icon_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::roles;
        let flat_variant = roles::dsl::roles
            .filter(roles::dsl::icon_id.eq(icon_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::roles;
        let flat_variant = roles::dsl::roles
            .filter(roles::dsl::name.eq(name))
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
    ///
    pub fn similarity_search_viewable(
filter: Option<&RoleFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::roles;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return roles::dsl::roles
            .filter(roles::dsl::icon_id.eq(icon_id))
            .filter(similarity_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .order_by(similarity_dist(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return roles::dsl::roles
            .filter(roles::dsl::color_id.eq(color_id))
            .filter(similarity_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .order_by(similarity_dist(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        roles::dsl::roles
            .filter(similarity_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .order_by(similarity_dist(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&RoleFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::roles;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return roles::dsl::roles
            .filter(roles::dsl::icon_id.eq(icon_id))
            .filter(word_similarity_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return roles::dsl::roles
            .filter(roles::dsl::color_id.eq(color_id))
            .filter(word_similarity_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        roles::dsl::roles
            .filter(word_similarity_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&RoleFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::roles;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return roles::dsl::roles
            .filter(roles::dsl::icon_id.eq(icon_id))
            .filter(strict_word_similarity_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return roles::dsl::roles
            .filter(roles::dsl::color_id.eq(color_id))
            .filter(strict_word_similarity_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        roles::dsl::roles
            .filter(strict_word_similarity_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_roles_name(roles::dsl::name, roles::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_bio_ott_taxon_items)]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(belongs_to(Sample, foreign_key = sample_id))]
#[diesel(belongs_to(BioOttTaxonItem, foreign_key = taxon_id))]
#[diesel(primary_key(sample_id, taxon_id))]
pub struct SampleBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub sample_id: Uuid,
    pub taxon_id: i32,
}

impl From<SampleBioOttTaxonItem> for web_common::database::tables::SampleBioOttTaxonItem {
    fn from(item: SampleBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            sample_id: item.sample_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl From<web_common::database::tables::SampleBioOttTaxonItem> for SampleBioOttTaxonItem {
    fn from(item: web_common::database::tables::SampleBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            sample_id: item.sample_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl SampleBioOttTaxonItem {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.sample_id, self.taxon_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( sample_id, taxon_id ): ( Uuid, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_sample_bio_ott_taxon_items(author_user_id, sample_id, taxon_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .order_by(sample_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( sample_id, taxon_id ): ( Uuid, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( sample_id, taxon_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::sample_bio_ott_taxon_items;
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.sample_id, self.taxon_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( sample_id, taxon_id ): ( Uuid, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_sample_bio_ott_taxon_items(author_user_id, sample_id, taxon_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .order_by(sample_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.sample_id, self.taxon_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( sample_id, taxon_id ): ( Uuid, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_id, taxon_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .order_by(sample_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.sample_id, self.taxon_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( sample_id, taxon_id ): ( Uuid, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( sample_id, taxon_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_container_categories)]
#[diesel(belongs_to(Material, foreign_key = material_id))]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct SampleContainerCategory {
    pub id: i32,
    pub name: String,
    pub volume: f64,
    pub unit: String,
    pub material_id: i32,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<SampleContainerCategory> for web_common::database::tables::SampleContainerCategory {
    fn from(item: SampleContainerCategory) -> Self {
        Self {
            id: item.id,
            name: item.name,
            volume: item.volume,
            unit: item.unit,
            material_id: item.material_id,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::SampleContainerCategory> for SampleContainerCategory {
    fn from(item: web_common::database::tables::SampleContainerCategory) -> Self {
        Self {
            id: item.id,
            name: item.name,
            volume: item.volume,
            unit: item.unit,
            material_id: item.material_id,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl SampleContainerCategory {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&SampleContainerCategoryFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_container_categories;
        let mut query = sample_container_categories::dsl::sample_container_categories
            .into_boxed();
        if let Some(material_id) = filter.and_then(|f| f.material_id) {
            query = query.filter(sample_container_categories::dsl::material_id.eq(material_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(sample_container_categories::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(sample_container_categories::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&SampleContainerCategoryFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_container_categories;
        let mut query = sample_container_categories::dsl::sample_container_categories
            .into_boxed();
        if let Some(material_id) = filter.and_then(|f| f.material_id) {
            query = query.filter(sample_container_categories::dsl::material_id.eq(material_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(sample_container_categories::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(sample_container_categories::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::sample_container_categories;
        sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
filter: Option<&SampleContainerCategoryFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::sample_container_categories;
 if filter.map(|f| f.material_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(material_id) = filter.and_then(|f| f.material_id) {
        return sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::material_id.eq(material_id))
            .filter(similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(similarity_dist(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::icon_id.eq(icon_id))
            .filter(similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(similarity_dist(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::color_id.eq(color_id))
            .filter(similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(similarity_dist(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_container_categories::dsl::sample_container_categories
            .filter(similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(similarity_dist(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&SampleContainerCategoryFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::sample_container_categories;
 if filter.map(|f| f.material_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(material_id) = filter.and_then(|f| f.material_id) {
        return sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::material_id.eq(material_id))
            .filter(word_similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::icon_id.eq(icon_id))
            .filter(word_similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::color_id.eq(color_id))
            .filter(word_similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_container_categories::dsl::sample_container_categories
            .filter(word_similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&SampleContainerCategoryFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::sample_container_categories;
 if filter.map(|f| f.material_id.is_some()&&f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(material_id) = filter.and_then(|f| f.material_id) {
        return sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::material_id.eq(material_id))
            .filter(strict_word_similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::icon_id.eq(icon_id))
            .filter(strict_word_similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::color_id.eq(color_id))
            .filter(strict_word_similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_container_categories::dsl::sample_container_categories
            .filter(strict_word_similarity_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_sample_container_categories_brand(sample_container_categories::dsl::name, sample_container_categories::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_containers)]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(belongs_to(SampleContainerCategory, foreign_key = category_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct SampleContainer {
    pub id: i32,
    pub barcode: String,
    pub project_id: i32,
    pub category_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}

impl From<SampleContainer> for web_common::database::tables::SampleContainer {
    fn from(item: SampleContainer) -> Self {
        Self {
            id: item.id,
            barcode: item.barcode,
            project_id: item.project_id,
            category_id: item.category_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::tables::SampleContainer> for SampleContainer {
    fn from(item: web_common::database::tables::SampleContainer) -> Self {
        Self {
            id: item.id,
            barcode: item.barcode,
            project_id: item.project_id,
            category_id: item.category_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl SampleContainer {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
id: i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_sample_containers(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&SampleContainerFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_containers;
        let mut query = sample_containers::dsl::sample_containers
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sample_containers::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(sample_containers::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_containers::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sample_containers::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&SampleContainerFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_containers;
        let mut query = sample_containers::dsl::sample_containers
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sample_containers::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(sample_containers::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_containers::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sample_containers::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .order_by(sample_containers::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::sample_containers;
        sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its barcode.
    ///
    /// * `barcode` - The barcode of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_barcode(
barcode: &str,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::sample_containers;
        let flat_variant = sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::barcode.eq(barcode))
            .first::<Self>(connection)?;
        if !flat_variant.can_view(author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
filter: Option<&SampleContainerFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_containers;
 if filter.map(|f| f.project_id.is_some()&&f.category_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::project_id.eq(project_id))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(category_id) = filter.and_then(|f| f.category_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::category_id.eq(category_id))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::created_by.eq(created_by))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::updated_by.eq(updated_by))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_containers::dsl::sample_containers
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&SampleContainerFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_containers;
 if filter.map(|f| f.project_id.is_some()&&f.category_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::project_id.eq(project_id))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(category_id) = filter.and_then(|f| f.category_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::category_id.eq(category_id))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::created_by.eq(created_by))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::updated_by.eq(updated_by))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_containers::dsl::sample_containers
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&SampleContainerFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_containers;
 if filter.map(|f| f.project_id.is_some()&&f.category_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::project_id.eq(project_id))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(category_id) = filter.and_then(|f| f.category_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::category_id.eq(category_id))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::created_by.eq(created_by))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::updated_by.eq(updated_by))
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_containers::dsl::sample_containers
            .filter(can_view_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_sample_containers(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&SampleContainerFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_containers;
        let mut query = sample_containers::dsl::sample_containers
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sample_containers::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(sample_containers::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_containers::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sample_containers::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&SampleContainerFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_containers;
        let mut query = sample_containers::dsl::sample_containers
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sample_containers::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(sample_containers::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_containers::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sample_containers::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .order_by(sample_containers::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_updatable(
filter: Option<&SampleContainerFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_containers;
 if filter.map(|f| f.project_id.is_some()&&f.category_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::project_id.eq(project_id))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(category_id) = filter.and_then(|f| f.category_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::category_id.eq(category_id))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::created_by.eq(created_by))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::updated_by.eq(updated_by))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_containers::dsl::sample_containers
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_updatable(
filter: Option<&SampleContainerFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_containers;
 if filter.map(|f| f.project_id.is_some()&&f.category_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::project_id.eq(project_id))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(category_id) = filter.and_then(|f| f.category_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::category_id.eq(category_id))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::created_by.eq(created_by))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::updated_by.eq(updated_by))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_containers::dsl::sample_containers
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_updatable(
filter: Option<&SampleContainerFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_containers;
 if filter.map(|f| f.project_id.is_some()&&f.category_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::project_id.eq(project_id))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(category_id) = filter.and_then(|f| f.category_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::category_id.eq(category_id))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::created_by.eq(created_by))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::updated_by.eq(updated_by))
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_containers::dsl::sample_containers
            .filter(can_update_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_sample_containers(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&SampleContainerFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_containers;
        let mut query = sample_containers::dsl::sample_containers
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sample_containers::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(sample_containers::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_containers::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sample_containers::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&SampleContainerFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_containers;
        let mut query = sample_containers::dsl::sample_containers
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sample_containers::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(sample_containers::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_containers::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sample_containers::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .order_by(sample_containers::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_administrable(
filter: Option<&SampleContainerFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_containers;
 if filter.map(|f| f.project_id.is_some()&&f.category_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::project_id.eq(project_id))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(category_id) = filter.and_then(|f| f.category_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::category_id.eq(category_id))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::created_by.eq(created_by))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::updated_by.eq(updated_by))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_containers::dsl::sample_containers
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_administrable(
filter: Option<&SampleContainerFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_containers;
 if filter.map(|f| f.project_id.is_some()&&f.category_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::project_id.eq(project_id))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(category_id) = filter.and_then(|f| f.category_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::category_id.eq(category_id))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::created_by.eq(created_by))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::updated_by.eq(updated_by))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_containers::dsl::sample_containers
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_administrable(
filter: Option<&SampleContainerFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_containers;
 if filter.map(|f| f.project_id.is_some()&&f.category_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::project_id.eq(project_id))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(category_id) = filter.and_then(|f| f.category_id) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::category_id.eq(category_id))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::created_by.eq(created_by))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::updated_by.eq(updated_by))
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_containers::dsl::sample_containers
            .filter(can_admin_sample_containers(author_user_id, sample_containers::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(self.id, author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::id.eq(id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_states)]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct SampleState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<SampleState> for web_common::database::tables::SampleState {
    fn from(item: SampleState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::SampleState> for SampleState {
    fn from(item: web_common::database::tables::SampleState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl SampleState {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&SampleStateFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_states;
        let mut query = sample_states::dsl::sample_states
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(sample_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(sample_states::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&SampleStateFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_states;
        let mut query = sample_states::dsl::sample_states
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(sample_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(sample_states::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::sample_states;
        sample_states::dsl::sample_states
            .filter(sample_states::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_color_id(
color_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::sample_states;
        let flat_variant = sample_states::dsl::sample_states
            .filter(sample_states::dsl::color_id.eq(color_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_icon_id(
icon_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::sample_states;
        let flat_variant = sample_states::dsl::sample_states
            .filter(sample_states::dsl::icon_id.eq(icon_id))
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
    ///
    pub fn similarity_search_viewable(
filter: Option<&SampleStateFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::sample_states;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return sample_states::dsl::sample_states
            .filter(sample_states::dsl::icon_id.eq(icon_id))
            .filter(similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .order_by(similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return sample_states::dsl::sample_states
            .filter(sample_states::dsl::color_id.eq(color_id))
            .filter(similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .order_by(similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_states::dsl::sample_states
            .filter(similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .order_by(similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&SampleStateFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::sample_states;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return sample_states::dsl::sample_states
            .filter(sample_states::dsl::icon_id.eq(icon_id))
            .filter(word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return sample_states::dsl::sample_states
            .filter(sample_states::dsl::color_id.eq(color_id))
            .filter(word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_states::dsl::sample_states
            .filter(word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&SampleStateFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::sample_states;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return sample_states::dsl::sample_states
            .filter(sample_states::dsl::icon_id.eq(icon_id))
            .filter(strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return sample_states::dsl::sample_states
            .filter(sample_states::dsl::color_id.eq(color_id))
            .filter(strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_states::dsl::sample_states
            .filter(strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sampled_individual_bio_ott_taxon_items)]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(belongs_to(SampledIndividual, foreign_key = sampled_individual_id))]
#[diesel(belongs_to(BioOttTaxonItem, foreign_key = taxon_id))]
#[diesel(primary_key(sampled_individual_id, taxon_id))]
pub struct SampledIndividualBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub sampled_individual_id: Uuid,
    pub taxon_id: i32,
}

impl From<SampledIndividualBioOttTaxonItem> for web_common::database::tables::SampledIndividualBioOttTaxonItem {
    fn from(item: SampledIndividualBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            sampled_individual_id: item.sampled_individual_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl From<web_common::database::tables::SampledIndividualBioOttTaxonItem> for SampledIndividualBioOttTaxonItem {
    fn from(item: web_common::database::tables::SampledIndividualBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            sampled_individual_id: item.sampled_individual_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl SampledIndividualBioOttTaxonItem {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.sampled_individual_id, self.taxon_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( sampled_individual_id, taxon_id ): ( Uuid, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_id, taxon_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query = sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query = sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .order_by(sampled_individual_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( sampled_individual_id, taxon_id ): ( Uuid, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( sampled_individual_id, taxon_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.sampled_individual_id, self.taxon_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( sampled_individual_id, taxon_id ): ( Uuid, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_id, taxon_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query = sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query = sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .order_by(sampled_individual_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.sampled_individual_id, self.taxon_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( sampled_individual_id, taxon_id ): ( Uuid, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_id, taxon_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query = sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query = sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .order_by(sampled_individual_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.sampled_individual_id, self.taxon_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( sampled_individual_id, taxon_id ): ( Uuid, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( sampled_individual_id, taxon_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sampled_individuals)]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct SampledIndividual {
    pub id: Uuid,
    pub notes: Option<String>,
    pub barcode: Option<String>,
    pub project_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub picture: Vec<u8>,
}

impl From<SampledIndividual> for web_common::database::tables::SampledIndividual {
    fn from(item: SampledIndividual) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            barcode: item.barcode,
            project_id: item.project_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            picture: item.picture,
        }
    }
}

impl From<web_common::database::tables::SampledIndividual> for SampledIndividual {
    fn from(item: web_common::database::tables::SampledIndividual) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            barcode: item.barcode,
            project_id: item.project_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            picture: item.picture,
        }
    }
}

impl SampledIndividual {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
id: Uuid,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_sampled_individuals(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&SampledIndividualFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&SampledIndividualFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .order_by(sampled_individuals::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: Uuid,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::sampled_individuals;
        sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
filter: Option<&SampledIndividualFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
 if filter.map(|f| f.project_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::project_id.eq(project_id))
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::created_by.eq(created_by))
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sampled_individuals::dsl::sampled_individuals
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&SampledIndividualFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
 if filter.map(|f| f.project_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::project_id.eq(project_id))
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::created_by.eq(created_by))
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sampled_individuals::dsl::sampled_individuals
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&SampledIndividualFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
 if filter.map(|f| f.project_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::project_id.eq(project_id))
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::created_by.eq(created_by))
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sampled_individuals::dsl::sampled_individuals
            .filter(can_view_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
id: Uuid,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_sampled_individuals(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&SampledIndividualFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&SampledIndividualFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .order_by(sampled_individuals::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_updatable(
filter: Option<&SampledIndividualFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
 if filter.map(|f| f.project_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::project_id.eq(project_id))
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::created_by.eq(created_by))
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sampled_individuals::dsl::sampled_individuals
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_updatable(
filter: Option<&SampledIndividualFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
 if filter.map(|f| f.project_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::project_id.eq(project_id))
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::created_by.eq(created_by))
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sampled_individuals::dsl::sampled_individuals
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_updatable(
filter: Option<&SampledIndividualFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
 if filter.map(|f| f.project_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::project_id.eq(project_id))
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::created_by.eq(created_by))
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sampled_individuals::dsl::sampled_individuals
            .filter(can_update_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
id: Uuid,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_sampled_individuals(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&SampledIndividualFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&SampledIndividualFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .order_by(sampled_individuals::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_administrable(
filter: Option<&SampledIndividualFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
 if filter.map(|f| f.project_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::project_id.eq(project_id))
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::created_by.eq(created_by))
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sampled_individuals::dsl::sampled_individuals
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(similarity_dist(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_administrable(
filter: Option<&SampledIndividualFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
 if filter.map(|f| f.project_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::project_id.eq(project_id))
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::created_by.eq(created_by))
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sampled_individuals::dsl::sampled_individuals
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_administrable(
filter: Option<&SampledIndividualFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sampled_individuals;
 if filter.map(|f| f.project_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::project_id.eq(project_id))
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::created_by.eq(created_by))
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::updated_by.eq(updated_by))
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sampled_individuals::dsl::sampled_individuals
            .filter(can_admin_sampled_individuals(author_user_id, sampled_individuals::dsl::id))
            .filter(strict_word_similarity_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .order_by(strict_word_similarity_dist_op(concat_sampled_individuals_notes_barcode(sampled_individuals::dsl::notes, sampled_individuals::dsl::barcode), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(self.id, author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
id: Uuid,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::id.eq(id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = samples)]
#[diesel(belongs_to(SampleContainer, foreign_key = container_id))]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(belongs_to(SampleState, foreign_key = state))]
#[diesel(primary_key(id))]
pub struct Sample {
    pub id: Uuid,
    pub container_id: i32,
    pub notes: Option<String>,
    pub project_id: i32,
    pub created_by: i32,
    pub sampled_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub state: i32,
}

impl From<Sample> for web_common::database::tables::Sample {
    fn from(item: Sample) -> Self {
        Self {
            id: item.id,
            container_id: item.container_id,
            notes: item.notes,
            project_id: item.project_id,
            created_by: item.created_by,
            sampled_by: item.sampled_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            state: item.state,
        }
    }
}

impl From<web_common::database::tables::Sample> for Sample {
    fn from(item: web_common::database::tables::Sample) -> Self {
        Self {
            id: item.id,
            container_id: item.container_id,
            notes: item.notes,
            project_id: item.project_id,
            created_by: item.created_by,
            sampled_by: item.sampled_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            state: item.state,
        }
    }
}

impl Sample {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
id: Uuid,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_samples(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&SampleFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::samples;
        let mut query = samples::dsl::samples
            .into_boxed();
        if let Some(container_id) = filter.and_then(|f| f.container_id) {
            query = query.filter(samples::dsl::container_id.eq(container_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(samples::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(samples::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
            query = query.filter(samples::dsl::sampled_by.eq(sampled_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(state) = filter.and_then(|f| f.state) {
            query = query.filter(samples::dsl::state.eq(state));
        }
        query
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&SampleFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::samples;
        let mut query = samples::dsl::samples
            .into_boxed();
        if let Some(container_id) = filter.and_then(|f| f.container_id) {
            query = query.filter(samples::dsl::container_id.eq(container_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(samples::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(samples::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
            query = query.filter(samples::dsl::sampled_by.eq(sampled_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(state) = filter.and_then(|f| f.state) {
            query = query.filter(samples::dsl::state.eq(state));
        }
        query
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .order_by(samples::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: Uuid,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::samples;
        samples::dsl::samples
            .filter(samples::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its container_id.
    ///
    /// * `container_id` - The container_id of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_container_id(
container_id: &i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::samples;
        let flat_variant = samples::dsl::samples
            .filter(samples::dsl::container_id.eq(container_id))
            .first::<Self>(connection)?;
        if !flat_variant.can_view(author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
filter: Option<&SampleFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::samples;
 if filter.map(|f| f.container_id.is_some()&&f.project_id.is_some()&&f.created_by.is_some()&&f.sampled_by.is_some()&&f.updated_by.is_some()&&f.state.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(container_id) = filter.and_then(|f| f.container_id) {
        return samples::dsl::samples
            .filter(samples::dsl::container_id.eq(container_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return samples::dsl::samples
            .filter(samples::dsl::project_id.eq(project_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return samples::dsl::samples
            .filter(samples::dsl::created_by.eq(created_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
        return samples::dsl::samples
            .filter(samples::dsl::sampled_by.eq(sampled_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return samples::dsl::samples
            .filter(samples::dsl::updated_by.eq(updated_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state) = filter.and_then(|f| f.state) {
        return samples::dsl::samples
            .filter(samples::dsl::state.eq(state))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        samples::dsl::samples
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&SampleFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::samples;
 if filter.map(|f| f.container_id.is_some()&&f.project_id.is_some()&&f.created_by.is_some()&&f.sampled_by.is_some()&&f.updated_by.is_some()&&f.state.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(container_id) = filter.and_then(|f| f.container_id) {
        return samples::dsl::samples
            .filter(samples::dsl::container_id.eq(container_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return samples::dsl::samples
            .filter(samples::dsl::project_id.eq(project_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return samples::dsl::samples
            .filter(samples::dsl::created_by.eq(created_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
        return samples::dsl::samples
            .filter(samples::dsl::sampled_by.eq(sampled_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return samples::dsl::samples
            .filter(samples::dsl::updated_by.eq(updated_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state) = filter.and_then(|f| f.state) {
        return samples::dsl::samples
            .filter(samples::dsl::state.eq(state))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        samples::dsl::samples
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&SampleFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::samples;
 if filter.map(|f| f.container_id.is_some()&&f.project_id.is_some()&&f.created_by.is_some()&&f.sampled_by.is_some()&&f.updated_by.is_some()&&f.state.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(container_id) = filter.and_then(|f| f.container_id) {
        return samples::dsl::samples
            .filter(samples::dsl::container_id.eq(container_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return samples::dsl::samples
            .filter(samples::dsl::project_id.eq(project_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return samples::dsl::samples
            .filter(samples::dsl::created_by.eq(created_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
        return samples::dsl::samples
            .filter(samples::dsl::sampled_by.eq(sampled_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return samples::dsl::samples
            .filter(samples::dsl::updated_by.eq(updated_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state) = filter.and_then(|f| f.state) {
        return samples::dsl::samples
            .filter(samples::dsl::state.eq(state))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        samples::dsl::samples
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_view_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
id: Uuid,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_samples(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&SampleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::samples;
        let mut query = samples::dsl::samples
            .into_boxed();
        if let Some(container_id) = filter.and_then(|f| f.container_id) {
            query = query.filter(samples::dsl::container_id.eq(container_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(samples::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(samples::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
            query = query.filter(samples::dsl::sampled_by.eq(sampled_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(state) = filter.and_then(|f| f.state) {
            query = query.filter(samples::dsl::state.eq(state));
        }
        query
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&SampleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::samples;
        let mut query = samples::dsl::samples
            .into_boxed();
        if let Some(container_id) = filter.and_then(|f| f.container_id) {
            query = query.filter(samples::dsl::container_id.eq(container_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(samples::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(samples::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
            query = query.filter(samples::dsl::sampled_by.eq(sampled_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(state) = filter.and_then(|f| f.state) {
            query = query.filter(samples::dsl::state.eq(state));
        }
        query
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .order_by(samples::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_updatable(
filter: Option<&SampleFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::samples;
 if filter.map(|f| f.container_id.is_some()&&f.project_id.is_some()&&f.created_by.is_some()&&f.sampled_by.is_some()&&f.updated_by.is_some()&&f.state.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(container_id) = filter.and_then(|f| f.container_id) {
        return samples::dsl::samples
            .filter(samples::dsl::container_id.eq(container_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return samples::dsl::samples
            .filter(samples::dsl::project_id.eq(project_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return samples::dsl::samples
            .filter(samples::dsl::created_by.eq(created_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
        return samples::dsl::samples
            .filter(samples::dsl::sampled_by.eq(sampled_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return samples::dsl::samples
            .filter(samples::dsl::updated_by.eq(updated_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state) = filter.and_then(|f| f.state) {
        return samples::dsl::samples
            .filter(samples::dsl::state.eq(state))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        samples::dsl::samples
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_updatable(
filter: Option<&SampleFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::samples;
 if filter.map(|f| f.container_id.is_some()&&f.project_id.is_some()&&f.created_by.is_some()&&f.sampled_by.is_some()&&f.updated_by.is_some()&&f.state.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(container_id) = filter.and_then(|f| f.container_id) {
        return samples::dsl::samples
            .filter(samples::dsl::container_id.eq(container_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return samples::dsl::samples
            .filter(samples::dsl::project_id.eq(project_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return samples::dsl::samples
            .filter(samples::dsl::created_by.eq(created_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
        return samples::dsl::samples
            .filter(samples::dsl::sampled_by.eq(sampled_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return samples::dsl::samples
            .filter(samples::dsl::updated_by.eq(updated_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state) = filter.and_then(|f| f.state) {
        return samples::dsl::samples
            .filter(samples::dsl::state.eq(state))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        samples::dsl::samples
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_updatable(
filter: Option<&SampleFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::samples;
 if filter.map(|f| f.container_id.is_some()&&f.project_id.is_some()&&f.created_by.is_some()&&f.sampled_by.is_some()&&f.updated_by.is_some()&&f.state.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(container_id) = filter.and_then(|f| f.container_id) {
        return samples::dsl::samples
            .filter(samples::dsl::container_id.eq(container_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return samples::dsl::samples
            .filter(samples::dsl::project_id.eq(project_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return samples::dsl::samples
            .filter(samples::dsl::created_by.eq(created_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
        return samples::dsl::samples
            .filter(samples::dsl::sampled_by.eq(sampled_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return samples::dsl::samples
            .filter(samples::dsl::updated_by.eq(updated_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state) = filter.and_then(|f| f.state) {
        return samples::dsl::samples
            .filter(samples::dsl::state.eq(state))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        samples::dsl::samples
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_update_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
id: Uuid,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_samples(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&SampleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::samples;
        let mut query = samples::dsl::samples
            .into_boxed();
        if let Some(container_id) = filter.and_then(|f| f.container_id) {
            query = query.filter(samples::dsl::container_id.eq(container_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(samples::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(samples::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
            query = query.filter(samples::dsl::sampled_by.eq(sampled_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(state) = filter.and_then(|f| f.state) {
            query = query.filter(samples::dsl::state.eq(state));
        }
        query
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&SampleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::samples;
        let mut query = samples::dsl::samples
            .into_boxed();
        if let Some(container_id) = filter.and_then(|f| f.container_id) {
            query = query.filter(samples::dsl::container_id.eq(container_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(samples::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(samples::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
            query = query.filter(samples::dsl::sampled_by.eq(sampled_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(state) = filter.and_then(|f| f.state) {
            query = query.filter(samples::dsl::state.eq(state));
        }
        query
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .order_by(samples::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_administrable(
filter: Option<&SampleFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::samples;
 if filter.map(|f| f.container_id.is_some()&&f.project_id.is_some()&&f.created_by.is_some()&&f.sampled_by.is_some()&&f.updated_by.is_some()&&f.state.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(container_id) = filter.and_then(|f| f.container_id) {
        return samples::dsl::samples
            .filter(samples::dsl::container_id.eq(container_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return samples::dsl::samples
            .filter(samples::dsl::project_id.eq(project_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return samples::dsl::samples
            .filter(samples::dsl::created_by.eq(created_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
        return samples::dsl::samples
            .filter(samples::dsl::sampled_by.eq(sampled_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return samples::dsl::samples
            .filter(samples::dsl::updated_by.eq(updated_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state) = filter.and_then(|f| f.state) {
        return samples::dsl::samples
            .filter(samples::dsl::state.eq(state))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        samples::dsl::samples
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(similarity_op(sample_containers::dsl::barcode, query))
            .order_by(similarity_dist(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_administrable(
filter: Option<&SampleFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::samples;
 if filter.map(|f| f.container_id.is_some()&&f.project_id.is_some()&&f.created_by.is_some()&&f.sampled_by.is_some()&&f.updated_by.is_some()&&f.state.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(container_id) = filter.and_then(|f| f.container_id) {
        return samples::dsl::samples
            .filter(samples::dsl::container_id.eq(container_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return samples::dsl::samples
            .filter(samples::dsl::project_id.eq(project_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return samples::dsl::samples
            .filter(samples::dsl::created_by.eq(created_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
        return samples::dsl::samples
            .filter(samples::dsl::sampled_by.eq(sampled_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return samples::dsl::samples
            .filter(samples::dsl::updated_by.eq(updated_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state) = filter.and_then(|f| f.state) {
        return samples::dsl::samples
            .filter(samples::dsl::state.eq(state))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        samples::dsl::samples
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_administrable(
filter: Option<&SampleFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::samples;
 if filter.map(|f| f.container_id.is_some()&&f.project_id.is_some()&&f.created_by.is_some()&&f.sampled_by.is_some()&&f.updated_by.is_some()&&f.state.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(container_id) = filter.and_then(|f| f.container_id) {
        return samples::dsl::samples
            .filter(samples::dsl::container_id.eq(container_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(project_id) = filter.and_then(|f| f.project_id) {
        return samples::dsl::samples
            .filter(samples::dsl::project_id.eq(project_id))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return samples::dsl::samples
            .filter(samples::dsl::created_by.eq(created_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
        return samples::dsl::samples
            .filter(samples::dsl::sampled_by.eq(sampled_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return samples::dsl::samples
            .filter(samples::dsl::updated_by.eq(updated_by))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state) = filter.and_then(|f| f.state) {
        return samples::dsl::samples
            .filter(samples::dsl::state.eq(state))
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        samples::dsl::samples
            .select(Sample::as_select())
            .inner_join(sample_containers::dsl::sample_containers.on(samples::dsl::container_id.eq(sample_containers::dsl::id)))
            .filter(can_admin_samples(author_user_id, samples::dsl::id))
            .filter(strict_word_similarity_op(sample_containers::dsl::barcode, query))
            .order_by(strict_word_similarity_dist_op(sample_containers::dsl::barcode, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(self.id, author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
id: Uuid,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(samples::dsl::samples
            .filter(samples::dsl::id.eq(id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = spectra)]
#[diesel(belongs_to(SpectraCollection, foreign_key = spectra_collection_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct Spectra {
    pub id: i32,
    pub notes: Option<String>,
    pub spectra_collection_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}

impl From<Spectra> for web_common::database::tables::Spectra {
    fn from(item: Spectra) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            spectra_collection_id: item.spectra_collection_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::tables::Spectra> for Spectra {
    fn from(item: web_common::database::tables::Spectra) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            spectra_collection_id: item.spectra_collection_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl Spectra {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
id: i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_spectra(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&SpectraFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra;
        let mut query = spectra::dsl::spectra
            .into_boxed();
        if let Some(spectra_collection_id) = filter.and_then(|f| f.spectra_collection_id) {
            query = query.filter(spectra::dsl::spectra_collection_id.eq(spectra_collection_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_spectra(author_user_id, spectra::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&SpectraFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra;
        let mut query = spectra::dsl::spectra
            .into_boxed();
        if let Some(spectra_collection_id) = filter.and_then(|f| f.spectra_collection_id) {
            query = query.filter(spectra::dsl::spectra_collection_id.eq(spectra_collection_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_spectra(author_user_id, spectra::dsl::id))
            .order_by(spectra::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::spectra;
        spectra::dsl::spectra
            .filter(spectra::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_spectra(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&SpectraFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra;
        let mut query = spectra::dsl::spectra
            .into_boxed();
        if let Some(spectra_collection_id) = filter.and_then(|f| f.spectra_collection_id) {
            query = query.filter(spectra::dsl::spectra_collection_id.eq(spectra_collection_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_spectra(author_user_id, spectra::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&SpectraFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra;
        let mut query = spectra::dsl::spectra
            .into_boxed();
        if let Some(spectra_collection_id) = filter.and_then(|f| f.spectra_collection_id) {
            query = query.filter(spectra::dsl::spectra_collection_id.eq(spectra_collection_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_spectra(author_user_id, spectra::dsl::id))
            .order_by(spectra::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_spectra(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&SpectraFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra;
        let mut query = spectra::dsl::spectra
            .into_boxed();
        if let Some(spectra_collection_id) = filter.and_then(|f| f.spectra_collection_id) {
            query = query.filter(spectra::dsl::spectra_collection_id.eq(spectra_collection_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_spectra(author_user_id, spectra::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&SpectraFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra;
        let mut query = spectra::dsl::spectra
            .into_boxed();
        if let Some(spectra_collection_id) = filter.and_then(|f| f.spectra_collection_id) {
            query = query.filter(spectra::dsl::spectra_collection_id.eq(spectra_collection_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_spectra(author_user_id, spectra::dsl::id))
            .order_by(spectra::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(self.id, author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(spectra::dsl::spectra
            .filter(spectra::dsl::id.eq(id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = spectra_collections)]
#[diesel(belongs_to(Sample, foreign_key = sample_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct SpectraCollection {
    pub id: i32,
    pub notes: Option<String>,
    pub sample_id: Uuid,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}

impl From<SpectraCollection> for web_common::database::tables::SpectraCollection {
    fn from(item: SpectraCollection) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            sample_id: item.sample_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::tables::SpectraCollection> for SpectraCollection {
    fn from(item: web_common::database::tables::SpectraCollection) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            sample_id: item.sample_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl SpectraCollection {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
id: i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_spectra_collections(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&SpectraCollectionFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections
            .into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_spectra_collections(author_user_id, spectra_collections::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&SpectraCollectionFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections
            .into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_spectra_collections(author_user_id, spectra_collections::dsl::id))
            .order_by(spectra_collections::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::spectra_collections;
        spectra_collections::dsl::spectra_collections
            .filter(spectra_collections::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_spectra_collections(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&SpectraCollectionFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections
            .into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_spectra_collections(author_user_id, spectra_collections::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&SpectraCollectionFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections
            .into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_spectra_collections(author_user_id, spectra_collections::dsl::id))
            .order_by(spectra_collections::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_spectra_collections(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&SpectraCollectionFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections
            .into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_spectra_collections(author_user_id, spectra_collections::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&SpectraCollectionFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections
            .into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_spectra_collections(author_user_id, spectra_collections::dsl::id))
            .order_by(spectra_collections::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(self.id, author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(spectra_collections::dsl::spectra_collections
            .filter(spectra_collections::dsl::id.eq(id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = team_states)]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct TeamState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<TeamState> for web_common::database::tables::TeamState {
    fn from(item: TeamState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::TeamState> for TeamState {
    fn from(item: web_common::database::tables::TeamState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl TeamState {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&TeamStateFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::team_states;
        let mut query = team_states::dsl::team_states
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(team_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(team_states::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&TeamStateFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::team_states;
        let mut query = team_states::dsl::team_states
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(team_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(team_states::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::team_states;
        team_states::dsl::team_states
            .filter(team_states::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_color_id(
color_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::team_states;
        let flat_variant = team_states::dsl::team_states
            .filter(team_states::dsl::color_id.eq(color_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_icon_id(
icon_id: &i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::team_states;
        let flat_variant = team_states::dsl::team_states
            .filter(team_states::dsl::icon_id.eq(icon_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::team_states;
        let flat_variant = team_states::dsl::team_states
            .filter(team_states::dsl::name.eq(name))
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
    ///
    pub fn similarity_search_viewable(
filter: Option<&TeamStateFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::team_states;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return team_states::dsl::team_states
            .filter(team_states::dsl::icon_id.eq(icon_id))
            .filter(similarity_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .order_by(similarity_dist(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return team_states::dsl::team_states
            .filter(team_states::dsl::color_id.eq(color_id))
            .filter(similarity_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .order_by(similarity_dist(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        team_states::dsl::team_states
            .filter(similarity_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .order_by(similarity_dist(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&TeamStateFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::team_states;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return team_states::dsl::team_states
            .filter(team_states::dsl::icon_id.eq(icon_id))
            .filter(word_similarity_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return team_states::dsl::team_states
            .filter(team_states::dsl::color_id.eq(color_id))
            .filter(word_similarity_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        team_states::dsl::team_states
            .filter(word_similarity_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&TeamStateFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::team_states;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return team_states::dsl::team_states
            .filter(team_states::dsl::icon_id.eq(icon_id))
            .filter(strict_word_similarity_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return team_states::dsl::team_states
            .filter(team_states::dsl::color_id.eq(color_id))
            .filter(strict_word_similarity_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        team_states::dsl::team_states
            .filter(strict_word_similarity_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_team_states_name_description(team_states::dsl::name, team_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams)]
#[diesel(belongs_to(FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(Color, foreign_key = color_id))]
#[diesel(belongs_to(TeamState, foreign_key = state_id))]
#[diesel(belongs_to(Team, foreign_key = parent_team_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
    pub state_id: i32,
    pub parent_team_id: Option<i32>,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}

impl From<Team> for web_common::database::tables::Team {
    fn from(item: Team) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
            state_id: item.state_id,
            parent_team_id: item.parent_team_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::tables::Team> for Team {
    fn from(item: web_common::database::tables::Team) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
            state_id: item.state_id,
            parent_team_id: item.parent_team_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl Team {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&TeamFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams;
        let mut query = teams::dsl::teams
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&TeamFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams;
        let mut query = teams::dsl::teams
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
        }
        query
            .order_by(teams::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::teams;
        teams::dsl::teams
            .filter(teams::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::teams;
        let flat_variant = teams::dsl::teams
            .filter(teams::dsl::name.eq(name))
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
    ///
    pub fn similarity_search_viewable(
filter: Option<&TeamFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::teams;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()&&f.state_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return teams::dsl::teams
            .filter(teams::dsl::icon_id.eq(icon_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return teams::dsl::teams
            .filter(teams::dsl::color_id.eq(color_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return teams::dsl::teams
            .filter(teams::dsl::state_id.eq(state_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return teams::dsl::teams
            .filter(teams::dsl::created_by.eq(created_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return teams::dsl::teams
            .filter(teams::dsl::updated_by.eq(updated_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        teams::dsl::teams
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&TeamFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::teams;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()&&f.state_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return teams::dsl::teams
            .filter(teams::dsl::icon_id.eq(icon_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return teams::dsl::teams
            .filter(teams::dsl::color_id.eq(color_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return teams::dsl::teams
            .filter(teams::dsl::state_id.eq(state_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return teams::dsl::teams
            .filter(teams::dsl::created_by.eq(created_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return teams::dsl::teams
            .filter(teams::dsl::updated_by.eq(updated_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        teams::dsl::teams
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&TeamFilter>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::schema::teams;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()&&f.state_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return teams::dsl::teams
            .filter(teams::dsl::icon_id.eq(icon_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return teams::dsl::teams
            .filter(teams::dsl::color_id.eq(color_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return teams::dsl::teams
            .filter(teams::dsl::state_id.eq(state_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return teams::dsl::teams
            .filter(teams::dsl::created_by.eq(created_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return teams::dsl::teams
            .filter(teams::dsl::updated_by.eq(updated_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        teams::dsl::teams
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_teams(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&TeamFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams;
        let mut query = teams::dsl::teams
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&TeamFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams;
        let mut query = teams::dsl::teams
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .order_by(teams::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_updatable(
filter: Option<&TeamFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::teams;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()&&f.state_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return teams::dsl::teams
            .filter(teams::dsl::icon_id.eq(icon_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return teams::dsl::teams
            .filter(teams::dsl::color_id.eq(color_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return teams::dsl::teams
            .filter(teams::dsl::state_id.eq(state_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return teams::dsl::teams
            .filter(teams::dsl::created_by.eq(created_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return teams::dsl::teams
            .filter(teams::dsl::updated_by.eq(updated_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        teams::dsl::teams
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_updatable(
filter: Option<&TeamFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::teams;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()&&f.state_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return teams::dsl::teams
            .filter(teams::dsl::icon_id.eq(icon_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return teams::dsl::teams
            .filter(teams::dsl::color_id.eq(color_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return teams::dsl::teams
            .filter(teams::dsl::state_id.eq(state_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return teams::dsl::teams
            .filter(teams::dsl::created_by.eq(created_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return teams::dsl::teams
            .filter(teams::dsl::updated_by.eq(updated_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        teams::dsl::teams
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_updatable(
filter: Option<&TeamFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::teams;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()&&f.state_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return teams::dsl::teams
            .filter(teams::dsl::icon_id.eq(icon_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return teams::dsl::teams
            .filter(teams::dsl::color_id.eq(color_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return teams::dsl::teams
            .filter(teams::dsl::state_id.eq(state_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return teams::dsl::teams
            .filter(teams::dsl::created_by.eq(created_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return teams::dsl::teams
            .filter(teams::dsl::updated_by.eq(updated_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        teams::dsl::teams
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_update_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_teams(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&TeamFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams;
        let mut query = teams::dsl::teams
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&TeamFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams;
        let mut query = teams::dsl::teams
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .order_by(teams::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_administrable(
filter: Option<&TeamFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::teams;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()&&f.state_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return teams::dsl::teams
            .filter(teams::dsl::icon_id.eq(icon_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return teams::dsl::teams
            .filter(teams::dsl::color_id.eq(color_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return teams::dsl::teams
            .filter(teams::dsl::state_id.eq(state_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return teams::dsl::teams
            .filter(teams::dsl::created_by.eq(created_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return teams::dsl::teams
            .filter(teams::dsl::updated_by.eq(updated_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        teams::dsl::teams
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(similarity_dist(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_administrable(
filter: Option<&TeamFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::teams;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()&&f.state_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return teams::dsl::teams
            .filter(teams::dsl::icon_id.eq(icon_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return teams::dsl::teams
            .filter(teams::dsl::color_id.eq(color_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return teams::dsl::teams
            .filter(teams::dsl::state_id.eq(state_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return teams::dsl::teams
            .filter(teams::dsl::created_by.eq(created_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return teams::dsl::teams
            .filter(teams::dsl::updated_by.eq(updated_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        teams::dsl::teams
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_administrable(
filter: Option<&TeamFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::teams;
 if filter.map(|f| f.icon_id.is_some()&&f.color_id.is_some()&&f.state_id.is_some()&&f.created_by.is_some()&&f.updated_by.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
        return teams::dsl::teams
            .filter(teams::dsl::icon_id.eq(icon_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(color_id) = filter.and_then(|f| f.color_id) {
        return teams::dsl::teams
            .filter(teams::dsl::color_id.eq(color_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(state_id) = filter.and_then(|f| f.state_id) {
        return teams::dsl::teams
            .filter(teams::dsl::state_id.eq(state_id))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return teams::dsl::teams
            .filter(teams::dsl::created_by.eq(created_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
        return teams::dsl::teams
            .filter(teams::dsl::updated_by.eq(updated_by))
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        teams::dsl::teams
            .filter(teams::dsl::parent_team_id.eq(filter.and_then(|f| f.parent_team_id)))
            .filter(can_admin_teams(author_user_id, teams::dsl::id))
            .filter(strict_word_similarity_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .order_by(strict_word_similarity_dist_op(concat_teams_name_description(teams::dsl::name, teams::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(self.id, author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(teams::dsl::teams
            .filter(teams::dsl::id.eq(id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams_teams_role_invitations)]
#[diesel(belongs_to(Team, foreign_key = table_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(primary_key(table_id, team_id))]
pub struct TeamsTeamsRoleInvitation {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<TeamsTeamsRoleInvitation> for web_common::database::tables::TeamsTeamsRoleInvitation {
    fn from(item: TeamsTeamsRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::TeamsTeamsRoleInvitation> for TeamsTeamsRoleInvitation {
    fn from(item: web_common::database::tables::TeamsTeamsRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl TeamsTeamsRoleInvitation {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_teams_teams_role_invitations(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&TeamsTeamsRoleInvitationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_teams_role_invitations;
        let mut query = teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(teams_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_teams_teams_role_invitations(author_user_id, teams_teams_role_invitations::dsl::table_id, teams_teams_role_invitations::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&TeamsTeamsRoleInvitationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_teams_role_invitations;
        let mut query = teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(teams_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_teams_teams_role_invitations(author_user_id, teams_teams_role_invitations::dsl::table_id, teams_teams_role_invitations::dsl::team_id))
            .order_by(teams_teams_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, team_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, team_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::teams_teams_role_invitations;
        teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .filter(teams_teams_role_invitations::dsl::table_id.eq(table_id))
            .filter(teams_teams_role_invitations::dsl::team_id.eq(team_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_teams_teams_role_invitations(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&TeamsTeamsRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_teams_role_invitations;
        let mut query = teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(teams_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_teams_teams_role_invitations(author_user_id, teams_teams_role_invitations::dsl::table_id, teams_teams_role_invitations::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&TeamsTeamsRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_teams_role_invitations;
        let mut query = teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(teams_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_teams_teams_role_invitations(author_user_id, teams_teams_role_invitations::dsl::table_id, teams_teams_role_invitations::dsl::team_id))
            .order_by(teams_teams_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.team_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_teams_teams_role_invitations(author_user_id, table_id, team_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&TeamsTeamsRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_teams_role_invitations;
        let mut query = teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(teams_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_teams_teams_role_invitations(author_user_id, teams_teams_role_invitations::dsl::table_id, teams_teams_role_invitations::dsl::team_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&TeamsTeamsRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_teams_role_invitations;
        let mut query = teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(teams_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_teams_teams_role_invitations(author_user_id, teams_teams_role_invitations::dsl::table_id, teams_teams_role_invitations::dsl::team_id))
            .order_by(teams_teams_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.team_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, team_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, team_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, team_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .filter(teams_teams_role_invitations::dsl::table_id.eq(table_id))
            .filter(teams_teams_role_invitations::dsl::team_id.eq(team_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams_users_role_invitations)]
#[diesel(belongs_to(Team, foreign_key = table_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct TeamsUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<TeamsUsersRoleInvitation> for web_common::database::tables::TeamsUsersRoleInvitation {
    fn from(item: TeamsUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::TeamsUsersRoleInvitation> for TeamsUsersRoleInvitation {
    fn from(item: web_common::database::tables::TeamsUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl TeamsUsersRoleInvitation {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_teams_users_role_invitations(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&TeamsUsersRoleInvitationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_invitations;
        let mut query = teams_users_role_invitations::dsl::teams_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_teams_users_role_invitations(author_user_id, teams_users_role_invitations::dsl::table_id, teams_users_role_invitations::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&TeamsUsersRoleInvitationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_invitations;
        let mut query = teams_users_role_invitations::dsl::teams_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_teams_users_role_invitations(author_user_id, teams_users_role_invitations::dsl::table_id, teams_users_role_invitations::dsl::user_id))
            .order_by(teams_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::teams_users_role_invitations;
        teams_users_role_invitations::dsl::teams_users_role_invitations
            .filter(teams_users_role_invitations::dsl::table_id.eq(table_id))
            .filter(teams_users_role_invitations::dsl::user_id.eq(user_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_teams_users_role_invitations(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&TeamsUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_invitations;
        let mut query = teams_users_role_invitations::dsl::teams_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_teams_users_role_invitations(author_user_id, teams_users_role_invitations::dsl::table_id, teams_users_role_invitations::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&TeamsUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_invitations;
        let mut query = teams_users_role_invitations::dsl::teams_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_teams_users_role_invitations(author_user_id, teams_users_role_invitations::dsl::table_id, teams_users_role_invitations::dsl::user_id))
            .order_by(teams_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_teams_users_role_invitations(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&TeamsUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_invitations;
        let mut query = teams_users_role_invitations::dsl::teams_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_teams_users_role_invitations(author_user_id, teams_users_role_invitations::dsl::table_id, teams_users_role_invitations::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&TeamsUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_invitations;
        let mut query = teams_users_role_invitations::dsl::teams_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_teams_users_role_invitations(author_user_id, teams_users_role_invitations::dsl::table_id, teams_users_role_invitations::dsl::user_id))
            .order_by(teams_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.user_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(teams_users_role_invitations::dsl::teams_users_role_invitations
            .filter(teams_users_role_invitations::dsl::table_id.eq(table_id))
            .filter(teams_users_role_invitations::dsl::user_id.eq(user_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams_users_role_requests)]
#[diesel(belongs_to(Team, foreign_key = table_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct TeamsUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<TeamsUsersRoleRequest> for web_common::database::tables::TeamsUsersRoleRequest {
    fn from(item: TeamsUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::TeamsUsersRoleRequest> for TeamsUsersRoleRequest {
    fn from(item: web_common::database::tables::TeamsUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl TeamsUsersRoleRequest {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_teams_users_role_requests(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&TeamsUsersRoleRequestFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_teams_users_role_requests(author_user_id, teams_users_role_requests::dsl::table_id, teams_users_role_requests::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&TeamsUsersRoleRequestFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_teams_users_role_requests(author_user_id, teams_users_role_requests::dsl::table_id, teams_users_role_requests::dsl::user_id))
            .order_by(teams_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::teams_users_role_requests;
        teams_users_role_requests::dsl::teams_users_role_requests
            .filter(teams_users_role_requests::dsl::table_id.eq(table_id))
            .filter(teams_users_role_requests::dsl::user_id.eq(user_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_teams_users_role_requests(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&TeamsUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_teams_users_role_requests(author_user_id, teams_users_role_requests::dsl::table_id, teams_users_role_requests::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&TeamsUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_teams_users_role_requests(author_user_id, teams_users_role_requests::dsl::table_id, teams_users_role_requests::dsl::user_id))
            .order_by(teams_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_teams_users_role_requests(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&TeamsUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_teams_users_role_requests(author_user_id, teams_users_role_requests::dsl::table_id, teams_users_role_requests::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&TeamsUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_teams_users_role_requests(author_user_id, teams_users_role_requests::dsl::table_id, teams_users_role_requests::dsl::user_id))
            .order_by(teams_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.user_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(teams_users_role_requests::dsl::teams_users_role_requests
            .filter(teams_users_role_requests::dsl::table_id.eq(table_id))
            .filter(teams_users_role_requests::dsl::user_id.eq(user_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams_users_roles)]
#[diesel(belongs_to(Team, foreign_key = table_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct TeamsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<TeamsUsersRole> for web_common::database::tables::TeamsUsersRole {
    fn from(item: TeamsUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::TeamsUsersRole> for TeamsUsersRole {
    fn from(item: web_common::database::tables::TeamsUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl TeamsUsersRole {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&TeamsUsersRoleFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_roles;
        let mut query = teams_users_roles::dsl::teams_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&TeamsUsersRoleFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_roles;
        let mut query = teams_users_roles::dsl::teams_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .order_by(teams_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, user_id ): ( i32, i32 ),
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::teams_users_roles;
        teams_users_roles::dsl::teams_users_roles
            .filter(teams_users_roles::dsl::table_id.eq(table_id))
            .filter(teams_users_roles::dsl::user_id.eq(user_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_teams_users_roles(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&TeamsUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_roles;
        let mut query = teams_users_roles::dsl::teams_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_teams_users_roles(author_user_id, teams_users_roles::dsl::table_id, teams_users_roles::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&TeamsUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_roles;
        let mut query = teams_users_roles::dsl::teams_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_teams_users_roles(author_user_id, teams_users_roles::dsl::table_id, teams_users_roles::dsl::user_id))
            .order_by(teams_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_teams_users_roles(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&TeamsUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_roles;
        let mut query = teams_users_roles::dsl::teams_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_teams_users_roles(author_user_id, teams_users_roles::dsl::table_id, teams_users_roles::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&TeamsUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::teams_users_roles;
        let mut query = teams_users_roles::dsl::teams_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_teams_users_roles(author_user_id, teams_users_roles::dsl::table_id, teams_users_roles::dsl::user_id))
            .order_by(teams_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.user_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(teams_users_roles::dsl::teams_users_roles
            .filter(teams_users_roles::dsl::table_id.eq(table_id))
            .filter(teams_users_roles::dsl::user_id.eq(user_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
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
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::units;
        units::dsl::units
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::units;
        units::dsl::units
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::units;
        units::dsl::units
            .filter(units::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
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
    ///
    pub fn similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::units;
        units::dsl::units
            .filter(similarity_op(concat_units_name_description_symbol(units::dsl::name, units::dsl::description, units::dsl::symbol), query))
            .order_by(similarity_dist(concat_units_name_description_symbol(units::dsl::name, units::dsl::description, units::dsl::symbol), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::units;
        units::dsl::units
            .filter(word_similarity_op(concat_units_name_description_symbol(units::dsl::name, units::dsl::description, units::dsl::symbol), query))
            .order_by(word_similarity_dist_op(concat_units_name_description_symbol(units::dsl::name, units::dsl::description, units::dsl::symbol), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::units;
        units::dsl::units
            .filter(strict_word_similarity_op(concat_units_name_description_symbol(units::dsl::name, units::dsl::description, units::dsl::symbol), query))
            .order_by(strict_word_similarity_dist_op(concat_units_name_description_symbol(units::dsl::name, units::dsl::description, units::dsl::symbol), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = user_emails)]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(belongs_to(LoginProvider, foreign_key = login_provider_id))]
#[diesel(primary_key(id))]
pub struct UserEmail {
    pub id: i32,
    pub email: String,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub login_provider_id: i32,
    pub primary_email: bool,
}

impl From<UserEmail> for web_common::database::tables::UserEmail {
    fn from(item: UserEmail) -> Self {
        Self {
            id: item.id,
            email: item.email,
            created_by: item.created_by,
            created_at: item.created_at,
            login_provider_id: item.login_provider_id,
            primary_email: item.primary_email,
        }
    }
}

impl From<web_common::database::tables::UserEmail> for UserEmail {
    fn from(item: web_common::database::tables::UserEmail) -> Self {
        Self {
            id: item.id,
            email: item.email,
            created_by: item.created_by,
            created_at: item.created_at,
            login_provider_id: item.login_provider_id,
            primary_email: item.primary_email,
        }
    }
}

impl UserEmail {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
id: i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_user_emails(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&UserEmailFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::user_emails;
        let mut query = user_emails::dsl::user_emails
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .filter(can_view_user_emails(author_user_id, user_emails::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&UserEmailFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::user_emails;
        let mut query = user_emails::dsl::user_emails
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .filter(can_view_user_emails(author_user_id, user_emails::dsl::id))
            .order_by(user_emails::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::user_emails;
        user_emails::dsl::user_emails
            .filter(user_emails::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its email and login_provider_id.
    ///
    /// * `email` - The email of the struct to get.
    /// * `login_provider_id` - The login_provider_id of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_email_and_login_provider_id(
email: &str,
login_provider_id: &i32,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::user_emails;
        let flat_variant = user_emails::dsl::user_emails
            .filter(user_emails::dsl::email.eq(email))
            .filter(user_emails::dsl::login_provider_id.eq(login_provider_id))
            .first::<Self>(connection)?;
        if !flat_variant.can_view(author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        Ok(flat_variant)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_user_emails(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&UserEmailFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::user_emails;
        let mut query = user_emails::dsl::user_emails
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .filter(can_update_user_emails(author_user_id, user_emails::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&UserEmailFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::user_emails;
        let mut query = user_emails::dsl::user_emails
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .filter(can_update_user_emails(author_user_id, user_emails::dsl::id))
            .order_by(user_emails::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_user_emails(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&UserEmailFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::user_emails;
        let mut query = user_emails::dsl::user_emails
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .filter(can_admin_user_emails(author_user_id, user_emails::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&UserEmailFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::user_emails;
        let mut query = user_emails::dsl::user_emails
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .filter(can_admin_user_emails(author_user_id, user_emails::dsl::id))
            .order_by(user_emails::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(self.id, author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(user_emails::dsl::user_emails
            .filter(user_emails::dsl::id.eq(id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub description: Option<String>,
    pub profile_picture: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for web_common::database::tables::User {
    fn from(item: User) -> Self {
        Self {
            id: item.id,
            first_name: item.first_name,
            middle_name: item.middle_name,
            last_name: item.last_name,
            description: item.description,
            profile_picture: item.profile_picture,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::tables::User> for User {
    fn from(item: web_common::database::tables::User) -> Self {
        Self {
            id: item.id,
            first_name: item.first_name,
            middle_name: item.middle_name,
            last_name: item.last_name,
            description: item.description,
            profile_picture: item.profile_picture,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl User {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users;
        users::dsl::users
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users;
        users::dsl::users
            .order_by(users::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::users;
        users::dsl::users
            .filter(users::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::users;
        users::dsl::users
            .filter(similarity_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .order_by(similarity_dist(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::users;
        users::dsl::users
            .filter(word_similarity_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .order_by(word_similarity_dist_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::users;
        users::dsl::users
            .filter(strict_word_similarity_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .order_by(strict_word_similarity_dist_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_users(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users;
        users::dsl::users
            .filter(can_update_users(author_user_id, users::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users;
        users::dsl::users
            .filter(can_update_users(author_user_id, users::dsl::id))
            .order_by(users::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_updatable(
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(author_user_id, limit, offset, connection);
        }
        use crate::schema::users;
        users::dsl::users
            .filter(can_update_users(author_user_id, users::dsl::id))
            .filter(similarity_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .order_by(similarity_dist(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_updatable(
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(author_user_id, limit, offset, connection);
        }
        use crate::schema::users;
        users::dsl::users
            .filter(can_update_users(author_user_id, users::dsl::id))
            .filter(word_similarity_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .order_by(word_similarity_dist_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_updatable(
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(author_user_id, limit, offset, connection);
        }
        use crate::schema::users;
        users::dsl::users
            .filter(can_update_users(author_user_id, users::dsl::id))
            .filter(strict_word_similarity_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .order_by(strict_word_similarity_dist_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_users(author_user_id, id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users;
        users::dsl::users
            .filter(can_admin_users(author_user_id, users::dsl::id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users;
        users::dsl::users
            .filter(can_admin_users(author_user_id, users::dsl::id))
            .order_by(users::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_administrable(
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(author_user_id, limit, offset, connection);
        }
        use crate::schema::users;
        users::dsl::users
            .filter(can_admin_users(author_user_id, users::dsl::id))
            .filter(similarity_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .order_by(similarity_dist(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_administrable(
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(author_user_id, limit, offset, connection);
        }
        use crate::schema::users;
        users::dsl::users
            .filter(can_admin_users(author_user_id, users::dsl::id))
            .filter(word_similarity_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .order_by(word_similarity_dist_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_administrable(
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(author_user_id, limit, offset, connection);
        }
        use crate::schema::users;
        users::dsl::users
            .filter(can_admin_users(author_user_id, users::dsl::id))
            .filter(strict_word_similarity_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .order_by(strict_word_similarity_dist_op(concat_users_name(users::dsl::first_name, users::dsl::middle_name, users::dsl::last_name), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(self.id, author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
id: i32,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(users::dsl::users
            .filter(users::dsl::id.eq(id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users_users_role_invitations)]
#[diesel(belongs_to(User, foreign_key = table_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct UsersUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<UsersUsersRoleInvitation> for web_common::database::tables::UsersUsersRoleInvitation {
    fn from(item: UsersUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::UsersUsersRoleInvitation> for UsersUsersRoleInvitation {
    fn from(item: web_common::database::tables::UsersUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl UsersUsersRoleInvitation {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_users_users_role_invitations(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&UsersUsersRoleInvitationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_invitations;
        let mut query = users_users_role_invitations::dsl::users_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_users_users_role_invitations(author_user_id, users_users_role_invitations::dsl::table_id, users_users_role_invitations::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&UsersUsersRoleInvitationFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_invitations;
        let mut query = users_users_role_invitations::dsl::users_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_users_users_role_invitations(author_user_id, users_users_role_invitations::dsl::table_id, users_users_role_invitations::dsl::user_id))
            .order_by(users_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::users_users_role_invitations;
        users_users_role_invitations::dsl::users_users_role_invitations
            .filter(users_users_role_invitations::dsl::table_id.eq(table_id))
            .filter(users_users_role_invitations::dsl::user_id.eq(user_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_users_users_role_invitations(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&UsersUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_invitations;
        let mut query = users_users_role_invitations::dsl::users_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_users_users_role_invitations(author_user_id, users_users_role_invitations::dsl::table_id, users_users_role_invitations::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&UsersUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_invitations;
        let mut query = users_users_role_invitations::dsl::users_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_users_users_role_invitations(author_user_id, users_users_role_invitations::dsl::table_id, users_users_role_invitations::dsl::user_id))
            .order_by(users_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_users_users_role_invitations(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&UsersUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_invitations;
        let mut query = users_users_role_invitations::dsl::users_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_users_users_role_invitations(author_user_id, users_users_role_invitations::dsl::table_id, users_users_role_invitations::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&UsersUsersRoleInvitationFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_invitations;
        let mut query = users_users_role_invitations::dsl::users_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_users_users_role_invitations(author_user_id, users_users_role_invitations::dsl::table_id, users_users_role_invitations::dsl::user_id))
            .order_by(users_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.user_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(users_users_role_invitations::dsl::users_users_role_invitations
            .filter(users_users_role_invitations::dsl::table_id.eq(table_id))
            .filter(users_users_role_invitations::dsl::user_id.eq(user_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users_users_role_requests)]
#[diesel(belongs_to(User, foreign_key = table_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct UsersUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<UsersUsersRoleRequest> for web_common::database::tables::UsersUsersRoleRequest {
    fn from(item: UsersUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::UsersUsersRoleRequest> for UsersUsersRoleRequest {
    fn from(item: web_common::database::tables::UsersUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl UsersUsersRoleRequest {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_users_users_role_requests(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&UsersUsersRoleRequestFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_requests;
        let mut query = users_users_role_requests::dsl::users_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_users_users_role_requests(author_user_id, users_users_role_requests::dsl::table_id, users_users_role_requests::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&UsersUsersRoleRequestFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_requests;
        let mut query = users_users_role_requests::dsl::users_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_view_users_users_role_requests(author_user_id, users_users_role_requests::dsl::table_id, users_users_role_requests::dsl::user_id))
            .order_by(users_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, user_id ): ( i32, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::users_users_role_requests;
        users_users_role_requests::dsl::users_users_role_requests
            .filter(users_users_role_requests::dsl::table_id.eq(table_id))
            .filter(users_users_role_requests::dsl::user_id.eq(user_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_users_users_role_requests(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&UsersUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_requests;
        let mut query = users_users_role_requests::dsl::users_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_users_users_role_requests(author_user_id, users_users_role_requests::dsl::table_id, users_users_role_requests::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&UsersUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_requests;
        let mut query = users_users_role_requests::dsl::users_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_users_users_role_requests(author_user_id, users_users_role_requests::dsl::table_id, users_users_role_requests::dsl::user_id))
            .order_by(users_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_users_users_role_requests(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&UsersUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_requests;
        let mut query = users_users_role_requests::dsl::users_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_users_users_role_requests(author_user_id, users_users_role_requests::dsl::table_id, users_users_role_requests::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&UsersUsersRoleRequestFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_role_requests;
        let mut query = users_users_role_requests::dsl::users_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_users_users_role_requests(author_user_id, users_users_role_requests::dsl::table_id, users_users_role_requests::dsl::user_id))
            .order_by(users_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.user_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(users_users_role_requests::dsl::users_users_role_requests
            .filter(users_users_role_requests::dsl::table_id.eq(table_id))
            .filter(users_users_role_requests::dsl::user_id.eq(user_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users_users_roles)]
#[diesel(belongs_to(User, foreign_key = table_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct UsersUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<UsersUsersRole> for web_common::database::tables::UsersUsersRole {
    fn from(item: UsersUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::UsersUsersRole> for UsersUsersRole {
    fn from(item: web_common::database::tables::UsersUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl UsersUsersRole {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&UsersUsersRoleFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_roles;
        let mut query = users_users_roles::dsl::users_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&UsersUsersRoleFilter>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_roles;
        let mut query = users_users_roles::dsl::users_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .order_by(users_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( table_id, user_id ): ( i32, i32 ),
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::users_users_roles;
        users_users_roles::dsl::users_users_roles
            .filter(users_users_roles::dsl::table_id.eq(table_id))
            .filter(users_users_roles::dsl::user_id.eq(user_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_users_users_roles(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&UsersUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_roles;
        let mut query = users_users_roles::dsl::users_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_users_users_roles(author_user_id, users_users_roles::dsl::table_id, users_users_roles::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&UsersUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_roles;
        let mut query = users_users_roles::dsl::users_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_update_users_users_roles(author_user_id, users_users_roles::dsl::table_id, users_users_roles::dsl::user_id))
            .order_by(users_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.table_id, self.user_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_users_users_roles(author_user_id, table_id, user_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&UsersUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_roles;
        let mut query = users_users_roles::dsl::users_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_users_users_roles(author_user_id, users_users_roles::dsl::table_id, users_users_roles::dsl::user_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&UsersUsersRoleFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::users_users_roles;
        let mut query = users_users_roles::dsl::users_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .filter(can_admin_users_users_roles(author_user_id, users_users_roles::dsl::table_id, users_users_roles::dsl::user_id))
            .order_by(users_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.table_id, self.user_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( table_id, user_id ): ( i32, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( table_id, user_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(users_users_roles::dsl::users_users_roles
            .filter(users_users_roles::dsl::table_id.eq(table_id))
            .filter(users_users_roles::dsl::user_id.eq(user_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
