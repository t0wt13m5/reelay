use rusqlite::{Connection, Result};

pub fn create_db_connection() -> Result<Connection> {
    let path = super::path_handling::get_complete_db_path();
    let conn = Connection::open(path)?;
    Ok(conn)
}
