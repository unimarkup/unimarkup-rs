use unimarkup_rs::middleend::ir::{get_single_ir_line, RetrieveFromIr, WriteToIr};
use unimarkup_rs::middleend::ir_metadata::MetadataIrLine;

use crate::middleend::ir_test_setup::{get_test_transaction, setup_test_ir};

#[test]
fn test_single_write_retrieve() {
    let first_metadata =
        MetadataIrLine::new(b"ccdec233ff78".to_vec(), "test.um", ".", "{}", "", true);
    let mut conn = setup_test_ir();

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_metadata.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_metadata_res =
        get_single_ir_line::<MetadataIrLine>(&transaction, first_metadata.get_pk_values());
    let commit_res = transaction.commit();

    assert!(
        retrieved_metadata_res.is_ok(),
        "Cause: {:?}",
        retrieved_metadata_res.err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_first_metadata = retrieved_metadata_res.unwrap();
    assert_eq!(first_metadata, retrieved_first_metadata);
}
