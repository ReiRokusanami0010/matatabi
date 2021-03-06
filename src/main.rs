use dotenv::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use logger::Logger;

mod database;
mod models;
mod server;
#[deprecated]
mod routes;
mod routing;
mod logger;

#[tokio::main]
//#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| "matatabi=debug".into())))
        .with(tracing_subscriber::fmt::layer())
        .init();
    //env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let logger = Logger::new(Some("Matatabi"));
    logger.info("Cats are crazy about Matatabi. ฅ^•ω•^ฅ");

    database::postgres_database::migration()
        .await
        .expect("An Error occurred by database migration.");
    let pool = database::postgres_database::connect()
        .await
        .expect("An Error occurred by database connection pool.");

    server::server_run(pool).await;

    Ok(())
}