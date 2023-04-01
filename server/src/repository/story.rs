use actix_web::HttpResponse;
use uuid::Uuid;

use crate::utils::{
    handler_response::{Error, ResponseSuccess, Success},
    Response,
};

use crate::schema::story::Story;
use sqlx::{Pool, Postgres};

// #[derive(Deserialize, Serialize, FromRow, Debug)]
// pub struct Story {
//     pub id: Uuid,
//     pub content: String,
//     pub published: Option<bool>,
//     pub user: User,
// }

pub struct StoryBMC;

impl StoryBMC {
    pub async fn get(db: &Pool<Postgres>, story_id: Uuid) -> Response<HttpResponse> {
        let query_result = sqlx::query_as::<_, Story>("SELECT * FROM stories WHERE id = $1")
            .bind(story_id)
            .fetch_one(db)
            .await;

        match query_result {
            Ok(story) => {
                let story = serde_json::json!({ "story": story });
                return Ok(Success::get(story));
            }
            Err(_) => {
                return Err(Error::BadRequest("Can't get data!".to_string()));
            }
        }
    }
}
