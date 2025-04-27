use sqlx::MySqlPool;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect("mysql://actix_user:actixpass@db:3306/blog_server").await

}

// fix environmental var