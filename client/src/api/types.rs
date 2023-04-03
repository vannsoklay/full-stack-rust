use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct CtxUser {
    pub is_authenticated: bool,
    pub user: User,
}


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub gender: String,
    pub photo: String,
    pub role: String,
    pub verified: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub status: String,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}