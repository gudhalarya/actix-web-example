use actix_web::{HttpResponse, get, post, web};
//we are gonna write the whole code here right now dude 
use serde::{Deserialize,Serialize};
use sqlx::PgPool;

use crate::error::{AppError, AppResponse};
#[derive(Debug,Deserialize)]
pub struct AddBooks{
    name:String,
    author:String
}

#[derive(Debug,Serialize,Deserialize,sqlx::FromRow)]
pub struct GetBooks{
    id:i32,
    name:String,
    author:String,
    date:String,
}


//this is the fn to get the books 
#[get("/get_books")]
pub async fn get_books(pool:web::Data<PgPool>)->AppResponse<HttpResponse>{
    let sql = "SELECT id,name,author,date FROM books";
    let result = sqlx::query_as::<_,GetBooks>(sql).fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/add_books")]
async fn add_books(pool:web::Data<PgPool>,payload:web::Json<AddBooks>)->AppResponse<HttpResponse>{
    if payload.name.trim().is_empty()||payload.author.trim().is_empty(){
        return Err(AppError::BadRequest("Book name and author name cant be empty ".to_string()));
    }
    let date = chrono::Utc::now().naive_utc().to_string();
    let sql = "INSERT INTO books (name,author,date) VALUES ($1,$2,$3) RETURNING id,name,author,date";
    let result = sqlx::query_as::<_,GetBooks>(sql)
    .bind(&payload.name)
    .bind(&payload.author)
    .bind(date)
    .fetch_one(pool.get_ref())
    .await.map_err(|e|{
        if let sqlx::Error::Database(db_err)=&e{
            if db_err.code().as_deref() == Some("23505"){
                return AppError::Conflict(format!("The book {} by {} is already present ",payload.name,payload.author));
            }
        }
        AppError::DatabaseError(e)
    })?;
    Ok(HttpResponse::Created().json(result))
}


