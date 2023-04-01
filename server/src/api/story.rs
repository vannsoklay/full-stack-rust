use actix_web::{get, web, HttpResponse};

use crate::AppState;
use crate::{repository::story::StoryBMC, utils::Response};

#[get("/get_story/{id}")]
pub async fn get_story(
    data: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
) -> Response<HttpResponse> {
    StoryBMC::get(&data.db, path.into_inner()).await
}
