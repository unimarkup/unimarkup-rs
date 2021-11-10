use rusqlite::params;
use unimarkup_rs::middleend::ir::{get_single_ir_line, WriteToIr};
use unimarkup_rs::middleend::ir_macros::MacroIrLine;

use crate::middleend::ir_test_setup::{setup_test_ir, get_test_transaction};

#[test]
fn test_single_write_retrieve() {
    let first_macro = MacroIrLine::new(
        "test",
        "{%{ list }val1 %{ paragraph } val2}",
        "paragraph",
        "test",
        "",
    );
    let mut conn = setup_test_ir();

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_macro.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_macro_res = get_single_ir_line::<MacroIrLine>(
        &transaction,
        "name = ?1 AND parameters = ?2",
        params![first_macro.name, first_macro.parameters],
    );
    let commit_res = transaction.commit();

    assert!(
        retrieved_macro_res.is_ok(),
        "Cause: {:?}",
        retrieved_macro_res.err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_first_macro = retrieved_macro_res.unwrap();
    assert_eq!(first_macro, retrieved_first_macro);
}
