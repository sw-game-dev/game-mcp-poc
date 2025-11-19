use super::board::Board;
use shared::{Cell, GameStatus, Player, WinningLine};

/// Check if there's a winner on the board and return winner with winning line
#[allow(dead_code)] // Will be used by game state management
pub fn check_winner(board: &Board) -> Option<(Player, WinningLine)> {
    // Check rows
    for row in 0..3 {
        let positions = [(row, 0), (row, 1), (row, 2)];
        if let Some(winner) = check_line(board, positions) {
            return Some((
                winner,
                WinningLine {
                    positions: positions.to_vec(),
                },
            ));
        }
    }

    // Check columns
    for col in 0..3 {
        let positions = [(0, col), (1, col), (2, col)];
        if let Some(winner) = check_line(board, positions) {
            return Some((
                winner,
                WinningLine {
                    positions: positions.to_vec(),
                },
            ));
        }
    }

    // Check diagonals
    let positions = [(0, 0), (1, 1), (2, 2)];
    if let Some(winner) = check_line(board, positions) {
        return Some((
            winner,
            WinningLine {
                positions: positions.to_vec(),
            },
        ));
    }

    let positions = [(0, 2), (1, 1), (2, 0)];
    if let Some(winner) = check_line(board, positions) {
        return Some((
            winner,
            WinningLine {
                positions: positions.to_vec(),
            },
        ));
    }

    None
}

/// Check if three cells contain the same player
fn check_line(board: &Board, positions: [(u8, u8); 3]) -> Option<Player> {
    let cells: Vec<Cell> = positions
        .iter()
        .filter_map(|(row, col)| board.get(*row, *col))
        .collect();

    if cells.len() != 3 {
        return None;
    }

    // Check if all three cells are occupied by the same player
    match (cells[0], cells[1], cells[2]) {
        (Cell::Occupied(p1), Cell::Occupied(p2), Cell::Occupied(p3)) if p1 == p2 && p2 == p3 => {
            Some(p1)
        }
        _ => None,
    }
}

/// Determine the current game status and return optional winning line
#[allow(dead_code)] // Will be used by game state management
pub fn get_game_status(board: &Board) -> (GameStatus, Option<WinningLine>) {
    if let Some((winner, line)) = check_winner(board) {
        (GameStatus::Won(winner), Some(line))
    } else if board.is_full() {
        (GameStatus::Draw, None)
    } else {
        (GameStatus::InProgress, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_winner_empty_board() {
        let board = Board::new();
        assert_eq!(check_winner(&board), None);
    }

    #[test]
    fn test_winner_top_row() {
        let mut board = Board::new();
        board.set(0, 0, Player::X).unwrap();
        board.set(0, 1, Player::X).unwrap();
        board.set(0, 2, Player::X).unwrap();
        let (winner, line) = check_winner(&board).unwrap();
        assert_eq!(winner, Player::X);
        assert_eq!(line.positions, vec![(0, 0), (0, 1), (0, 2)]);
    }

    #[test]
    fn test_winner_middle_row() {
        let mut board = Board::new();
        board.set(1, 0, Player::O).unwrap();
        board.set(1, 1, Player::O).unwrap();
        board.set(1, 2, Player::O).unwrap();
        let (winner, _) = check_winner(&board).unwrap();
        assert_eq!(winner, Player::O);
    }

    #[test]
    fn test_winner_bottom_row() {
        let mut board = Board::new();
        board.set(2, 0, Player::X).unwrap();
        board.set(2, 1, Player::X).unwrap();
        board.set(2, 2, Player::X).unwrap();
        let (winner, _) = check_winner(&board).unwrap();
        assert_eq!(winner, Player::X);
    }

    #[test]
    fn test_winner_left_column() {
        let mut board = Board::new();
        board.set(0, 0, Player::O).unwrap();
        board.set(1, 0, Player::O).unwrap();
        board.set(2, 0, Player::O).unwrap();
        let (winner, _) = check_winner(&board).unwrap();
        assert_eq!(winner, Player::O);
    }

    #[test]
    fn test_winner_middle_column() {
        let mut board = Board::new();
        board.set(0, 1, Player::X).unwrap();
        board.set(1, 1, Player::X).unwrap();
        board.set(2, 1, Player::X).unwrap();
        let (winner, _) = check_winner(&board).unwrap();
        assert_eq!(winner, Player::X);
    }

    #[test]
    fn test_winner_right_column() {
        let mut board = Board::new();
        board.set(0, 2, Player::O).unwrap();
        board.set(1, 2, Player::O).unwrap();
        board.set(2, 2, Player::O).unwrap();
        let (winner, _) = check_winner(&board).unwrap();
        assert_eq!(winner, Player::O);
    }

    #[test]
    fn test_winner_diagonal_top_left_to_bottom_right() {
        let mut board = Board::new();
        board.set(0, 0, Player::X).unwrap();
        board.set(1, 1, Player::X).unwrap();
        board.set(2, 2, Player::X).unwrap();
        let (winner, _) = check_winner(&board).unwrap();
        assert_eq!(winner, Player::X);
    }

    #[test]
    fn test_winner_diagonal_top_right_to_bottom_left() {
        let mut board = Board::new();
        board.set(0, 2, Player::O).unwrap();
        board.set(1, 1, Player::O).unwrap();
        board.set(2, 0, Player::O).unwrap();
        let (winner, _) = check_winner(&board).unwrap();
        assert_eq!(winner, Player::O);
    }

    #[test]
    fn test_no_winner_mixed_row() {
        let mut board = Board::new();
        board.set(0, 0, Player::X).unwrap();
        board.set(0, 1, Player::O).unwrap();
        board.set(0, 2, Player::X).unwrap();
        assert_eq!(check_winner(&board), None);
    }

    #[test]
    fn test_game_status_in_progress() {
        let mut board = Board::new();
        board.set(0, 0, Player::X).unwrap();
        let (status, line) = get_game_status(&board);
        assert_eq!(status, GameStatus::InProgress);
        assert_eq!(line, None);
    }

    #[test]
    fn test_game_status_won() {
        let mut board = Board::new();
        board.set(0, 0, Player::X).unwrap();
        board.set(0, 1, Player::X).unwrap();
        board.set(0, 2, Player::X).unwrap();
        let (status, line) = get_game_status(&board);
        assert_eq!(status, GameStatus::Won(Player::X));
        assert!(line.is_some());
    }

    #[test]
    fn test_game_status_draw() {
        let mut board = Board::new();
        // Create a draw scenario
        // X O X
        // X O O
        // O X X
        board.set(0, 0, Player::X).unwrap();
        board.set(0, 1, Player::O).unwrap();
        board.set(0, 2, Player::X).unwrap();
        board.set(1, 0, Player::X).unwrap();
        board.set(1, 1, Player::O).unwrap();
        board.set(1, 2, Player::O).unwrap();
        board.set(2, 0, Player::O).unwrap();
        board.set(2, 1, Player::X).unwrap();
        board.set(2, 2, Player::X).unwrap();

        let (status, line) = get_game_status(&board);
        assert_eq!(status, GameStatus::Draw);
        assert_eq!(line, None);
    }
}
