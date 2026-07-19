#![allow(unused)]
use std::env;

use actix_web::{App, HttpResponse, HttpServer, get, web};
use sqlx::{PgPool, postgres::PgPoolOptions};
mod models;
mod errors;

//This is where the whole code will be written
//first we will connect to the database 
async fn get_db()->PgPool{
    let db_url = env::var("DATABASE_URL").expect("Could not find the database url in the env file");
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .min_connections(2)
    .connect(&db_url).await
    .expect("Could not connect to the database");

    pool
}
//This is the health fn here 
#[get("/health")]
async fn health()->HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({"Ok":"Status is green"}))
}


//this is the main server file here 
#[actix_web::main]
async fn main()->std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();
    let pool  = get_db().await;
    HttpServer::new(move||{
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(actix_web::middleware::Logger::default())
        .service(health)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}