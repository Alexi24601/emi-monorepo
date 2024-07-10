use crate::database::*;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedTaxon {
    pub inner: crate::database::flat_variants::Taxon,
    pub ott_rank: crate::database::nested_variants::NestedRank,
    pub domain: Option<crate::database::flat_variants::Taxon>,
    pub kingdom: Option<crate::database::flat_variants::Taxon>,
    pub phylum: Option<crate::database::flat_variants::Taxon>,
    pub class: Option<crate::database::flat_variants::Taxon>,
    pub order: Option<crate::database::flat_variants::Taxon>,
    pub family: Option<crate::database::flat_variants::Taxon>,
    pub genus: Option<crate::database::flat_variants::Taxon>,
    pub parent: crate::database::flat_variants::Taxon,
    pub icon: crate::database::flat_variants::FontAwesomeIcon,
    pub color: crate::database::flat_variants::Color,
}

unsafe impl Send for NestedTaxon {}
unsafe impl Sync for NestedTaxon {}
impl NestedTaxon {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `connection` - The database connection.
    pub(crate) fn from_flat(
        flat_variant: Taxon,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        Ok(Self {
            ott_rank: crate::database::nested_variants::NestedRank::get(
                flat_variant.ott_rank_id,
                connection,
            )?,
            domain: flat_variant
                .domain_id
                .map(|domain_id| crate::database::flat_variants::Taxon::get(domain_id, connection))
                .transpose()?,
            kingdom: flat_variant
                .kingdom_id
                .map(|kingdom_id| {
                    crate::database::flat_variants::Taxon::get(kingdom_id, connection)
                })
                .transpose()?,
            phylum: flat_variant
                .phylum_id
                .map(|phylum_id| crate::database::flat_variants::Taxon::get(phylum_id, connection))
                .transpose()?,
            class: flat_variant
                .class_id
                .map(|class_id| crate::database::flat_variants::Taxon::get(class_id, connection))
                .transpose()?,
            order: flat_variant
                .order_id
                .map(|order_id| crate::database::flat_variants::Taxon::get(order_id, connection))
                .transpose()?,
            family: flat_variant
                .family_id
                .map(|family_id| crate::database::flat_variants::Taxon::get(family_id, connection))
                .transpose()?,
            genus: flat_variant
                .genus_id
                .map(|genus_id| crate::database::flat_variants::Taxon::get(genus_id, connection))
                .transpose()?,
            parent: crate::database::flat_variants::Taxon::get(flat_variant.parent_id, connection)?,
            icon: crate::database::flat_variants::FontAwesomeIcon::get(
                flat_variant.icon_id,
                connection,
            )?,
            color: crate::database::flat_variants::Color::get(flat_variant.color_id, connection)?,
            inner: flat_variant,
        })
    }
    /// Check whether the user can view the struct.
    pub fn can_view(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_view()
    }
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id() -> Result<bool, web_common::api::ApiError> {
        Taxon::can_view_by_id()
    }
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        filter: Option<&web_common::database::filter_variants::TaxonFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        Taxon::all_viewable(filter, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, connection))
            .collect()
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable_sorted(
        filter: Option<&web_common::database::filter_variants::TaxonFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        Taxon::all_viewable_sorted(filter, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, connection))
            .collect()
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn get(
        id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        Taxon::get(id, connection)
            .and_then(|flat_variant| Self::from_flat(flat_variant, connection))
    }
    /// Get the struct from the database by its ott_id.
    ///
    /// * `ott_id` - The ott_id of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn from_ott_id(
        ott_id: &i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        Taxon::from_ott_id(ott_id, connection)
            .and_then(|flat_variant| Self::from_flat(flat_variant, connection))
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_viewable(
        filter: Option<&web_common::database::filter_variants::TaxonFilter>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        Taxon::strict_word_similarity_search_viewable(filter, query, limit, offset, connection)?
            .into_iter()
            .map(|flat_variant| Self::from_flat(flat_variant, connection))
            .collect()
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_with_score_viewable(
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<(Self, f32)>, web_common::api::ApiError> {
        Taxon::strict_word_similarity_search_with_score_viewable(query, limit, offset, connection)?
            .into_iter()
            .map(|(flat_variant, score)| Ok((Self::from_flat(flat_variant, connection)?, score)))
            .collect()
    }
    /// Check whether the user can update the struct.
    pub fn can_update(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_update()
    }
    /// Check whether the user can update the struct associated to the provided ids.
    pub fn can_update_by_id() -> Result<bool, web_common::api::ApiError> {
        Taxon::can_update_by_id()
    }
    /// Check whether the user can admin the struct.
    pub fn can_admin(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_admin()
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    pub fn can_admin_by_id() -> Result<bool, web_common::api::ApiError> {
        Taxon::can_admin_by_id()
    }
}
impl From<web_common::database::nested_variants::NestedTaxon>
    for crate::database::nested_variants::NestedTaxon
{
    fn from(item: web_common::database::nested_variants::NestedTaxon) -> Self {
        Self {
            inner: crate::database::flat_variants::Taxon::from(item.inner.as_ref().clone()),
            ott_rank: crate::database::nested_variants::NestedRank::from(
                item.ott_rank.as_ref().clone(),
            ),
            domain: item
                .domain
                .as_deref()
                .cloned()
                .map(crate::database::flat_variants::Taxon::from),
            kingdom: item
                .kingdom
                .as_deref()
                .cloned()
                .map(crate::database::flat_variants::Taxon::from),
            phylum: item
                .phylum
                .as_deref()
                .cloned()
                .map(crate::database::flat_variants::Taxon::from),
            class: item
                .class
                .as_deref()
                .cloned()
                .map(crate::database::flat_variants::Taxon::from),
            order: item
                .order
                .as_deref()
                .cloned()
                .map(crate::database::flat_variants::Taxon::from),
            family: item
                .family
                .as_deref()
                .cloned()
                .map(crate::database::flat_variants::Taxon::from),
            genus: item
                .genus
                .as_deref()
                .cloned()
                .map(crate::database::flat_variants::Taxon::from),
            parent: crate::database::flat_variants::Taxon::from(item.parent.as_ref().clone()),
            icon: crate::database::flat_variants::FontAwesomeIcon::from(item.icon.as_ref().clone()),
            color: crate::database::flat_variants::Color::from(item.color.as_ref().clone()),
        }
    }
}
impl From<crate::database::nested_variants::NestedTaxon>
    for web_common::database::nested_variants::NestedTaxon
{
    fn from(item: crate::database::nested_variants::NestedTaxon) -> Self {
        Self {
            inner: Rc::from(web_common::database::flat_variants::Taxon::from(item.inner)),
            ott_rank: Rc::from(web_common::database::nested_variants::NestedRank::from(
                item.ott_rank,
            )),
            domain: item
                .domain
                .map(web_common::database::flat_variants::Taxon::from)
                .map(Rc::from),
            kingdom: item
                .kingdom
                .map(web_common::database::flat_variants::Taxon::from)
                .map(Rc::from),
            phylum: item
                .phylum
                .map(web_common::database::flat_variants::Taxon::from)
                .map(Rc::from),
            class: item
                .class
                .map(web_common::database::flat_variants::Taxon::from)
                .map(Rc::from),
            order: item
                .order
                .map(web_common::database::flat_variants::Taxon::from)
                .map(Rc::from),
            family: item
                .family
                .map(web_common::database::flat_variants::Taxon::from)
                .map(Rc::from),
            genus: item
                .genus
                .map(web_common::database::flat_variants::Taxon::from)
                .map(Rc::from),
            parent: Rc::from(web_common::database::flat_variants::Taxon::from(
                item.parent,
            )),
            icon: Rc::from(web_common::database::flat_variants::FontAwesomeIcon::from(
                item.icon,
            )),
            color: Rc::from(web_common::database::flat_variants::Color::from(item.color)),
        }
    }
}
