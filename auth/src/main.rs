use actix_web::{App, HttpResponse, HttpServer, get, web};
use tracing_subscriber::fmt::init;

use crate::db::get_db;

//This is the main file of the code ----------> 
mod db;
mod models;
mod error;

//This is a common fn to check the health of the system 
#[get("/health")]
async fn health()->HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({"Ok":"Status is good "}))
}


//This is the main server code here 
#[actix_web::main]
async fn main()->std::io::Result<()>{
    dotenvy::dotenv().ok();
    let pool = get_db().await;
    tracing_subscriber::fmt().init();
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