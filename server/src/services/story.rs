use crate::schema::story::User;
use crate::utils::handler_error::ServiceError;
use crate::{
    middleware::auth::AuthCheck,
    schema::story::{CreateStorySchema, FilterOptions, Story},
    AppState,
};

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[derive(Debug, FromRow)]
struct RTest {
    name: String,
}
#[derive(Debug, FromRow)]
struct Test {
    content: String,
    user: RTest,
}

#[get("/find/all_story")]
pub async fn find_all_story(
    opts: web::Query<FilterOptions>,
    context: AuthCheck,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as::<_, Story>("SELECT * FROM stories")
        .bind(limit as i32)
        .bind(offset as i32)
        .bind(&context.user.id.to_owned().clone())
        .fetch_all(&data.db)
        .await;
    println!("{}", context.user.id.to_owned().clone());
    let select_query = sqlx::query("SELECT content, users.name FROM stories, users WHERE users.id=$1").bind(context.user.id.to_owned().clone());
    let tickets = select_query
        .map(|row: PgRow| Test {
            content: row.get("content"),
            user: RTest { name: row.get("name") },
        })
        .fetch_all(&data.db)
        .await;

    println!("data {:?}", tickets);
    if query_result.is_err() {
        let message = "Something bad happened while fetching all stories items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let stories = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": stories.len(),
        "stories": stories
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/create/story")]
async fn create_story(
    body: web::Json<CreateStorySchema>,
    context: AuthCheck,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ServiceError> {
    println!("user id{}", &context.user.id);
    let query_result = sqlx::query_as::<_, Story>(
        "INSERT INTO stories (user_id, content, story_image, published) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(&context.user.id)
    .bind(body.content.to_string())
    .bind(body.story_image.to_string())
    .bind(body.published.to_owned().unwrap_or(false))
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(story) => {
            let story = serde_json::json!({"status": 200,"data": serde_json::json!({
                "story": story
            })});

            Ok(HttpResponse::Ok().json(&story))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return Err(ServiceError::BadRequest(
                    "User with that title already exists".to_string(),
                ));
            }

            return Err(ServiceError::InternalServerError(format!(
                "Please try later"
            )));
        }
    }
}

#[get("/story/{id}")]
async fn find_story(
    path: web::Path<uuid::Uuid>,
    context: AuthCheck,
    data: web::Data<AppState>,
) -> impl Responder {
    let story_id = path.into_inner();
    let query_result =
        sqlx::query_as::<_, Story>("SELECT stories.id, stories.user_id, stories.content, stories.story_image, stories.published, stories.name, (user.name) as user!: User FROM stories, users WHERE id = $1")
            .bind(story_id)
            .fetch_one(&data.db)
            .await;

    match query_result {
        Ok(story) => {
            let story_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "story": story
            })});

            return HttpResponse::Ok().json(story_response);
        }
        Err(_) => {
            let message = format!("Story with ID: {} not found", story_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}
