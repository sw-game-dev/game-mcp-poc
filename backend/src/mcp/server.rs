use super::protocol::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, METHOD_NOT_FOUND};
use super::tools;
use crate::game::manager::GameManager;
use serde_json::Value;
use std::io::{self, BufRead, Write};

/// MCP server that handles JSON-RPC 2.0 requests via stdio
#[allow(dead_code)] // Will be used by binary entry point
pub struct McpServer {
    manager: GameManager,
}

#[allow(dead_code)] // Will be used by binary entry point
impl McpServer {
    /// Create a new MCP server with the given database path
    pub fn new(db_path: &str) -> Result<Self, shared::GameError> {
        let manager = GameManager::new(db_path)?;
        Ok(Self { manager })
    }

    /// Run the server loop, reading from stdin and writing to stdout
    pub fn run(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            let response = self.handle_request(&line);
            writeln!(stdout, "{}", response)?;
            stdout.flush()?;
        }

        Ok(())
    }

    /// Handle a single JSON-RPC request
    fn handle_request(&mut self, json: &str) -> String {
        // Parse the request
        let request = match JsonRpcRequest::from_json(json) {
            Ok(req) => req,
            Err(e) => {
                let response = JsonRpcResponse::error(Value::Null, e);
                return response.to_json();
            }
        };

        // Validate the request
        if let Err(e) = request.validate() {
            let response = JsonRpcResponse::error(request.id.clone(), e);
            return response.to_json();
        }

        // Dispatch to the appropriate tool
        let result = self.dispatch(&request.method, request.params.clone());

        // Create response
        let response = match result {
            Ok(value) => JsonRpcResponse::success(request.id, value),
            Err(error) => JsonRpcResponse::error(request.id, error),
        };

        response.to_json()
    }

    /// Dispatch a method call to the appropriate tool handler
    fn dispatch(&mut self, method: &str, params: Value) -> Result<Value, JsonRpcError> {
        match method {
            "view_game_state" => tools::view_game_state(&mut self.manager, params),
            "get_turn" => tools::get_turn(&mut self.manager, params),
            "make_move" => tools::make_move(&mut self.manager, params),
            "taunt_player" => tools::taunt_player(&mut self.manager, params),
            "restart_game" => tools::restart_game(&mut self.manager, params),
            "get_game_history" => tools::get_game_history(&mut self.manager, params),
            _ => Err(JsonRpcError {
                code: METHOD_NOT_FOUND,
                message: format!("Method '{}' not found", method),
                data: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    fn create_test_server() -> McpServer {
        let db_path = format!("/tmp/test-server-{}.db", Uuid::new_v4());
        McpServer::new(&db_path).unwrap()
    }

    #[test]
    fn test_server_initialization() {
        let db_path = format!("/tmp/test-init-{}.db", Uuid::new_v4());
        let result = McpServer::new(&db_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_valid_request() {
        let mut server = create_test_server();
        let request = r#"{"jsonrpc":"2.0","id":1,"method":"view_game_state","params":{}}"#;

        let response = server.handle_request(request);

        assert!(response.contains(r#""jsonrpc":"2.0""#));
        assert!(response.contains(r#""id":1"#));
        assert!(response.contains(r#""result""#));
    }

    #[test]
    fn test_handle_invalid_json() {
        let mut server = create_test_server();
        let request = r#"{"invalid json"#;

        let response = server.handle_request(request);

        assert!(response.contains(r#""error""#));
        assert!(response.contains(r#""code":-32700"#)); // PARSE_ERROR
    }

    #[test]
    fn test_handle_unknown_method() {
        let mut server = create_test_server();
        let request = r#"{"jsonrpc":"2.0","id":1,"method":"unknown_method","params":{}}"#;

        let response = server.handle_request(request);

        assert!(response.contains(r#""error""#));
        assert!(response.contains(r#""code":-32601"#)); // METHOD_NOT_FOUND
    }

    #[test]
    fn test_dispatch_view_game_state() {
        let mut server = create_test_server();
        let result = server.dispatch("view_game_state", json!({}));

        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.get("id").is_some());
        assert!(value.get("board").is_some());
    }

    #[test]
    fn test_dispatch_make_move() {
        let mut server = create_test_server();
        let result = server.dispatch("make_move", json!({"row": 0, "col": 0}));

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["success"], true);
    }

    #[test]
    fn test_dispatch_get_turn() {
        let mut server = create_test_server();
        let result = server.dispatch("get_turn", json!({}));

        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.get("currentTurn").is_some());
    }

    #[test]
    fn test_dispatch_taunt_player() {
        let mut server = create_test_server();
        let result = server.dispatch("taunt_player", json!({"message": "Good luck!"}));

        assert!(result.is_ok());
    }

    #[test]
    fn test_dispatch_restart_game() {
        let mut server = create_test_server();
        // Make a move first
        server
            .dispatch("make_move", json!({"row": 0, "col": 0}))
            .unwrap();

        let result = server.dispatch("restart_game", json!({}));

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["success"], true);
    }

    #[test]
    fn test_dispatch_get_game_history() {
        let mut server = create_test_server();
        server
            .dispatch("make_move", json!({"row": 0, "col": 0}))
            .unwrap();

        let result = server.dispatch("get_game_history", json!({}));

        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.get("moves").is_some());
    }

    #[test]
    fn test_multiple_requests() {
        let mut server = create_test_server();

        // First request
        let req1 = r#"{"jsonrpc":"2.0","id":1,"method":"view_game_state","params":{}}"#;
        let resp1 = server.handle_request(req1);
        assert!(resp1.contains(r#""id":1"#));

        // Second request
        let req2 = r#"{"jsonrpc":"2.0","id":2,"method":"make_move","params":{"row":0,"col":0}}"#;
        let resp2 = server.handle_request(req2);
        assert!(resp2.contains(r#""id":2"#));

        // Third request
        let req3 = r#"{"jsonrpc":"2.0","id":3,"method":"get_turn","params":{}}"#;
        let resp3 = server.handle_request(req3);
        assert!(resp3.contains(r#""id":3"#));
    }
}
