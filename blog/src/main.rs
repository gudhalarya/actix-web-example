#![allow(unused)]

use actix_web::{App, HttpResponse, HttpServer, get, web};
use crate::db::get_db;

//This is the main file here -----------> 
mod error;//custom erros are here
mod db;

#[get("/health")]
async fn health()->HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({"ok":"Status is ok "}))
}

#[actix_web::main]
async fn main ()->std::io::Result<()>{
    dotenvy::dotenv().ok();
    let pool = get_db().await.unwrap();
    tracing_subscriber::fmt().init();
    HttpServer::new(move||{
        App::new()
        .wrap(actix_web::middleware::Logger::default())
        .app_data(web::Data::new(pool.clone()))
        .service(health)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}