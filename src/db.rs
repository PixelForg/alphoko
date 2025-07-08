use rusqlite::{Connection, Result, params};

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
            manga_name TEXT NOT NULL
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

pub fn add_manga_panel_to_db(
    database_connection: &Connection,
    manga_panel_file_path: &String,
    manga_panel_text: &String,
    manga_name: &String,
) -> Result<()> {
    let res = database_connection.execute(
        "INSERT INTO manga_panels (
            manga_panel_file_path, 
            manga_panel_text, 
            number_of_times_copied,
            manga_name
        ) values (?1, ?2, ?3, ?4)",
        params![manga_panel_file_path, manga_panel_text, 0, manga_name],
    );
    match res {
        Ok(count) => println!("Inserted, rows affected: {}", count),
        Err(e) => println!("insert error: {}", e),
    }
    Ok(())
}
