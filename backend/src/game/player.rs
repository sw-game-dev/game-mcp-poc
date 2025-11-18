use rand::Rng;
use shared::Player;

/// Randomly assign players and determine who goes first
#[allow(dead_code)] // Will be used by game initialization
pub fn assign_players() -> (Player, Player, Player) {
    let mut rng = rand::thread_rng();
    let human_is_x = rng.gen_bool(0.5);

    let (human_player, ai_player) = if human_is_x {
        (Player::X, Player::O)
    } else {
        (Player::O, Player::X)
    };

    // Flip coin to see who goes first
    let first_player = if rng.gen_bool(0.5) {
        human_player
    } else {
        ai_player
    };

    (human_player, ai_player, first_player)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_players_returns_valid_players() {
        let (human, ai, first) = assign_players();

        // Players must be opposite
        assert_ne!(human, ai);
        assert_eq!(human.opponent(), ai);

        // First player must be one of the two players
        assert!(first == human || first == ai);
    }

    #[test]
    fn test_assign_players_randomness() {
        // Run multiple times to check randomness (statistical test)
        let mut human_x_count = 0;
        let mut ai_x_count = 0;
        let iterations = 100;

        for _ in 0..iterations {
            let (human, _ai, _) = assign_players();
            if human == Player::X {
                human_x_count += 1;
            } else {
                ai_x_count += 1;
            }
        }

        // Both should have been assigned X at least once in 100 iterations
        // (This test has a very small chance of failing due to randomness)
        assert!(human_x_count > 0);
        assert!(ai_x_count > 0);
    }

    #[test]
    fn test_assign_players_first_turn_randomness() {
        // Run multiple times to check first turn randomness
        let mut human_first_count = 0;
        let mut ai_first_count = 0;
        let iterations = 100;

        for _ in 0..iterations {
            let (human, ai, first) = assign_players();
            if first == human {
                human_first_count += 1;
            } else if first == ai {
                ai_first_count += 1;
            }
        }

        // Both should have gone first at least once in 100 iterations
        assert!(human_first_count > 0);
        assert!(ai_first_count > 0);
    }
}
