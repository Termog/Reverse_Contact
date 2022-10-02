mod db;
mod routes;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use async_mutex::Mutex;
use cookie::Key;
use db::errors;
use routes::config;
use sqlx::postgres::PgPoolOptions;

// use uuid::Uuid;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Httpserver setup with all available routes
    let redis = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    let pool = PgPoolOptions::new()
        .connect("postgres://postgres@localhost/auth")
        .await
        .unwrap();
    //        .map_err(|_err| AuthError::Error)?;

    //creating database for usernames and passwords if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shadow (
        id bigserial,
        name text,
        password text,
        UNIQUE (name)
        );"#,
    )
    .execute(&pool)
    .await
    .unwrap();
    //    .map_err(|_err| AuthError::Error)?;
    let pool = Data::new(Mutex::new(pool));

    let key = Key::generate();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(redis.clone(), key.clone()))
            .app_data(Data::clone(&pool))
            .configure(config)
            .default_service(web::route().to(errors::not_found))
    });
    println!("Serving on http://localhost:8080...");
    //Starting up the server
    server.bind(("127.0.0.1", 8080))?.run().await
}
