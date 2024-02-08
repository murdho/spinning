use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::variables;

/// A simple Spin HTTP component.
#[http_component]
fn handle_redirecta(req: Request) -> anyhow::Result<impl IntoResponse> {
    let request_url = match req.header("spin-full-url") {
        Some(header_value) => header_value.clone().into_utf8_lossy(),
        None => "<unknown>".to_string(),
    };

    let request_path = match req.header("spin-path-info") {
        Some(header_value) => header_value.clone().into_utf8_lossy(),
        None => "".to_string(),
    };

    let destination_url = variables::get("destination_url")?;
    let location_url = format!("{destination_url}{request_path}");
    println!("{} => {}", request_url, location_url);

    Ok(Response::builder()
        .status(302)
        .header("Location", location_url)
        .build())
}
