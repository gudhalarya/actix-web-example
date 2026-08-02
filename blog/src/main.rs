/*This is the main file of the code*/
//The first thing we will do is the connection to the database 
mod db;
use actix_web::{App, HttpResponse, HttpServer, post};
#[post("/health")]
async fn health()->HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({"Ok":"Status is ok "}))
}

//This is the main server 
#[actix_web::main]
async fn main ()->std::io::Result<()>{
    HttpServer::new(move||{
        App::new()
        .service(health)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
