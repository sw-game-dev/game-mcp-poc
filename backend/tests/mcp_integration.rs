use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use uuid::Uuid;

/// Mock AI client for MCP integration testing
struct MockAiClient {
    process: Child,
}

impl MockAiClient {
    /// Start the MCP server with a test database
    fn start() -> Self {
        let db_path = format!("/tmp/test-mcp-{}.db", Uuid::new_v4());

        let process = Command::new("cargo")
            .args(["run", "--bin", "game-mcp-server"])
            .env("GAME_DB_PATH", db_path)
            .env("RUST_LOG", "error") // Suppress logs during tests
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start MCP server");

        // Give server time to initialize
        std::thread::sleep(std::time::Duration::from_millis(100));

        Self { process }
    }

    /// Send a JSON-RPC request and get the response
    fn send_request(&mut self, method: &str, params: Value) -> Value {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        });

        let stdin = self.process.stdin.as_mut().expect("Failed to get stdin");
        let stdout = self.process.stdout.as_mut().expect("Failed to get stdout");

        // Send request
        writeln!(stdin, "{}", request).expect("Failed to write request");
        stdin.flush().expect("Failed to flush stdin");

        // Read response
        let mut reader = BufReader::new(stdout);
        let mut response_line = String::new();
        reader
            .read_line(&mut response_line)
            .expect("Failed to read response");

        serde_json::from_str(&response_line).expect("Failed to parse response")
    }

    /// View the current game state
    fn view_game_state(&mut self) -> Value {
        self.send_request("view_game_state", json!({}))
    }

    /// Get whose turn it is
    fn get_turn(&mut self) -> Value {
        self.send_request("get_turn", json!({}))
    }

    /// Make a move
    fn make_move(&mut self, row: u8, col: u8) -> Value {
        self.send_request("make_move", json!({"row": row, "col": col}))
    }

    /// Send a taunt
    fn taunt_player(&mut self, message: &str) -> Value {
        self.send_request("taunt_player", json!({"message": message}))
    }

    /// Restart the game
    fn restart_game(&mut self) -> Value {
        self.send_request("restart_game", json!({}))
    }

    /// Get game history
    fn get_game_history(&mut self) -> Value {
        self.send_request("get_game_history", json!({}))
    }
}

impl Drop for MockAiClient {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

#[test]
fn test_mcp_view_game_state() {
    let mut client = MockAiClient::start();
    let response = client.view_game_state();

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"].is_object());
    assert!(response["result"]["id"].is_string());
    assert!(response["result"]["board"].is_array());
    assert_eq!(response["result"]["status"], "InProgress");
}

#[test]
fn test_mcp_get_turn() {
    let mut client = MockAiClient::start();
    let response = client.get_turn();

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["currentTurn"].is_string());
    assert!(response["result"]["isHumanTurn"].is_boolean());
    assert!(response["result"]["isAiTurn"].is_boolean());
}

#[test]
fn test_mcp_make_move() {
    let mut client = MockAiClient::start();
    let response = client.make_move(1, 1);

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["result"]["success"], true);
    assert!(response["result"]["gameState"].is_object());
    assert_eq!(response["result"]["message"], "Move made successfully");
}

#[test]
fn test_mcp_make_move_invalid_position() {
    let mut client = MockAiClient::start();
    let response = client.make_move(5, 5);

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32602); // INVALID_PARAMS
    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("out of bounds"));
}

#[test]
fn test_mcp_make_move_occupied_cell() {
    let mut client = MockAiClient::start();

    // Make first move
    client.make_move(1, 1);

    // Try to make move in same position
    let response = client.make_move(1, 1);

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32602); // INVALID_PARAMS
    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("occupied"));
}

#[test]
fn test_mcp_taunt_player() {
    let mut client = MockAiClient::start();
    let response = client.taunt_player("You're going down!");

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["result"]["success"], true);
    assert_eq!(response["result"]["message"], "Taunt sent successfully");
}

#[test]
fn test_mcp_restart_game() {
    let mut client = MockAiClient::start();

    // Make a move
    client.make_move(0, 0);

    // Restart game
    let response = client.restart_game();

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["result"]["success"], true);
    assert_eq!(response["result"]["gameState"]["status"], "InProgress");
}

#[test]
fn test_mcp_get_game_history() {
    let mut client = MockAiClient::start();

    // Make some moves
    client.make_move(0, 0);
    client.make_move(1, 1);

    let response = client.get_game_history();

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["moves"].is_array());
    assert_eq!(response["result"]["moves"].as_array().unwrap().len(), 2);
}

#[test]
fn test_mcp_complete_game_playthrough() {
    let mut client = MockAiClient::start();

    // View initial state
    let state = client.view_game_state();
    assert_eq!(state["result"]["status"], "InProgress");

    // Get first player (just to verify the API works)
    let turn = client.get_turn();
    assert!(turn["result"]["currentTurn"].is_string());

    // Play a game (X wins with top row)
    // X moves
    let move1 = client.make_move(0, 0);
    assert_eq!(move1["result"]["success"], true);

    // O moves
    let move2 = client.make_move(1, 0);
    assert_eq!(move2["result"]["success"], true);

    // X moves
    let move3 = client.make_move(0, 1);
    assert_eq!(move3["result"]["success"], true);

    // O moves
    let move4 = client.make_move(1, 1);
    assert_eq!(move4["result"]["success"], true);

    // X wins
    let move5 = client.make_move(0, 2);
    assert_eq!(move5["result"]["success"], true);
    let status = move5["result"]["gameState"]["status"].as_str().unwrap();
    assert!(status.starts_with("Won_")); // Either Won_X or Won_O depending on randomization

    // Try to make another move (should fail)
    let move6 = client.make_move(2, 0);
    assert!(move6["error"].is_object());
    assert!(move6["error"]["message"]
        .as_str()
        .unwrap()
        .contains("already over"));

    // Check history
    let history = client.get_game_history();
    assert_eq!(history["result"]["moves"].as_array().unwrap().len(), 5);

    // Restart and verify clean state
    let restart = client.restart_game();
    assert_eq!(restart["result"]["success"], true);

    let new_history = client.get_game_history();
    assert_eq!(new_history["result"]["moves"].as_array().unwrap().len(), 0);
}

#[test]
fn test_mcp_taunt_persistence() {
    let mut client = MockAiClient::start();

    // Send taunts
    client.taunt_player("First taunt");
    client.taunt_player("Second taunt");

    // View game state
    let state = client.view_game_state();
    let taunts = state["result"]["taunts"].as_array().unwrap();

    assert_eq!(taunts.len(), 2);
    assert_eq!(taunts[0], "First taunt");
    assert_eq!(taunts[1], "Second taunt");
}

#[test]
fn test_mcp_invalid_method() {
    let mut client = MockAiClient::start();
    let response = client.send_request("invalid_method", json!({}));

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32601); // METHOD_NOT_FOUND
}

#[test]
fn test_mcp_invalid_params() {
    let mut client = MockAiClient::start();
    let response = client.send_request("make_move", json!({})); // Missing row/col

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32602); // INVALID_PARAMS
}
