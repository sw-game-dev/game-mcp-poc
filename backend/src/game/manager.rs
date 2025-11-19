use super::board::Board;
use super::logic::get_game_status;
use super::player::assign_players;
use crate::db::repository::GameRepository;
use shared::{Cell, GameError, GameState, GameStatus, Move, MoveSource};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Game state manager for coordinating game operations
#[allow(dead_code)] // Will be used by MCP and API layers
pub struct GameManager {
    current_game_id: Option<String>,
    repository: GameRepository,
}

#[allow(dead_code)] // Will be used by MCP and API layers
impl GameManager {
    /// Create a new game manager with the given database path
    pub fn new(db_path: &str) -> Result<Self, GameError> {
        let repository = GameRepository::new(db_path)?;
        Ok(Self {
            current_game_id: None,
            repository,
        })
    }

    /// Get the current game, or create a new one if none exists
    pub fn get_or_create_game(&mut self) -> Result<GameState, GameError> {
        // First, check if there's a current game ID in the database (shared across processes)
        if let Some(game_id) = self.repository.get_current_game_id()?
            && let Ok(game) = self.repository.load_game(&game_id)
        {
            self.current_game_id = Some(game_id);
            return Ok(game);
        }

        // If we have a local current game ID, try to load it
        if let Some(game_id) = &self.current_game_id
            && let Ok(game) = self.repository.load_game(game_id)
        {
            return Ok(game);
        }

        // Otherwise, create a new game
        self.create_new_game()
    }

    /// Create a new game
    fn create_new_game(&mut self) -> Result<GameState, GameError> {
        let game_id = Uuid::new_v4().to_string();
        let (human_player, ai_player, first_player) = assign_players();

        let game = GameState {
            id: game_id.clone(),
            board: [[Cell::Empty; 3]; 3],
            current_turn: first_player,
            human_player,
            ai_player,
            status: GameStatus::InProgress,
            move_history: vec![],
            taunts: vec![],
            winning_line: None,
        };

        self.repository.save_game(&game)?;
        self.repository.set_current_game_id(&game_id)?; // Register as current game (shared across processes)
        self.current_game_id = Some(game_id);

        Ok(game)
    }

    /// Make a move on the board
    pub fn make_move(
        &mut self,
        row: u8,
        col: u8,
        source: MoveSource,
    ) -> Result<GameState, GameError> {
        let mut game = self.get_or_create_game()?;

        // Check if game is already over
        if game.status != GameStatus::InProgress {
            return Err(GameError::GameOver {
                status: game.status.clone(),
            });
        }

        // Validate bounds
        if row >= 3 || col >= 3 {
            return Err(GameError::OutOfBounds { row, col });
        }

        // Check if cell is empty
        if game.board[row as usize][col as usize] != Cell::Empty {
            return Err(GameError::CellOccupied { row, col });
        }

        // Make the move
        let current_player = game.current_turn;
        game.board[row as usize][col as usize] = Cell::Occupied(current_player);

        // Record the move
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mov = Move {
            player: current_player,
            row,
            col,
            timestamp,
            source: Some(source),
        };

        game.move_history.push(mov.clone());
        self.repository.save_move(&game.id, &mov)?;

        // Update game status
        let mut board = Board::new();
        for m in &game.move_history {
            board.set(m.row, m.col, m.player).ok();
        }
        let (status, winning_line) = get_game_status(&board);
        game.status = status;
        game.winning_line = winning_line;

        // Switch turns if game is still in progress
        if game.status == GameStatus::InProgress {
            game.current_turn = current_player.opponent();
        }

        // Save updated game state
        self.repository.save_game(&game)?;

        Ok(game)
    }

    /// Restart the game with a new board
    pub fn restart_game(&mut self) -> Result<GameState, GameError> {
        self.current_game_id = None;
        self.create_new_game()
    }

    /// Add a taunt message
    pub fn add_taunt(&mut self, message: String, source: MoveSource) -> Result<(), GameError> {
        let game = self.get_or_create_game()?;
        let source_str = match source {
            MoveSource::UI => Some("UI"),
            MoveSource::MCP => Some("MCP"),
        };
        self.repository.save_taunt(&game.id, &message, source_str)?;
        Ok(())
    }

    /// Get the current game state
    pub fn get_game_state(&mut self) -> Result<GameState, GameError> {
        self.get_or_create_game()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::MoveSource;

    fn create_test_manager() -> GameManager {
        let db_path = format!("/tmp/test-game-{}.db", Uuid::new_v4());
        GameManager::new(&db_path).unwrap()
    }

    #[test]
    fn test_create_new_game() {
        let mut manager = create_test_manager();
        let game = manager.get_or_create_game().unwrap();

        assert!(!game.id.is_empty());
        assert_eq!(game.status, GameStatus::InProgress);
        assert_eq!(game.move_history.len(), 0);
        assert_ne!(game.human_player, game.ai_player);
    }

    #[test]
    fn test_get_existing_game() {
        let mut manager = create_test_manager();
        let game1 = manager.get_or_create_game().unwrap();
        let game1_id = game1.id.clone();

        let game2 = manager.get_or_create_game().unwrap();

        assert_eq!(game1_id, game2.id);
    }

    #[test]
    fn test_make_valid_move() {
        let mut manager = create_test_manager();
        let game = manager.make_move(0, 0, MoveSource::UI).unwrap();

        assert_eq!(game.move_history.len(), 1);
        assert_eq!(game.move_history[0].row, 0);
        assert_eq!(game.move_history[0].col, 0);
        assert_eq!(
            game.board[0][0],
            Cell::Occupied(game.move_history[0].player)
        );
    }

    #[test]
    fn test_make_move_out_of_bounds() {
        let mut manager = create_test_manager();
        let result = manager.make_move(3, 0, MoveSource::UI);

        assert!(matches!(result, Err(GameError::OutOfBounds { .. })));
    }

    #[test]
    fn test_make_move_cell_occupied() {
        let mut manager = create_test_manager();
        manager.make_move(1, 1, MoveSource::UI).unwrap();
        let result = manager.make_move(1, 1, MoveSource::UI);

        assert!(matches!(result, Err(GameError::CellOccupied { .. })));
    }

    #[test]
    fn test_turn_switching() {
        let mut manager = create_test_manager();
        let game1 = manager.make_move(0, 0, MoveSource::UI).unwrap();
        let first_player = game1.move_history[0].player;

        let game2 = manager.make_move(0, 1, MoveSource::UI).unwrap();
        assert_eq!(game2.current_turn, first_player);
        assert_eq!(game2.move_history[1].player, first_player.opponent());
    }

    #[test]
    fn test_restart_game() {
        let mut manager = create_test_manager();
        manager.make_move(0, 0, MoveSource::UI).unwrap();
        let game1_id = manager.current_game_id.clone().unwrap();

        let new_game = manager.restart_game().unwrap();

        assert_ne!(new_game.id, game1_id);
        assert_eq!(new_game.move_history.len(), 0);
        assert_eq!(new_game.status, GameStatus::InProgress);
    }

    #[test]
    fn test_add_taunt() {
        let mut manager = create_test_manager();
        manager.get_or_create_game().unwrap();

        let result =
            manager.add_taunt("You call that a move?".to_string(), shared::MoveSource::MCP);
        assert!(result.is_ok());

        // Verify taunt is persisted
        let game = manager.get_game_state().unwrap();
        assert_eq!(game.taunts.len(), 1);
        assert_eq!(game.taunts[0].message, "You call that a move?");
        assert_eq!(game.taunts[0].source, Some(shared::MoveSource::MCP));
    }

    #[test]
    fn test_game_state_persistence() {
        let mut manager = create_test_manager();

        // Make moves and get game state
        manager.make_move(0, 0, MoveSource::UI).unwrap();
        manager.make_move(1, 1, MoveSource::UI).unwrap();
        let game_id = manager.current_game_id.clone().unwrap();

        // Get game state again - should have persistent moves
        let game = manager.get_game_state().unwrap();
        assert_eq!(game.id, game_id);
        assert_eq!(game.move_history.len(), 2);

        // Verify data is in database by loading directly
        let loaded_game = manager.repository.load_game(&game_id).unwrap();
        assert_eq!(loaded_game.move_history.len(), 2);
    }

    #[test]
    fn test_game_over_prevents_moves() {
        let mut manager = create_test_manager();

        // Create a winning condition for X
        // X X X
        // O O .
        // . . .

        // Simulate the moves directly to create a win
        manager.make_move(0, 0, MoveSource::UI).unwrap(); // X
        manager.make_move(1, 0, MoveSource::UI).unwrap(); // O
        manager.make_move(0, 1, MoveSource::UI).unwrap(); // X
        manager.make_move(1, 1, MoveSource::UI).unwrap(); // O
        manager.make_move(0, 2, MoveSource::UI).unwrap(); // X wins

        // Try to make another move
        let result = manager.make_move(2, 0, MoveSource::UI);
        assert!(matches!(result, Err(GameError::GameOver { .. })));
    }
}
