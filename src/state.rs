use std::env;

use sqlx::SqlitePool;

// Shared application state
#[derive(Clone, Debug)]
pub struct State {
    pub db_pool: SqlitePool,
    pub host_whitelist: Vec<String>,
    pub id_length: usize,
}

impl State {
    pub(crate) async fn new() -> anyhow::Result<State> {
        let db_url = env::var("DATABASE_URL").expect("Missing `DATABASE_URL` environment variable");
        let db_pool = SqlitePool::connect(&db_url).await?;
        tide::log::info!("DATABASE_URL: {}", db_url);

        let whitelist =
            env::var("HOST_WHITELIST").expect("Missing `HOST_WHITELIST` environment variable");
        let host_whitelist: Vec<String> = whitelist.split_whitespace().map(String::from).collect();
        tide::log::info!("HOST_WHITELIST: {}", whitelist);

        let id_length = env::var("ID_LENGTH")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .expect("Failed parsing to integer");
        tide::log::info!("ID_LENGTH: {}", id_length);

        Ok(State {
            db_pool,
            host_whitelist,
            id_length,
        })
    }
}
