use clap::Parser;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // Load .env variables
    dotenv::dotenv().ok();

    // Start logging
    tide::log::start();

    // Config
    let config = abbreviator::Config::parse();

    // Compose server
    let app = abbreviator::server(&config).await?;

    // Listen...
    let address = format!("{}:{}", config.host, config.port);
    app.listen(address).await?;

    Ok(())
}
