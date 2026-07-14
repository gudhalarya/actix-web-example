use std::env;
use anyhow::Context;
use sqlx::{PgPool, postgres::PgPoolOptions};
use colored::{Colorize};

use crate::error::AppResponse;

//This is the file for esatblishing the connection with the database
pub async fn get_db()->AppResponse<PgPool>{
    let db_url = env::var("DATABASE_URL").context("Could not find the database url in the env file 'SET THE DATABASE VARIABLE MF'")?;
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .min_connections(2)
    .acquire_timeout(std::time::Duration::from_secs(30))
    .connect(&db_url)
    .await.context("Could not connect to the database")?;

    eprintln!("{}", "✔ CONNECTED TO THE DATABASE".bold().truecolor(0, 200, 0));
    
    Ok(pool)
}