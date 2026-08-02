/*This is the main file of the code*/
//The first thing we will do is the connection to the database 
mod db;
mod error;
use actix_web::{App, HttpResponse, HttpServer, post, web};

use crate::db::get_db;

//This is to check the health of the server 
#[post("/health")]
async fn health()->HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({"Ok":"Status is ok "}))
}

//This is the main server 
#[actix_web::main]
async fn main ()->std::io::Result<()>{
    tracing_subscriber::fmt().init();
    let pool = get_db().await;
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
