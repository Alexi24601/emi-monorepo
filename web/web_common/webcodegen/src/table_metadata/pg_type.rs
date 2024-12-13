//! Submodule providing the `PgType` struct and associated methods.

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable,
    QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use syn::{parse_str, Type};

use crate::errors::WebCodeGenError;

use super::{PgAttribute, PgEnum};

/// Constant listing types supporting `Copy`.
const COPY_TYPES: [&str; 6] = ["i32", "i16", "i64", "f32", "f64", "bool"];

/// Constant listing types supporting `Eq`.
const EQ_TYPES: [&str; 4] = ["i32", "i16", "i64", "bool"];

/// Constant listing types supporting `Hash`.
const HASH_TYPES: [&str; 4] = ["i32", "i16", "i64", "bool"];

#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Hash, Clone)]
#[diesel(table_name = crate::schema::pg_type)]
#[allow(clippy::struct_excessive_bools)]
pub struct PgType {
    pub oid: u32,
    pub typname: String,
    pub typnamespace: u32,
    pub typowner: u32,
    pub typlen: i16,
    pub typbyval: bool,
    pub typtype: String,
    pub typcategory: String,
    pub typispreferred: bool,
    pub typisdefined: bool,
    pub typdelim: String,
    pub typrelid: u32,
    #[cfg(feature = "postgres_17")]
    pub typsubscript: u32,
    pub typelem: u32,
    pub typarray: u32,
    pub typinput: u32,
    pub typoutput: u32,
    pub typreceive: u32,
    pub typsend: u32,
    pub typmodin: u32,
    pub typmodout: u32,
    pub typanalyze: u32,
    pub typalign: String,
    pub typstorage: String,
    pub typnotnull: bool,
    pub typbasetype: u32,
    pub typtypmod: i32,
    pub typndims: i32,
    pub typcollation: u32,
    pub typdefaultbin: Option<Vec<u8>>,
    pub typdefault: Option<String>,
}

/// Returns the Syn rust type of the column.
/// 
/// # Arguments
/// 
/// * `type_name` - The name of the type.
/// 
/// # Returns
/// 
/// A string slice containing the rust type.
/// 
/// # Panics
/// 
/// Panics if the type is not supported.
pub fn rust_type_str<S: AsRef<str>>(type_name: S) -> &'static str {
    match type_name.as_ref() {
        // Numeric types
        "integer" => "i32",
        "smallint" => "i16",
        "bigint" => "i64",
        "real" => "f32",
        "double precision" | "float8" | "numeric" => "f64",
        "money" => "BigDecimal",
        "oid" => "u32",

        // Text types
        "character varying" | "text" | "name" | "xml" | "character" | "char" => "String",

        // Boolean types
        "boolean" => "bool",

        // Temporal types
        "timestamp with time zone" | "timestamp without time zone" => "chrono::NaiveDateTime",
        "date" => "chrono::NaiveDate",
        "time without time zone" | "time with time zone" => "chrono::NaiveTime",
        "interval" => "chrono::Duration",

        // Binary types
        "bytea" | "bit" | "bit varying" => "Vec<u8>",

        // JSON types
        "json" | "jsonb" => "serde_json::Value",

        // Network address types
        "inet" | "cidr" => "std::net::IpAddr",
        "macaddr" | "macaddr8" => "std::net::MacAddr",

        // GIS types
        "point" => "Point",
        "line" => "Line",
        "lseg" => "LineSegment",

        other => todo!("Unsupported data type: {}", other),
    }
}

/// Converts a `PostgreSQL` type to a `Diesel` type.
///
/// # Arguments
///
/// * `postgres_type` - A string slice that holds the `PostgreSQL` type name.
/// * `nullable` - A boolean indicating whether the type is nullable.
///
/// # Returns
///
/// A `Type` representing the corresponding Diesel type.
pub fn postgres_type_to_diesel(postgres_type: &str, nullable: bool) -> Type {
    let rust_type_str = match postgres_type {
        // Numeric types
        "integer" | "int4" => "diesel::sql_types::Integer",
        "smallint" | "int2" => "diesel::sql_types::SmallInt",
        "bigint" | "int8" => "diesel::sql_types::BigInt",
        "real" | "float4" => "diesel::sql_types::Float",
        "double precision" | "float8" => "diesel::sql_types::Double",
        "money" => "diesel::pg::sql_types::Money",

        // Text types
        "text" | "character varying" | "name" | "cstring" => "diesel::sql_types::Text",
        "char" => "diesel::sql_types::CChar",
        "bpchar" => "diesel::sql_types::Bpchar",

        // Boolean types
        "boolean" | "bool" => "diesel::sql_types::Bool",

        // Temporal types
        "timestamp without time zone" | "timestamp" => "diesel::sql_types::Timestamp",
        "timestamp with time zone" | "timestamptz" => "diesel::sql_types::Timestamptz",
        "time" => "diesel::sql_types::Time",
        "date" => "diesel::sql_types::Date",
        "interval" => "diesel::sql_types::Interval",

        // Binary types
        "bytea" => "diesel::sql_types::Bytea",

        // JSON types
        "json" => "diesel::sql_types::Json",
        "jsonb" => "diesel::sql_types::Jsonb",

        // Network address types
        "macaddr" => "diesel::sql_types::Macaddr",
        "inet" => "diesel::sql_types::Inet",

        // Object Identifier types
        "oid" => "diesel::sql_types::Oid",

        // Full-text search types
        "tsvector" => "diesel_full_text_search::TsVector",
        "tsquery" => "diesel_full_text_search::TsQuery",

        // GIS types
        "geometry" | "geography" | "point" | "polygon" | "geometry(Point,4326)" | "line" => "postgis_diesel::sql_types::Geometry",

        _ => todo!("Unsupported data type: '{postgres_type}'"),
    };

    let rust_type_str = if nullable {
        format!("diesel::sql_types::Nullable<{rust_type_str}>")
    } else {
        rust_type_str.to_string()
    };

    parse_str::<Type>(&rust_type_str)
        .expect(format!("Failed to parse rust type: '{rust_type_str}'").as_str())
}


impl PgType {
    /// Returns the Syn rust type of the `PgType`.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing the Syn rust type of the `PgType`, or an error if the type is not supported.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    /// 
    pub fn rust_type(&self, conn: &mut PgConnection) -> Result<Type, WebCodeGenError> {
        if self.is_composite() {
            let struct_name = Ident::new(&self.camelcased_name(), proc_macro2::Span::call_site());
            Ok(parse_str::<Type>(&struct_name.to_string()).unwrap())
        } else if self.is_user_defined(conn)? {
            let Some(base_type) = self.base_type(conn)? else {
                return Err(WebCodeGenError::MissingBaseType(Box::new(self.clone())));
            };
            base_type.rust_type(conn)
        } else {
            Ok(parse_str(rust_type_str(&self.typname)).unwrap())
        }
    }

    /// Returns the Syn postgres type of the `PgType`.
    /// 
    /// # Arguments
    /// 
    /// * `nullable` - A boolean indicating whether the type is nullable.
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing the Syn postgres type of the `PgType`, or an error if the type is not supported.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    /// 
    pub fn diesel_type(
        &self,
        nullable: bool,
        conn: &mut PgConnection,
    ) -> Result<Type, WebCodeGenError> {
        if self.is_composite() {
            Ok(parse_str::<Type>(&format!("crate::Pg{}", &self.camelcased_name())).unwrap())
        } else if self.is_user_defined(conn)? {
            let base_type = self.base_type(conn)?;
            if let Some(base_type) = base_type {
                base_type.diesel_type(nullable, conn)
            } else {
                return Err(WebCodeGenError::MissingBaseType(Box::new(self.clone())));
            }
        } else {
            Ok(postgres_type_to_diesel(&self.typname, nullable))
        }
    }

    /// Returns the internal custom types of the `PgType`, if any.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing the internal custom types of the `PgType`, or an error if the type is not supported.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    /// 
    pub fn internal_custom_types(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgType>, WebCodeGenError> {
        let mut internal_custom_types = Vec::new();
        for attribute in self.attributes(conn)? {
            let pg_type = attribute.pg_type(conn)?;
            if pg_type.is_composite() || pg_type.is_enum(){
                internal_custom_types.extend(pg_type.internal_custom_types(conn)?);
                internal_custom_types.push(pg_type);
            }
        }

        Ok(internal_custom_types)
    }

    /// Returns the Type Base Type of the `PgType`.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing the Type Base Type of the `PgType`, or an error if the type is not supported.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    pub fn base_type(&self, conn: &mut PgConnection) -> Result<Option<PgType>, WebCodeGenError> {
        if self.typbasetype == 0 {
            Ok(None)
        } else {
            use crate::schema::pg_type;
            Ok(pg_type::table
                .filter(pg_type::oid.eq(self.typbasetype))
                .first::<PgType>(conn)
                .ok())
        }
    }

    /// Returns whether the Postgres type is a user-defined type.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing a boolean indicating whether the Postgres type is a user-defined type, or an error if the type is not supported.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    pub fn is_user_defined(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(&self.typcategory == "U" && self.base_type(conn)?.is_some())
    }

    #[must_use]
    /// Returns whether the Postgres type is a composite type.
    pub fn is_composite(&self) -> bool {
        &self.typcategory == "C"
    }

    #[must_use]
    /// Returns whether the Postgres type is an enum type.
    pub fn is_enum(&self) -> bool {
        &self.typcategory == "E"
    }

    /// Returns whether the associated rust type supports `Copy`.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing a boolean indicating whether the associated rust type supports `Copy`, or an error if the type is not supported.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    pub fn supports_copy(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_composite() {
            self.attributes(conn)?
                .into_iter()
                .try_fold(true, |acc, attribute| {
                    attribute.supports_copy(conn).map(|b| acc && b)
                })
        } else if self.is_user_defined(conn)? {
            self.base_type(conn)?
                .ok_or(WebCodeGenError::MissingBaseType(Box::new(self.clone())))
                .and_then(|base_type| base_type.supports_copy(conn))
        } else {
            Ok(COPY_TYPES.contains(&rust_type_str(&self.typname)))
        }
    }

    /// Returns whether the associated rust type supports `Hash`.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing a boolean indicating whether the associated rust type supports `Hash`, or an error if the type is not supported.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    pub fn supports_hash(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_user_defined(conn)? || self.is_composite() {
            self.attributes(conn)?
                .into_iter()
                .try_fold(true, |acc, attribute| {
                    attribute.supports_hash(conn).map(|b| acc && b)
                })
        } else {
            Ok(HASH_TYPES.contains(&rust_type_str(&self.typname)))
        }
    }

    /// Returns whether the associated rust type supports `Eq`.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing a boolean indicating whether the associated rust type supports `Eq`, or an error if the type is not supported.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    pub fn supports_eq(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_user_defined(conn)? || self.is_composite() {
            self.attributes(conn)?
                .into_iter()
                .try_fold(true, |acc, attribute| {
                    attribute.supports_eq(conn).map(|b| acc && b)
                })
        } else {
            Ok(EQ_TYPES.contains(&rust_type_str(&self.typname)))
        }
    }

    #[must_use]
    /// Returns the `CamelCased` name of the `PgType`.
    pub fn camelcased_name(&self) -> String {
        self.typname
            .split('_')
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                }
            })
            .collect()
    }

    #[must_use]
    /// Returns the `CamelCased` name of the `PgType` for the Postgres binding.
    pub fn pg_binding_name(&self) -> String {
        format!("Pg{}", self.camelcased_name())
    }

    /// Returns the syn of the struct or enum associated to the `PgType`.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing the syn of the struct or enum associated to the `PgType`, or an error if the type is not supported.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    pub fn to_syn(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        if self.is_composite() {
            let struct_name = Ident::new(&self.camelcased_name(), proc_macro2::Span::call_site());
            let mut fields = Vec::new();
            let mut diesel_types = Vec::new();
            let mut rust_types = Vec::new();
            let mut struct_attributes = Vec::new();
            let mut field_names = Vec::new();
            let attributes = self.attributes(conn)?;
            for attribute in &attributes {
                let field_name = Ident::new(&attribute.attname, proc_macro2::Span::call_site());
                let field_pg_type = attribute.pg_type(conn)?;
                let field_type = field_pg_type.rust_type(conn)?;
                field_names.push(quote! {
                    #field_name
                });
                rust_types.push(quote! {
                    #field_type
                });
                let diesel_type = field_pg_type.diesel_type(attribute.attnotnull, conn)?;
                if field_pg_type.supports_copy(conn)? || attributes.len() == 1 {
                    struct_attributes.push(quote! {
                        self.#field_name
                    });
                } else {
                    struct_attributes.push(quote! {
                        self.#field_name.clone()
                    });
                }

                fields.push(quote! {
                    pub #field_name: #field_type
                });
                diesel_types.push(quote! {
                    #diesel_type
                });
            }

            let this_typname: &str = &self.typname;
            let postgres_struct_name =
                Ident::new(&self.pg_binding_name(), proc_macro2::Span::call_site());

            let mut derives = vec![
                Ident::new("Debug", proc_macro2::Span::call_site()),
                Ident::new("Clone", proc_macro2::Span::call_site()),
                Ident::new("PartialEq", proc_macro2::Span::call_site()),
            ];

            if self.supports_eq(conn)? {
                derives.push(Ident::new("Eq", proc_macro2::Span::call_site()));
            }

            if self.supports_hash(conn)? {
                derives.push(Ident::new("Hash", proc_macro2::Span::call_site()));
            }

            if self.supports_copy(conn)? {
                derives.push(Ident::new("Copy", proc_macro2::Span::call_site()));
            }

            let to_sql_operation = if diesel_types.len() > 1 {
                quote! {
                    diesel::serialize::WriteTuple::<(#(#diesel_types),*)>::write_tuple(
                        &(#(#struct_attributes),*),
                        &mut out.reborrow(),
                    )
                }
            } else {
                quote! {
                    diesel::serialize::ToSql::<#(#diesel_types)*, diesel::pg::Pg>::to_sql(
                        &#(#struct_attributes)*,
                        out,
                    )
                }
            };

            let from_sql_ops = if diesel_types.len() > 1 {
                quote! {
                    let (#(#field_names),*): (#(#rust_types),*) = diesel::deserialize::FromSql::<diesel::sql_types::Record<(#(#diesel_types),*)>, diesel::pg::Pg>::from_sql(bytes)?;
                    Ok(Self {
                        #(#field_names),*
                    })
                }
            } else {
                quote! {
                    let #(#field_names)*: #(#rust_types),* = diesel::deserialize::FromSql::<#(#diesel_types)*, diesel::pg::Pg>::from_sql(bytes)?;
                    Ok(Self {
                        #(#field_names),*
                    })
                }
            };

            Ok(quote! {
                #[cfg(feature = "diesel")]
                #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
                #[diesel(postgres_type(name = #this_typname))]
                pub struct #postgres_struct_name;

                #[derive(#(#derives),*)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                #[cfg_attr(feature = "diesel", derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression))]
                #[cfg_attr(feature = "diesel", diesel(sql_type = #postgres_struct_name))]
                pub struct #struct_name {
                    #(#fields),*
                }

                #[cfg(feature = "diesel")]
                impl diesel::serialize::ToSql<#postgres_struct_name, diesel::pg::Pg> for #struct_name {
                    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
                        #to_sql_operation
                    }
                }

                #[cfg(feature = "diesel")]
                impl diesel::deserialize::FromSql<#postgres_struct_name, diesel::pg::Pg> for #struct_name {
                    fn from_sql(
                        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
                    ) -> diesel::deserialize::Result<Self> {
                        #from_sql_ops
                    }
                }
            })
        } else if self.is_enum() {
            let struct_name = Ident::new(&self.camelcased_name(), proc_macro2::Span::call_site());
            let variants = self.variants(conn)?;
            let mut variant_names = Vec::new();
            let mut in_variants = Vec::new();
            let mut out_variants = Vec::new();
            for variant in &variants {
                let variant_name = Ident::new(&variant.enumlabel, proc_macro2::Span::call_site());
                variant_names.push(quote! {
                    #variant_name
                });
                let variant = variant.enumlabel.clone();
                in_variants.push(quote! {
                    #variant => Ok(#struct_name::#variant_name),
                });
                out_variants.push(quote! {
                    #struct_name::#variant_name => std::io::Write::write_all(out, #variant.as_bytes())?,         
                });
            }

            let this_typname: &str = &self.typname;
            let postgres_enum_name =
                Ident::new(&self.pg_binding_name(), proc_macro2::Span::call_site());

            Ok(quote! {
                #[cfg(feature = "diesel")]
                #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
                #[diesel(postgres_type(name = #this_typname))]
                pub struct #postgres_enum_name;

                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                #[cfg_attr(feature = "diesel", derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression))]
                #[cfg_attr(feature = "diesel", diesel(sql_type = #postgres_enum_name))]
                pub enum #struct_name {
                    #(#variant_names),*
                }

                #[cfg(feature = "diesel")]
                impl diesel::serialize::ToSql<#postgres_enum_name, diesel::pg::Pg> for #struct_name {
                    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
                        match *self {
                            #(#out_variants)*
                        }
                        Ok(diesel::serialize::IsNull::No)
                    }
                }

                #[cfg(feature = "diesel")]
                impl diesel::deserialize::FromSql<#postgres_enum_name, diesel::pg::Pg> for #struct_name {
                    fn from_sql(
                        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
                    ) -> diesel::deserialize::Result<Self> {
                        let s: String = diesel::deserialize::FromSql::<diesel::sql_types::Text, diesel::pg::Pg>::from_sql(bytes)?;
                        match s.as_str() {
                            #(#in_variants)*
                            unknown => Err(format!("Unknown variant: {}", unknown).into()),
                        }
                    }
                }
            })

        } else {
            panic!("Unsupported type: {self:?}");
        }
    }

    /// Returns a new `PgType` struct from the given type name.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The name of the type.
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the `PgType` struct if the type exists, or an error if it does not.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    ///
    pub fn from_name(type_name: &str, conn: &mut PgConnection) -> Result<Self, WebCodeGenError> {
        use crate::schema::pg_type;
        Ok(pg_type::table
            .filter(pg_type::typname.eq(type_name))
            .first::<PgType>(conn)?)
    }

    /// Returns the attributes of the type, if it is a composite type.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing the attributes of the type if it is a composite type, or an error if it is not.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    pub fn attributes(&self, conn: &mut PgConnection) -> Result<Vec<PgAttribute>, WebCodeGenError> {
        use crate::schema::pg_attribute;
        use crate::schema::pg_type;

        Ok(pg_attribute::table
            .inner_join(
                pg_type::table.on(pg_attribute::attrelid.eq(pg_type::typrelid)),
            )
            .filter(
                pg_type::typname
                    .eq(&self.typname)
                    .and(pg_attribute::attisdropped.eq(false)),
            )
            .select(PgAttribute::as_select())
            .load::<PgAttribute>(conn)?)
    }

    /// Returns the variants of the type, if it is an enum type.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - The Postgres connection.
    /// 
    /// # Returns
    /// 
    /// A Result containing the variants of the type if it is an enum type, or an error if it is not.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the provided database connection fails.
    pub fn variants(&self, conn: &mut PgConnection) -> Result<Vec<PgEnum>, WebCodeGenError> {
        use crate::schema::pg_enum;

        Ok(pg_enum::table
            .filter(pg_enum::enumtypid.eq(self.oid))
            .order_by(pg_enum::enumsortorder)
            .select(PgEnum::as_select())
            .load::<PgEnum>(conn)?)
    }
}
