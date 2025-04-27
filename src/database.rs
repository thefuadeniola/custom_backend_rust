use sqlx::MySqlPool;
use std::env;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    let database = env::var("DATABASE_URL").expect("Failed to load connection string");
    MySqlPool::connect(&database).await

}

// fix environmental var