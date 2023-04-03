use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    models::story::UpdateStorySchema,
    utils::{
        handler_response::{Error, ResponseSuccess, Success},
        Response,
    },
};

use crate::models::story::{CreateStorySchema, FilterOptions, Story};
use chrono::Utc;
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
    pub async fn get(db: &Pool<Postgres>, story_id: Uuid, context: &Uuid) -> Response<HttpResponse> {
        let query_result = sqlx::query_as::<_, Story>("SELECT * FROM stories WHERE id = $1 AND user_id = $2;")
            .bind(story_id)
            .bind(context)
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
    pub async fn create(
        db: &Pool<Postgres>,
        body: web::Json<CreateStorySchema>,
        context: &Uuid,
    ) -> Response<HttpResponse> {
        let query_result = sqlx::query_as::<_, Story>("INSERT INTO stories (user_id, content, story_image, published) VALUES ($1, $2, $3, $4) RETURNING *")
            .bind(context)
            .bind(body.content.to_string())
            .bind(body.story_image.to_string())
            .bind(body.published.to_owned().unwrap_or(false))
            .fetch_one(db)
            .await;

        match query_result {
            Ok(story) => {
                let story = serde_json::json!({ "story": story });
                return Ok(Success::get(story));
            }
            Err(_) => {
                return Err(Error::BadRequest("Can't insert data!".to_string()));
            }
        }
    }

    pub async fn update(
        db: &Pool<Postgres>,
        body: web::Json<UpdateStorySchema>,
        story_id: Uuid,
        context: &Uuid,
    ) -> Response<HttpResponse> {
        let find_result = sqlx::query_as!(Story, "SELECT * FROM stories WHERE id = $1 AND user_id = $2;", story_id, context)
            .fetch_one(db)
            .await;

        if find_result.is_err() {
            return Err(Error::BadRequest("Invalid update story!".to_string()));
        }
        let story = find_result.unwrap();
        let now = Utc::now();

        let update_result = sqlx::query_as!(Story, "UPDATE stories SET content=$1, story_image=$2, published=$3, updated_at=$4 WHERE id=$5 RETURNING *;", 
        body.content.to_owned().unwrap_or(story.content),
        body.story_image.to_owned().unwrap_or(story.story_image),
        body.published.unwrap_or(story.published.unwrap()),
        now,
        story_id
        ).fetch_one(db).await;
        match update_result {
            Ok(story) => {
                let story = serde_json::json!({ "story": story });
                return Ok(Success::get(story));
            }
            Err(_) => {
                return Err(Error::BadRequest("Can't get data!".to_string()));
            }
        }
    }

    pub async fn delete(
        db: &Pool<Postgres>,
            story_id: Uuid,
            context: &Uuid,
    ) -> Response<HttpResponse> {
        StoryBMC::get(db, story_id, context).await?;
        let rows_affected = sqlx::query!("DELETE FROM stories WHERE id = $1", story_id).execute(db).await.unwrap().rows_affected();
        
        if rows_affected == 0 {
            return Err(Error::NotFound(format!("Story not found")));
        }

        Ok(Success::message(format!("Delete successfully!")))
    }

    pub async fn get_all(
        db: &Pool<Postgres>,
        opts: web::Query<FilterOptions>,
        context: &Uuid,
    ) -> Response<HttpResponse> {
        let limit = opts.limit.unwrap_or(10);
        let offset = (opts.page.unwrap_or(1) - 1) * limit;

        let query_result = sqlx::query_as::<_, Story>(
            "SELECT * FROM stories WHERE user_id=$3 ORDER BY created_at DESC LIMIT $1 OFFSET $2;",
        )
        .bind(limit as i32)
        .bind(offset as i32)
        .bind(context)
        .fetch_all(db)
        .await;

        match query_result {
            Ok(stories) => {
                return Ok(Success::get(
                    serde_json::json!({ "results": stories.len(), "stories": stories }),
                ));
            }
            Err(_) => {
                return Err(Error::BadRequest("Can't get data!".to_string()));
            }
        }
    }
}
