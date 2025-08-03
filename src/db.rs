use rusqlite::{Connection, Result, params};

#[allow(dead_code)]
pub struct MangaPanels {
    pub manga_panel_file_path: String,
    pub manga_panel_text: String,
    pub number_of_times_copied: u32,
    pub manga_name: String,
}

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
    let mut statement = database_connection.prepare_cached(
        "INSERT INTO manga_panels (
            manga_panel_file_path,
            manga_panel_text,
            number_of_times_copied,
            manga_name
        ) values (?1, ?2, ?3, ?4)",
    )?;
    match statement.execute(params![
        manga_panel_file_path,
        manga_panel_text,
        0,
        manga_name
    ]) {
        Ok(count) => println!("Inserted, rows affected: {}", count),
        Err(e) => println!("insert error: {}", e),
    }

    Ok(())
}

pub fn retrieve_manga_panels_from_db(
    database_connection: &Connection,
    manga_panel_text_to_search: &String,
    manga_panel_name_to_search: &String,
) -> Result<Vec<MangaPanels>> {
    let mut statement = database_connection.prepare_cached(
        "SELECT manga_panel_file_path, manga_panel_text, number_of_times_copied, manga_name
        FROM manga_panels
        WHERE manga_panel_text LIKE '%' || ?1 || '%'
        AND (?2 = '' OR manga_name = ?2)",
    )?;
    let manga_panels_from_db = statement.query_map(
        [manga_panel_text_to_search, manga_panel_name_to_search],
        |row| {
            Ok(MangaPanels {
                manga_panel_file_path: row.get(0)?,
                manga_panel_text: row.get(1)?,
                number_of_times_copied: row.get(2)?,
                manga_name: row.get(3)?,
            })
        },
    )?;

    let mut manga_panels_to_show = Vec::new();

    for manga_panel in manga_panels_from_db {
        manga_panels_to_show.push(manga_panel?);
    }
    Ok(manga_panels_to_show)
}

pub fn retrieve_manga_names_from_db(database_connection: &Connection) -> Result<Vec<String>> {
    let mut statement =
        database_connection.prepare("SELECT DISTINCT manga_name FROM manga_panels")?;
    let manga_names = statement
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;
    Ok(manga_names)
}

pub fn retrieve_manga_panels_text_from_db(database_connection: &Connection) -> Result<Vec<String>> {
    let mut statement =
        database_connection.prepare("SELECT DISTINCT manga_panel_text FROM manga_panels")?;
    let manga_names = statement
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;
    Ok(manga_names)
}
