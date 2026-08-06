use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub title: String,
    pub short_text: Option<String>,
    pub url: String,
    pub cover_url: Option<String>,
    pub published: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    detail: String,
}

pub async fn fetch_games() -> Result<Vec<Game>, String> {
    let response = Request::get("/api/games")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();

    if !(200..300).contains(&status) {
        return Err(
            response
                .json::<ErrorResponse>()
                .await
                .map(|e| e.detail)
                .unwrap_or_else(|_| format!("Server Error: {status}"))
        );
    }

    response
        .json::<Vec<Game>>()
        .await
        .map_err(|e| e.to_string())
}