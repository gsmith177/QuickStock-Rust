use quickstock_rust::inventory::InventoryItem;

#[tokio::test]
async fn test_inventory_item_serialization() {
    let item = InventoryItem {
        id: 1,
        name: "Keyboard".to_string(),
        quantity: 10,
    };

    let json = serde_json::to_string(&item).unwrap();
    let deserialized: InventoryItem = serde_json::from_str(&json).unwrap();

    assert_eq!(item.id, deserialized.id);
    assert_eq!(item.name, deserialized.name);
    assert_eq!(item.quantity, deserialized.quantity);
}
