use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect_db() -> PgPool {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB")
}
