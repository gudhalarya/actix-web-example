mod error;
mod db;
mod handlers;
//This is the health fn here 
use actix_web::{App, HttpResponse, HttpServer, get, web};

use crate::{db::get_db, handlers::{add_books, get_books}};
#[get("/health")]
async fn health()->HttpResponse{
    return HttpResponse::Ok().json(serde_json::json!({"status":"ok"}));
}

#[actix_web::main]
async fn main ()->std::io::Result<()>{
    dotenvy::dotenv().ok();
    let pool = get_db().await.expect("Could not connect to the database");
    tracing_subscriber::fmt::init();
    HttpServer::new(move||{
        App::new()
        .wrap(actix_web::middleware::Logger::default())
        .app_data(web::Data::new(pool.clone()))
        .service(health)
        .service(get_books)
        .service(add_books)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}