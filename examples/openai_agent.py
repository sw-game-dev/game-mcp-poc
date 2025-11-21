#!/usr/bin/env python3
"""
OpenAI GPT-4 agent that plays tic-tac-toe using the MCP HTTP server.

Usage:
    export OPENAI_API_KEY="your-api-key-here"
    python3 examples/openai_agent.py
"""

import openai
import requests
import json
import os

# MCP server endpoint
MCP_URL = "http://localhost:7397/mcp"

def call_mcp_tool(method, params=None):
    """Call an MCP tool via HTTP."""
    payload = {
        "jsonrpc": "2.0",
        "method": method,
        "params": params or {},
        "id": 1
    }
    response = requests.post(MCP_URL, json=payload)
    result = response.json()
    if "error" in result:
        raise Exception(f"MCP Error: {result['error']}")
    return result.get("result", {})

# Define OpenAI function definitions
functions = [
    {
        "name": "view_game_state",
        "description": "View the current tic-tac-toe game state including board, turn, status, and move history",
        "parameters": {
            "type": "object",
            "properties": {},
            "required": []
        }
    },
    {
        "name": "get_turn",
        "description": "Get whose turn it is (X or O)",
        "parameters": {
            "type": "object",
            "properties": {},
            "required": []
        }
    },
    {
        "name": "make_move",
        "description": "Make a move on the tic-tac-toe board",
        "parameters": {
            "type": "object",
            "properties": {
                "row": {
                    "type": "integer",
                    "description": "Row index (0-2)",
                    "minimum": 0,
                    "maximum": 2
                },
                "col": {
                    "type": "integer",
                    "description": "Column index (0-2)",
                    "minimum": 0,
                    "maximum": 2
                }
            },
            "required": ["row", "col"]
        }
    },
    {
        "name": "taunt_player",
        "description": "Send a trash talk message to your opponent",
        "parameters": {
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "The taunt message to send"
                }
            },
            "required": ["message"]
        }
    },
    {
        "name": "restart_game",
        "description": "Restart the game with a fresh board",
        "parameters": {
            "type": "object",
            "properties": {},
            "required": []
        }
    }
]

def execute_function(function_name, arguments):
    """Execute a function call by calling the MCP server."""
    print(f"\n🎮 Calling {function_name} with args: {arguments}")
    result = call_mcp_tool(function_name, arguments)
    print(f"✅ Result: {json.dumps(result, indent=2)}")
    return result

def run_agent():
    """Run the OpenAI agent to play tic-tac-toe."""
    client = openai.OpenAI(api_key=os.environ.get("OPENAI_API_KEY"))

    messages = [
        {
            "role": "system",
            "content": "You are a competitive tic-tac-toe player who loves trash talk. "
                      "Play strategically and taunt your opponent with creative messages. "
                      "Always check the game state first, then make your move, then taunt."
        },
        {
            "role": "user",
            "content": "Let's play tic-tac-toe! Make your first move and trash talk me!"
        }
    ]

    print("🤖 Starting OpenAI agent...")
    print("=" * 60)

    # Allow up to 10 function calls
    for turn in range(10):
        print(f"\n--- Turn {turn + 1} ---")

        response = client.chat.completions.create(
            model="gpt-4",
            messages=messages,
            functions=functions,
            function_call="auto"
        )

        message = response.choices[0].message

        # Check if the model wants to call a function
        if message.function_call:
            function_name = message.function_call.name
            arguments = json.loads(message.function_call.arguments)

            # Execute the function
            result = execute_function(function_name, arguments)

            # Add function call and result to messages
            messages.append({
                "role": "assistant",
                "content": None,
                "function_call": {
                    "name": function_name,
                    "arguments": message.function_call.arguments
                }
            })
            messages.append({
                "role": "function",
                "name": function_name,
                "content": json.dumps(result)
            })

        else:
            # Model responded with text
            print(f"\n💬 GPT-4 says: {message.content}")
            messages.append({"role": "assistant", "content": message.content})
            break

    print("\n" + "=" * 60)
    print("🎮 Game session complete!")

if __name__ == "__main__":
    if not os.environ.get("OPENAI_API_KEY"):
        print("❌ Error: OPENAI_API_KEY environment variable not set")
        print("Usage: export OPENAI_API_KEY='your-key' && python3 examples/openai_agent.py")
        exit(1)

    try:
        run_agent()
    except KeyboardInterrupt:
        print("\n\n👋 Game interrupted by user")
    except Exception as e:
        print(f"\n❌ Error: {e}")
