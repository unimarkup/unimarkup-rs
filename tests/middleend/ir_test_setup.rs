use rusqlite::{Connection, Transaction};
use unimarkup_rs::middleend::{setup_ir, setup_ir_connection};

pub fn setup_test_ir() -> Connection {
    let res_conn = setup_ir_connection();
    assert!(res_conn.is_ok(), "Cause: {:?}", res_conn.err());
    let conn = res_conn.unwrap();

    let setup_res = setup_ir(&conn);
    assert!(setup_res.is_ok(), "Cause: {:?}", setup_res.err());

    conn
}

pub fn get_test_transaction(conn: &mut Connection) -> Transaction<'_> {
    let transaction_res = conn.transaction();
    assert!(
        transaction_res.is_ok(),
        "Cause: {:?}",
        transaction_res.err()
    );
    transaction_res.unwrap()
}
