use rusqlite::{Connection, params};

/// This function simulates login verification using a SQLite connection.
/// It returns true if the username exists and the password matches.
fn login(conn: &Connection, username: &str, password: &str) -> bool {
    let mut stmt = conn
        .prepare("SELECT password FROM users WHERE username = ?1")
        .expect("Failed to prepare statement");

    let mut rows = stmt.query(params![username]).expect("Failed to execute query");

    // If a matching row is found, compare the stored password
    if let Some(row) = rows.next().expect("Failed to fetch row") {
        let stored_password: String = row.get(0).expect("Failed to get password");
        stored_password == password
    } else {
        false
    }
}

/// Tests login with valid credentials.
#[test]
fn test_valid_login() {
    let conn = Connection::open_in_memory().expect("Failed to open DB");

    conn.execute(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, username TEXT NOT NULL, password TEXT NOT NULL)",
        [],
    ).expect("Failed to create table");

    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        params!["admin", "password"],
    ).expect("Failed to insert user");

    assert!(login(&conn, "admin", "password"));
}

/// Tests login with a valid username but incorrect password.
#[test]
fn test_invalid_password() {
    let conn = Connection::open_in_memory().expect("Failed to open DB");

    conn.execute(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, username TEXT NOT NULL, password TEXT NOT NULL)",
        [],
    ).expect("Failed to create table");

    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        params!["admin", "password"],
    ).expect("Failed to insert user");

    assert!(!login(&conn, "admin", "wrongpass"));
}

/// Tests login with a username that does not exist in the database.
#[test]
fn test_unknown_user() {
    let conn = Connection::open_in_memory().expect("Failed to open DB");

    conn.execute(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, username TEXT NOT NULL, password TEXT NOT NULL)",
        [],
    ).expect("Failed to create table");

    assert!(!login(&conn, "ghost", "anything"));
}
