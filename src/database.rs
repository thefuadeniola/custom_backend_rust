use sqlx::MySqlPool;

use dotenv::dotenv;
use std::env;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();

    let database = env::var("DATABASE_URL").expect("Failed to load connection string");
    MySqlPool::connect(&database).await

}

// fix environmental var