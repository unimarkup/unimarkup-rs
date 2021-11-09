use crate::middleend::middleend_error::UmMiddleendError;
use rusqlite::Connection;

pub fn setup_ir_connection() -> Result<Connection, UmMiddleendError> {
    let connection = Connection::open_in_memory();
    if connection.is_err() {
        return Err(UmMiddleendError {
            tablename: "-".to_string(),
            column: "-".to_string(),
            message: "Could not create a database connection".to_string(),
        });
    }
    Ok(connection.unwrap())
}

pub fn setup_ir(ir_connection: &Connection) -> Result<(), UmMiddleendError> {
    let sql_setup_content = r#"CREATE TABLE IF NOT EXISTS "content" (
			"id"	TEXT NOT NULL,
			"line_nr"	INTEGER NOT NULL,
			"um_type"	TEXT NOT NULL,
			"text"	TEXT,
			"fallback_text"	TEXT,
			"attributes"	TEXT,
			"fallback_attributes"	TEXT,
			PRIMARY KEY("id","line_nr")
		);"#;

    let sql_setup_macros = r#"CREATE TABLE "macros" (
			"name"	TEXT NOT NULL,
			"parameters"	BLOB NOT NULL,
			"um_type"	TEXT NOT NULL,
			"body"	TEXT,
			"fallback_body"	TEXT,
			PRIMARY KEY("name","parameters")
		);"#;

    let sql_setup_variables = r#"CREATE TABLE "variables" (
			"name"	TEXT NOT NULL,
			"um_type"	TEXT NOT NULL,
			"value"	TEXT,
			"fallback_value"	TEXT,
			PRIMARY KEY("name")
		);"#;

    let sql_setup_metadata = r#"CREATE TABLE "metadata" (
			"filehash"	BLOB NOT NULL,
			"filename"	TEXT NOT NULL,
			"path"	TEXT NOT NULL,
			"preamble"	TEXT,
			"fallback_preamble"	TEXT,
			"root"	INTEGER,
			PRIMARY KEY("filehash")
		);"#;

    let sql_setup_resources = r#"CREATE TABLE "resources" (
			"filename"	TEXT NOT NULL,
			"path"	TEXT NOT NULL,
			PRIMARY KEY("path","filename")
		);"#;

    let sql = format!(
        "{}{}{}{}{}",
        sql_setup_content,
        sql_setup_macros,
        sql_setup_variables,
        sql_setup_metadata,
        sql_setup_resources
    );
    let setup_res = ir_connection.execute_batch(&sql);

    if setup_res.is_err() {
        return Err(UmMiddleendError {
            tablename: "-".to_string(),
            column: "-".to_string(),
            message: "Could not setup tables on given database connection".to_string(),
        });
    }
    Ok(())
}
