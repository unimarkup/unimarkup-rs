use rusqlite::params;
use unimarkup_rs::middleend::ir::{get_single_ir_line, WriteToIr};
use unimarkup_rs::middleend::ir_resources::ResourceIrLine;
use unimarkup_rs::middleend::ir_setup::{setup_ir, setup_ir_connection};

#[test]
fn test_single_write_retrieve() {
    let res_conn = setup_ir_connection();
    assert!(res_conn.is_ok(), "Cause: {:?}", res_conn.err());
    let mut conn = res_conn.unwrap();

    let setup_res = setup_ir(&conn);
    assert!(setup_res.is_ok(), "Cause: {:?}", setup_res.err());

    let transaction_res = conn.transaction();
    assert!(
        transaction_res.is_ok(),
        "Cause: {:?}",
        transaction_res.err()
    );
    let transaction = transaction_res.unwrap();

    let first_resources = ResourceIrLine::new("test.png", ".");

    let write_res = first_resources.write_to_ir(&transaction);
    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());

    let retrieved_resources_res = get_single_ir_line::<ResourceIrLine>(
        &transaction,
        "filename = ?1 AND path = ?2",
        params![first_resources.filename, first_resources.path],
    );
    assert!(
        retrieved_resources_res.is_ok(),
        "Cause: {:?}",
        retrieved_resources_res.err()
    );

    let retrieved_first_resources = retrieved_resources_res.unwrap();
    assert_eq!(first_resources, retrieved_first_resources);
}
