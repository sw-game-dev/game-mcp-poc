#!/usr/bin/env python3
"""
Google Gemini agent that plays tic-tac-toe using the MCP HTTP server.

Usage:
    export GOOGLE_API_KEY="your-api-key-here"
    python3 examples/gemini_agent.py
"""

import google.generativeai as genai
import requests
import json
import os

# MCP server endpoint
MCP_URL = "http://localhost:3000/mcp"

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

# Define Gemini function declarations
function_declarations = [
    {
        "name": "view_game_state",
        "description": "View the current tic-tac-toe game state including board, turn, status, and move history",
        "parameters": {
            "type": "object",
            "properties": {}
        }
    },
    {
        "name": "get_turn",
        "description": "Get whose turn it is (X or O)",
        "parameters": {
            "type": "object",
            "properties": {}
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
                    "description": "Row index (0-2)"
                },
                "col": {
                    "type": "integer",
                    "description": "Column index (0-2)"
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
            "properties": {}
        }
    }
]

def execute_function_call(function_call):
    """Execute a function call by calling the MCP server."""
    function_name = function_call.name
    arguments = dict(function_call.args)

    print(f"\n🎮 Calling {function_name} with args: {arguments}")
    result = call_mcp_tool(function_name, arguments)
    print(f"✅ Result: {json.dumps(result, indent=2)}")
    return result

def run_agent():
    """Run the Gemini agent to play tic-tac-toe."""
    genai.configure(api_key=os.environ.get("GOOGLE_API_KEY"))

    # Create model with function calling enabled
    model = genai.GenerativeModel(
        'gemini-pro',
        tools=function_declarations
    )

    chat = model.start_chat(enable_automatic_function_calling=False)

    print("🤖 Starting Gemini agent...")
    print("=" * 60)

    # Initial prompt
    prompt = (
        "Let's play tic-tac-toe! You are a competitive player who loves trash talk. "
        "First, check the game state, then make a strategic move, then send a taunt. "
        "Keep playing until the game is over."
    )

    print(f"\n💬 User: {prompt}")

    # Allow up to 15 turns
    for turn in range(15):
        print(f"\n--- Turn {turn + 1} ---")

        response = chat.send_message(prompt)

        # Check for function calls
        if response.candidates[0].content.parts:
            part = response.candidates[0].content.parts[0]

            if hasattr(part, 'function_call'):
                function_call = part.function_call
                result = execute_function_call(function_call)

                # Send function response back
                prompt = genai.protos.Content(
                    parts=[genai.protos.Part(
                        function_response=genai.protos.FunctionResponse(
                            name=function_call.name,
                            response={'result': result}
                        )
                    )]
                )

            elif hasattr(part, 'text'):
                # Model responded with text
                print(f"\n💬 Gemini says: {part.text}")

                # Check if game is over
                if any(word in part.text.lower() for word in ['game over', 'won', 'draw', 'tie']):
                    break

                prompt = "Continue playing."

        else:
            print("\n✅ No more actions from Gemini")
            break

    print("\n" + "=" * 60)
    print("🎮 Game session complete!")

if __name__ == "__main__":
    if not os.environ.get("GOOGLE_API_KEY"):
        print("❌ Error: GOOGLE_API_KEY environment variable not set")
        print("Usage: export GOOGLE_API_KEY='your-key' && python3 examples/gemini_agent.py")
        print("\nGet your API key from: https://makersuite.google.com/app/apikey")
        exit(1)

    try:
        run_agent()
    except KeyboardInterrupt:
        print("\n\n👋 Game interrupted by user")
    except Exception as e:
        print(f"\n❌ Error: {e}")
        import traceback
        traceback.print_exc()
