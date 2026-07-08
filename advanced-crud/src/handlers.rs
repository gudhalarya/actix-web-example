//We have 4 basic fn 
//1. Get books ---------Get (Done)
//2. Add books ---------- Post (Done)
//3. delete books ------delete {id} (Done)
//4. update book complete update ---- put {id} (Done)
//5. update is done partially --------patch {id}
//6. search by name or author ------ get 
use serde::{Deserialize,Serialize};

//This is where the models lie 
#[derive(Debug,Serialize,sqlx::FromRow)]
struct GetBooks{
    id:i32,
    name:String,
    author:String,
    date:String
}

#[derive(Deserialize)]
pub struct AddBooks{
    name:String,
    author:String
}


#[derive(Debug,Deserialize)]
pub struct UpdateBooks{
    name:String,
    author:String
}


//This is where the first get fn is written 
use actix_web::{HttpResponse, delete, get, post, put, web};
use sqlx::PgPool;

use crate::error::{AppError, AppResponse};
#[get("/get_books")]
pub async fn get_books(pool:web::Data<PgPool>)->AppResponse<HttpResponse>{
    let sql = "SELECT * FROM books ";
    let result = sqlx::query_as::<_,GetBooks>(sql)
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(result))
}

//This is for the posting of the books here 
#[post("/add_books")]
pub async fn add_books(pool:web::Data<PgPool>,payload:web::Json<AddBooks>)->AppResponse<HttpResponse>{
    if payload.name.trim().is_empty()||payload.author.trim().is_empty(){
        return Err(crate::error::AppError::BadRequest("Name and author entry cant be empty ".to_string()));
    }
    let sql = "INSERT INTO books  (name,author,date) VALUES ($1,$2,$3) RETURNING * ";
    let date = chrono::Utc::now().to_string();
    let result = sqlx::query_as::<_,GetBooks>(sql)
    .bind(&payload.name)
    .bind(&payload.author)
    .bind(date)
    .fetch_one(pool.get_ref())
    .await.map_err(|err|{
        if let sqlx::Error::Database(db_err)=&err{
            if db_err.code().as_deref()==Some("23505"){//This number can be find in the postgress official documentation it stands for the unique violation each db has its own error codes 
                return AppError::Conflict(format!("The books {} from {} already exist in the database ",payload.name,payload.author));
            }
        }
        AppError::DatabaseError(err)
    })?;
    Ok(HttpResponse::Created().json(result))

}

//This is the fn to delete the books by using the id 
#[delete("/delete_books/{id}")]
pub async fn delete_books(pool:web::Data<PgPool>,id:web::Path<i32>)->AppResponse<HttpResponse>{
    let sql = "DELETE FROM books WHERE id = $1";
    let rows_affetced = sqlx::query(sql)
    .bind(*id)
    .execute(pool.get_ref())
    .await?
    .rows_affected();
    if rows_affetced == 0{
        return Err(AppError::NotFound);
    }
    Ok(HttpResponse::Ok().body("Book deleted successfully"))
}

//This is where the update fn is 
#[put("/update_books/{id}")]
pub async fn update_books(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    payload: web::Json<UpdateBooks>,
) -> AppResponse<HttpResponse> {
    if payload.name.trim().is_empty() || payload.author.trim().is_empty() {
        return Err(AppError::BadRequest("Fields cannot be empty".to_string()));
    }

    let sql = "UPDATE books SET name = $1, author = $2 WHERE id = $3 RETURNING id, name, author, date";
    let result = sqlx::query_as::<_, GetBooks>(sql)
        .bind(&payload.name)
        .bind(&payload.author)
        .bind(*id)
        .fetch_optional(pool.get_ref()) // fetch_optional returns None if no rows match
        .await?;

    match result {
        Some(book) => Ok(HttpResponse::Ok().json(book)),
        None => Err(AppError::NotFound),
    }
}

