use std::str::FromStr;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

use crate::config::Config;

// Shared application state
#[derive(Clone, Debug)]
pub struct State {
    pub db_pool: SqlitePool,
    pub host_whitelist: Vec<String>,
    pub id_length: usize,
}

impl State {
    pub async fn new(config: Config) -> anyhow::Result<State> {
        // Database pool
        let db_options =
            SqliteConnectOptions::from_str(config.database_url.as_str())?.create_if_missing(true);
        let db_pool = SqlitePool::connect_with(db_options).await?;

        Ok(State {
            db_pool,
            host_whitelist: config
                .host_whitelist
                .split_whitespace()
                .map(String::from)
                .collect(),
            id_length: config.id_length,
        })
    }
}
