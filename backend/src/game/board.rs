use shared::{Cell, Player};

/// Represents a 3x3 tic-tac-toe board
#[derive(Debug, Clone)]
pub struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    /// Create a new empty board
    pub fn new() -> Self {
        Self {
            cells: [[Cell::default(); 3]; 3],
        }
    }

    /// Get the cell at the given position
    #[allow(dead_code)] // Will be used by game state management
    pub fn get(&self, row: u8, col: u8) -> Option<Cell> {
        if row < 3 && col < 3 {
            Some(self.cells[row as usize][col as usize])
        } else {
            None
        }
    }

    /// Set a cell to the given player
    #[allow(dead_code)] // Will be used by game state management
    pub fn set(&mut self, row: u8, col: u8, player: Player) -> Result<(), String> {
        if row >= 3 || col >= 3 {
            return Err(format!("Position ({}, {}) is out of bounds", row, col));
        }

        if self.cells[row as usize][col as usize] != Cell::Empty {
            return Err(format!("Cell ({}, {}) is already occupied", row, col));
        }

        self.cells[row as usize][col as usize] = Cell::Occupied(player);
        Ok(())
    }

    /// Check if the board is full
    #[allow(dead_code)] // Will be used by game state management
    pub fn is_full(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|cell| *cell != Cell::Empty))
    }

    /// Convert board to 2D array for serialization
    #[allow(dead_code)] // Will be used by API layer
    pub fn to_array(&self) -> [[Cell; 3]; 3] {
        self.cells
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(board.get(row, col), Some(Cell::Empty));
            }
        }
    }

    #[test]
    fn test_set_cell_valid() {
        let mut board = Board::new();
        assert!(board.set(0, 0, Player::X).is_ok());
        assert_eq!(board.get(0, 0), Some(Cell::Occupied(Player::X)));
    }

    #[test]
    fn test_set_cell_out_of_bounds() {
        let mut board = Board::new();
        assert!(board.set(3, 0, Player::X).is_err());
        assert!(board.set(0, 3, Player::O).is_err());
        assert!(board.set(5, 5, Player::X).is_err());
    }

    #[test]
    fn test_set_cell_already_occupied() {
        let mut board = Board::new();
        board.set(1, 1, Player::X).unwrap();
        let result = board.set(1, 1, Player::O);
        assert!(result.is_err());
        assert_eq!(board.get(1, 1), Some(Cell::Occupied(Player::X)));
    }

    #[test]
    fn test_is_full_empty_board() {
        let board = Board::new();
        assert!(!board.is_full());
    }

    #[test]
    fn test_is_full_partial_board() {
        let mut board = Board::new();
        board.set(0, 0, Player::X).unwrap();
        board.set(1, 1, Player::O).unwrap();
        assert!(!board.is_full());
    }

    #[test]
    fn test_is_full_complete_board() {
        let mut board = Board::new();
        // Fill the board alternating X and O
        for row in 0..3 {
            for col in 0..3 {
                let player = if (row + col) % 2 == 0 {
                    Player::X
                } else {
                    Player::O
                };
                board.set(row, col, player).unwrap();
            }
        }
        assert!(board.is_full());
    }

    #[test]
    fn test_get_out_of_bounds() {
        let board = Board::new();
        assert_eq!(board.get(3, 0), None);
        assert_eq!(board.get(0, 3), None);
        assert_eq!(board.get(5, 5), None);
    }
}
