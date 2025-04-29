use crate::inventory::InventoryItem;
use sqlx::SqlitePool;

pub async fn get_inventory(pool: &SqlitePool) -> Result<Vec<InventoryItem>, sqlx::Error> {
    let items = sqlx::query_as::<_, InventoryItem>("SELECT * FROM inventory")
        .fetch_all(pool)
        .await?;
    Ok(items)
}
