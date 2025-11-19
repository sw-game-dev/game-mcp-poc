use log::info;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    info!("Rendering App component");

    html! {
        <div class="app-container">
            <h1>{"Tic-Tac-Toe MCP Game"}</h1>
            <div class="game-info">
                <p>{"Game is loading..."}</p>
            </div>
            <div class="game-board">
                {(0..9).map(|i| {
                    html! {
                        <div class="cell" key={i}>
                            {""}
                        </div>
                    }
                }).collect::<Html>()}
            </div>
            <div class="controls">
                <button class="btn-primary">{"New Game"}</button>
            </div>
            <div class="log-container">
                <h3>{"Game Log"}</h3>
                <div class="log-entry">{"Welcome to Tic-Tac-Toe!"}</div>
            </div>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    // Initialize logging
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    info!("Starting Tic-Tac-Toe frontend");

    yew::Renderer::<App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    println!("This application is designed to run in the browser with WASM");
}

#[cfg(test)]
mod tests {
    use shared::{Cell, Player};

    #[test]
    fn test_player_types_used_in_component() {
        // Validate that Player enum used in the game component works correctly
        let player_x = Player::X;
        let player_o = Player::O;
        assert_ne!(player_x, player_o);
        assert_eq!(player_x.opponent(), player_o);
        assert_eq!(player_o.opponent(), player_x);
    }

    #[test]
    fn test_cell_types_used_in_board() {
        // Validate that Cell enum used to render the board works correctly
        let empty = Cell::Empty;
        let occupied_x = Cell::Occupied(Player::X);
        let occupied_o = Cell::Occupied(Player::O);

        assert_eq!(empty, Cell::default());
        assert_ne!(empty, occupied_x);
        assert_ne!(occupied_x, occupied_o);
    }
}
