//This is the database file here 
use std::env;

use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn get_db()->PgPool{
    let db_url = env::var("DATABASE_URL").expect("Could not find the database url in the env file "); 
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .min_connections(2)
    .acquire_timeout(std::time::Duration::from_secs(40))
    .connect(&db_url)
    .await.expect("COuld not connect to the database"); 

    pool
}