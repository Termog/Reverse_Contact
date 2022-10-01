mod db;
use actix_session::storage::RedisSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::http::header::LOCATION;
use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use async_mutex::Mutex;
use cookie::Key;
use db::db_lib;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
// use uuid::Uuid;

//structure of data resived from the registration form
#[derive(Deserialize)]
struct RegisterData {
    username: String,
    password: String,
}

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
            .route("/", web::get().to(get_index))
            .route("/register", web::get().to(get_register))
            .route("/register", web::post().to(post_register))
            .route("/login", web::get().to(get_login))
            .route("/login", web::post().to(post_login))
    });
    println!("Serving on http://localhost:8080...");
    //Starting up the server
    server.bind(("127.0.0.1", 8080))?.run().await
}

//function returning the registration page
async fn get_register() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <form action="/register" method="post">
            <input type="text" name="username"/>
            <input type="password" name="password"/>
            <button type="subimt">Register</button>
            </form>
            "#,
    )
}

//function processing the registration post request
async fn post_register(pool: Data<Mutex<PgPool>>, form: web::Form<RegisterData>) -> impl Responder {
    match db_lib::register_to_db(&form.username, &form.password, pool).await {
        Ok(_) => HttpResponse::Ok().content_type("text/html").body(
            r#"
            Registation succsesfull
            "#,
        ),
        Err(e) => match e {
            db_lib::AuthError::UserExists => HttpResponse::Ok().content_type("text/html").body(
                r#"
            Username Taken
            <form action="/register" method="post">
            <input type="text" name="username"/>
            <input type="password" name="password"/>
            <button type="subimt">Register</button>
            </form>
            "#,
            ),
            _ => HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(
                    r#"
            WE FUCKED UP THE DATABASE
            "#,
                ),
        },
    }
}

//function returning the login page
async fn get_login() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <form action="/login" method="post">
            <input type="text" name="username"/>
            <input type="password" name="password"/>
            <button type="subimt">Login</button>
            </form>
        "#,
    )
}

//function processing the login post request
async fn post_login(
    pool: Data<Mutex<PgPool>>,
    session: Session,
    form: web::Form<RegisterData>,
) -> HttpResponse {
    match db_lib::check_login_information(&form.username, &form.password, pool).await {
        Ok(_) => {
            session.insert("user_id", "BRUH".to_string());
            HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish()
        }
        Err(_) => HttpResponse::Ok().content_type("text/html").body(
            r#"
            Incorrect password
            "#,
        ),
    }
}

//function returning the main page
async fn get_index(session: Session) -> impl Responder {
    match session.get::<String>("user_id").unwrap() {
        Some(_) => HttpResponse::Ok().content_type("text/html").body(
            r#"
            You're logged in
            "#,
        ),
        None => HttpResponse::Ok().content_type("text/html").body(
            r#"
            <button onclick="window.location='register';" value="register" />
            <button onclick="window.location='login';" value="login" />
            "#,
        ),
    }
}
