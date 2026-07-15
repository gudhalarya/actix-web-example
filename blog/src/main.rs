//this is the main file here --------> 
mod error;//This is the errors file
mod db;//This is the db file
mod handlers;//Main logic file is here 
mod auth;//This is for the authentication of the users

use actix_web::{App, HttpResponse, HttpServer, get, web};

use crate::{auth::{login, register}, db::get_db};
#[get("/health")]
async fn health()->HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({"Ok":"Status is all good"}))
}

#[actix_web::main]
async fn main ()->std::io::Result<()>{
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();
    let pool = get_db().await.unwrap();


    HttpServer::new(move||{
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(actix_web::middleware::Logger::default())
        .service(health)
        .service(login)
        .service(register)
    }).bind(("127.0.0.1",8080))?
    .run()
    .await
}