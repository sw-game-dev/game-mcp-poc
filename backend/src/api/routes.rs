use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{
        IntoResponse, Json, Response,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{get, post},
};
use futures::stream::{Stream, StreamExt};
use serde_json::json;
use shared::{GameError, GameState, MakeMoveRequest, MoveSource, TauntRequest};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tracing::info;

use crate::game::manager::GameManager;
use crate::mcp::protocol::JsonRpcRequest;
use crate::mcp::server::McpServer;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub game_manager: Arc<Mutex<GameManager>>,
    pub sse_tx: broadcast::Sender<GameState>,
}

/// Wrapper for GameError to implement IntoResponse
struct ApiError(GameError);

impl From<GameError> for ApiError {
    fn from(err: GameError) -> Self {
        ApiError(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            GameError::CellOccupied { .. }
            | GameError::OutOfBounds { .. }
            | GameError::WrongTurn { .. }
            | GameError::GameOver { .. } => (StatusCode::BAD_REQUEST, self.0.to_string()),
            GameError::GameNotFound => (StatusCode::NOT_FOUND, self.0.to_string()),
            GameError::DatabaseError { .. } | GameError::InternalError { .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string())
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

/// Helper to broadcast game state changes via SSE
fn broadcast_state(state: &AppState, game_state: &GameState) {
    // Ignore send errors (no clients connected is fine)
    let _ = state.sse_tx.send(game_state.clone());
}

/// GET /api/game - Get current game state
async fn get_game_state(State(state): State<AppState>) -> Result<Json<GameState>, ApiError> {
    info!("GET /api/game");

    let mut manager = state.game_manager.lock().unwrap();
    let game_state = manager.get_or_create_game()?;

    info!("Returning game state: {}", game_state.id);
    Ok(Json(game_state))
}

/// POST /api/game/new - Create a new game
async fn create_new_game(State(state): State<AppState>) -> Result<Json<GameState>, ApiError> {
    info!("POST /api/game/new");

    let mut manager = state.game_manager.lock().unwrap();
    let game_state = manager.restart_game()?;

    info!("Created new game: {}", game_state.id);
    broadcast_state(&state, &game_state);
    Ok(Json(game_state))
}

/// POST /api/game/move - Make a move
async fn make_move(
    State(state): State<AppState>,
    Json(request): Json<MakeMoveRequest>,
) -> Result<Json<GameState>, ApiError> {
    info!(
        "POST /api/game/move - row: {}, col: {}",
        request.row, request.col
    );

    let mut manager = state.game_manager.lock().unwrap();
    let game_state = manager.make_move(request.row, request.col, MoveSource::UI)?;

    info!("Move made successfully");
    broadcast_state(&state, &game_state);
    Ok(Json(game_state))
}

/// POST /api/game/taunt - Add a taunt message
async fn add_taunt(
    State(state): State<AppState>,
    Json(request): Json<TauntRequest>,
) -> Result<StatusCode, ApiError> {
    info!("POST /api/game/taunt - message: {}", request.message);

    let mut manager = state.game_manager.lock().unwrap();
    manager.add_taunt(request.message)?;

    info!("Taunt added successfully");
    Ok(StatusCode::OK)
}

/// GET /api/events - Server-Sent Events stream for game state updates
async fn game_events(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("Client connected to SSE stream");

    let rx = state.sse_tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|result| async move {
        match result {
            Ok(game_state) => match serde_json::to_string(&game_state) {
                Ok(json) => Some(Ok(Event::default().data(json))),
                Err(e) => {
                    tracing::error!("Failed to serialize game state for SSE: {}", e);
                    None
                }
            },
            Err(e) => {
                tracing::error!("SSE broadcast error: {}", e);
                None
            }
        }
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    )
}

/// POST /mcp - MCP protocol over HTTP (JSON-RPC 2.0)
async fn mcp_handler(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("MCP HTTP request received");

    // Parse JSON-RPC request
    let json_str = serde_json::to_string(&request).map_err(|_| StatusCode::BAD_REQUEST)?;
    let rpc_request = JsonRpcRequest::from_json(&json_str).map_err(|e| {
        tracing::error!("Failed to parse JSON-RPC request: {}", e.message);
        StatusCode::BAD_REQUEST
    })?;

    // Validate request
    if let Err(e) = rpc_request.validate() {
        tracing::error!("Invalid JSON-RPC request: {}", e.message);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create temporary MCP server (it's stateless except for the game manager)
    let mut manager = state.game_manager.lock().unwrap();
    let mut mcp_server = McpServer::new_with_manager(&mut manager);

    // Handle the request
    let response_str = mcp_server.handle_request(&json_str);
    drop(manager); // Release lock before broadcasting

    // Parse response
    let response: serde_json::Value =
        serde_json::from_str(&response_str).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // If the MCP call modified the game state, broadcast it
    // (We check if it's a successful result for state-modifying methods)
    if let Some(result) = response.get("result")
        && !result.is_null()
    {
        let method = rpc_request.method.as_str();
        if matches!(method, "make_move" | "restart_game") {
            // Fetch updated state and broadcast
            let mut manager = state.game_manager.lock().unwrap();
            if let Ok(game_state) = manager.get_game_state() {
                broadcast_state(&state, &game_state);
            }
        }
    }

    Ok(Json(response))
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// Create the API router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/game", get(get_game_state))
        .route("/api/game/new", post(create_new_game))
        .route("/api/game/move", post(make_move))
        .route("/api/game/taunt", post(add_taunt))
        .route("/api/events", get(game_events))
        .route("/mcp", post(mcp_handler))
        .with_state(state)
}

// Unit tests removed - see api_integration.rs for comprehensive API tests via actual HTTP requests
