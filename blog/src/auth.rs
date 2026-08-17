use actix_web::{HttpResponse, post, web};
use anyhow::Context;
/*This is the file for the auth stufff dude 
1. Models  + Compelete routes will be stored here 
2. Hashing + verification will be done in the same file Algo which will be used is ( ARGON2 )
3.Jwt and verification + Redis maybe 
*/
use serde::{Deserialize,Serialize};
use sqlx::PgPool;
use crate::error::AppError; 

use crate::error::AppResponse; 
//Register models we will use
#[derive(Debug,Deserialize,Serialize)]
pub struct Register{
    name:String,
    email:String, 
    password:String
}

//Login model is here

#[derive(Debug,Deserialize,Serialize)]
pub struct Login{
    email:String, 
    password:String
}


//Routes start here first the register 
#[post("/register")]
pub async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    //Checking the existing users 
    let check = sqlx::query("SELECT * FROM users WHERE email = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the users from the database")? ; 
    if check.is_some(){
        return Err(AppError::AlreadyExist);
    }
    
}