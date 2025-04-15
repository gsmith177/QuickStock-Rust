use quickstock_rust::sales::SaleRecord;

#[test]
fn test_sale_record_creation() {
    let sale = SaleRecord {
        id: 1,
        item_id: 101,
        quantity_sold: 2,
        date: "2025-04-15".to_string(),
    };

    assert_eq!(sale.item_id, 101);
}
