/*This is the auth file we will be writting 
First the models will be written 
second there will be the routes ( login and register) 
third Jwt extraction will be done */

use std::f32::consts::E;

use actix_web::{HttpResponse, post, web};
use anyhow::Context;
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand_core::OsRng;
use serde::{Deserialize,Serialize}; 
use sqlx::{FromRow, PgPool};

use crate::error::AppResponse; 

//this is the struct for the register
#[derive(Debug,Deserialize,Serialize)]
pub struct Register{
    name:String,
    email:String, 
    password:String
}

//This is the login struct 
#[derive(Debug)]
pub struct Login{
    email:String,
    password:String
}

//This is where the routes will begin 
#[post("/register")]
pub async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    //This is to check the users 
    let check = sqlx::query("SELECT * FROM users WHERE email = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the users from the database")?; 
    if check.is_some(){
        return Err(crate::error::AppError::AlreadyExist);
    }
    let salt = SaltString::generate(&mut OsRng); 
    let argon2 = Argon2::default(); 
    let pass = argon2.hash_password(&payload.password.as_bytes(), &salt).map_err(|err|anyhow::anyhow!("Could not hash the password : {}",err))?.to_string();
    sqlx::query("INSERT INTO users (name,email,pass) VALUES ($1,$2,$3)").bind(&payload.name).bind(&payload.email).bind(pass).execute(pool.get_ref()).await.context("Could not isert user into the database")? ; 
    Ok(HttpResponse::Created().json(serde_json::json!({"OK":"User is created successfully"})))
}

//This is the route for thr login now we will add the jwt inside it 
