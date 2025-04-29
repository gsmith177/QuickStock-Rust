use sqlx::SqlitePool;

/// Wrapper around the inventory DB pool
pub struct InventoryDb(pub SqlitePool);

/// Wrapper around the user DB pool
pub struct UserDb(pub SqlitePool);
