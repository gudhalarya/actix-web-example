/* This is the file that will  handle all of the auth of the full website 
1. First is the register and login auth we will do.
*/

use actix_web::error::PayloadError::Http2Payload;
use actix_web::{HttpResponse, post, web};
use anyhow::Context;
use argon2::{PasswordHash, PasswordVerifier};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand_core::OsRng;
use serde::{Deserialize,Serialize};
use sqlx::{PgPool, Row};
use crate::error::AppError; 
//This is the model of the register route here 
use crate::error::AppResponse; 
#[derive(Debug,Deserialize,Serialize)]
pub struct Register{
    name:String, 
    email:String, 
    password:String
}

//This is the login model we will use 
#[derive(Debug,Deserialize,Serialize)]
pub struct Login{
    email:String, 
    password:String
}
 
//This is the register route here 
#[post("/register")]
pub async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    //The first thing to do is to check the user wether it already exist or not 
    let check = sqlx::query("SELECT * FROM users  WHERE EMAIL = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the users ")? ; 
    if check.is_some(){
       return  Err(AppError::AlreadyExist);
    }

    //We will hash password here 
    let argon = Argon2::default(); 
    let salt = SaltString::generate(&mut OsRng); 
    let hashed_pass = argon.hash_password(&payload.password.as_bytes(), &salt).map_err(|err|anyhow::anyhow!("Error occurreed {}",err))?.to_string(); 
    sqlx::query("INSERT INTO users (name,email,pass) VALUES ( $1,$2,$3) ").bind(&payload.name).bind(&payload.email).bind(&hashed_pass).execute(pool.get_ref()).await.context("Could not add users ")? ; 
    Ok(HttpResponse::Created().json(serde_json::json!({"Ok":"User added successfully "})))
}

//This is the login route here 
#[post("/login")]
pub async fn login(pool:web::Data<PgPool>,payload:web::Json<Login>)->AppResponse<HttpResponse>{
    let check = sqlx::query("SELECT * FROM users WHERE email = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the users ")? ;
    let user = check.ok_or(AppError::NotFound)?;
    let parsed_hash = user.get("pass"); 
    let   argon2 = Argon2::default(); 
    argon2.verify_password(&payload.password.as_bytes(), &parsed_hash).map_err(|_|AppError::Unauthorized)?; 
    Ok(HttpResponse::Ok().json(serde_json::json!("User is verified")))
}