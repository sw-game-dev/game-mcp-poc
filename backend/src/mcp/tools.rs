#![allow(dead_code)] // Will be used by MCP server

use super::protocol::JsonRpcError;
use crate::game::manager::GameManager;
use serde_json::{Value, json};
use shared::{GameError, MoveSource};

/// Handle the view_game_state tool call
pub fn view_game_state(manager: &mut GameManager, _params: Value) -> Result<Value, JsonRpcError> {
    let game = manager
        .get_game_state()
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to get game state: {}", e)))?;

    Ok(json!({
        "id": game.id,
        "board": game.board,
        "currentTurn": match game.current_turn {
            shared::Player::X => "X",
            shared::Player::O => "O",
        },
        "humanPlayer": match game.human_player {
            shared::Player::X => "X",
            shared::Player::O => "O",
        },
        "aiPlayer": match game.ai_player {
            shared::Player::X => "X",
            shared::Player::O => "O",
        },
        "status": match &game.status {
            shared::GameStatus::InProgress => "InProgress",
            shared::GameStatus::Won(p) => match p {
                shared::Player::X => "Won_X",
                shared::Player::O => "Won_O",
            },
            shared::GameStatus::Draw => "Draw",
        },
        "moveHistory": game.move_history,
        "taunts": game.taunts,
    }))
}

/// Handle the get_turn tool call
pub fn get_turn(manager: &mut GameManager, _params: Value) -> Result<Value, JsonRpcError> {
    let game = manager
        .get_game_state()
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to get game state: {}", e)))?;

    let current_turn_str = match game.current_turn {
        shared::Player::X => "X",
        shared::Player::O => "O",
    };

    Ok(json!({
        "currentTurn": current_turn_str,
        "isHumanTurn": game.current_turn == game.human_player,
        "isAiTurn": game.current_turn == game.ai_player,
    }))
}

/// Handle the make_move tool call
pub fn make_move(manager: &mut GameManager, params: Value) -> Result<Value, JsonRpcError> {
    let row = params["row"].as_u64().ok_or_else(|| {
        JsonRpcError::invalid_params("Missing or invalid 'row' parameter".to_string())
    })? as u8;

    let col = params["col"].as_u64().ok_or_else(|| {
        JsonRpcError::invalid_params("Missing or invalid 'col' parameter".to_string())
    })? as u8;

    let game = manager
        .make_move(row, col, MoveSource::MCP)
        .map_err(|e| match e {
            GameError::OutOfBounds { .. } => {
                JsonRpcError::invalid_params(format!("Move out of bounds: {}", e))
            }
            GameError::CellOccupied { .. } => {
                JsonRpcError::invalid_params(format!("Cell already occupied: {}", e))
            }
            GameError::GameOver { .. } => {
                JsonRpcError::invalid_params(format!("Game is already over: {}", e))
            }
            _ => JsonRpcError::internal_error(format!("Failed to make move: {}", e)),
        })?;

    Ok(json!({
        "success": true,
        "gameState": {
            "id": game.id,
            "board": game.board,
            "currentTurn": match game.current_turn {
                shared::Player::X => "X",
                shared::Player::O => "O",
            },
            "status": match &game.status {
                shared::GameStatus::InProgress => "InProgress",
                shared::GameStatus::Won(p) => match p {
                    shared::Player::X => "Won_X",
                    shared::Player::O => "Won_O",
                },
                shared::GameStatus::Draw => "Draw",
            },
        },
        "message": "Move made successfully"
    }))
}

/// Handle the taunt_player tool call
pub fn taunt_player(manager: &mut GameManager, params: Value) -> Result<Value, JsonRpcError> {
    let message = params["message"]
        .as_str()
        .ok_or_else(|| {
            JsonRpcError::invalid_params("Missing or invalid 'message' parameter".to_string())
        })?
        .to_string();

    manager
        .add_taunt(message, shared::MoveSource::MCP)
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to add taunt: {}", e)))?;

    Ok(json!({
        "success": true,
        "message": "Taunt sent successfully"
    }))
}

/// Handle the restart_game tool call
pub fn restart_game(manager: &mut GameManager, _params: Value) -> Result<Value, JsonRpcError> {
    let game = manager
        .restart_game()
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to restart game: {}", e)))?;

    Ok(json!({
        "success": true,
        "gameState": {
            "id": game.id,
            "board": game.board,
            "currentTurn": match game.current_turn {
                shared::Player::X => "X",
                shared::Player::O => "O",
            },
            "humanPlayer": match game.human_player {
                shared::Player::X => "X",
                shared::Player::O => "O",
            },
            "aiPlayer": match game.ai_player {
                shared::Player::X => "X",
                shared::Player::O => "O",
            },
            "status": "InProgress",
        },
        "message": "Game restarted"
    }))
}

/// Handle the get_game_history tool call
pub fn get_game_history(manager: &mut GameManager, _params: Value) -> Result<Value, JsonRpcError> {
    let game = manager
        .get_game_state()
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to get game state: {}", e)))?;

    Ok(json!({
        "moves": game.move_history
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::manager::GameManager;
    use serde_json::json;
    use uuid::Uuid;

    fn create_test_manager() -> GameManager {
        let db_path = format!("/tmp/test-tools-{}.db", Uuid::new_v4());
        GameManager::new(&db_path).unwrap()
    }

    // view_game_state tests
    #[test]
    fn test_view_game_state_success() {
        let mut manager = create_test_manager();
        let result = view_game_state(&mut manager, json!({}));

        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value["id"].is_string());
        assert!(value["board"].is_array());
        assert!(value["currentTurn"].is_string());
        assert_eq!(value["status"], "InProgress");
    }

    #[test]
    fn test_view_game_state_includes_all_fields() {
        let mut manager = create_test_manager();
        let result = view_game_state(&mut manager, json!({})).unwrap();

        assert!(result.get("id").is_some());
        assert!(result.get("board").is_some());
        assert!(result.get("currentTurn").is_some());
        assert!(result.get("humanPlayer").is_some());
        assert!(result.get("aiPlayer").is_some());
        assert!(result.get("status").is_some());
        assert!(result.get("moveHistory").is_some());
        assert!(result.get("taunts").is_some());
    }

    // get_turn tests
    #[test]
    fn test_get_turn_success() {
        let mut manager = create_test_manager();
        let result = get_turn(&mut manager, json!({}));

        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value["currentTurn"].is_string());
        assert!(value["isHumanTurn"].is_boolean());
        assert!(value["isAiTurn"].is_boolean());
    }

    #[test]
    fn test_get_turn_alternates() {
        let mut manager = create_test_manager();
        let game = manager.get_or_create_game().unwrap();
        let first_player = game.current_turn;
        let human_player = game.human_player;

        let turn1 = get_turn(&mut manager, json!({})).unwrap();
        assert_eq!(
            turn1["isHumanTurn"].as_bool().unwrap(),
            first_player == human_player
        );
    }

    // make_move tests
    #[test]
    fn test_make_move_success() {
        let mut manager = create_test_manager();
        let result = make_move(&mut manager, json!({"row": 0, "col": 0}));

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["success"], true);
        assert!(value.get("gameState").is_some());
        assert_eq!(value["message"], "Move made successfully");
    }

    #[test]
    fn test_make_move_missing_params() {
        let mut manager = create_test_manager();
        let result = make_move(&mut manager, json!({}));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, super::super::protocol::INVALID_PARAMS);
    }

    #[test]
    fn test_make_move_out_of_bounds() {
        let mut manager = create_test_manager();
        let result = make_move(&mut manager, json!({"row": 5, "col": 0}));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, super::super::protocol::INVALID_PARAMS);
        assert!(err.message.contains("out of bounds"));
    }

    #[test]
    fn test_make_move_cell_occupied() {
        let mut manager = create_test_manager();
        make_move(&mut manager, json!({"row": 1, "col": 1})).unwrap();
        let result = make_move(&mut manager, json!({"row": 1, "col": 1}));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, super::super::protocol::INVALID_PARAMS);
        assert!(err.message.contains("occupied"));
    }

    // taunt_player tests
    #[test]
    fn test_taunt_player_success() {
        let mut manager = create_test_manager();
        let result = taunt_player(&mut manager, json!({"message": "Nice try!"}));

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["success"], true);
        assert_eq!(value["message"], "Taunt sent successfully");
    }

    #[test]
    fn test_taunt_player_missing_message() {
        let mut manager = create_test_manager();
        let result = taunt_player(&mut manager, json!({}));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, super::super::protocol::INVALID_PARAMS);
    }

    #[test]
    fn test_taunt_player_persists() {
        let mut manager = create_test_manager();
        taunt_player(&mut manager, json!({"message": "You're going down!"})).unwrap();

        let game_state = view_game_state(&mut manager, json!({})).unwrap();
        let taunts = game_state["taunts"].as_array().unwrap();
        assert_eq!(taunts.len(), 1);
    }

    // restart_game tests
    #[test]
    fn test_restart_game_success() {
        let mut manager = create_test_manager();
        make_move(&mut manager, json!({"row": 0, "col": 0})).unwrap();

        let result = restart_game(&mut manager, json!({}));

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["success"], true);
        assert!(value.get("gameState").is_some());
        assert_eq!(value["gameState"]["status"], "InProgress");
    }

    #[test]
    fn test_restart_game_clears_board() {
        let mut manager = create_test_manager();
        make_move(&mut manager, json!({"row": 0, "col": 0})).unwrap();
        make_move(&mut manager, json!({"row": 1, "col": 1})).unwrap();

        restart_game(&mut manager, json!({})).unwrap();

        let game_state = view_game_state(&mut manager, json!({})).unwrap();
        let moves = game_state["moveHistory"].as_array().unwrap();
        assert_eq!(moves.len(), 0);
    }

    // get_game_history tests
    #[test]
    fn test_get_game_history_empty() {
        let mut manager = create_test_manager();
        let result = get_game_history(&mut manager, json!({}));

        assert!(result.is_ok());
        let value = result.unwrap();
        let moves = value["moves"].as_array().unwrap();
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_get_game_history_with_moves() {
        let mut manager = create_test_manager();
        make_move(&mut manager, json!({"row": 0, "col": 0})).unwrap();
        make_move(&mut manager, json!({"row": 1, "col": 1})).unwrap();

        let result = get_game_history(&mut manager, json!({}));

        assert!(result.is_ok());
        let value = result.unwrap();
        let moves = value["moves"].as_array().unwrap();
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_get_game_history_order() {
        let mut manager = create_test_manager();
        make_move(&mut manager, json!({"row": 0, "col": 0})).unwrap();
        make_move(&mut manager, json!({"row": 0, "col": 1})).unwrap();
        make_move(&mut manager, json!({"row": 0, "col": 2})).unwrap();

        let result = get_game_history(&mut manager, json!({})).unwrap();
        let moves = result["moves"].as_array().unwrap();

        assert_eq!(moves[0]["row"], 0);
        assert_eq!(moves[0]["col"], 0);
        assert_eq!(moves[1]["row"], 0);
        assert_eq!(moves[1]["col"], 1);
        assert_eq!(moves[2]["row"], 0);
        assert_eq!(moves[2]["col"], 2);
    }
}
