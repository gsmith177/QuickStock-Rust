# init_user_db.py

import sqlite3

# Connect or create user.db in the current directory
conn = sqlite3.connect("user.db")
cursor = conn.cursor()

# 1. Create the users table if it doesn't exist
cursor.execute("""
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);
""")

# 2. Insert default 'admin' user with password "password"
#    The following bcrypt hash corresponds to "password"
pw_hash = "$2b$12$Q/..8/a8ynQUeqgnIWvuAOwyCKG/oH.ht6bXnaAa00oNs2DVqQiB."
cursor.execute(
    "INSERT OR IGNORE INTO users (username, password_hash) VALUES (?, ?);",
    ("admin", pw_hash)
)

conn.commit()
conn.close()

print("✅ user.db initialized: table created and 'admin' user added (password='password')")
