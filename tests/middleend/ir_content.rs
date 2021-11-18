use unimarkup_rs::middleend::ir_content::{prepare_content_rows, ContentIrLine};
use unimarkup_rs::middleend::{
    entry_already_exists, get_single_ir_line, RetrieveFromIr, WriteToIr,
};

use crate::middleend::ir_test_setup::{get_test_transaction, setup_test_ir};

#[test]
fn test_single_write_retrieve() {
    let first_content = ContentIrLine::new("1", 1, "paragraph", "test", "", "{}", "");
    let mut conn = setup_test_ir();

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_content.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_content_res =
        get_single_ir_line::<ContentIrLine>(&transaction, first_content.get_pk_values());
    let commit_res = transaction.commit();

    assert!(
        retrieved_content_res.is_ok(),
        "Cause: {:?}",
        retrieved_content_res.err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_first_content = retrieved_content_res.unwrap();
    assert_eq!(first_content, retrieved_first_content);
}

#[test]
fn test_entry_exists() {
    let mut conn = setup_test_ir();
    let first_content = ContentIrLine::new("1", 1, "paragraph", "test", "", "{}", "");

    //--- ENTRY NOT IN IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);

    assert!(
        !entry_already_exists(&first_content, &transaction),
        "FAIL: Entry can not be in IR"
    );

    let commit_res = transaction.commit();
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- WRITE TO IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_content.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- ENTRY EXISTS IN IR --------------------------------------------------------
    let transaction = get_test_transaction(&mut conn);

    assert!(
        entry_already_exists(&first_content, &transaction),
        "FAIL: Entry not in IR"
    );

    let commit_res = transaction.commit();
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());
}

#[test]
fn test_write_update() {
    let mut conn = setup_test_ir();

    //--- FIRST: WRITE TO IR --------------------------------------------------------
    let first_content = ContentIrLine::new("1", 1, "paragraph", "test", "", "{}", "");
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_content.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- SECOND: WRITE TO IR -------------------------------------------------------
    let updated_content = ContentIrLine::new(
        &first_content.id,
        first_content.line_nr,
        "paragraph",
        "overwritten test",
        "",
        "{ }",
        "",
    );
    let transaction = get_test_transaction(&mut conn);
    let write_res = updated_content.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- RETRIEVE FROM IR ---------------------------------------------------
    let transaction = get_test_transaction(&mut conn);
    let retrieved_content_res =
        get_single_ir_line::<ContentIrLine>(&transaction, first_content.get_pk_values());
    let commit_res = transaction.commit();

    assert!(
        retrieved_content_res.is_ok(),
        "Cause: {:?}",
        retrieved_content_res.err()
    );
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- COMPARE ------------------------------------------------------------
    let retrieved_updated_content = retrieved_content_res.unwrap();
    assert_eq!(updated_content, retrieved_updated_content);
}

#[test]
fn test_rows_query() {
    let mut conn = setup_test_ir();

    //--- FIRST: WRITE TO IR --------------------------------------------------------
    let first_content = ContentIrLine::new("1", 1, "paragraph", "test", "", "{}", "");
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_content.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- SECOND: WRITE TO IR -------------------------------------------------------
    let second_content = ContentIrLine::new("2", 2, "paragraph", "other paragraph", "", "{ }", "");
    let transaction = get_test_transaction(&mut conn);
    let write_res = second_content.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- QUERY ROWS ---------------------------------------------------
    let stmt_res = prepare_content_rows(&conn, false);
    assert!(stmt_res.is_ok(), "Cause: {:?}", stmt_res.err());

    let mut stmt = stmt_res.unwrap();
    let rows_res = stmt.query([]);
    assert!(rows_res.is_ok(), "Cause: {:?}", rows_res.err());

    //--- COMPARE ------------------------------------------------------------
    let mut rows = rows_res.unwrap();
    if let Some(row) = rows.next().unwrap() {
        assert_eq!(first_content, ContentIrLine::from_ir(row).unwrap());
    }

    if let Some(row) = rows.next().unwrap() {
        assert_eq!(second_content, ContentIrLine::from_ir(row).unwrap());
    }
}

#[test]
fn test_rows_query_ordered() {
    let mut conn = setup_test_ir();

    //--- FIRST: WRITE TO IR --------------------------------------------------------
    // setting first_content.line_nr > second_content.line_nr is important for ordering below!
    let first_content = ContentIrLine::new("1", 10, "paragraph", "test", "", "{}", "");
    let transaction = get_test_transaction(&mut conn);
    let write_res = first_content.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- SECOND: WRITE TO IR -------------------------------------------------------
    let second_content = ContentIrLine::new("2", 2, "paragraph", "other paragraph", "", "{ }", "");
    let transaction = get_test_transaction(&mut conn);
    let write_res = second_content.write_to_ir(&transaction);
    let commit_res = transaction.commit();

    assert!(write_res.is_ok(), "Cause: {:?}", write_res.err());
    assert!(commit_res.is_ok(), "Cause: {:?}", commit_res.err());

    //--- QUERY ROWS ---------------------------------------------------
    let stmt_res = prepare_content_rows(&conn, true);
    assert!(stmt_res.is_ok(), "Cause: {:?}", stmt_res.err());

    let mut stmt = stmt_res.unwrap();
    let rows_res = stmt.query([]);
    assert!(rows_res.is_ok(), "Cause: {:?}", rows_res.err());

    //--- COMPARE ------------------------------------------------------------
    let mut rows = rows_res.unwrap();
    if let Some(row) = rows.next().unwrap() {
        assert_eq!(second_content, ContentIrLine::from_ir(row).unwrap());
    }

    if let Some(row) = rows.next().unwrap() {
        assert_eq!(first_content, ContentIrLine::from_ir(row).unwrap());
    }
}
