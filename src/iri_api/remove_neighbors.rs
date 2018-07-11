use super::responses::RemoveNeighborsResponse;
use crate::Result;
use reqwest::header::{ContentType, Headers};

/// Removes a list of neighbors to your node.
/// This is only temporary, and if you have your neighbors
/// added via the command line, they will be retained after
/// you restart your node.
pub fn remove_neighbors(uri: &str, uris: &[String]) -> Result<RemoveNeighborsResponse> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "removeNeighbors",
        "uris": uris,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}