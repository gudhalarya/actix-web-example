use actix_web::{HttpResponse, http::KeepAlive::Os, post, web};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
//This is the file for the auth of the projects 
//Models are the first thing we will do 
use serde::{Deserialize,Serialize};
use sqlx::PgPool;

use crate::error::AppResponse; 
#[derive(Debug,Deserialize)]
pub struct Register{
    name:String, 
    email:String, 
    password:String
}

#[derive(Debug,Serialize)]
pub struct Login{
    email:String, 
    password:String
}

//These are the helper fn here 
pub fn hash_pass(password:&str)->AppResponse<String>{
    let salt = SaltString::generate(&mut OsRng); 
    let argon2 = Argon2::default(); 
    let password = argon2.hash_password(&password.as_bytes(), &salt).map_err(|e|anyhow::anyhow!("Error ocurred : {}",e))?;
    Ok(password.to_string())
}

//This is the login route 
#[post("/Register")]
async fn login(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    let passwrd = hash_pass(payload.password); 
    let sql = "INSERT INTO users WHERE username "
}