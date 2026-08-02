use std::env;

use sqlx::{PgPool, postgres::PgPoolOptions};

//This is the file for db connection 
async fn get_db()->PgPool{
    let db_url = env::var("DATABASE_KEY").expect("Could not find the database key in the env file");
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .min_connections(2)
    .acquire_timeout(std::time::Duration::from_secs(5))
    .connect(&db_url)
    .await.expect("Could not connect ");

    pool
}