use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use spin_sdk::{
    config,
    http::{Request, Response},
    http_component,
};

#[derive(Serialize, Deserialize, Debug)]
struct RDSResponse {
    rds: String,
    // program: String,
    // host: String,
    // slogan: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn raadio_tallinn(_req: Request) -> Result<Response> {
    let current_track = get_current_track()?;

    Ok(http::Response::builder()
        .status(200)
        .body(Some(current_track.into()))?)
}

fn get_current_track() -> Result<String> {
    let url = config::get("current_track_api_url")?;
    let response = spin_sdk::outbound_http::send_request(
        http::Request::builder().method("GET").uri(url).body(None)?,
    )?;

    let body = response
        .into_body()
        .ok_or(anyhow!("Failed to get current track from API"))?;

    let rds_response: RDSResponse = serde_json::from_slice(&body)?;
    Ok(rds_response.rds)
}
