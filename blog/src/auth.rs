/*This is the file that wil handle the auth stuff 
We will need two routes here 
1. Login Route (Post)
2. Register route (Post)
3. Modles ( Both) 
4. Claims For JWT 
*/
use actix_web::{
    FromRequest, HttpRequest, cookie::Expiration::Session, dev::Payload, get,
};
use futures_util::future::{Ready, ready};
use rand::Rng;
use redis::AsyncCommands;

use std::{env,time::{SystemTime, UNIX_EPOCH}};

use actix_web::{HttpResponse, post, web};
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand_core::OsRng;
use serde::{Deserialize,Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::error::{AppError, AppResponse}; 
#[derive(Debug,Deserialize,Serialize)]
pub struct Register{
    pub name:String, 
    pub email:String, 
    pub pass:String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Login{
    email:String, 
    password:String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
    pub sub:String,
    pub exp:usize,
    pub sid:String,
}

//This is teh struct for the auth user here 
pub struct AuthUser{
    pub user_id:Uuid
}
//This is the register route i have made the hash password inside the same fn 
#[post("/register")]
pub async fn register(pool:web::Data<PgPool>,payload:web::Json<Register>)->AppResponse<HttpResponse>{
    let sql = "INSERT INTO users (name,email,pass) VALUES ($1,$2,$3)"; 
    let check = sqlx::query("SELECT * FROM users WHERE email = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the result ")?; 
    if check.is_some(){
        return Err(AppError::AlreadyExist);
    }
    let argon2 = Argon2::default(); 
    let salt = SaltString::generate(&mut OsRng); 
    let hashed_pass = argon2.hash_password(&payload.pass.as_bytes(), &salt).map_err(|err|anyhow::anyhow!("Could not hash the password: {}",err))?.to_string();

    sqlx::query(sql).bind(&payload.name).bind(&payload.email).bind(&hashed_pass).execute(pool.get_ref()).await.context("Cant add users ")? ; 
    Ok(HttpResponse::Created().json(serde_json::json!({"OK":"User created successfully"})))

}

//This is the login route here dude 
#[post("/login")]
pub async fn login(pool:web::Data<PgPool>, payload:web::Json<Login>,redis:web::Data<redis::Client>)->AppResponse<HttpResponse>{
    let check = sqlx::query("SELECT * FROM users WHERE email = $1").bind(&payload.email).fetch_optional(pool.get_ref()).await.context("Could not fetch the users from the database")? ; 
    let user = check.ok_or((AppError::NotFound))? ; 
    let stored_hash :String = user.get("pass"); 
    let parsed_hash = PasswordHash::new(&stored_hash).map_err(|_|AppError::Unauthorized)? ;
    Argon2::default().verify_password(&payload.password.as_bytes(), &parsed_hash).map_err(|_|AppError::Unauthorized)? ; 
    //Till now we have checked the password only now we will add the jwt in it 
    let user_id :Uuid= user.get("id"); 
    let mut session_bytes = [0u8; 32]; 
    rand::rng().fill_bytes(&mut session_bytes);
    let session_id = hex::encode(session_bytes);

    let mut connection = redis.get_multiplexed_async_connection().await.context("Could not conenct to the redis ")?; 

    let seesion_key = format!("session : {}",session_id); 
    let _ :()= connection.set_ex(&seesion_key, user_id.to_string(), 60*60).await.context("COuld not store session in the redis")? ; 

    let exp = SystemTime::now().duration_since(UNIX_EPOCH).context("Could not create the proper expiration time ")?.as_secs() + 60*60; 
    let claims = Claims{
        sub : user_id.to_string(),
        sid:session_id, 
        exp : exp as usize,
    }; 
    let secret = env::var("JWT_SECRET").expect("Could not find teh jwt key"); 
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()),).context("Could not create the proper token")? ; 

    Ok(HttpResponse::Ok().json(serde_json::json!({"Token":token})))
}

//This is the extractor here -------------> 
impl FromRequest for AuthUser{
    type Error = AppError;
    type Future = Ready<Result<Self,Self::Error>>;
    

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let auth_header = match req.headers().get("Authorization"){
            Some(header)=>header, 
            None=>{
                return ready(Err(AppError::Unauthorized));
            }
        }; 

        let auth_header = match auth_header.to_str(){
            Ok(value)=>value, 
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

        let secret = env::var("JWT_SECRET").expect("Could not find the jwt key in the config");
        let token_data = match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default()){
            Ok(data)=>data, 
            Err(_)=>return ready(Err(AppError::Unauthorized))
        }; 
        let user_id = match Uuid::parse_str(&token_data.claims.sub){
            Ok(id)=>id, 
            Err(_)=>{
            return ready(Err(AppError::Unauthorized));
        }
    };
    ready(Ok(AuthUser { user_id }))
    }
}


//--------------This is where the redis part will start --------------------//
#[get("/test_redis")]
pub async fn test_redis(redis:web::Data<redis::Client>)->AppResponse<HttpResponse>{
    let mut connection =redis.get_multiplexed_async_connection().await.context("Could not establish a prope connection ")?;
    let _ : () =connection.set("actix_test", "hello from actix")
    .await.context("Could not set the redis value")? ; 

    let value :String = connection.get("actix_test").await.context("Could not fetch the redis key value ")? ; 

    Ok(HttpResponse::Ok().json(serde_json::json!({"value":value})))
}