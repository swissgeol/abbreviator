use std::env;

use sqlx::SqlitePool;
use tide::{Request, Response, Server, StatusCode};

mod handlers;
#[cfg(test)]
mod tests;

// Shared application state
#[derive(Clone, Debug)]
struct State {
    db_pool: SqlitePool,
    id_length: usize,
    host_whitelist: Vec<String>,
}

impl State {
    // Setup state from environment
    async fn setup(whitelist: String) -> anyhow::Result<State> {
        let id_length = env::var("ID_LENGTH")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .expect("Failed to parse `ID_LENGTH` to integer");

        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| String::from("sqlite::memory:"));
        let db_pool = SqlitePool::connect(&db_url).await?;

        let host_whitelist = whitelist.split_whitespace().map(String::from).collect::<Vec<_>>();

        Ok(State {
            db_pool,
            id_length,
            host_whitelist,
        })
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // Start logging
    tide::log::start();

    // Create application state
    let whitelist = env::var("HOST_WHITELIST").expect("Missing whitelist");
    let state = State::setup(whitelist).await?;

    // Run any pending database migrations
    sqlx::migrate!().run(&state.db_pool).await?;

    // Compose server
    let app = server(state);

    // Listen...
    let address = format!(
        "{}:{}",
        env::var("HOST").unwrap_or_else(|_| String::from("0.0.0.0")),
        env::var("PORT").unwrap_or_else(|_| 8080.to_string())
    );
    app.listen(address).await?;

    Ok(())
}

fn server(state: State) -> Server<State> {
    // Create app
    let mut app = tide::with_state(state);

    // Add routes & handlers
    app.at("/").post(handlers::shorten);
    app.at("/:id").get(handlers::resolve);

    // Health check
    app.at("/health").get(|req: Request<State>| async move {
        let pool = &req.state().db_pool;
        if sqlx::query("SELECT 1").fetch_one(pool).await.is_ok() {
            Ok(Response::new(StatusCode::Ok))
        } else {
            Ok(Response::new(StatusCode::ServiceUnavailable))
        }
    });

    app
}
