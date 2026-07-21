//This is the shit where we will cook up the main routes of everything 

use actix_web::{HttpResponse, post, web};
use sqlx::PgPool;

use crate::{error::{AppError, AppResponse}, helper::hash_password, models::Register};
#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    payload: web::Json<Register>,
) -> AppResponse<HttpResponse> {
    let final_pwd = hash_password(&payload.pwd)?;

    let query = "INSERT INTO users (name, email, pass) VALUES ($1, $2, $3)";

    let result = sqlx::query(query)
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&final_pwd)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::Created().json(serde_json::json!({
            "message": "User created successfully"
        }))),
        Err(err) => {
            if err.as_database_error().map_or(false, |e| e.is_unique_violation()) {
                return Err(AppError::BadRequest);
            }
            Err(AppError::InternalServerError(err.into()))
        }
    }
}