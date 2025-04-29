CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    cost_price REAL NOT NULL,
    sell_price REAL NOT NULL,
    available BOOLEAN NOT NULL,
    date_stocked TEXT NOT NULL,
    contact TEXT NOT NULL,
    quantity_sold INTEGER NOT NULL
);
