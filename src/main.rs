use std::env;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // Load .env variables
    dotenv::dotenv().ok();

    // Start logging
    tide::log::start();

    // Compose server
    let app = abbreviator::server().await?;

    // Listen...
    let address = format!(
        "{}:{}",
        env::var("HOST").unwrap_or_else(|_| String::from("0.0.0.0")),
        env::var("PORT").unwrap_or_else(|_| 8080.to_string())
    );
    app.listen(address).await?;

    Ok(())
}
