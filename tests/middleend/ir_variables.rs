use rusqlite::params;
use unimarkup_rs::middleend::ir::{get_single_ir_line, WriteToIr};
use unimarkup_rs::middleend::ir_variables::VariableIrLine;

use crate::middleend::ir_test_setup::{get_test_transaction, setup_test_ir};

#[test]
fn test_single_write_retrieve() {
    let first_variable = VariableIrLine::new("test", "paragraph", "test paragraph", "");
    let mut conn = setup_test_ir();

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_variable.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_variable_res = get_single_ir_line::<VariableIrLine>(
        &transaction,
        "name = ?1",
        params![first_variable.name],
    );
    let commit_res = transaction.commit();

    assert!(
        retrieved_variable_res.is_ok(),
        "Cause: {:?}",
        retrieved_variable_res.err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_first_variable = retrieved_variable_res.unwrap();
    assert_eq!(first_variable, retrieved_first_variable);
}
