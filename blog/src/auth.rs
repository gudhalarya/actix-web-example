use std::{env, fs::read};

use actix_web::{FromRequest, HttpResponse, post, web::{self, Data}};
use anyhow::{Context};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use chrono::{Duration, Utc};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use hex::decode;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode};
use rand_core::OsRng;
//This is the final file we will write for this project auths 
//First the models will be done 
use serde::{Deserialize,Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::error::{AppError, AppResponse}; 
#[derive(Debug,Deserialize,Serialize)]
pub struct Register{
    pub name:String,
    pub email:String,
    pub password:String
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Login{
    email:String,
    password:String
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Claims{
    sub:String,
    exp:usize
}

#[derive(Debug)]
pub struct AuthUser{
    pub user_id : Uuid
}

#[post("/register")]
pub async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    let check = sqlx::query("SELECT * FROM users WHERE email = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the users form the database")?; 
    if check.is_some(){
        return Err(crate::error::AppError::AlreadyExist);
    }
    let argon2 = Argon2::default(); 
    let salt = SaltString::generate(&mut OsRng); 
    
    let hash_passwrd = argon2.hash_password(&payload.password.as_bytes(),&salt).map_err(|err|anyhow::anyhow!("Could not hash the password  {}",err))?.to_string();
    sqlx::query("INSERT INTO users (name,email,pass) VALUES ($1,$2,$3)").bind(&payload.name).bind(&payload.email).bind(&hash_passwrd).fetch_one(pool.get_ref()).await.context("Could not insert the user in the database ")? ; 
    Ok(HttpResponse::Created().json(serde_json::json!({"message":"Created successfully"})))
}

//This is the route for the login now 
#[post("/login")]
pub async fn login(pool:web::Data<PgPool>,payload:web::Json<Login>)->AppResponse<HttpResponse>{
    let check = sqlx::query("SELECT * FROM users WHERE email = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the users from the database")?;
    let user = check.ok_or(AppError::NotFound)?;
    let id : Uuid = user.try_get("id").context("Could not get the user id from the database")?;
    let pass:String = user.try_get("pass").context("Could not get the password")?;
    //We will now verify the user
    let parsed_hash = PasswordHash::new(&pass).map_err(|err|anyhow::anyhow!("Could not pars the hashed password : {}",err))?;
    let argon2 = Argon2::default(); 
    argon2.verify_password(&payload.password.as_bytes(), &parsed_hash).map_err(|_|AppError::Unauthorized)?;
    //This is for the jwt now 
    let exp = (Utc::now()+Duration::hours(1)).timestamp() as usize; 
    let sub = id.to_string(); 
    let claims = Claims{
        sub:sub,
        exp:exp
    }; 
    let jwt_secret = env::var("JWT_SECRET").expect("Could not find the jwt secret in the env file ");
    let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes())).context("Could not make the jwt token")?; 
    Ok(HttpResponse::Ok().json(serde_json::json!({"token":token})))
}

//Now the jwt extractor will be here 
impl FromRequest for AuthUser{
    type Error = AppError;
    type Future = Ready<Result<Self,Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = match req.headers().get("Authorization"){
            Some(header)=>header,
            None=>{
               return ready(Err(AppError::Unauthorized)); 
            }
        };
        let auth_header = match auth_header.to_str(){
            Ok(values)=>values,
            Err(_)=>{
                return ready(Err(AppError::Unauthorized));
            }
        };
        let token = match auth_header.strip_prefix("Bearer"){
            Some(token)=>token,
            None=>{
                return ready(Err(AppError::Unauthorized));
            }
        };
        let secret = match env::var("JWT_SECRET"){
            Ok(secret )=>secret,
            Err(_)=>{
                return ready(Err(AppError::Unauthorized));
            }
        };
        let token_data =match decode::<Claims>(token, &DecodingKey::from_secret(secret), &Validation::default()){
            Ok(data),
            Err(_)=>{
                return ready(Err(AppError::Unauthorized));
            }
        };
    }
}