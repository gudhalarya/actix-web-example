use std::env;
use serde::{Deserialize,Serialize};
use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, put, web};
use sqlx::{PgPool, postgres::PgPoolOptions};


//This is the fn for the pool here 
async fn get_db()->PgPool{
    let db_url = env::var("DATABASE_URL").expect("Could not find the database url in  the env file ");
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&db_url)
    .await.expect("Could not connect to the database");

    pool
}

//This is the models we will use 
#[derive(Debug,Deserialize,Serialize,sqlx::FromRow)]
pub struct GetBooks{
    id:i32,
    name:String,
    author:String,
    date:String,
}

#[derive(Debug,Deserialize,sqlx::FromRow,Serialize)]
struct CommonBooks{
    name:String,
    author:String,
}

#[derive(Debug,Deserialize,sqlx::FromRow,Serialize)]
struct UpdateBooks{
    name:String,
    author:String,
}

//This is just for the checking of the health here 
#[get("/health")]
async fn health()->HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({"Status is Ok ":"200"}))
}

//This is the fn to add the books to the database
#[post("/add_books")]
async fn add_books(pool:web::Data<PgPool>,payload:web::Json<CommonBooks>)-> impl Responder{
    let sql = "INSERT INTO books (name,author,date) VALUES ($1,$2,$3) RETURNING *";
    if payload.name.trim().is_empty() || payload.author.trim().is_empty(){
        return HttpResponse::BadRequest().body("Name and author cant be empty");
    }
    let date = chrono::Utc::now().naive_utc().to_string();
    let result = sqlx::query_as::<_,GetBooks>(sql).bind(&payload.name).bind(&payload.author).bind(date).fetch_one(pool.get_ref()).await;
    match result {
        Ok(res)=>HttpResponse::Ok().json(res),
        Err(e)=>{
            eprintln!("Error is : {}",e);
            HttpResponse::InternalServerError().body("Database Error")
        }
    }
}

//This is the fn here for the getting the book
#[get("/get_books")]
async fn get_books(pool: web::Data<PgPool>)->impl Responder{
    let sql = "SELECT id,name,author,date FROM books";
    let result = sqlx::query_as::<_,GetBooks>(sql)
    .fetch_all(pool.get_ref())
    .await;

    match result{
        Ok(book)=>HttpResponse::Ok().json(book),

        Err(e)=>{
            eprintln!("Error is : {}",e);
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}

//This is to delete the books
#[delete("/delete_books/{id}")]
async fn delete_books(pool:web::Data<PgPool>,id:web::Path<i32>)->impl Responder{
    let sql = "DELETE FROM books WHERE id = $1";
    let result = sqlx::query(sql).bind(*id).execute(pool.get_ref()).await;

    match result{
        Ok(res)=>{
            if res.rows_affected() == 0{
                HttpResponse::NotFound().body("Book not found ")
            }
            else {
                HttpResponse::Ok().body("Book deleted successfully")
            }
        },
        Err(e)=>{
            eprintln!("Error is : {}",e);
            HttpResponse::InternalServerError().body("database error")
        }
    }
}

//This is to get by id
#[get("/get_books/{id}")]
async fn get_by_id(pool:web::Data<PgPool>,id:web::Path<i32>)-> impl Responder{
    let sql = "SELECT name,author,date FROM books WHERE id = $1";
    let result = sqlx::query_as::<_,GetBooks>(sql).bind(*id).fetch_one(pool.get_ref()).await;
    match result {
        Ok(res)=>HttpResponse::Ok().json(res),
        Err(e)=>{
            eprintln!("Error is : {}",e);
            HttpResponse::InternalServerError().body("Database Error")
        }
    }
}

//This is to update the books
#[put("/update_books/{id}")]
async fn update_books(pool:web::Data<PgPool>,id:web::Path<i32>,payload:web::Json<UpdateBooks>)->impl Responder{
    let sql = "UPDATE books SET name = $1, author = $2, date = $3 WHERE id = $4 RETURNING *";
    let date = chrono::Utc::now().naive_utc().to_string();
    let result = sqlx::query_as::<_,GetBooks>(sql).bind(&payload.name).bind(&payload.author).bind(date).bind(*id).fetch_one(pool.get_ref()).await;
    match result{
        Ok(res)=>HttpResponse::Ok().json(res),
        Err(e)=>{
            eprintln!("Error is : {}",e);
            HttpResponse::InternalServerError().body("Database Error")
        }
    }
}

//This is main the server here 
#[actix_web::main]
async fn main ()->std::io::Result<()>{
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let pool = get_db().await;
    sqlx::migrate!("./migrations")
    .run(&pool)
    .await.expect("Could not run the migartions");

    HttpServer::new(move||{
        App::new()
        .wrap(actix_web::middleware::Logger::default())
        .app_data(web::Data::new(pool.clone()))
        .service(health)
        .service(get_books)
        .service(add_books)
        .service(delete_books)
        .service(get_by_id)
        .service(update_books)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
} 