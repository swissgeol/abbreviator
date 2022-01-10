use std::env;

use sqlx::SqlitePool;
use tide::Server;

mod handlers;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
struct State {
    db_pool: SqlitePool,
    id_length: usize,
    host_whitelist: Option<Vec<String>>,
}

impl State {
    async fn setup() -> anyhow::Result<State> {
        let id_length = env::var("ID_LENGTH")
            .expect("Missing `ID_LENGTH` env variable")
            .parse()
            .unwrap();

        let db_url = env::var("DATABASE_URL").expect("Missing `DATABASE_URL` env variable");
        let db_pool = SqlitePool::connect(&db_url).await?;
        let host_whitelist = env::var("HOST_WHITELIST")
            .map(|s| s.split_whitespace().map(String::from).collect())
            .ok();

        Ok(State {
            db_pool,
            id_length,
            host_whitelist,
        })
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tide::log::start();

    let state = State::setup().await?;

    sqlx::migrate!().run(&state.db_pool).await?;

    let app = server(state);

    app.listen(format!("0.0.0.0:{}", env::var("PORT")?)).await?;

    Ok(())
}

fn server(state: State) -> Server<State> {
    let mut app = tide::with_state(state);

    app.at("/").post(handlers::shorten);
    app.at("/:id").get(handlers::resolve);

    app
}
