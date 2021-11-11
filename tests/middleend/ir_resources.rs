use unimarkup_rs::middleend::ir::{get_single_ir_line, RetrieveFromIr, WriteToIr};
use unimarkup_rs::middleend::ir_resources::ResourceIrLine;

use crate::middleend::ir_test_setup::{get_test_transaction, setup_test_ir};

#[test]
fn test_single_write_retrieve() {
    let first_resources = ResourceIrLine::new("test.png", ".");
    let mut conn = setup_test_ir();

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_resources.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_resources_res =
        get_single_ir_line::<ResourceIrLine>(&transaction, first_resources.get_pk_values());
    let commit_res = transaction.commit();

    assert!(
        retrieved_resources_res.is_ok(),
        "Cause: {:?}",
        retrieved_resources_res.err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_first_resources = retrieved_resources_res.unwrap();
    assert_eq!(first_resources, retrieved_first_resources);
}
