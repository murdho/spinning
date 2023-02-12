use anyhow::{anyhow, Result};
use serde::Deserialize;
use spin_sdk::{
    config,
    http::{Request, Response},
    http_component,
};

#[derive(Deserialize, Debug)]
struct PlaylistItem {
    artist: String,
    song: String,
    // date: String,
    // channel: i32,
    // timestamp: i64,
}

/// A simple Spin HTTP component.
#[http_component]
fn supla_fi(_req: Request) -> Result<Response> {
    match get_latest_tracks() {
        Ok(latest_tracks) => Ok(http::Response::builder()
            .status(200)
            .body(Some(latest_tracks.join("\n").into()))?),

        Err(err) => {
            eprintln!("ERROR: {}", err);
            Err(anyhow!("Internal Server Error"))
        }
    }
}

fn get_latest_tracks() -> Result<Vec<String>> {
    let api_url = config::get("playlist_api_url")?;
    let channel_id = config::get("channel_id")?;

    let url = format!("{api_url}?channel={channel_id}&next_token=&limit=20");
    let response = spin_sdk::outbound_http::send_request(
        http::Request::builder().method("GET").uri(url).body(None)?,
    )?;

    let body = response
        .into_body()
        .ok_or(anyhow!("failed to consume playlist items response body"))?;

    #[derive(Deserialize)]
    struct ResponseBody {
        items: Vec<PlaylistItem>,
    }

    let response: ResponseBody = serde_json::from_slice(&body)?;

    Ok(response
        .items
        .iter()
        .map(|item| format!("{} - {}", item.artist, item.song))
        .collect())
}
