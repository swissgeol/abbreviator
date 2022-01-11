use tide::http::{Method, Request, Response};
use url::Url;

use super::*;

#[async_std::test]
async fn basic_integration() -> tide::Result<()> {
    let state = State::setup(String::from("beta.swissgeol.ch toto")).await?;

    sqlx::migrate!().run(&state.db_pool).await?;

    let app = server(state);


    let incorrect_url = "https://betina.swissgeol.ch/?layers=ch.swisst";

    // Create shortlink
    let mut req400 = Request::new(Method::Post, "https://link.swissgeol.ch/");
    req400.set_body(format!("{{\"url\": \"{}\"}}", incorrect_url));
    let res400: Response = app.respond(req400).await?;
    assert_eq!(StatusCode::BadRequest, res400.status());


    let url = "https://beta.swissgeol.ch/?anything_ok";

    // Create shortlink
    let mut req = Request::new(Method::Post, "https://link.swissgeol.ch/");
    req.set_body(format!("{{\"url\": \"{}\"}}", url));

    let res: Response = app.respond(req).await?;
    assert_eq!(201, res.status());

    let location = Url::parse(res.header("Location").unwrap().as_str()).unwrap();
    assert_eq!(Some("link.swissgeol.ch"), location.host_str());

    let id = location.path_segments().unwrap().last().unwrap();
    assert_eq!(5, id.len());

    // Fetch shortlink
    let req = Request::new(
        Method::Get,
        format!("https://link.swissgeol.ch/{}", id).as_str(),
    );
    let res: Response = app.respond(req).await?;

    assert_eq!(url, res.header("Location").unwrap().as_str());
    Ok(())
}
