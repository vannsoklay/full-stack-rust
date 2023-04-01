use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String, 
    pub photo: String,
    pub gender: String
}