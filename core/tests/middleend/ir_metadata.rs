use unimarkup_core::middleend::MetadataIrLine;
use unimarkup_core::middleend::{
    entry_already_exists, get_single_ir_line, RetrieveFromIr, WriteToIr,
};

use crate::middleend::ir_test_setup::{get_test_transaction, setup_test_ir};

#[test]
fn test__ir_single_write_retrieve__metadata() {
    let first_metadata =
        MetadataIrLine::new(b"ccdec233ff78".to_vec(), "test.um", ".", "{}", "", true);
    let mut conn = setup_test_ir();

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_metadata.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.unwrap_err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_metadata_res =
        get_single_ir_line::<MetadataIrLine>(&transaction, first_metadata.get_pk_values());
    let commit_res = transaction.commit();

    assert!(
        retrieved_metadata_res.is_ok(),
        "Cause: {:?}",
        retrieved_metadata_res.unwrap_err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_first_metadata = retrieved_metadata_res.unwrap();
    assert_eq!(first_metadata, retrieved_first_metadata);
}

#[test]
fn test__ir_entry_exists__metadata() {
    let mut conn = setup_test_ir();
    let first_metadata =
        MetadataIrLine::new(b"ccdec233ff78".to_vec(), "test.um", ".", "{}", "", true);

    //--- ENTRY NOT IN IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);

    assert!(
        !entry_already_exists(&first_metadata, &transaction),
        "FAIL: Entry can not be in IR"
    );

    let commit_res = transaction.commit();
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_metadata.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.unwrap_err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- ENTRY EXISTS IN IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);

    assert!(
        entry_already_exists(&first_metadata, &transaction),
        "FAIL: Entry not in IR"
    );

    let commit_res = transaction.commit();
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());
}

#[test]
fn test__ir_write_update__metadata() {
    let mut conn = setup_test_ir();

    //--- FIRST: WRITE TO IR --------------------------------------------------------
    let first_metadata =
        MetadataIrLine::new(b"ccdec233ff78".to_vec(), "test.um", ".", "{}", "", true);
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_metadata.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.unwrap_err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- SECOND: WRITE TO IR -------------------------------------------------------
    let updated_metadata = MetadataIrLine::new(
        first_metadata.filehash.clone(),
        "test2.um",
        ".",
        "",
        "",
        false,
    );
    let transaction = get_test_transaction(&mut conn);
    let write_res = updated_metadata.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.unwrap_err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_metadata_res =
        get_single_ir_line::<MetadataIrLine>(&transaction, first_metadata.get_pk_values());
    let commit_res = transaction.commit();

    assert!(
        retrieved_metadata_res.is_ok(),
        "Cause: {:?}",
        retrieved_metadata_res.unwrap_err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.unwrap_err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_updated_metadata = retrieved_metadata_res.unwrap();
    assert_eq!(updated_metadata, retrieved_updated_metadata);
}
