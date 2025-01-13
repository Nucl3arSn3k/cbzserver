use rusqlite::{Connection, Result};

fn initialize_db() -> Result<Connection> {
    let conn = Connection::open("users.db")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        (), // empty params
    )?;

    Ok(conn)
}