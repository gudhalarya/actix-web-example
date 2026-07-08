mod error;
mod db;
//This is the health fn here 
use actix_web::{App, HttpResponse, HttpServer, get, web};

use crate::db::get_db;
#[get("/health")]
async fn health()->HttpResponse{
    return HttpResponse::Ok().json(serde_json::json!({"status":"ok"}));
}

#[actix_web::main]
async fn main ()->std::io::Result<()>{
    let pool = get_db().await.expect("Could not connect to the database");
    tracing_subscriber::fmt::init();
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