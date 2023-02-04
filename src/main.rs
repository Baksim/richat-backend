use richat_backend::start_server;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sqlx::mysql::MySqlPoolOptions;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool: &str = dotenv!("DATABASE_URL");
    let pool = MySqlPoolOptions::new()
    .max_connections(100)
    .connect(&pool)
    .await.expect("Unable to connect to Maria");
    start_server(pool).await
}