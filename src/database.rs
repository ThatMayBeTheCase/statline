use rusqlite::{Connection, params};

use crate::game::Game;

pub fn init_database() -> rusqlite::Result<Connection> {
    let conn = Connection::open("statline.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        executable TEXT NOT NULL UNIQUE
    )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
            game_id INTEGER NOT NULL,
            started_at INTEGER NOT NULL,
            ended_at INTEGER NOT NULL,
            duration_seconds INTEGER NOT NULL,
            FOREIGN KEY (game_id) REFERENCES games(id)
        )",
        [],
    )?;

    Ok(conn)
}

pub fn register_games(conn: &Connection, games: &[Game]) -> rusqlite::Result<()> {
    for game in games.iter() {
        conn.execute(
            "INSERT OR IGNORE INTO games (name, executable)
         VALUES (?1, ?2)",
            params![&game.name, &game.executable],
        )?;
    }

    Ok(())
}

pub fn save_session(
    conn: &Connection,
    game_id: i64,
    started_at: i64,
    ended_at: i64,
    duration_seconds: i64,
) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO sessions (game_id, started_at, ended_at, duration_seconds)
         VALUES (?1, ?2, ?3, ?4)",
        params![game_id, started_at, ended_at, duration_seconds],
    )?;

    Ok(())
}

pub fn get_game_id(conn: &Connection, executable: &str) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT id FROM games WHERE executable = ?1",
        params![executable],
        |row| row.get(0),
    )
}

pub fn get_total_playtime(conn: &Connection, game_id: i64) -> rusqlite::Result<u64> {
    let total: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration_seconds), 0)
         FROM sessions
         WHERE game_id = ?1",
        params![game_id],
        |row| row.get(0),
    )?;

    u64::try_from(total).map_err(|_| rusqlite::Error::IntegralValueOutOfRange(0, total))
}
