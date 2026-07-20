use std::env;

use sqlx::{PgPool, postgres::PgPoolOptions};

//This is for the connection with the datbaase for the establishing of the connection 
pub async fn get_db()->PgPool{
    let db_url = env::var("DATABASE_URL").expect("Could not find the database url in the env file");
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .min_connections(2)
    .acquire_timeout(std::time::Duration::from_secs(20))
    .connect(&db_url)
    .await.expect("Could not connect to the database");

    pool
}