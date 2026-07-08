use actix_web::{HttpResponse, Responder, get, post, web};
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
pub async fn add_books(pool:web::Data<PgPool>,payload:web::Json<AddBooks>)->AppResponse<HttpResponse>{
    if payload.name.trim().is_empty() || payload.author.trim().is_empty(){
        return Err(AppError::BadRequest("Name and author are required".into()));
    }

    let sql = "INSERT INTO books (name, author) VALUES ($1, $2) RETURNING id, name, author, date";
    let book = sqlx::query_as::<_, GetBooks>(sql)
        .bind(&payload.name)
        .bind(&payload.author)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(book))
}