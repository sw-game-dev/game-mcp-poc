use rusqlite::{Connection, params};
use shared::{Cell, GameError, GameState, GameStatus, Move, Player};
use std::time::{SystemTime, UNIX_EPOCH};

/// Game repository for database operations
#[allow(dead_code)] // Will be used by API and MCP layers
pub struct GameRepository {
    conn: Connection,
}

#[allow(dead_code)] // Will be used by API and MCP layers
impl GameRepository {
    /// Create a new repository with the given database path
    pub fn new(db_path: &str) -> Result<Self, GameError> {
        let conn = Connection::open(db_path).map_err(|e| GameError::DatabaseError {
            message: e.to_string(),
        })?;

        // Initialize schema
        super::schema::init_schema(&conn)?;

        Ok(Self { conn })
    }

    /// Create a new repository with an in-memory database (for testing)
    #[cfg(test)]
    pub fn new_in_memory() -> Result<Self, GameError> {
        let conn = Connection::open_in_memory().map_err(|e| GameError::DatabaseError {
            message: e.to_string(),
        })?;

        super::schema::init_schema(&conn)?;

        Ok(Self { conn })
    }

    /// Save a new game to the database
    pub fn save_game(&self, game: &GameState) -> Result<(), GameError> {
        let status_str = match &game.status {
            GameStatus::InProgress => "InProgress".to_string(),
            GameStatus::Won(Player::X) => "Won_X".to_string(),
            GameStatus::Won(Player::O) => "Won_O".to_string(),
            GameStatus::Draw => "Draw".to_string(),
        };

        let human_str = match game.human_player {
            Player::X => "X",
            Player::O => "O",
        };

        let ai_str = match game.ai_player {
            Player::X => "X",
            Player::O => "O",
        };

        let turn_str = match game.current_turn {
            Player::X => "X",
            Player::O => "O",
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.conn
            .execute(
                "INSERT INTO games (id, human_player, ai_player, current_turn, status, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                 ON CONFLICT(id) DO UPDATE SET
                     current_turn = ?4,
                     status = ?5,
                     updated_at = ?7",
                params![
                    &game.id,
                    human_str,
                    ai_str,
                    turn_str,
                    status_str,
                    now,
                    now
                ],
            )
            .map_err(|e| GameError::DatabaseError {
                message: e.to_string(),
            })?;

        Ok(())
    }

    /// Load a game from the database
    pub fn load_game(&self, game_id: &str) -> Result<GameState, GameError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, human_player, ai_player, current_turn, status FROM games WHERE id = ?1",
            )
            .map_err(|e| GameError::DatabaseError {
                message: e.to_string(),
            })?;

        let game = stmt
            .query_row(params![game_id], |row| {
                let id: String = row.get(0)?;
                let human_str: String = row.get(1)?;
                let ai_str: String = row.get(2)?;
                let turn_str: String = row.get(3)?;
                let status_str: String = row.get(4)?;

                let human_player = if human_str == "X" {
                    Player::X
                } else {
                    Player::O
                };

                let ai_player = if ai_str == "X" { Player::X } else { Player::O };

                let current_turn = if turn_str == "X" {
                    Player::X
                } else {
                    Player::O
                };

                let status = match status_str.as_str() {
                    "InProgress" => GameStatus::InProgress,
                    "Won_X" => GameStatus::Won(Player::X),
                    "Won_O" => GameStatus::Won(Player::O),
                    "Draw" => GameStatus::Draw,
                    _ => GameStatus::InProgress,
                };

                Ok((id, human_player, ai_player, current_turn, status))
            })
            .map_err(|_| GameError::GameNotFound)?;

        // Load moves to reconstruct the board
        let moves = self.load_moves(&game.0)?;
        let board = Self::reconstruct_board(&moves)?;

        // Load taunts
        let taunts = self.load_taunts(&game.0)?;

        Ok(GameState {
            id: game.0,
            board,
            current_turn: game.3,
            human_player: game.1,
            ai_player: game.2,
            status: game.4,
            move_history: moves,
            taunts,
        })
    }

    /// Save a move to the database
    pub fn save_move(&self, game_id: &str, mov: &Move) -> Result<(), GameError> {
        let player_str = match mov.player {
            Player::X => "X",
            Player::O => "O",
        };

        self.conn
            .execute(
                "INSERT INTO moves (game_id, player, row, col, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![game_id, player_str, mov.row, mov.col, mov.timestamp],
            )
            .map_err(|e| GameError::DatabaseError {
                message: e.to_string(),
            })?;

        Ok(())
    }

    /// Load all moves for a game
    pub fn load_moves(&self, game_id: &str) -> Result<Vec<Move>, GameError> {
        let mut stmt = self
            .conn
            .prepare("SELECT player, row, col, timestamp FROM moves WHERE game_id = ?1 ORDER BY timestamp ASC")
            .map_err(|e| GameError::DatabaseError {
                message: e.to_string(),
            })?;

        let moves = stmt
            .query_map(params![game_id], |row| {
                let player_str: String = row.get(0)?;
                let player = if player_str == "X" {
                    Player::X
                } else {
                    Player::O
                };

                Ok(Move {
                    player,
                    row: row.get(1)?,
                    col: row.get(2)?,
                    timestamp: row.get(3)?,
                })
            })
            .map_err(|e| GameError::DatabaseError {
                message: e.to_string(),
            })?;

        moves
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| GameError::DatabaseError {
                message: e.to_string(),
            })
    }

    /// Save a taunt to the database
    pub fn save_taunt(&self, game_id: &str, message: &str) -> Result<(), GameError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.conn
            .execute(
                "INSERT INTO taunts (game_id, message, timestamp) VALUES (?1, ?2, ?3)",
                params![game_id, message, now],
            )
            .map_err(|e| GameError::DatabaseError {
                message: e.to_string(),
            })?;

        Ok(())
    }

    /// Load all taunts for a game
    pub fn load_taunts(&self, game_id: &str) -> Result<Vec<String>, GameError> {
        let mut stmt = self
            .conn
            .prepare("SELECT message FROM taunts WHERE game_id = ?1 ORDER BY timestamp ASC")
            .map_err(|e| GameError::DatabaseError {
                message: e.to_string(),
            })?;

        let taunts = stmt
            .query_map(params![game_id], |row| row.get(0))
            .map_err(|e| GameError::DatabaseError {
                message: e.to_string(),
            })?;

        taunts
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| GameError::DatabaseError {
                message: e.to_string(),
            })
    }

    /// Reconstruct board from moves
    fn reconstruct_board(moves: &[Move]) -> Result<[[Cell; 3]; 3], GameError> {
        let mut board = [[Cell::Empty; 3]; 3];

        for mov in moves {
            if mov.row >= 3 || mov.col >= 3 {
                return Err(GameError::OutOfBounds {
                    row: mov.row,
                    col: mov.col,
                });
            }

            board[mov.row as usize][mov.col as usize] = Cell::Occupied(mov.player);
        }

        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_game() -> GameState {
        GameState {
            id: Uuid::new_v4().to_string(),
            board: [[Cell::Empty; 3]; 3],
            current_turn: Player::X,
            human_player: Player::X,
            ai_player: Player::O,
            status: GameStatus::InProgress,
            move_history: vec![],
            taunts: vec![],
        }
    }

    #[test]
    fn test_save_and_load_game() {
        let repo = GameRepository::new_in_memory().unwrap();
        let game = create_test_game();
        let game_id = game.id.clone();

        // Save game
        assert!(repo.save_game(&game).is_ok());

        // Load game
        let loaded = repo.load_game(&game_id).unwrap();
        assert_eq!(loaded.id, game_id);
        assert_eq!(loaded.human_player, Player::X);
        assert_eq!(loaded.ai_player, Player::O);
        assert_eq!(loaded.current_turn, Player::X);
        assert_eq!(loaded.status, GameStatus::InProgress);
    }

    #[test]
    fn test_load_nonexistent_game() {
        let repo = GameRepository::new_in_memory().unwrap();
        let result = repo.load_game("nonexistent");
        assert!(matches!(result, Err(GameError::GameNotFound)));
    }

    #[test]
    fn test_save_and_load_moves() {
        let repo = GameRepository::new_in_memory().unwrap();
        let game = create_test_game();
        let game_id = game.id.clone();

        repo.save_game(&game).unwrap();

        // Save some moves
        let move1 = Move {
            player: Player::X,
            row: 0,
            col: 0,
            timestamp: 1000,
        };
        let move2 = Move {
            player: Player::O,
            row: 1,
            col: 1,
            timestamp: 2000,
        };

        repo.save_move(&game_id, &move1).unwrap();
        repo.save_move(&game_id, &move2).unwrap();

        // Load moves
        let moves = repo.load_moves(&game_id).unwrap();
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0].player, Player::X);
        assert_eq!(moves[0].row, 0);
        assert_eq!(moves[0].col, 0);
        assert_eq!(moves[1].player, Player::O);
        assert_eq!(moves[1].row, 1);
        assert_eq!(moves[1].col, 1);
    }

    #[test]
    fn test_save_and_load_taunts() {
        let repo = GameRepository::new_in_memory().unwrap();
        let game = create_test_game();
        let game_id = game.id.clone();

        repo.save_game(&game).unwrap();

        // Save taunts
        repo.save_taunt(&game_id, "You call that a move?").unwrap();
        repo.save_taunt(&game_id, "I've seen better from a toddler!")
            .unwrap();

        // Load taunts
        let taunts = repo.load_taunts(&game_id).unwrap();
        assert_eq!(taunts.len(), 2);
        assert_eq!(taunts[0], "You call that a move?");
        assert_eq!(taunts[1], "I've seen better from a toddler!");
    }

    #[test]
    fn test_reconstruct_board() {
        let moves = vec![
            Move {
                player: Player::X,
                row: 0,
                col: 0,
                timestamp: 1000,
            },
            Move {
                player: Player::O,
                row: 1,
                col: 1,
                timestamp: 2000,
            },
            Move {
                player: Player::X,
                row: 2,
                col: 2,
                timestamp: 3000,
            },
        ];

        let board = GameRepository::reconstruct_board(&moves).unwrap();

        assert_eq!(board[0][0], Cell::Occupied(Player::X));
        assert_eq!(board[1][1], Cell::Occupied(Player::O));
        assert_eq!(board[2][2], Cell::Occupied(Player::X));
        assert_eq!(board[0][1], Cell::Empty);
    }

    #[test]
    fn test_update_game() {
        let repo = GameRepository::new_in_memory().unwrap();
        let mut game = create_test_game();
        let game_id = game.id.clone();

        // Save initial game
        repo.save_game(&game).unwrap();

        // Update game state
        game.current_turn = Player::O;
        game.status = GameStatus::Won(Player::X);

        // Save updated game
        repo.save_game(&game).unwrap();

        // Load and verify
        let loaded = repo.load_game(&game_id).unwrap();
        assert_eq!(loaded.current_turn, Player::O);
        assert_eq!(loaded.status, GameStatus::Won(Player::X));
    }
}
