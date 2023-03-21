use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Story {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub story_image: String,
    pub published: Option<bool>,
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
