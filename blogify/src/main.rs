use actix_web::{App, HttpResponse, HttpServer, get, web};

use crate::{db::get_db, error::AppResponse};

mod error;
mod db;

#[get("/health")]
async fn health()->AppResponse<HttpResponse>{
    Ok(HttpResponse::Ok().json(serde_json::json!({"Status":"ok"})))
}

#[actix_web::main]
async fn main ()->std::io::Result<()>{
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

