#[derive(Debug)]
pub struct IrError {
    pub tablename: String,
    pub column: String,
    pub message: String,
}
