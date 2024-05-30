use rust_sstable::model::sstable::SSTable;

#[test]
fn test_insert() {
    let mut table = SSTable::new();
    // FIXME: This is wrong way to insert
    table.insert(1, 1);
}
