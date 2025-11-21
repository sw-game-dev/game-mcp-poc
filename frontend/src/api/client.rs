/// HTTP client for backend API
/// Only compiled for WASM target
use gloo_net::http::Request;
use shared::{GameState, MakeMoveRequest, TauntRequest};

const API_BASE: &str = "/api";

pub async fn fetch_game_state() -> Result<GameState, String> {
    let response = Request::get(&format!("{}/game", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn create_new_game() -> Result<GameState, String> {
    let response = Request::post(&format!("{}/game/new", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn make_move(row: u8, col: u8) -> Result<GameState, String> {
    let request_body = MakeMoveRequest { row, col };

    let response = Request::post(&format!("{}/game/move", API_BASE))
        .json(&request_body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP {}", response.status()));
    }

    response.json().await.map_err(|e| e.to_string())
}

pub async fn send_taunt(message: String) -> Result<(), String> {
    let request_body = TauntRequest { message };

    let response = Request::post(&format!("{}/game/taunt", API_BASE))
        .json(&request_body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.ok() {
        return Err(format!("HTTP {}", response.status()));
    }

    Ok(())
}
