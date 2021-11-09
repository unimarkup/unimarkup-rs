use crate::frontend::{parser::CursorPos, syntax_error::UmSyntaxError};
use crate::middleend::middleend_error::UmMiddleendError;
use rusqlite::{params, ToSql, Transaction};
use serde_bytes::ByteBuf;

#[derive(Debug)]
pub struct ContentIrLine {
    pub id: String,
    pub um_type: String,
    pub line_nr: usize,
    pub text: String,
    pub fallback_text: String,
    pub attributes: String,
    pub fallback_attributes: String,
}

impl Default for ContentIrLine {
    fn default() -> Self {
        ContentIrLine {
            id: String::from("0"),
            um_type: String::default(),
            line_nr: 0,
            text: String::default(),
            fallback_text: String::default(),
            attributes: String::default(),
            fallback_attributes: String::default(),
        }
    }
}

impl ContentIrLine {
    pub fn new(
        id: impl Into<String>,
        um_type: impl Into<String>,
        line_nr: usize,
        text: impl Into<String>,
        fallback_text: impl Into<String>,
        attributes: impl Into<String>,
        fallback_attributes: impl Into<String>,
    ) -> Self {
        ContentIrLine {
            id: id.into(),
            um_type: um_type.into(),
            line_nr,
            text: text.into(),
            fallback_text: fallback_text.into(),
            attributes: attributes.into(),
            fallback_attributes: fallback_attributes.into(),
        }
    }
}

#[derive(Debug)]
pub struct VariableIrLine {
    pub name: String,
    pub um_type: String,
    pub value: String,
    pub fallback_value: String,
}

impl Default for VariableIrLine {
    fn default() -> Self {
        VariableIrLine {
            name: String::default(),
            um_type: String::default(),
            value: String::default(),
            fallback_value: String::default(),
        }
    }
}

impl VariableIrLine {
    pub fn new(
        name: impl Into<String>,
        um_type: impl Into<String>,
        value: impl Into<String>,
        fallback_value: impl Into<String>,
    ) -> Self {
        VariableIrLine {
            name: name.into(),
            um_type: um_type.into(),
            value: value.into(),
            fallback_value: fallback_value.into(),
        }
    }
}

#[derive(Debug)]
pub struct MacroIrLine {
    pub name: String,
    pub um_type: String,
    pub parameters: String,
    pub body: String,
    pub fallback_body: String,
}

impl Default for MacroIrLine {
    fn default() -> Self {
        MacroIrLine {
            name: String::default(),
            um_type: String::default(),
            parameters: String::default(),
            body: String::default(),
            fallback_body: String::default(),
        }
    }
}

impl MacroIrLine {
    pub fn new(
        name: impl Into<String>,
        um_type: impl Into<String>,
        parameters: impl Into<String>,
        body: impl Into<String>,
        fallback_body: impl Into<String>,
    ) -> Self {
        MacroIrLine {
            name: name.into(),
            um_type: um_type.into(),
            parameters: parameters.into(),
            body: body.into(),
            fallback_body: fallback_body.into(),
        }
    }
}

#[derive(Debug)]
pub struct MetadataIrLine {
    pub filename: String,
    pub filehash: ByteBuf,
    pub path: String,
    pub preamble: String,
    pub fallback_preamble: String,
    pub root: bool,
}

impl Default for MetadataIrLine {
    fn default() -> Self {
        MetadataIrLine {
            filename: String::default(),
            filehash: ByteBuf::new(),
            path: String::default(),
            preamble: String::default(),
            fallback_preamble: String::default(),
            root: false,
        }
    }
}

impl MetadataIrLine {
    pub fn new(
        filename: impl Into<String>,
        filehash: ByteBuf,
        path: impl Into<String>,
        preamble: impl Into<String>,
        fallback_preamble: impl Into<String>,
        root: bool,
    ) -> Self {
        MetadataIrLine {
            filename: filename.into(),
            filehash,
            path: path.into(),
            preamble: preamble.into(),
            fallback_preamble: fallback_preamble.into(),
            root,
        }
    }
}

#[derive(Debug)]
pub struct ResourceIrLine {
    pub filename: String,
    pub path: String,
}

impl Default for ResourceIrLine {
    fn default() -> Self {
        ResourceIrLine {
            filename: String::default(),
            path: String::default(),
        }
    }
}

impl ResourceIrLine {
    pub fn new(filename: impl Into<String>, path: impl Into<String>) -> Self {
        ResourceIrLine {
            filename: filename.into(),
            path: path.into(),
        }
    }
}

#[derive(Debug)]
pub struct IrBlock {
    content_lines: Vec<ContentIrLine>,
    variable_lines: Vec<VariableIrLine>,
    macro_lines: Vec<MacroIrLine>,
    metadata_lines: Vec<MetadataIrLine>,
    resource_lines: Vec<ResourceIrLine>,
}

impl IrBlock {
    pub fn new() -> Self {
        IrBlock {
            // Vec::new() does not allocate on heap.
            // First allocation occurs when first element is pushed into the Vec
            content_lines: Vec::new(),
            variable_lines: Vec::new(),
            macro_lines: Vec::new(),
            metadata_lines: Vec::new(),
            resource_lines: Vec::new(),
        }
    }

    pub fn push_content_line(&mut self, line: ContentIrLine) {
        self.content_lines.push(line);
    }

    pub fn append_content_lines(&mut self, lines: &mut Vec<ContentIrLine>) {
        self.content_lines.append(lines);
    }

    pub fn get_content_lines(&self) -> &Vec<ContentIrLine> {
        &self.content_lines
    }

    pub fn push_variable_line(&mut self, line: VariableIrLine) {
        self.variable_lines.push(line);
    }

    pub fn append_variable_lines(&mut self, lines: &mut Vec<VariableIrLine>) {
        self.variable_lines.append(lines);
    }

    pub fn get_variable_lines(&self) -> &Vec<VariableIrLine> {
        &self.variable_lines
    }

    pub fn push_macro_line(&mut self, line: MacroIrLine) {
        self.macro_lines.push(line);
    }

    pub fn append_macro_lines(&mut self, lines: &mut Vec<MacroIrLine>) {
        self.macro_lines.append(lines);
    }

    pub fn get_macro_lines(&self) -> &Vec<MacroIrLine> {
        &self.macro_lines
    }

    pub fn push_metadata_line(&mut self, line: MetadataIrLine) {
        self.metadata_lines.push(line);
    }

    pub fn append_metadata_lines(&mut self, lines: &mut Vec<MetadataIrLine>) {
        self.metadata_lines.append(lines);
    }

    pub fn get_metadata_lines(&self) -> &Vec<MetadataIrLine> {
        &self.metadata_lines
    }

    pub fn push_resource_line(&mut self, line: ResourceIrLine) {
        self.resource_lines.push(line);
    }

    pub fn append_resource_lines(&mut self, lines: &mut Vec<ResourceIrLine>) {
        self.resource_lines.append(lines);
    }

    pub fn get_resource_lines(&self) -> &Vec<ResourceIrLine> {
        &self.resource_lines
    }
}

impl Default for IrBlock {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ParseForIr {
    fn parse_for_ir(
        content: &[&str],
        cursor_pos: &CursorPos,
    ) -> Result<(IrBlock, CursorPos), UmSyntaxError>;

    fn generate_ir_lines(&self, line_nr: usize) -> Vec<ContentIrLine>;
}

pub trait WriteToIr {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError>;
}

fn write_ir_lines(
    ir_lines: &[impl WriteToIr],
    ir_transaction: &Transaction,
) -> Result<(), UmMiddleendError> {
    for ir_line in ir_lines {
        let res = ir_line.write_to_ir(ir_transaction);
        if res.is_err() {
            return Err(res.err().unwrap());
        }
    }
    Ok(())
}

fn entry_already_exists(
    ir_transaction: &Transaction,
    sql_table: &str,
    sql_condition: &str,
    params: &[&dyn ToSql],
) -> bool {
    let sql = format!(
        "SELECT count(*) FROM {} WHERE {} VALUES (?)",
        sql_table, sql_condition
    );
    let res: Result<i64, rusqlite::Error> =
        ir_transaction.query_row(&sql, params, |row| row.get(0));
    if res.is_ok() {
        return true;
    }
    false
}

fn insert_ir_line_execute(
    ir_transaction: &Transaction,
    sql_table: &str,
    params: &[&dyn ToSql],
    column: &str,
) -> Result<(), UmMiddleendError> {
    let sql = format!("INSERT INTO {} VALUES (?)", sql_table);

    if ir_transaction.execute(&sql, params).is_err() {
        return Err(UmMiddleendError {
            tablename: sql_table.to_string(),
            column: column.to_string(),
            message: "Could not insert values on given database connection".to_string(),
        });
    }
    Ok(())
}

fn update_ir_line_execute(
    ir_transaction: &Transaction,
    sql_table: &str,
    sql_set: &str,
    sql_condition: &str,
    params: &[&dyn ToSql],
    column: &str,
) -> Result<(), UmMiddleendError> {
    let sql = format!(
        "UPDATE {} SET {} WHERE {}",
        sql_table, sql_set, sql_condition
    );

    if ir_transaction.execute(&sql, params).is_err() {
        return Err(UmMiddleendError {
            tablename: sql_table.to_string(),
            column: column.to_string(),
            message: "Could not update values on given database connection".to_string(),
        });
    }
    Ok(())
}

impl WriteToIr for IrBlock {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        write_ir_lines(self.get_content_lines(), ir_transaction)?;
        write_ir_lines(self.get_macro_lines(), ir_transaction)?;
        write_ir_lines(self.get_variable_lines(), ir_transaction)?;
        write_ir_lines(self.get_metadata_lines(), ir_transaction)?;
        write_ir_lines(self.get_resource_lines(), ir_transaction)?;

        Ok(())
    }
}

impl WriteToIr for ContentIrLine {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        let sql_table = "content";
        let column_pk = format!("id: {} at line: {}", self.id, self.line_nr);
        let new_values = params![
            self.id,
            self.line_nr,
            self.um_type,
            self.text,
            self.fallback_text,
            self.attributes,
            self.fallback_attributes,
        ];

        let sql_exists_condition = "id = ?1 AND line_nr = ?2";
        let exists_params = params![self.id, self.line_nr];

        if entry_already_exists(
            ir_transaction,
            sql_table,
            sql_exists_condition,
            exists_params,
        ) {
            // TODO: set warning that values are overwritten
            let sql_condition = "id = ?1 AND line_nr = ?2";
            let sql_set = "um_type = ?3, text = ?4, fallback_text = ?5, attributes = ?6, fallback_attributes = ?7";
            update_ir_line_execute(
                ir_transaction,
                sql_table,
                sql_set,
                sql_condition,
                new_values,
                &column_pk,
            )
        } else {
            insert_ir_line_execute(ir_transaction, sql_table, new_values, &column_pk)
        }
    }
}

impl WriteToIr for MacroIrLine {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        let sql_table = "macros";
        let column_pk = format!("name: {} with parameters: {}", self.name, self.parameters);
        let new_values = params![
            self.name,
            self.parameters,
            self.um_type,
            self.body,
            self.fallback_body,
        ];

        let sql_exists_condition = "name = ?1 AND parameters = ?2";
        let exists_params = params![self.name, self.parameters];

        if entry_already_exists(
            ir_transaction,
            sql_table,
            sql_exists_condition,
            exists_params,
        ) {
            // TODO: set warning that values are overwritten
            let sql_condition = "name = ?1 AND parameters = ?2";
            let sql_set = "um_type = ?3, body = ?4, fallback_body = ?5";
            update_ir_line_execute(
                ir_transaction,
                sql_table,
                sql_set,
                sql_condition,
                new_values,
                &column_pk,
            )
        } else {
            insert_ir_line_execute(ir_transaction, sql_table, new_values, &column_pk)
        }
    }
}

impl WriteToIr for VariableIrLine {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        let sql_table = "variables";
        let column_pk = format!("name: {}", self.name);
        let new_values = params![self.name, self.um_type, self.value, self.fallback_value,];

        let sql_exists_condition = "name = ?1";
        let exists_params = params![self.name];

        if entry_already_exists(
            ir_transaction,
            sql_table,
            sql_exists_condition,
            exists_params,
        ) {
            // TODO: set warning that values are overwritten
            let sql_condition = "name = ?1";
            let sql_set = "um_type = ?2, value = ?3, fallback_value = ?4";
            update_ir_line_execute(
                ir_transaction,
                sql_table,
                sql_set,
                sql_condition,
                new_values,
                &column_pk,
            )
        } else {
            insert_ir_line_execute(ir_transaction, sql_table, new_values, &column_pk)
        }
    }
}

impl WriteToIr for MetadataIrLine {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        let sql_table = "metadata";
        let column_pk = format!(
            "filename: {} with hash: {}",
            self.filename,
            String::from_utf8(self.filehash.to_vec()).unwrap()
        );
        let new_values = params![
            self.filehash.to_vec(),
            self.filename,
            self.path,
            self.preamble,
            self.fallback_preamble,
            self.root,
        ];

        let sql_exists_condition = "filehash = ?1";
        let exists_params = params![self.filehash.to_vec()];

        if entry_already_exists(
            ir_transaction,
            sql_table,
            sql_exists_condition,
            exists_params,
        ) {
            // TODO: set warning that values are overwritten
            let sql_condition = "filehash = ?1";
            let sql_set =
                "filename = ?2, path = ?3, preamble = ?4, fallback_preamble = ?5, root = ?6";
            update_ir_line_execute(
                ir_transaction,
                sql_table,
                sql_set,
                sql_condition,
                new_values,
                &column_pk,
            )
        } else {
            insert_ir_line_execute(ir_transaction, sql_table, new_values, &column_pk)
        }
    }
}

impl WriteToIr for ResourceIrLine {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), UmMiddleendError> {
        let sql_table = "resources";
        let column_pk = format!("filename: {} with path: {}", self.filename, self.path);
        let new_values = params![self.filename, self.path];

        let sql_exists_condition = "filename = ?1 AND path = ?2";

        if entry_already_exists(ir_transaction, sql_table, sql_exists_condition, new_values) {
            // All resources columns are used for private key, no update needed
            return Ok(());
        }
        insert_ir_line_execute(ir_transaction, sql_table, new_values, &column_pk)
    }
}
