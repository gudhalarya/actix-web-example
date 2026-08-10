/*This is the main file here ---------> 
First thing we will do is the backend server 
second the database setup 
Third we will do the tracing and logs dude
Fourth what we will do is to make the custom errors (most important) 
*/

//This is the health route to check the health of teh fn here 
mod db;
mod error;
use actix_web::{App, HttpResponse, HttpServer, post, web};
use anyhow::Context;
mod auth;

use crate::{auth::{login, register}, db::get_db};
#[post("/what")]
async fn health()->HttpResponse{
    HttpResponse::Ok().json(serde_json::json!({"Ok":"Status is ok "}))
}

#[actix_web::main]
async fn main ()->std::io::Result<()>{
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();
    let pool = get_db().await; 
    sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .context("Could not run the migrations "); 

    HttpServer::new(move||{
        App::new()
        .wrap(actix_web::middleware::Logger::default())
        .app_data(web::Data::new(pool.clone()))
        .service(health)
        .service(login)
        .service(register)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}