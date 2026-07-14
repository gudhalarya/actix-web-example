use std::io::Error;

use actix_web::{HttpResponse, post, web};
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


//This is the main auth route here 
#[post("/register")]
pub async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    let sql = "INSERT  INTO users (name,email,password) VALUES ($1,$2,$3)";
    let pass = hash_password(&payload.password)?;
    let result = sqlx::query(sql).bind(&payload.name).bind(&payload.email).bind(&pass).execute(pool.get_ref()).await;
    match result{
        Ok(_)=>Ok(HttpResponse::Created().json(serde_json::json!({"Message":"Created successfully"}))),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() =>{Err(AppError::Conflict("Email already registered".into()))}
        Err(e)=>Err(anyhow::Error::new(e).into()),
    }
}