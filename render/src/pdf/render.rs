use std::io::Write;

use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptions};
use tempfile::Builder;

use crate::log_id::RenderError;

pub fn render_pdf(html: &str) -> Result<Vec<u8>, RenderError> {
    let mut temp_html_file = Builder::new()
        .suffix(".html")
        .tempfile()
        .map_err(|_| RenderError::Unimplemented)?;

    temp_html_file
        .write_all(html.as_bytes())
        .map_err(|_| RenderError::Unimplemented)?;
    let temp_file_url = format!(
        "file://{}",
        temp_html_file
            .as_ref()
            .as_os_str()
            .to_str()
            .ok_or(RenderError::Unimplemented)?
    );

    let browser = Browser::new(LaunchOptions::default()).expect("Error");
    let pdf_bytes = browser
        .new_tab()
        .map_err(|_| RenderError::Unimplemented)?
        .navigate_to(temp_file_url.as_str())
        .map_err(|_| RenderError::Unimplemented)?
        .wait_until_navigated()
        .map_err(|_| RenderError::Unimplemented)?
        .print_to_pdf(Some(PrintToPdfOptions::default()))
        .map_err(|_| RenderError::Unimplemented)?;

    Ok(pdf_bytes)
}
