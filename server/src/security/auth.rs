use crate::{
    middleware::auth::AuthCheck,
    models::user::{LoginUserSchema, RegisterUserSchema, User},
    schema::user::filter_user_record,
    utils::{handler_error::ServiceError, handler_jwt, hash_password, verify},
    AppState,
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpRequest, HttpResponse, Responder,
};
use redis::AsyncCommands;
use sqlx::Row;
use uuid::Uuid;

#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ServiceError> {
    let exists: bool = sqlx::query("SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)")
        .bind(body.email.to_owned())
        .fetch_one(&data.db)
        .await
        .unwrap()
        .get(0);

    if exists {
        return Ok(HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        ));
    }

    let hashed_password = hash_password(&body.password).await?;
    let query_result = sqlx::query_as::<_, User>(
        "INSERT INTO users (name,email,phone_number,password) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(body.name.to_string())
    .bind(body.email.to_string().to_lowercase())
    .bind(body.phone_number.to_string())
    .bind(hashed_password)
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": filter_user_record(&user)
            })});

            return Ok(HttpResponse::Ok().json(user_response));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format_args!("{:?}", e)})));
        }
    }
}
#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ServiceError> {
    let query_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(body.email.to_string())
        .fetch_optional(&data.db)
        .await
        .unwrap();

    let user = match query_result {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::BadRequest().json(
                serde_json::json!({"status": "fail", "message": "Invalid email or password"}),
            ));
        }
    };

    let is_valid = verify(&user.password, &body.password).await?;

    if !is_valid {
        return Ok(HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "fail", "message": "Invalid email or password"})));
    }

    let access_token_details = match handler_jwt::generate_jwt_token(
        user.id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    ) {
        Ok(token_details) => token_details,
        Err(e) => {
            return Ok(HttpResponse::BadGateway()
                .json(serde_json::json!({"status": "fail", "message": format_args!("{}", e)})));
        }
    };

    let refresh_token_details = match handler_jwt::generate_jwt_token(
        user.id,
        data.env.refresh_token_max_age,
        data.env.refresh_token_private_key.to_owned(),
    ) {
        Ok(token_details) => token_details,
        Err(e) => {
            return Ok(HttpResponse::BadGateway()
                .json(serde_json::json!({"status": "fail", "message": format_args!("{}", e)})));
        }
    };

    let mut redis_client = match data.redis_client.get_async_connection().await {
        Ok(redis_client) => redis_client,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "fail", "message": format_args!("{}", e)})));
        }
    };

    let access_result: redis::RedisResult<()> = redis_client
        .set_ex(
            access_token_details.token_uuid.to_string(),
            user.id.to_string(),
            (data.env.access_token_max_age * 60) as usize,
        )
        .await;

    if let Err(e) = access_result {
        return Ok(HttpResponse::UnprocessableEntity()
            .json(serde_json::json!({"status": "error", "message": format_args!("{}", e)})));
    }

    let refresh_result: redis::RedisResult<()> = redis_client
        .set_ex(
            refresh_token_details.token_uuid.to_string(),
            user.id.to_string(),
            (data.env.refresh_token_max_age * 60) as usize,
        )
        .await;

    if let Err(e) = refresh_result {
        return Ok(HttpResponse::UnprocessableEntity()
            .json(serde_json::json!({"status": "error", "message": format_args!("{}", e)})));
    }

    let access_cookie = Cookie::build("access_token", access_token_details.token.clone().unwrap())
        .path("/")
        .max_age(ActixWebDuration::new(data.env.access_token_max_age * 60, 0))
        .http_only(true)
        .finish();
    let refresh_cookie = Cookie::build("refresh_token", refresh_token_details.token.unwrap())
        .path("/")
        .max_age(ActixWebDuration::new(
            data.env.refresh_token_max_age * 60,
            0,
        ))
        .http_only(true)
        .finish();
    let logged_in_cookie = Cookie::build("logged_in", "true")
        .path("/")
        .max_age(ActixWebDuration::new(data.env.access_token_max_age * 60, 0))
        .http_only(false)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .cookie(logged_in_cookie)
        .json(serde_json::json!({"status": "success", "access_token": access_token_details.token.unwrap()})))
}

#[get("/auth/refresh")]
async fn refresh_access_token_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    let message = "could not refresh access token";

    let refresh_token = match req.cookie("refresh_token") {
        Some(c) => c.value().to_string(),
        None => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    };

    let refresh_token_details = match handler_jwt::verify_jwt_token(
        data.env.refresh_token_public_key.to_owned(),
        &refresh_token,
    ) {
        Ok(token_details) => token_details,
        Err(e) => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"status": "fail", "message": format_args!("{:?}", e)}));
        }
    };

    let result = data.redis_client.get_async_connection().await;
    let mut redis_client = match result {
        Ok(redis_client) => redis_client,
        Err(e) => {
            return HttpResponse::Forbidden().json(
                serde_json::json!({"status": "fail", "message": format!("Could not connect to Redis: {}", e)}),
            );
        }
    };
    let redis_result: redis::RedisResult<String> = redis_client
        .get(refresh_token_details.token_uuid.to_string())
        .await;

    let user_id = match redis_result {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    };

    let user_id_uuid = Uuid::parse_str(&user_id).unwrap();
    let query_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id_uuid)
        .fetch_optional(&data.db)
        .await
        .unwrap();

    if query_result.is_none() {
        return HttpResponse::Forbidden()
            .json(serde_json::json!({"status": "fail", "message": "the user belonging to this token no logger exists"}));
    }

    let user = query_result.unwrap();

    let access_token_details = match handler_jwt::generate_jwt_token(
        user.id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    ) {
        Ok(token_details) => token_details,
        Err(e) => {
            return HttpResponse::BadGateway()
                .json(serde_json::json!({"status": "fail", "message": format_args!("{:?}", e)}));
        }
    };

    let redis_result: redis::RedisResult<()> = redis_client
        .set_ex(
            access_token_details.token_uuid.to_string(),
            user.id.to_string(),
            (data.env.access_token_max_age * 60) as usize,
        )
        .await;

    if redis_result.is_err() {
        return HttpResponse::UnprocessableEntity().json(
            serde_json::json!({"status": "error", "message": format_args!("{:?}", redis_result.unwrap_err())}),
        );
    }

    let access_cookie = Cookie::build("access_token", access_token_details.token.clone().unwrap())
        .path("/")
        .max_age(ActixWebDuration::new(data.env.access_token_max_age * 60, 0))
        .http_only(true)
        .finish();

    let logged_in_cookie = Cookie::build("logged_in", "true")
        .path("/")
        .max_age(ActixWebDuration::new(data.env.access_token_max_age * 60, 0))
        .http_only(false)
        .finish();

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(logged_in_cookie)
        .json(serde_json::json!({"status": "success", "access_token": access_token_details.token.unwrap()}))
}

#[get("/auth/logout")]
async fn logout_handler(
    req: HttpRequest,
    auth_guard: AuthCheck,
    data: web::Data<AppState>,
) -> impl Responder {
    let message = "Token is invalid or session has expired";

    let refresh_token = match req.cookie("refresh_token") {
        Some(c) => c.value().to_string(),
        None => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    };

    let refresh_token_details = match handler_jwt::verify_jwt_token(
        data.env.refresh_token_public_key.to_owned(),
        &refresh_token,
    ) {
        Ok(token_details) => token_details,
        Err(e) => {
            return HttpResponse::Forbidden()
                .json(serde_json::json!({"status": "fail", "message": format_args!("{:?}", e)}));
        }
    };

    let mut redis_client = data.redis_client.get_async_connection().await.unwrap();
    let redis_result: redis::RedisResult<usize> = redis_client
        .del(&[
            refresh_token_details.token_uuid.to_string(),
            auth_guard.access_token_uuid.to_string(),
        ])
        .await;

    if redis_result.is_err() {
        return HttpResponse::UnprocessableEntity().json(
            serde_json::json!({"status": "error", "message": format_args!("{:?}", redis_result.unwrap_err())}),
        );
    }

    let access_cookie = Cookie::build("access_token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();
    let refresh_cookie = Cookie::build("refresh_token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();
    let logged_in_cookie = Cookie::build("logged_in", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .cookie(logged_in_cookie)
        .json(serde_json::json!({"status": "success"}))
}

#[get("/users/me")]
async fn get_me_handler(auth_guard: AuthCheck) -> impl Responder {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filter_user_record(&auth_guard.user)
        })
    });

    HttpResponse::Ok().json(json_response)
}
