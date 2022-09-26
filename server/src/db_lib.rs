use rand::Rng;
use argon2::{self, Config};
use sqlx::postgres::PgPoolOptions;

#[derive(Debug)]
pub enum AuthError {
    Error
}

pub async fn register_to_db(username: &String, password: &String) -> Result<(), AuthError> {
    //generating password hash
    let salt = rand::thread_rng().gen::<[u8; 16]>();
    let config = Config::default();
    let hash: String = argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();

    //making a connection to our database
    let pool = PgPoolOptions::new()
        .connect("postgres://termog@localhost/auth").await.map_err(|_err| AuthError::Error)?;

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
        .await.map_err(|_err| AuthError::Error)?;

    //inserting new user into the database
    let _ = sqlx::query("INSERT into shadow (name, password) values ($1, $2)")
        .bind(username)
        .bind(hash)
        .execute(&pool)
        .await.map_err(|_err| AuthError::Error)?;


    Ok(())

}

pub async fn check_login_information(username: &String, password: &String) -> Result<(), AuthError> {


    //making a connection to our database
    let pool = PgPoolOptions::new()
        .connect("postgres://termog@localhost/auth").await.map_err(|_err| AuthError::Error)?;

    //extracting hash from the database
    let db_hash: (String,) = sqlx::query_as("SELECT password FROM shadow WHERE name = $1")
        .bind(username)
        .fetch_one(&pool)
        .await.map_err(|_err| AuthError::Error)?;

     let matches = argon2::verify_encoded(&db_hash.0, password.as_bytes()).unwrap();
     if !matches {
         return Err(AuthError::Error);
     }
         

    Ok(())
}
