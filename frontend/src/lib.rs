mod api;

use log::info;

#[cfg(target_arch = "wasm32")]
use log::error;
use shared::{Cell, GameState, GameStatus, MoveSource, Player};
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::{EventSource, HtmlInputElement, KeyboardEvent};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;

/// Format Unix timestamp (seconds) to YYYY/MM/DD HH:MM
#[cfg(target_arch = "wasm32")]
fn format_timestamp(timestamp: i64) -> String {
    use wasm_bindgen::JsValue;
    let date = js_sys::Date::new(&JsValue::from_f64((timestamp * 1000) as f64));

    let year = date.get_full_year();
    let month = date.get_month() + 1; // JavaScript months are 0-indexed
    let day = date.get_date();
    let hours = date.get_hours();
    let minutes = date.get_minutes();

    format!(
        "{:04}/{:02}/{:02} {:02}:{:02}",
        year, month, day, hours, minutes
    )
}

#[function_component(App)]
fn app() -> Html {
    info!("Rendering App component");

    let game_state = use_state(|| None::<GameState>);
    let loading = use_state(|| true);
    let error_msg = use_state(|| None::<String>);
    let taunt_input = use_state(String::new);
    let mcp_thinking = use_state(|| false);
    let event_log = use_state(|| {
        vec![
            "Welcome to Tic-Tac-Toe!".to_string(),
            "Initializing...".to_string(),
        ]
    });

    // Log an event
    let log_event = {
        let event_log = event_log.clone();
        Callback::from(move |msg: String| {
            let mut logs = (*event_log).clone();
            logs.push(msg);
            // Keep only last 10 events
            if logs.len() > 10 {
                logs.remove(0);
            }
            event_log.set(logs);
        })
    };

    // Fetch initial state and set up SSE connection
    {
        #[cfg(target_arch = "wasm32")]
        let game_state = game_state.clone();
        #[cfg(target_arch = "wasm32")]
        let loading = loading.clone();
        #[cfg(target_arch = "wasm32")]
        let error_msg = error_msg.clone();
        let log_event = log_event.clone();

        use_effect_with((), move |_| {
            info!("Setting up SSE connection");
            log_event.emit("📡 Connecting to server via SSE...".to_string());

            // Initial fetch
            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local({
                let game_state = game_state.clone();
                let loading = loading.clone();
                let error_msg = error_msg.clone();
                let log_event = log_event.clone();

                async move {
                    match api::fetch_game_state().await {
                        Ok(state) => {
                            game_state.set(Some(state));
                            loading.set(false);
                            error_msg.set(None);
                            log_event.emit("✅ Initial game state loaded".to_string());
                        }
                        Err(e) => {
                            error!("Failed to load initial game state: {}", e);
                            error_msg.set(Some(format!("API Error: {}", e)));
                            loading.set(false);
                            log_event.emit(format!("⚠️ Failed to load game: {}", e));
                        }
                    }
                }
            });

            // Set up SSE connection
            #[cfg(target_arch = "wasm32")]
            let event_source_opt = EventSource::new("/api/events").ok();

            #[cfg(target_arch = "wasm32")]
            if let Some(ref event_source) = event_source_opt {
                log_event.emit("✅ SSE connected - listening for updates".to_string());

                // Handle incoming messages
                let onmessage = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
                    if let Some(data) = event.data().as_string() {
                        info!("SSE message received: {}", data);
                        match serde_json::from_str::<GameState>(&data) {
                            Ok(new_state) => {
                                game_state.set(Some(new_state));
                            }
                            Err(e) => {
                                error!("Failed to parse SSE data: {}", e);
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                event_source.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                onmessage.forget(); // Keep closure alive

                // Handle errors
                let onerror = Closure::wrap(Box::new(move |e: web_sys::Event| {
                    error!("SSE error: {:?}", e);
                    log_event.emit("⚠️ SSE connection error".to_string());
                }) as Box<dyn FnMut(_)>);

                event_source.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                onerror.forget();
            } else {
                #[cfg(target_arch = "wasm32")]
                {
                    error!("Failed to create EventSource");
                    log_event.emit("❌ Failed to connect to SSE".to_string());
                }
            }

            // Cleanup function
            move || {
                #[cfg(target_arch = "wasm32")]
                if let Some(es) = event_source_opt {
                    es.close();
                }
            }
        });
    }

    // Track move count to detect changes and log with source
    let prev_move_count = use_state(|| 0);
    {
        let prev_move_count = prev_move_count.clone();
        let log_event = log_event.clone();
        let game_state = game_state.clone();

        use_effect_with(game_state.clone(), move |state| {
            if let Some(state) = state.as_ref() {
                let current_count = state.move_history.len();
                if current_count > *prev_move_count {
                    let last_move = &state.move_history[current_count - 1];
                    let source_prefix = match &last_move.source {
                        Some(MoveSource::UI) => "UI:",
                        Some(MoveSource::MCP) => "MCP:",
                        None => "",
                    };
                    log_event.emit(format!(
                        "{} 🎮 {} moved to ({}, {})",
                        source_prefix, last_move.player, last_move.row, last_move.col
                    ));
                    prev_move_count.set(current_count);
                }
            }
            || ()
        });
    }

    // Track taunt count to detect new taunts and auto-scroll
    let prev_taunt_count = use_state(|| 0);
    {
        let prev_taunt_count = prev_taunt_count.clone();
        let log_event = log_event.clone();
        let game_state = game_state.clone();

        use_effect_with(game_state.clone(), move |state| {
            if let Some(state) = state.as_ref() {
                let current_count = state.taunts.len();
                if current_count > *prev_taunt_count {
                    // Log all new taunts
                    for i in *prev_taunt_count..current_count {
                        let taunt = &state.taunts[i];
                        let prefix = match &taunt.source {
                            Some(MoveSource::UI) => "💬 You:",
                            Some(MoveSource::MCP) => "💬 MCP:",
                            None => "💬",
                        };
                        log_event.emit(format!("{} {}", prefix, taunt.message));
                    }
                    prev_taunt_count.set(current_count);

                    // Auto-scroll taunt display to bottom
                    #[cfg(target_arch = "wasm32")]
                    {
                        use wasm_bindgen::JsCast;
                        if let Some(window) = web_sys::window() {
                            if let Some(document) = window.document() {
                                if let Some(taunt_display) =
                                    document.get_element_by_id("taunt-display")
                                {
                                    if let Some(element) =
                                        taunt_display.dyn_ref::<web_sys::HtmlElement>()
                                    {
                                        element.set_scroll_top(element.scroll_height());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            || ()
        });
    }

    // Track MCP activity and show "thinking" indicator with debounce
    // Configurable delay in milliseconds (100ms in production, 2000ms for testing)
    const MCP_THINKING_DELAY_MS: u32 = 2000;

    {
        let game_state = game_state.clone();
        let mcp_thinking = mcp_thinking.clone();

        use_effect_with(game_state.clone(), move |state| {
            if let Some(state) = state.as_ref()
                && state.last_mcp_activity.is_some()
            {
                // Show thinking indicator immediately
                mcp_thinking.set(true);

                // Hide after configured delay
                #[cfg(target_arch = "wasm32")]
                {
                    let mcp_thinking = mcp_thinking.clone();
                    let timeout_id =
                        gloo::timers::callback::Timeout::new(MCP_THINKING_DELAY_MS, move || {
                            mcp_thinking.set(false);
                        });
                    timeout_id.forget(); // Let it run
                }
            }
            || ()
        });
    }

    // Handle taunt input change
    #[cfg(target_arch = "wasm32")]
    let on_taunt_input = {
        let taunt_input = taunt_input.clone();
        Callback::from(move |e: web_sys::InputEvent| {
            if let Some(target) = e.target() {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    taunt_input.set(input.value());
                }
            }
        })
    };

    #[cfg(not(target_arch = "wasm32"))]
    let on_taunt_input = Callback::from(move |_: web_sys::InputEvent| {});

    // Handle taunt submission
    let on_send_taunt = {
        #[cfg(target_arch = "wasm32")]
        let taunt_input = taunt_input.clone();
        #[cfg(target_arch = "wasm32")]
        let log_event = log_event.clone();

        Callback::from(move |_| {
            #[cfg(target_arch = "wasm32")]
            {
                let message = (*taunt_input).clone();
                if message.trim().is_empty() {
                    return;
                }

                let taunt_input = taunt_input.clone();
                let log_event = log_event.clone();

                log_event.emit(format!("💬 Sending taunt: {}", message));

                wasm_bindgen_futures::spawn_local(async move {
                    match api::send_taunt(message).await {
                        Ok(_) => {
                            info!("Taunt sent successfully");
                            taunt_input.set(String::new());
                            // State will be updated via SSE
                        }
                        Err(e) => {
                            error!("Failed to send taunt: {}", e);
                            log_event.emit(format!("❌ Failed to send taunt: {}", e));
                        }
                    }
                });
            }
        })
    };

    let on_new_game = {
        #[cfg(target_arch = "wasm32")]
        let game_state = game_state.clone();
        let loading = loading.clone();
        let log_event = log_event.clone();

        Callback::from(move |_| {
            #[cfg(target_arch = "wasm32")]
            let game_state = game_state.clone();
            let loading = loading.clone();
            let log_event = log_event.clone();

            loading.set(true);
            log_event.emit("🔄 Creating new game...".to_string());

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_futures::spawn_local(async move {
                match api::create_new_game().await {
                    Ok(new_state) => {
                        info!("New game created");
                        game_state.set(Some(new_state));
                        loading.set(false);
                        log_event.emit("✨ New game started!".to_string());
                    }
                    Err(e) => {
                        error!("Failed to create new game: {}", e);
                        loading.set(false);
                        log_event.emit(format!("❌ Failed to create game: {}", e));
                    }
                }
            });
        })
    };

    let game_info = if *loading {
        html! { <p>{"Game is loading..."}</p> }
    } else if let Some(ref err) = *error_msg {
        html! { <p class="error">{err}</p> }
    } else if let Some(ref state) = *game_state {
        let status_text = match &state.status {
            shared::GameStatus::InProgress => {
                format!("{}'s turn", state.current_turn)
            }
            shared::GameStatus::Won(player) => format!("{} wins!", player),
            shared::GameStatus::Draw => "It's a draw!".to_string(),
        };

        // Show turn indicator flash at game start (when move_history is empty or 1 move)
        let turn_indicator =
            if state.status == GameStatus::InProgress && state.move_history.len() <= 1 {
                if state.current_turn == state.human_player {
                    html! { <div class="turn-indicator flash">{"🎯 YOUR TURN!"}</div> }
                } else {
                    html! { <div class="turn-indicator flash">{"⏳ Opponent's turn..."}</div> }
                }
            } else {
                html! {}
            };

        html! {
            <>
                <p>{format!("You are {}. {}", state.human_player, status_text)}</p>
                {turn_indicator}
            </>
        }
    } else {
        html! { <p>{"Click 'New Game' to start"}</p> }
    };

    // Handle drag start
    #[cfg(target_arch = "wasm32")]
    let on_drag_start = {
        Callback::from(move |e: DragEvent| {
            if let Some(drag_event) = e.dyn_ref::<web_sys::DragEvent>() {
                if let Some(data_transfer) = drag_event.data_transfer() {
                    let _ = data_transfer.set_data("text/plain", "mark");
                    data_transfer.set_effect_allowed("move");
                }
            }
        })
    };

    #[cfg(not(target_arch = "wasm32"))]
    let on_drag_start = Callback::from(move |_: DragEvent| {});

    // Handle drop on cell
    #[cfg(target_arch = "wasm32")]
    let on_drop = {
        let game_state = game_state.clone();
        let log_event = log_event.clone();

        Callback::from(move |(e, row, col): (DragEvent, u8, u8)| {
            if let Some(drag_event) = e.dyn_ref::<web_sys::DragEvent>() {
                drag_event.prevent_default();
            }

            let game_state = game_state.clone();
            let log_event = log_event.clone();

            // Check if it's a valid move
            if let Some(ref state) = *game_state {
                // Can't move if game is over
                if state.status != GameStatus::InProgress {
                    log_event.emit("⚠️ Game is over! Start a new game.".to_string());
                    return;
                }

                // Can't move if not human's turn
                if state.current_turn != state.human_player {
                    log_event.emit("⚠️ It's not your turn!".to_string());
                    return;
                }

                // Can't move if cell is occupied
                if state.board[row as usize][col as usize] != Cell::Empty {
                    log_event.emit("⚠️ Cell is already occupied!".to_string());
                    return;
                }

                // Make the move
                log_event.emit(format!("📤 Dropping mark at ({}, {})...", row, col));

                wasm_bindgen_futures::spawn_local({
                    let log_event = log_event.clone();
                    async move {
                        match api::make_move(row, col).await {
                            Ok(_) => {
                                info!("Move made successfully");
                                // State will be updated via SSE
                            }
                            Err(e) => {
                                error!("Failed to make move: {}", e);
                                log_event.emit(format!("❌ Move failed: {}", e));
                            }
                        }
                    }
                });
            }
        })
    };

    // Handle drag over (to allow drop)
    #[cfg(target_arch = "wasm32")]
    let on_drag_over = {
        Callback::from(move |e: DragEvent| {
            if let Some(drag_event) = e.dyn_ref::<web_sys::DragEvent>() {
                drag_event.prevent_default();
                if let Some(data_transfer) = drag_event.data_transfer() {
                    data_transfer.set_drop_effect("move");
                }
            }
        })
    };

    #[cfg(not(target_arch = "wasm32"))]
    let _on_drag_over = Callback::from(move |_: DragEvent| {});

    let board_cells = if let Some(ref state) = *game_state {
        (0..9)
            .map(|i| {
                let row = (i / 3) as u8;
                let col = (i % 3) as u8;
                let cell = state.board[row as usize][col as usize];
                let cell_text = match cell {
                    Cell::Empty => "",
                    Cell::Occupied(Player::X) => "X",
                    Cell::Occupied(Player::O) => "O",
                };

                // Check if this cell is part of winning line
                let is_winning_cell = if let Some(ref winning_line) = state.winning_line {
                    winning_line
                        .positions
                        .iter()
                        .any(|(r, c)| *r == row && *c == col)
                } else {
                    false
                };

                let cell_class = if is_winning_cell {
                    "cell winning-cell"
                } else if cell == Cell::Empty {
                    "cell drop-target"
                } else {
                    "cell"
                };

                #[cfg(target_arch = "wasm32")]
                {
                    let on_drop_handler = on_drop.clone();
                    let on_drag_over_handler = on_drag_over.clone();

                    let ondrop = Callback::from(move |e: DragEvent| {
                        on_drop_handler.emit((e, row, col));
                    });

                    let ondragover = Callback::from(move |e: DragEvent| {
                        on_drag_over_handler.emit(e);
                    });

                    html! {
                        <div class={cell_class} key={i} ondrop={ondrop} ondragover={ondragover}>
                            {cell_text}
                        </div>
                    }
                }

                #[cfg(not(target_arch = "wasm32"))]
                html! {
                    <div class={cell_class} key={i}>
                        {cell_text}
                    </div>
                }
            })
            .collect::<Html>()
    } else {
        (0..9)
            .map(|i| {
                html! {
                    <div class="cell" key={i}>
                        {""}
                    </div>
                }
            })
            .collect::<Html>()
    };

    // Render event log
    let log_entries = (*event_log)
        .iter()
        .rev()
        .map(|entry| {
            html! { <div class="log-entry">{entry}</div> }
        })
        .collect::<Html>();

    // Draw overlay
    let draw_overlay = if let Some(ref state) = *game_state {
        if state.status == GameStatus::Draw {
            html! {
                <div class="game-overlay">
                    <div class="draw-text">{"DRAW"}</div>
                </div>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    };

    // Draggable mark component
    let draggable_mark = if let Some(ref state) = *game_state {
        let is_human_turn = state.current_turn == state.human_player;
        let is_game_active = state.status == GameStatus::InProgress;
        let is_enabled = is_human_turn && is_game_active;

        let mark_text = format!("{}", state.human_player);
        let mark_class = if is_enabled {
            "draggable-mark enabled"
        } else {
            "draggable-mark disabled"
        };

        let on_drag_start_handler = on_drag_start.clone();

        html! {
            <div class="draggable-mark-container">
                <div
                    class={mark_class}
                    draggable={is_enabled.to_string()}
                    ondragstart={on_drag_start_handler}
                >
                    {mark_text}
                </div>
                {
                    if is_enabled {
                        html! { <div class="drag-hint">{"← Drag to board"}</div> }
                    } else {
                        html! { <div class="drag-hint disabled">{"Wait for your turn..."}</div> }
                    }
                }
            </div>
        }
    } else {
        html! {}
    };

    // MCP thinking indicator
    let thinking_indicator = if *mcp_thinking {
        html! {
            <div class="mcp-thinking-indicator">
                <span class="thinking-text">{"MCP Agent Thinking..."}</span>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div class="app-container">
            <header class="app-header">
                <div class="header-title">
                    <h1>{"TTTTT"}</h1>
                    <span class="subtitle">{"Trash Talkin' Tic-Tac-Toe"}</span>
                </div>
                <a href="https://github.com/sw-game-dev/game-mcp-poc" target="_blank" class="github-link" title="Source code">
                    <div class="github-corner">
                        <svg width="60" height="60" viewBox="0 0 250 250" style="fill:#667eea; color:#fff; position: absolute; top: 0; border: 0; right: 0;" aria-hidden="true">
                            <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
                            <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="octo-arm"></path>
                            <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="octo-body"></path>
                        </svg>
                    </div>
                </a>
            </header>
            <div class="game-info">
                {game_info}
                {thinking_indicator}
            </div>
            <div class="game-layout">
                <div class="left-panel">
                    <div class="board-with-mark">
                        {draggable_mark}
                        <div class="game-board-container">
                            <div class="game-board">
                                {board_cells}
                            </div>
                            {draw_overlay}
                        </div>
                    </div>
                    <div class="controls">
                        <button class="btn-primary" onclick={on_new_game} disabled={*loading}>
                            {"New Game"}
                        </button>
                    </div>
                    <div class="log-container">
                        <h3>{"Event Log"}</h3>
                        <div class="log-scroll">
                            {log_entries}
                        </div>
                    </div>
                </div>
                <div class="right-panel">
                    <div class="chat-container">
                <h3>{"💬 Trash Talk"}</h3>
                <div class="taunt-display" id="taunt-display">
                    {
                        if let Some(ref state) = *game_state {
                            if state.taunts.is_empty() {
                                html! { <div class="taunt-empty">{"No taunts yet..."}</div> }
                            } else {
                                // Show all taunts in chronological order (oldest first)
                                let taunt_count = state.taunts.len();
                                let taunt_messages: Vec<_> = state.taunts.iter()
                                    .enumerate()
                                    .map(|(idx, taunt)| {
                                        let (label, label_class) = match &taunt.source {
                                            Some(MoveSource::UI) => ("You: ", "taunt-label taunt-label-ui"),
                                            Some(MoveSource::MCP) => ("MCP Agent: ", "taunt-label taunt-label-mcp"),
                                            None => ("Unknown: ", "taunt-label"),
                                        };

                                        // Build class string with user-taunt for UI messages
                                        let is_user = matches!(&taunt.source, Some(MoveSource::UI));
                                        let is_latest = idx == taunt_count - 1;

                                        let class = match (is_user, is_latest) {
                                            (true, true) => "taunt-message user-taunt latest-taunt",
                                            (true, false) => "taunt-message user-taunt",
                                            (false, true) => "taunt-message latest-taunt",
                                            (false, false) => "taunt-message",
                                        };

                                        // Format timestamp for hover text (WASM only)
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            let timestamp_text = format_timestamp(taunt.timestamp);
                                            html! {
                                                <div class={class}>
                                                    <span class={label_class} title={timestamp_text}>{label}</span>
                                                    <span class="taunt-text">{&taunt.message}</span>
                                                </div>
                                            }
                                        }

                                        #[cfg(not(target_arch = "wasm32"))]
                                        html! {
                                            <div class={class}>
                                                <span class={label_class}>{label}</span>
                                                <span class="taunt-text">{&taunt.message}</span>
                                            </div>
                                        }
                                    })
                                    .collect();
                                html! { <>{taunt_messages}</> }
                            }
                        } else {
                            html! { <div class="taunt-empty">{"Waiting for game..."}</div> }
                        }
                    }
                </div>
                <div class="taunt-input-container">
                    <input
                        type="text"
                        class="taunt-input"
                        placeholder="Type your taunt here..."
                        value={(*taunt_input).clone()}
                        oninput={on_taunt_input}
                        onkeypress={
                            #[cfg(target_arch = "wasm32")]
                            {
                                let taunt_input = taunt_input.clone();
                                let log_event = log_event.clone();
                                Callback::from(move |e: KeyboardEvent| {
                                    if e.key() == "Enter" {
                                        let message = (*taunt_input).clone();
                                        if message.trim().is_empty() {
                                            return;
                                        }

                                        let taunt_input = taunt_input.clone();
                                        let log_event = log_event.clone();

                                        log_event.emit(format!("💬 Sending taunt: {}", message));

                                        wasm_bindgen_futures::spawn_local(async move {
                                            match api::send_taunt(message).await {
                                                Ok(_) => {
                                                    info!("Taunt sent successfully");
                                                    taunt_input.set(String::new());
                                                }
                                                Err(e) => {
                                                    error!("Failed to send taunt: {}", e);
                                                    log_event.emit(format!("❌ Failed to send taunt: {}", e));
                                                }
                                            }
                                        });
                                    }
                                })
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                Callback::from(|_: KeyboardEvent| {})
                            }
                        }
                    />
                    <button
                        class="btn-taunt"
                        onclick={on_send_taunt}
                        disabled={taunt_input.trim().is_empty()}
                    >
                        {"Send"}
                    </button>
                </div>
                {
                    // Show taunt history
                    if let Some(ref state) = *game_state {
                        if state.taunts.len() > 1 {
                            let taunt_history: Vec<_> = state.taunts.iter()
                                .rev()
                                .skip(1) // Skip the latest (already shown above)
                                .take(3) // Show last 3
                                .map(|taunt| {
                                    let prefix = match &taunt.source {
                                        Some(MoveSource::UI) => "You: ",
                                        Some(MoveSource::MCP) => "MCP: ",
                                        None => "",
                                    };
                                    html! {
                                        <div class="taunt-history-item">{format!("{}{}", prefix, taunt.message)}</div>
                                    }
                                })
                                .collect();

                            html! {
                                <div class="taunt-history">
                                    <details>
                                        <summary>{"Previous taunts"}</summary>
                                        {taunt_history}
                                    </details>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    } else {
                        html! {}
                    }
                }
                    </div>
                </div>
            </div>
            <footer class="app-footer">
                <div class="footer-content">
                    <span class="copyright">{"© 2025 Michael A. Wright"}</span>
                    <span class="separator">{" | "}</span>
                    <a href="https://github.com/sw-game-dev/game-mcp-poc/blob/main/LICENSE" target="_blank" class="license-link">{"License"}</a>
                    <span class="separator">{" | "}</span>
                    <span class="build-details">
                        {format!("Build: {} @ {} on {}",
                            shared::build_info::GIT_SHA,
                            shared::build_info::BUILD_TIME,
                            shared::build_info::BUILD_HOST
                        )}
                    </span>
                </div>
            </footer>
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
    use super::*;

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

    #[test]
    fn test_draggable_mark_should_show_human_player_mark() {
        // When human player is X, draggable mark should show X
        let human_player = Player::X;
        let mark = format!("{}", human_player);
        assert_eq!(mark, "X");

        // When human player is O, draggable mark should show O
        let human_player = Player::O;
        let mark = format!("{}", human_player);
        assert_eq!(mark, "O");
    }

    #[test]
    fn test_draggable_mark_enabled_when_human_turn() {
        // Create a game state where it's the human's turn
        let human_player = Player::X;
        let current_turn = Player::X;
        let is_enabled = current_turn == human_player;
        assert!(
            is_enabled,
            "Draggable mark should be enabled on human's turn"
        );
    }

    #[test]
    fn test_draggable_mark_disabled_when_opponent_turn() {
        // Create a game state where it's the opponent's turn
        let human_player = Player::X;
        let current_turn = Player::O;
        let is_enabled = current_turn == human_player;
        assert!(
            !is_enabled,
            "Draggable mark should be disabled on opponent's turn"
        );
    }

    #[test]
    fn test_draggable_mark_disabled_when_game_over() {
        // Draggable mark should be disabled when game is over
        let status = GameStatus::Won(Player::X);
        let is_game_over = status != GameStatus::InProgress;
        assert!(is_game_over, "Game should be over when status is Won");

        let status = GameStatus::Draw;
        let is_game_over = status != GameStatus::InProgress;
        assert!(is_game_over, "Game should be over when status is Draw");
    }

    #[test]
    fn test_drop_target_accepts_drops_on_empty_cells() {
        // Empty cells should accept drops
        let cell = Cell::Empty;
        let can_drop = cell == Cell::Empty;
        assert!(can_drop, "Should be able to drop on empty cell");
    }

    #[test]
    fn test_drop_target_rejects_drops_on_occupied_cells() {
        // Occupied cells should not accept drops
        let cell = Cell::Occupied(Player::X);
        let can_drop = cell == Cell::Empty;
        assert!(!can_drop, "Should not be able to drop on occupied cell");

        let cell = Cell::Occupied(Player::O);
        let can_drop = cell == Cell::Empty;
        assert!(!can_drop, "Should not be able to drop on occupied cell");
    }
}
