use std::{env, future::{Ready, ready}, time::{SystemTime, UNIX_EPOCH}};

use actix_web::{FromRequest, HttpResponse, error::{ErrorInternalServerError, ErrorUnauthorized}, post, web};
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString,rand_core::OsRng}};
use jsonwebtoken::{EncodingKey, Header, encode};
//This is the file for the auth of the projects 
//Models are the first thing we will do 
use serde::{Deserialize,Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use uuid::Uuid;

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

#[derive(Debug,Deserialize,Serialize)]
pub struct ClaimsZ{
    pub sub:String,
    pub exp:usize
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
pub async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    let passwrd = hash_pass(&payload.password)?; 
    let sql = "INSERT INTO users(name,email,pass) VALUES ($1,$2,$3)"; 
    sqlx::query(sql).bind(&payload.name).bind(&payload.email).bind(passwrd).execute(pool.get_ref()).await.context("Could not register the user")?;
    Ok(HttpResponse::Created().json(serde_json::json!({"OK":"User created successfully"})))
}


//This is the login fn here
#[post("/login")]
pub async fn login(pool:web::Data<PgPool>,payload:web::Json<Login>)->AppResponse<HttpResponse>{
    let user = sqlx::query(r#"SELECT id,name,email,pass FROM users WHERE email = $1"#).bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not find the user in the database")?.ok_or(AppError::Unauthorized)?;
    let password_hash :String = user.try_get("pass").context("Could not find the hashed password")? ; 
    let password_hash = PasswordHash::new(&password_hash).map_err(|e|anyhow::anyhow!("Invalid password or user id {}",e))?; 


    let argon = Argon2::default() ; 
    argon.verify_password(payload.password.as_bytes(), &password_hash).map_err(|_|crate::error::AppError::Unauthorized)? ;
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()+60*60; 

    let user_id:Uuid = user.try_get("id").context("Could not get the user id ")? ; 
    let claims = ClaimsZ{
        sub:user_id.to_string(),
        exp:expiration as usize,
    }; 
    let secret = env::var("JWT_SECRET").context("Could not find the jwt secret in the file ")? ; 
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()),).map_err(|e|anyhow::anyhow!("Could not create jwt : {}",e))? ; 


    Ok(HttpResponse::Ok().json(serde_json::json!({"token":token})))
}

