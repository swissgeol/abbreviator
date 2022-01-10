use tide::http::{Method, Request, Response};
use url::Url;

use super::*;

#[async_std::test]
async fn basic_integration() -> tide::Result<()> {
    let state = State::setup().await?;

    sqlx::migrate!().run(&state.db_pool).await?;

    let app = server(state);

    let url = "https://beta.swissgeol.ch/?layers=ch.swisstopo.geologie-geocover%1Cboreholes%2Ccross_section%2Cearthquakes&layers_visibility=true%2Cfalse%2Cfalse%2Cfalse&layers_transparency=0.3%2C0%2C0%2C0&lang=en&map_transparency=0&map=ch.swisstopo.pixelkarte-grau&lon=6.06749&lat=43.77784&elevation=204227&heading=26&pitch=-33";

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
