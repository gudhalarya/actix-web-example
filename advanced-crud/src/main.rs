//This is the health fn here 

use actix_web::{App, HttpResponse, HttpServer, get};
#[get("/health")]
async fn health()->HttpResponse{
    return HttpResponse::Ok().json(serde_json::json!({"status":"ok"}));
}


#[actix_web::main]
async fn main ()->std::io::Result<()>{
    tracing_subscriber::fmt::init();
    HttpServer::new(||{
        App::new()
        .wrap(actix_web::middleware::Logger::default())
        .service(health)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}