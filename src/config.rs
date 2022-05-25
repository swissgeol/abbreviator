use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Config {
    /// SQLite database url
    #[clap(long, env, hide_env_values = true, parse(try_from_str))]
    pub database_url: url::Url,
    /// Whitespace separated list of allowed hosts of the URL to be shortened
    #[clap(long, env, hide_env_values = true, parse(try_from_str))]
    pub host_whitelist: String,
    /// Length of the generated key
    #[clap(long, env, default_value = "5")]
    pub id_length: usize,
    /// Listening port of the server
    #[clap(long, env, default_value = "8080")]
    pub port: String,
    /// istening host address of the server
    #[clap(long, env, default_value = "0.0.0.0")]
    pub host: String,
}
