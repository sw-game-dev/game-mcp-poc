use serde::{Deserialize, Serialize};
use thiserror::Error;

// Build info module
pub mod build_info;

/// Player representation (X or O)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    X,
    O,
}

impl Player {
    /// Get the opposite player
    pub fn opponent(self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

/// Cell state on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Cell {
    #[default]
    Empty,
    Occupied(Player),
}

/// Game status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    InProgress,
    Won(Player),
    Draw,
}

/// Source of the move (UI or MCP)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveSource {
    #[serde(rename = "UI")]
    UI,
    #[serde(rename = "MCP")]
    MCP,
}

/// A single move in the game
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Move {
    pub player: Player,
    pub row: u8,
    pub col: u8,
    pub timestamp: i64,
    #[serde(default)]
    pub source: Option<MoveSource>,
}

/// Winning line positions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WinningLine {
    pub positions: Vec<(u8, u8)>,
}

/// Complete game state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub id: String,
    pub board: [[Cell; 3]; 3],
    pub current_turn: Player,
    pub human_player: Player,
    pub ai_player: Player,
    pub status: GameStatus,
    pub move_history: Vec<Move>,
    pub taunts: Vec<String>,
    #[serde(default)]
    pub winning_line: Option<WinningLine>,
}

/// API request to make a move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeMoveRequest {
    pub row: u8,
    pub col: u8,
}

/// API request to add a taunt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TauntRequest {
    pub message: String,
}

/// Error types for the game
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum GameError {
    #[error("Cell ({row}, {col}) is already occupied")]
    CellOccupied { row: u8, col: u8 },

    #[error("Position ({row}, {col}) is out of bounds (must be 0-2)")]
    OutOfBounds { row: u8, col: u8 },

    #[error("It is not {player:?}'s turn")]
    WrongTurn { player: Player },

    #[error("Game is already over: {status:?}")]
    GameOver { status: GameStatus },

    #[error("Game not found")]
    GameNotFound,

    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Internal error: {message}")]
    InternalError { message: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_opponent() {
        assert_eq!(Player::X.opponent(), Player::O);
        assert_eq!(Player::O.opponent(), Player::X);
    }

    #[test]
    fn test_cell_default() {
        assert_eq!(Cell::default(), Cell::Empty);
    }
}
