use std::env;

use anyhow::Context;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::error::{AppResponse};

//This is to connect to the database
pub async fn get_db()->AppResponse<PgPool>{
    let db_url = env::var("DATABASE_URL").context("Could not find the database url")?;
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&db_url)
    .await.context("Failed to connect ")?;

    Ok(pool)
}