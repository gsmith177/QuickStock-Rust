# dump_users.py
import sqlite3

conn = sqlite3.connect("user.db")
cursor = conn.cursor()

cursor.execute("SELECT username, password_hash FROM users;")
rows = cursor.fetchall()

print("Users in DB:")
for username, pw_hash in rows:
    print(f"  • {username}  →  {pw_hash}")

conn.close()
