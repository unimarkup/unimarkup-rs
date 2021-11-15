use unimarkup_rs::middleend::ir::{
    entry_already_exists, get_single_ir_line, RetrieveFromIr, WriteToIr,
};
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

#[test]
fn test_entry_exists() {
    let mut conn = setup_test_ir();
    let first_resource = ResourceIrLine::new("test.um", ".");

    //--- ENTRY NOT IN IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);

    assert!(
        !entry_already_exists(&first_resource, &transaction),
        "FAIL: Entry can not be in IR"
    );

    let commit_res = transaction.commit();
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_resource.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- ENTRY EXISTS IN IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);

    assert!(
        entry_already_exists(&first_resource, &transaction),
        "FAIL: Entry not in IR"
    );

    let commit_res = transaction.commit();
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());
}

#[test]
fn test_write_update() {
    let mut conn = setup_test_ir();

    //--- FIRST: WRITE TO IR --------------------------------------------------------
    let first_resource = ResourceIrLine::new("test.um", ".");
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_resource.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- SECOND: WRITE TO IR -------------------------------------------------------
    let updated_resource = ResourceIrLine::new(&first_resource.filename, &first_resource.path); // resources only has pk columns
    let transaction = get_test_transaction(&mut conn);
    let write_res = updated_resource.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_resource_res =
        get_single_ir_line::<ResourceIrLine>(&transaction, first_resource.get_pk_values());
    let commit_res = transaction.commit();

    assert!(
        retrieved_resource_res.is_ok(),
        "Cause: {:?}",
        retrieved_resource_res.err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_updated_resource = retrieved_resource_res.unwrap();
    assert_eq!(updated_resource, retrieved_updated_resource);
}
