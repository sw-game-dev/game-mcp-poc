use rusqlite::Connection;
use shared::GameError;

/// Initialize the database schema
pub fn init_schema(conn: &Connection) -> Result<(), GameError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id TEXT PRIMARY KEY,
            human_player TEXT NOT NULL,
            ai_player TEXT NOT NULL,
            current_turn TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        [],
    )
    .map_err(|e| GameError::DatabaseError {
        message: e.to_string(),
    })?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS moves (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            game_id TEXT NOT NULL,
            player TEXT NOT NULL,
            row INTEGER NOT NULL,
            col INTEGER NOT NULL,
            timestamp INTEGER NOT NULL,
            source TEXT,
            FOREIGN KEY (game_id) REFERENCES games(id)
        )",
        [],
    )
    .map_err(|e| GameError::DatabaseError {
        message: e.to_string(),
    })?;

    // Add source column to existing tables (migration)
    // Ignore error if column already exists
    let _ = conn.execute("ALTER TABLE moves ADD COLUMN source TEXT", []);
    let _ = conn.execute("ALTER TABLE taunts ADD COLUMN source TEXT", []);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS taunts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            game_id TEXT NOT NULL,
            message TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            source TEXT,
            FOREIGN KEY (game_id) REFERENCES games(id)
        )",
        [],
    )
    .map_err(|e| GameError::DatabaseError {
        message: e.to_string(),
    })?;

    // Table to track the current active game (singleton pattern)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS current_game (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            game_id TEXT NOT NULL,
            FOREIGN KEY (game_id) REFERENCES games(id)
        )",
        [],
    )
    .map_err(|e| GameError::DatabaseError {
        message: e.to_string(),
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_schema_creates_tables() {
        let conn = Connection::open_in_memory().unwrap();
        assert!(init_schema(&conn).is_ok());

        // Verify games table exists
        let result: Result<String, _> = conn.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='games'",
            [],
            |row| row.get(0),
        );
        assert_eq!(result.unwrap(), "games");

        // Verify moves table exists
        let result: Result<String, _> = conn.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='moves'",
            [],
            |row| row.get(0),
        );
        assert_eq!(result.unwrap(), "moves");

        // Verify taunts table exists
        let result: Result<String, _> = conn.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='taunts'",
            [],
            |row| row.get(0),
        );
        assert_eq!(result.unwrap(), "taunts");
    }

    #[test]
    fn test_init_schema_idempotent() {
        let conn = Connection::open_in_memory().unwrap();

        // Should succeed the first time
        assert!(init_schema(&conn).is_ok());

        // Should succeed the second time (idempotent)
        assert!(init_schema(&conn).is_ok());
    }
}
