/*This is the file that will handle the full auth of the systems 
We will write the models too in this file only 
*/

use actix_web::{HttpResponse, post, web};
use anyhow::Context;
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
use serde::{Deserialize,Serialize};
use sqlx::{PgPool, pool::PoolOptions};

use crate::error::AppResult;
#[derive(Debug,Deserialize,Serialize)]
pub struct Register{
    name:String,
    email:String,
    password:String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Login{
    email:String,
    password:String,
}

//Still this is not proper we will make the new changes wait for it 
#[post("/register")]
pub async fn register(pool:web::Data<PgPool>,register:web::Json<Register>)->AppResult<HttpResponse>{
    let sql = "INSERT INTO users (name,email,password) VALUES($1,$2,$3)";
    let pass = hash_password(&register.password)?;
    sqlx::query(sql).bind(&register.name).bind(&register.email).bind(pass).execute(pool.get_ref()).await.context("Could not innsert into the database")?;
    Ok(HttpResponse::Created().json(serde_json::json!({"Ok":"User created succeesfully"})))
}





//These are the helper fn here 
pub fn hash_password(password:&str)->AppResult<String>{
    let argon2 = Argon2::default(); 
    let salt = SaltString::generate(&mut OsRng); 
    let hashed_pass = argon2.hash_password(&password.as_bytes(), &salt).map_err(|e|anyhow::anyhow!("Could not hash the password : {}",e))?;
    Ok(hashed_pass.to_string())
}
