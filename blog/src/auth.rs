use actix_web::{HttpResponse, get, post, web};
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString,rand_core::OsRng}};

//Auth Code is here 
use serde::{Deserialize,Serialize};
use sqlx::PgPool;

use crate::error::{AppError, AppResponse};

#[derive(Debug,Deserialize,Serialize)]
pub struct Login{
    email:String,
    password:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Register{
    name:String,
    email:String,
    password:String
}


//These are the helper fn here 
pub fn hash_password(password:&str)->AppResponse<String>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash_password = argon2.hash_password(&password.as_bytes(), &salt).map_err(|e|anyhow::anyhow!("Could not hash the password :{e}"))?.to_string();
    Ok(hash_password)
}

pub fn verify_password(password:&str,hash:&str)->AppResponse<()>{
    let argon2 = Argon2::default();
    let parsed = PasswordHash::new(hash).map_err(|e|anyhow::anyhow!("Stored hash is malformed"))?;
    argon2.verify_password(&password.as_bytes(), &parsed).map_err(|e|AppError::Unauthorized)?;
    Ok(())
}


//This is for the register of the users 
#[post("/register")]
pub async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    let sql = "INSERT INTO users (name,email,password) VALUES ($1,$2,$3)";
    let hash = hash_password(&payload.password)?;
    let result = sqlx::query(sql).bind(&payload.name)
    .bind(&payload.email)
    .bind(hash)
    .execute(pool.get_ref())
    .await;

    if let Err(err) =result{
        if err.as_database_error().map_or(false, |e|e.is_unique_violation()){
            return Err(AppError::Conflict("An account with this email already exist".to_string()));
        }
        return Err(AppError::InternalError(anyhow::Error::from(err).context("Could not register user")));
    } 

    Ok(HttpResponse::Created().json(serde_json::json!({"Ok":"User created successfully"})))
}


//This is the login route
#[derive(sqlx::FromRow)]
struct UserRow {
    id: i32,
    name: String,
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    payload: web::Json<Login>,
) -> AppResponse<HttpResponse> {
    let user: Option<UserRow> = sqlx::query_as("SELECT id, name, email, password FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(pool.get_ref())
        .await
        .context("Database error during login validation")?;

    let user = match user {
        Some(u) => u,
        None => return Err(AppError::Unauthorized),
    };

    let password = payload.password.clone();
    let stored_hash = user.password.clone();

   verify_password(&password, &stored_hash)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Logged in successfully"
    })))
}