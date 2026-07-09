use std::env;

use anyhow::Context;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::error::AppResponse;

//This is the database fn here 
async fn get_db()->AppResponse<PgPool>{
    let db_url= env::var("DATABASE_URL").context("Could not find the database error ")?;
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .acquire_timeout(std::time::Duration::from_secs(10))
    .connect(&db_url).await.context("Could not connect to the database")?;

    Ok(pool)
}