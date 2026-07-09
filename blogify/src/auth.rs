use actix_web::{HttpResponse, get, post, web};
use anyhow::Context;
//These are the auth file 
//These are the models we will use
use sqlx::{FromRow, PgPool};
use serde::{Deserialize,Serialize};

use crate::{error::AppResponse, helpers::hash_password};

#[derive(Debug,Deserialize)]
pub struct SignUp{
    name:String,
    email:String,
    password:String
}

#[derive(Debug,Serialize,FromRow)]
pub struct Login{
    email:String,
    password:String
}

#[derive(Serialize,FromRow)]
pub struct Show{
    name:String,
    email:String
}

#[post("/signup")]
pub async fn signup(pool:web::Data<PgPool>,payload:web::Json<SignUp>)->AppResponse<HttpResponse>{
    let sql = "INSERT INTO users (name,email,password) VALUES ($1,$2,$3) RETURNING (name,email)";
    let hash_pass = hash_password(&payload.password)?;
    let res = sqlx::query_as::<_,Show>(sql).bind(&payload.name).bind(&payload.email)
    .bind(&hash_pass)
    .fetch_one(pool.get_ref())
    .await.context("Database error")?;

    Ok(HttpResponse::Created().json(res))
}

#[get("login")]
pub async fn login(pool:web::Data<PgPool>,payload:web::Json<Login>)->AppResponse<HttpResponse>{
    let sql = "SELECT password FROM users WHERE email=$1";
    
}