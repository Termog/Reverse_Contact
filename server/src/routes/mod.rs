use crate::db::db_lib;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use async_mutex::Mutex;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct RegisterData {
    username: String,
    password: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::get().to(get_login))
            .route(web::post().to(post_login))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    )
    .service(
        web::resource("/register")
            .route(web::get().to(get_register))
            .route(web::post().to(post_register))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    )
    .service(
        web::resource("/")
            .route(web::get().to(get_index))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
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
#[allow(unused_variables)]
async fn post_login(
    pool: Data<Mutex<PgPool>>,
    session: Session,
    form: web::Form<RegisterData>,
) -> HttpResponse {
    match db_lib::check_login_information(&form.username, &form.password, pool).await {
        Ok(_) => match session.insert("user_id", "BRUH".to_string()) {
            Ok(_) => HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish(),
            Err(_) => HttpResponse::Ok().content_type("text/html").body(
                r#"
                Couldn't register
                "#,
            ),
        },
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
            <h1>Your're logged in.</h1>
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
