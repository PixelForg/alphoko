use rusqlite::{Connection, Result};

pub fn start_connection() -> Result<Connection> {
    Connection::open("manga-panels.db")
}

pub fn create_tables(database_connection: &Connection) -> Result<()> {
    database_connection.execute(
        "CREATE TABLE IF NOT EXISTS manga_panels (
            id INTEGER PRIMARY KEY,
            manga_panel_file_path TEXT NOT NULL UNIQUE,
            manga_panel_text TEXT NOT NULL,
            number_of_times_copied INTEGER,
            manga_name TEXT NOT NULL UNIQUE,
            manga_panel_width INTEGER,
            manga_panel_height INTEGER
        )",
        (),
    )?;
    let virtual_table_exists = database_connection.table_exists(None, "manga_panels_fts")?;
    if !virtual_table_exists {
        database_connection.execute(
            "CREATE VIRTUAL TABLE manga_panels_fts USING fts5 (
                manga_panel_text,
                manga_name
            )",
            (),
        )?;
    }
    Ok(())
}
