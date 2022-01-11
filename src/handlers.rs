use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use tide::{Request, Response, StatusCode};
use url::{Position, Url};

use crate::State;

#[derive(Debug, Deserialize)]
struct RequestBody {
    url: Url,
}

pub(crate) async fn shorten(mut req: Request<State>) -> tide::Result {
    let RequestBody { url } = req.body_json().await?;

    // Validate url host against whitelist
    if let Some(whitelist) = &req.state().host_whitelist {
        if !url.has_host() || !whitelist.contains(&url.host_str().unwrap().to_owned()) {
            return Ok(Response::builder(StatusCode::BadRequest)
                .body("Url host not whitelisted!")
                .build());
        }
    }

    // Create random alphanumeric id with a given length
    let id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(req.state().id_length)
        .map(char::from)
        .collect();

    // Create short url
    let remote_url = req
        .remote()
        .and_then(|r| Url::parse(r).ok())
        .unwrap_or_else(|| req.url().to_owned());
    let remote_host = remote_url[Position::BeforeHost..Position::AfterPort].to_owned();
    let short_url = Url::parse(&format!("https://{}/{}", remote_host, id))?;

    // Insert record
    sqlx::query("INSERT INTO urls (id, url) VALUES (?, ?)")
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
    let row: (String,) = sqlx::query_as("SELECT url FROM urls WHERE id = ?")
        .bind(id)
        .fetch_one(&req.state().db_pool)
        .await?;

    Ok(Response::builder(StatusCode::MovedPermanently)
        .header("Location", row.0)
        .build())
}
