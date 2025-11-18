#![allow(dead_code)] // Will be used by MCP server

use serde::{Deserialize, Serialize};
use serde_json::Value;

// JSON-RPC 2.0 error codes
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Value,
    pub method: String,
    pub params: Value,
}

impl JsonRpcRequest {
    /// Create a new JSON-RPC request
    pub fn new(id: Value, method: String, params: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method,
            params,
        }
    }

    /// Parse from JSON string
    pub fn from_json(json: &str) -> Result<Self, JsonRpcError> {
        serde_json::from_str(json).map_err(|e| JsonRpcError {
            code: PARSE_ERROR,
            message: format!("Parse error: {}", e),
            data: None,
        })
    }

    /// Validate the request
    pub fn validate(&self) -> Result<(), JsonRpcError> {
        if self.jsonrpc != "2.0" {
            return Err(JsonRpcError {
                code: INVALID_REQUEST,
                message: "Invalid jsonrpc version".to_string(),
                data: None,
            });
        }

        if self.method.is_empty() {
            return Err(JsonRpcError {
                code: INVALID_REQUEST,
                message: "Method cannot be empty".to_string(),
                data: None,
            });
        }

        Ok(())
    }
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    /// Create a success response
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(id: Value, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcError {
    /// Create a parse error
    pub fn parse_error(message: String) -> Self {
        Self {
            code: PARSE_ERROR,
            message,
            data: None,
        }
    }

    /// Create an invalid request error
    pub fn invalid_request(message: String) -> Self {
        Self {
            code: INVALID_REQUEST,
            message,
            data: None,
        }
    }

    /// Create a method not found error
    pub fn method_not_found(method: String) -> Self {
        Self {
            code: METHOD_NOT_FOUND,
            message: format!("Method '{}' not found", method),
            data: None,
        }
    }

    /// Create an invalid params error
    pub fn invalid_params(message: String) -> Self {
        Self {
            code: INVALID_PARAMS,
            message,
            data: None,
        }
    }

    /// Create an internal error
    pub fn internal_error(message: String) -> Self {
        Self {
            code: INTERNAL_ERROR,
            message,
            data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_valid_request() {
        let json = r#"{"jsonrpc":"2.0","id":1,"method":"test_method","params":{}}"#;
        let request = JsonRpcRequest::from_json(json).unwrap();

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, json!(1));
        assert_eq!(request.method, "test_method");
        assert_eq!(request.params, json!({}));
    }

    #[test]
    fn test_parse_invalid_json() {
        let json = r#"{"jsonrpc":"2.0","id":1,invalid}"#;
        let result = JsonRpcRequest::from_json(json);

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code, PARSE_ERROR);
    }

    #[test]
    fn test_validate_request() {
        let request = JsonRpcRequest::new(json!(1), "test_method".to_string(), json!({}));

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_version() {
        let mut request = JsonRpcRequest::new(json!(1), "test_method".to_string(), json!({}));
        request.jsonrpc = "1.0".to_string();

        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, INVALID_REQUEST);
    }

    #[test]
    fn test_validate_empty_method() {
        let request = JsonRpcRequest::new(json!(1), "".to_string(), json!({}));

        let result = request.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, INVALID_REQUEST);
    }

    #[test]
    fn test_success_response() {
        let response = JsonRpcResponse::success(json!(1), json!({"result": "ok"}));

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(1));
        assert_eq!(response.result, Some(json!({"result": "ok"})));
        assert_eq!(response.error, None);
    }

    #[test]
    fn test_error_response() {
        let error = JsonRpcError::internal_error("Something went wrong".to_string());
        let response = JsonRpcResponse::error(json!(1), error);

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(1));
        assert_eq!(response.result, None);
        assert!(response.error.is_some());
        assert_eq!(response.error.as_ref().unwrap().code, INTERNAL_ERROR);
    }

    #[test]
    fn test_serialize_response() {
        let response = JsonRpcResponse::success(json!(1), json!({"test": "value"}));
        let json = response.to_json();

        assert!(json.contains(r#""jsonrpc":"2.0""#));
        assert!(json.contains(r#""id":1"#));
        assert!(json.contains(r#""result""#));
    }

    #[test]
    fn test_error_constructors() {
        let parse = JsonRpcError::parse_error("test".to_string());
        assert_eq!(parse.code, PARSE_ERROR);

        let invalid = JsonRpcError::invalid_request("test".to_string());
        assert_eq!(invalid.code, INVALID_REQUEST);

        let not_found = JsonRpcError::method_not_found("test".to_string());
        assert_eq!(not_found.code, METHOD_NOT_FOUND);

        let params = JsonRpcError::invalid_params("test".to_string());
        assert_eq!(params.code, INVALID_PARAMS);

        let internal = JsonRpcError::internal_error("test".to_string());
        assert_eq!(internal.code, INTERNAL_ERROR);
    }

    #[test]
    fn test_round_trip_serialization() {
        let original = JsonRpcRequest::new(
            json!(42),
            "test_method".to_string(),
            json!({"param": "value"}),
        );

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: JsonRpcRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }
}
