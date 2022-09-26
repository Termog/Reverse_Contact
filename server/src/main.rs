mod db_lib;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

//structure of data resived from the registration form
#[derive(Deserialize)]
struct RegisterData {
    username: String,
    password: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
//Httpserver setup with all available routes
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/register",web::get().to(get_register))
            .route("/register",web::post().to(post_register))
            .route("/login",web::get().to(get_login))
            .route("/login",web::post().to(post_login))
    });
    println!("Serving on http://localhost:8080...");
    //Starting up the server 
    server
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

//function returning the registration page
async fn get_register() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
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
async fn post_register(form: web::Form<RegisterData>) -> impl Responder {
    match db_lib::register_to_db(&form.username,&form.password).await {
        Ok(_) => { HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            Registation succsesfull
            "#,
            )},
        Err(_) => {  HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            Username taken
            "#,
            )}
    }
}

//function returning the login page
async fn get_login() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
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
async fn post_login(form: web::Form<RegisterData>) -> impl Responder {
    match db_lib::check_login_information(&form.username,&form.password).await {
        Ok(_) => { HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            Login succsesfull
            "#,
            )},
        Err(_) => {  HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            Incorrect password
            "#,
            )}
    }
}

//function returning the main page
async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <button onclick="window.location='register';" value="register" />
            <button onclick="window.location='login';" value="login" />
            "#,
            )
}
