use crate::um_error::UmError;

pub trait Render {
    fn render_html(&self) -> Result<String, UmError>;
}
