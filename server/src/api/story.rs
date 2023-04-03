use actix_web::{get, post, put, delete, web, HttpResponse};

use crate::middleware::auth::AuthCheck;
use crate::models::story::{CreateStorySchema, FilterOptions, UpdateStorySchema};
use crate::AppState;
use crate::{repository::story::StoryBMC, utils::Response};

#[get("/story/{id}")]
pub async fn get_story(
    data: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
    context: AuthCheck,
) -> Response<HttpResponse> {
    let user_id = &context.user.id.to_owned().clone();
    StoryBMC::get(&data.db, path.into_inner(), user_id).await
}

#[post("/story/create")]
pub async fn create_story(
    data: web::Data<AppState>,
    body: web::Json<CreateStorySchema>,
    context: AuthCheck,
) -> Response<HttpResponse> {
    let user_id = &context.user.id.to_owned().clone();
    StoryBMC::create(&data.db, body, user_id).await
}

#[get("/stories")]
pub async fn get_story_all(
    data: web::Data<AppState>,
    opts: web::Query<FilterOptions>,
    context: AuthCheck,
) -> Response<HttpResponse> {
    let user_id = &context.user.id.to_owned().clone();
    StoryBMC::get_all(&data.db, opts, user_id).await
}

#[put("/story/{id}")]
pub async fn update_story(
    data: web::Data<AppState>,
    body: web::Json<UpdateStorySchema>,
    path: web::Path<uuid::Uuid>,
    context: AuthCheck,
) -> Response<HttpResponse> {
    let user_id = &context.user.id.to_owned().clone();
    StoryBMC::update(&data.db, body, path.into_inner(), user_id).await
}

#[delete("/story/{id}")]
pub async fn delete_story(
    data: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
    context: AuthCheck,
) -> Response<HttpResponse> {
    let user_id = &context.user.id.to_owned().clone();
    StoryBMC::delete(&data.db, path.into_inner(), user_id).await
}
