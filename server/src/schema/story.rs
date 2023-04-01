#![allow(unused)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::error::BoxDynError;
use sqlx::postgres::types::PgRecordDecoder;
use sqlx::postgres::{self, *};
use sqlx::Decode;
use sqlx::Postgres;
use sqlx::Type;
use sqlx::{types::time, FromRow, QueryBuilder, Row};
use uuid::Uuid;

// use crate::models::user::User;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateStorySchema {
    pub content: String,
    pub story_image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateStorySchema {
    pub content: Option<String>,
    pub story_image: Option<String>,
    pub category: Option<String>,
    pub published: Option<bool>,
}

// *** model ***

// #[allow(non_snake_case)]
// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct User {
//     pub id: uuid::Uuid,
//     pub name: String,
//     pub email: String,
//     pub phone_number: String,
//     pub gender: String,
//     pub password: String,
//     pub role: String,
//     pub photo: String,
//     pub verified: bool,
//     #[serde(rename = "createdAt")]
//     pub created_at: Option<DateTime<Utc>>,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: Option<DateTime<Utc>>,
// }
#[allow(non_snake_case)]
#[derive(Debug, FromRow, Deserialize, Serialize, Clone, sqlx::Type)]
pub struct User {
    pub name: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Story {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub story_image: String,
    pub published: Option<bool>,
    // pub user: User,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

// impl FromRow<'_, PgRow> for Story {
//     fn from_row(row: &PgRow) -> sqlx::Result<Self> {
//         row.try_get::<User, &str>("user")?; // Just simplified here
//         todo!()
//     }
// }

// //* Comment these two impls will raise: the trait bound xxx is not satisfied
// impl Type<Postgres> for Story {
//     fn type_info() -> PgTypeInfo {
//         PgTypeInfo::with_name("User")
//     }
// }

// impl<'r> Decode<'r, Postgres> for User {
//     fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
//         let mut decoder = PgRecordDecoder::new(value)?;

//         //$(let $idx: $T = decoder.try_decode()?;)*
//         //Ok(($($idx,)*))
//         todo!()
//     }
// }
