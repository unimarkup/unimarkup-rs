use unimarkup_rs::middleend::ir::{get_single_ir_line, WriteToIr};
use unimarkup_rs::middleend::ir_macros::MacroIrLine;
use unimarkup_rs::middleend::ir_setup::{setup_ir, setup_ir_connection};

#[test]
fn test_single_write_retrieve() {
    let res_conn = setup_ir_connection();
    assert!(res_conn.is_ok(), "Cause: {:?}", res_conn.err());
    let mut conn = res_conn.unwrap();

    let setup_res = setup_ir(&conn);
    assert!(setup_res.is_ok(), "Cause: {:?}", setup_res.err());

    let transaction_res = conn.transaction();
    assert!(transaction_res.is_ok(), "Cause: {:?}", transaction_res.err());
    let transaction = transaction_res.unwrap();

    let first_macro = MacroIrLine::new("test", "", "paragraph", "test", "");

    let write_res = first_macro.write_to_ir(&transaction);
    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());

    let retrieved_macro_res = get_single_ir_line::<MacroIrLine>(
        &transaction,
        &format!(
            "name = '{}' AND parameters = '{}'",
            first_macro.name, first_macro.parameters
        ),
    );
    assert!(retrieved_macro_res.is_ok(), "Cause: {:?}", retrieved_macro_res.err());

    let retrieved_first_macro = retrieved_macro_res.unwrap();
    assert_eq!(first_macro, retrieved_first_macro);
}
