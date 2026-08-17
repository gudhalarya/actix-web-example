use std::env;

use actix_web::{HttpResponse, post, web};
use anyhow::Context;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use argon2::password_hash::SaltString;
use chrono::Duration;
use chrono::Utc;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::encode;
use rand_core::OsRng;

/*This is the file for the auth stufff dude 
1. Models  + Compelete routes will be stored here 
2. Hashing + verification will be done in the same file Algo which will be used is ( ARGON2 )
3.Jwt and verification + Redis maybe 
*/
use serde::{Deserialize,Serialize};
use sqlx::PgPool;
use sqlx::Row;
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

//This is the jwt struct here
#[derive(Debug,Deserialize,Serialize,sqlx::FromRow)]
pub struct Claims{
    sub:String,
    exp:usize
}

//Routes start here first the register 
#[post("/register")]
pub async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    //Checking the existing users 
    let check = sqlx::query("SELECT * FROM users WHERE email = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the users from the database")? ; 
    if check.is_some(){
        return Err(AppError::AlreadyExist);
    }
    //Calling the hash algo here (Argon2)......
    let salt = SaltString::generate(&mut OsRng); 
    let argon2 = Argon2::default(); 
    let sql = ("INSERT INTO users (name,email,password) VALUES ($1,$2,$3)");
    let pass = argon2.hash_password(&payload.password.as_bytes(), &salt).map_err(|err|anyhow::anyhow!("Could not hash the password : {}",err))?.to_string() ; 
    sqlx::query(sql).bind(&payload.name).bind(&payload.email).bind(pass).execute(pool.get_ref()).await.context("Could not insert the user in the database")?; 
    Ok(HttpResponse::Created().json(serde_json::json!({"Ok":"User created successfully"})))
    //The register route is complete 
}

//This is the login route here 
#[post("/login")]
pub async fn login(pool:web::Data<PgPool>,payload:web::Json<Login>)->AppResponse<HttpResponse>{
    //first checking wether the user actually does exist or not
    let check = sqlx::query("SELECT * FROM users WHERE email = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the users from the database")? ; 
    let user = check.ok_or(AppError::NotFound)? ; 
    let password_hash:String = user.try_get("password").context("Could not found the password in the database")? ; 
    let user_id : String= user.get("id");
    let parsed_hash = PasswordHash::new(&password_hash).map_err(|err|anyhow::anyhow!("Invalid password hash : {}",err))?; 
    Argon2::default().verify_password(&payload.password.as_bytes(), &parsed_hash).map_err(|_|AppError::Unauthorized)?; 
    let exp = (Utc::now() + Duration::hours(1)).timestamp() as usize; 
    let claims  = Claims{
        sub:user_id,
        exp:exp
    };
    let secret = env::var("JWT_SECRET").expect("Could not find the jwt key ");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(&secret.as_bytes())).map_err(|err|anyhow::anyhow!("Could not create the jwt tokens {}",err))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"Message":"User is verified","token":token})))
    //No jwt till now 
}
