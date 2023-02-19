use crate::{
    schema::story::{CreateStorySchema, FilterOptions, Story},
    AppState,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/stories")]
pub async fn show_many_story(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result =
        sqlx::query_as::<_, Story>("SELECT * FROM stories ORDER by id LIMIT $1 OFFSET $2")
            .bind(limit as i32)
            .bind(offset as i32)
            .fetch_all(&data.db)
            .await;

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

#[post("/story")]
async fn create_story(
    body: web::Json<CreateStorySchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as::<_, Story>(
        "INSERT INTO stories (title,content,category) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(body.title.to_string())
    .bind(body.content.to_string())
    .bind(body.category.to_owned().unwrap_or("".to_string()))
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(story) => {
            let story_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "story": story
            })});

            return HttpResponse::Ok().json(story_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Story with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[get("/stories/{id}")]
async fn show_one_story(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let story_id = path.into_inner();
    let query_result = sqlx::query_as::<_, Story>("SELECT * FROM stories WHERE id = $1")
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
