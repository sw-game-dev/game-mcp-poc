/// API client module for communicating with the backend
/// Only compiled for WASM target
#[cfg(target_arch = "wasm32")]
mod client;

#[cfg(target_arch = "wasm32")]
pub use client::{create_new_game, fetch_game_state, make_move, send_taunt};
