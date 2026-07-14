use std::env;

use anyhow::Context;
use sqlx::{PgPool, postgres::{PgPoolOptions}};

use crate::error::AppResponse;

//This is to esatblish connection with the database
pub async fn get_db()->AppResponse<PgPool>{
    let db_url = env::var("DATABASE_URL").context("Env variable is not found ")?;
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .min_connections(2)
    .connect(&db_url).await.context("Could not connect to the database")?;

    Ok(pool)
}