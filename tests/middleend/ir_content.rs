use rusqlite::params;
use unimarkup_rs::middleend::ir::{get_single_ir_line, WriteToIr};
use unimarkup_rs::middleend::ir_content::ContentIrLine;
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

    let first_content = ContentIrLine::new("1", 1, "paragraph", "test", "", "{}", "");

    let write_res = first_content.write_to_ir(&transaction);
    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());

    let retrieved_content_res = get_single_ir_line::<ContentIrLine>(
        &transaction,
        "id = ?1 AND line_nr = ?2",
        params![first_content.id, first_content.line_nr],
    );
    assert!(
        retrieved_content_res.is_ok(),
        "Cause: {:?}",
        retrieved_content_res.err()
    );

    let retrieved_first_content = retrieved_content_res.unwrap();
    assert_eq!(first_content, retrieved_first_content);
}
