use rusqlite::Connection;
use rusqlite::Error::QueryReturnedNoRows;
use unimarkup_core::middleend::{setup_ir, setup_ir_connection};

fn table_exists(conn: &Connection, table: &str, columns: &str) -> bool {
    let sql = format!("SELECT {} FROM {}", columns, table);
    let res = conn.query_row(&sql, [], |_| Ok(()));
    if res.is_ok() {
        return true;
    }

    let err = res.err().unwrap();
    match err {
        QueryReturnedNoRows => true,
        _ => {
            println!("ERROR: In table {}. Reason: {}", table, err);
            false
        }
    }
}

#[test]
fn test_ir_setup() {
    let res_conn = setup_ir_connection();
    assert!(res_conn.is_ok(), "Cause: {:?}", res_conn.err());
    let conn = res_conn.unwrap();

    let setup_res = setup_ir(&conn);
    assert!(setup_res.is_ok(), "Cause: {:?}", setup_res.err());

    let sql_content_columns =
        "id, line_nr, um_type, text, fallback_text, attributes, fallback_attributes";
    assert!(table_exists(&conn, "content", sql_content_columns));

    let sql_macros_columns = "name, parameters, um_type, body, fallback_body";
    assert!(table_exists(&conn, "macros", sql_macros_columns));

    let sql_variables_columns = "name, um_type, value, fallback_value";
    assert!(table_exists(&conn, "variables", sql_variables_columns));

    let sql_metadata_columns = "filehash, filename, path, preamble, fallback_preamble, root";
    assert!(table_exists(&conn, "metadata", sql_metadata_columns));

    let sql_resources_columns = "filename, path";
    assert!(table_exists(&conn, "resources", sql_resources_columns));

    let res_close = conn.close();
    assert!(res_close.is_ok(), "Cause: {:?}", res_close.err());
}