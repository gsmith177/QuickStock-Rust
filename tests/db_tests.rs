use rusqlite::{Connection, params};

/// This test verifies that a user can be inserted and then queried from an in-memory database.
#[test]
fn test_user_db_insert_and_query() {
    // Use an in-memory SQLite database to avoid file I/O
    let conn = Connection::open_in_memory().expect("Failed to open in-memory DB");

    // Create a temporary `users` table
    conn.execute(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, username TEXT NOT NULL, password TEXT NOT NULL)",
        [],
    ).expect("Failed to create table");

    // Insert a mock user
    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        params!["test_user", "test_pass"],
    ).expect("Failed to insert user");

    // Query the user by username
    let mut stmt = conn.prepare("SELECT username, password FROM users WHERE username = ?1")
        .expect("Failed to prepare query");

    let mut rows = stmt.query(params!["test_user"]).expect("Query failed");

    // Check that the retrieved data matches what was inserted
    if let Some(row) = rows.next().expect("Row fetch failed") {
        let username: String = row.get(0).expect("Username fetch failed");
        let password: String = row.get(1).expect("Password fetch failed");
        assert_eq!(username, "test_user");
        assert_eq!(password, "test_pass");
    } else {
        panic!("No user found");
    }
}
