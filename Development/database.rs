
use rusqlite::{Connection, Result};

fn read_items() -> Result<()> {
    let conn = Connection::open("items.db")?;
    let mut stmt = conn.prepare("SELECT * FROM items_2025_03_31")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?, // e.g., name
            row.get::<_, i32>(1)?,    // e.g., quantity
        ))
    })?;

    for item in rows {
        println!("{:?}", item?);
    }
    Ok(())
}

fn main() -> Result<()> {
    read_items()?;
    Ok(())
}
