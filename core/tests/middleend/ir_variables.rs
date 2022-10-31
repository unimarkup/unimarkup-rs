use unimarkup_core::elements::types;
use unimarkup_core::middleend::VariableIrLine;
use unimarkup_core::middleend::{
    entry_already_exists, get_single_ir_line, RetrieveFromIr, WriteToIr,
};

use crate::middleend::ir_test_setup::{get_test_transaction, setup_test_ir};

#[test]
fn test__ir_single_write_retrieve__variable() {
    let first_variable = VariableIrLine::new("test", "paragraph", "test paragraph", "");
    let mut conn = setup_test_ir();

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_variable.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.unwrap_err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_variable_res =
        get_single_ir_line::<VariableIrLine>(&transaction, first_variable.get_pk_values());
    let commit_res = transaction.commit();

    assert!(
        retrieved_variable_res.is_ok(),
        "Cause: {:?}",
        retrieved_variable_res.unwrap_err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_first_variable = retrieved_variable_res.unwrap();
    assert_eq!(first_variable, retrieved_first_variable);
}

#[test]
fn test__ir_entry_exists__variable() {
    let mut conn = setup_test_ir();
    let first_macro = VariableIrLine::new("test", "paragraph", "test paragraph", "");

    //--- ENTRY NOT IN IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);

    assert!(
        !entry_already_exists(&first_macro, &transaction),
        "FAIL: Entry can not be in IR"
    );

    let commit_res = transaction.commit();
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_macro.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.unwrap_err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- ENTRY EXISTS IN IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);

    assert!(
        entry_already_exists(&first_macro, &transaction),
        "FAIL: Entry not in IR"
    );

    let commit_res = transaction.commit();
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());
}

#[test]
fn test__ir_write_update__variable() {
    let mut conn = setup_test_ir();

    //--- FIRST: WRITE TO IR --------------------------------------------------------
    let first_macro = VariableIrLine::new("test", "paragraph", "test paragraph", "");
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_macro.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.unwrap_err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- SECOND: WRITE TO IR -------------------------------------------------------
    let updated_macro = VariableIrLine::new(
        &first_macro.name,
        format!("heading{delim}level{delim}2", delim = types::ELEMENT_TYPE_DELIMITER),
        "head",
        "",
    );
    let transaction = get_test_transaction(&mut conn);
    let write_res = updated_macro.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.unwrap_err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_macro_res =
        get_single_ir_line::<VariableIrLine>(&transaction, first_macro.get_pk_values());
    let commit_res = transaction.commit();

    assert!(
        retrieved_macro_res.is_ok(),
        "Cause: {:?}",
        retrieved_macro_res.unwrap_err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_updated_macro = retrieved_macro_res.unwrap();
    assert_eq!(updated_macro, retrieved_updated_macro);
}
