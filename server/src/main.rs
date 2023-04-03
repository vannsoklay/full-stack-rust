extern crate sqlx;

mod config;
mod utils;
mod models;
mod middleware;
mod repository;
mod api;
mod router;
mod security;

use actix_cors::Cors;

use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use config::Config;
use dotenv::dotenv;
use redis::Client;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug)]
pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
    redis_client: Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let redis_client = match Client::open(config.redis_url.to_owned()) {
        Ok(client) => {
            println!("✅ Connection to the redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully at {}:{}", "0.0.0.0", 8001);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.client_origin)
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
                redis_client: redis_client.clone(),
            }))
            .wrap(cors)
            .configure(router::config)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8001))?
    .run()
    .await
}
