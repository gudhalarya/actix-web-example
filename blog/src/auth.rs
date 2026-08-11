/*This is the file that wil handle the auth stuff 
We will need two routes here 
1. Login Route (Post)
2. Register route (Post)
3. Modles ( Both) 
4. Claims For JWT 
*/

use std::rc::Weak;

use actix_web::{HttpResponse, post, web};
use anyhow::Context;
use serde::{Deserialize,Serialize};
use sqlx::PgPool;

use crate::error::{AppError, AppResponse}; 
#[derive(Debug,Deserialize,Serialize)]
pub struct Register{
    pub name:String, 
    pub email:String, 
    pub pass:String,
}

#[derive(Debug,Serialize)]
pub struct Login{
    email:String, 
    password:String,
}

pub struct Claims{
    sub:String,
    exp:usize
}

#[post("/register")]
pub async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    let sql = "INSERT INTO users (name,email,pass) VALUES ($!,$2,$3)"; 
    let check = sqlx::query("SELECT * FROM users WHERE email = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the result ")?; 
    if check.is_some(){
        return Err(AppError::AlreadyExist);
    }

    let result = sqlx::query(sql).bind(&payload.name).bind(&payload.email).bind(&payload.pass).execute(pool.get_ref()).await.context("Cant add users ")? ; 
    Ok(HttpResponse::Created().json(serde_json::json!({"OK":"User created successfully"})))

}