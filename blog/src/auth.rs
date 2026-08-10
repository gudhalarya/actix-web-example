use std::rc::Weak;

use actix_web::{HttpResponse, http::KeepAlive::Os, post, web};
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
//This is the file for the auth of the projects 
//Models are the first thing we will do 
use serde::{Deserialize,Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};

use crate::error::{AppError, AppResponse}; 
#[derive(Debug,Deserialize)]
pub struct Register{
    name:String, 
    email:String, 
    password:String
}

#[derive(Debug,Deserialize,sqlx::FromRow)]
pub struct Login{
    email:String, 
    password:String
}
//--------------------------------------------------------------------//
//These are the helper fn here 
pub fn hash_pass(password:&str)->AppResponse<String>{
    let salt = SaltString::generate(&mut OsRng); 
    let argon2 = Argon2::default(); 
    let password = argon2.hash_password(&password.as_bytes(), &salt).map_err(|e|anyhow::anyhow!("Error ocurred : {}",e))?;
    Ok(password.to_string())
}


//-----------------------------------------------------------------//

//This is the login route 
#[post("/register")]
async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    let passwrd = hash_pass(&payload.password)?; 
    let sql = "INSERT INTO users(name,email,pass) VALUES ($1,$2,$3)"; 
    sqlx::query(sql).bind(&payload.name).bind(&payload.email).bind(passwrd).execute(pool.get_ref()).await.context("Could not register the user")?;
    Ok(HttpResponse::Created().json(serde_json::json!({"OK":"User created successfully"})))
}


//This is the login fn here
#[post("/login")]
async fn login(pool:web::Data<PgPool>,payload:web::Json<Login>)->AppResponse<HttpResponse>{
    let user = sqlx::query(r#"SELECT id,name,email,pass FROM users WHERE email = $1"#).bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not find the user in the database")?.ok_or(AppError::Unauthorized)?;
    let password_hash :String = user.try_get("pass").context("Could not find the hashed password")? ; 
    let password_hash = PasswordHash::new(&password_hash).map_err(|e|anyhow::anyhow!("Invalid password or user id {}",e))?; 


    let argon = Argon2::default() ; 
    argon.verify_password(payload.password.as_bytes(), &password_hash).map_err(|_|crate::error::AppError::Unauthorized)? ;
    Ok(HttpResponse::Ok().json(serde_json::json!({"Ok":"User is verified"})))
}