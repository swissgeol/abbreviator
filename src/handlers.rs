use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use tide::{Request, Response, StatusCode};
use url::{Position, Url};

use crate::state::State;

#[derive(Debug, Deserialize)]
struct RequestBody {
    url: Url,
}

pub(crate) async fn shorten(mut req: Request<State>) -> tide::Result {
    let RequestBody { url } = req.body_json().await?;

    // Validate url host against whitelist
    let host = url.host_str();
    let whitelist = &req.state().host_whitelist;
    if host.is_none() || !whitelist.contains(&host.unwrap().to_owned()) {
        return Ok(Response::builder(StatusCode::BadRequest)
            .body(format!("Url host `{}` not whitelisted!", &host.unwrap()))
            .build());
    }

    // Get short link id
    let id = if let Some((id,)) =
        // Check if the url already exist and reuse the id
        sqlx::query_as::<_, (String,)>(
            "SELECT id FROM urls WHERE url = ? ORDER BY created DESC LIMIT 1",
        )
        .bind(url.as_str())
        .fetch_optional(&req.state().db_pool)
        .await?
    {
        id
    } else {
        // Create random alphanumeric id with a given length
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(req.state().id_length)
            .map(char::from)
            .collect()
    };

    // Create short url
    let remote_url = req
        .remote()
        .and_then(|r| Url::parse(r).ok())
        .unwrap_or_else(|| req.url().to_owned());
    let remote_host = remote_url[Position::BeforeHost..Position::AfterPort].to_owned();
    let short_url = Url::parse(&format!("https://{}/{}", remote_host, id))?;

    // Insert or replace record
    sqlx::query("INSERT OR REPLACE INTO urls (id, url) VALUES (?, ?)")
        .bind(id)
        .bind(url.as_str())
        .execute(&req.state().db_pool)
        .await?;

    Ok(Response::builder(StatusCode::Created)
        .header("Location", short_url.as_str())
        .build())
}

pub(crate) async fn resolve(req: Request<State>) -> tide::Result {
    let id = req.param("id")?;

    // Fetch url
    let row: Option<(String,)> = sqlx::query_as("SELECT url FROM urls WHERE id = ?")
        .bind(id)
        .fetch_optional(&req.state().db_pool)
        .await?;

    match row {
        Some((url,)) => Ok(Response::builder(StatusCode::MovedPermanently)
            .header("Location", url)
            .build()),
        None => Ok(Response::new(StatusCode::NotFound)),
    }
}
