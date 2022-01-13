use tide::{Request, Response, Server, StatusCode};

pub mod handlers;
mod state;

pub use state::State;

pub async fn server() -> anyhow::Result<Server<State>> {
    // Create application state
    let state = State::new().await?;

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

    // Run any pending database migrations
    sqlx::migrate!().run(&app.state().db_pool).await?;

    Ok(app)
}
