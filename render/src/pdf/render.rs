use std::io::Write;

use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptions};
use tempfile::Builder;

use crate::log_id::RenderError;
use crate::log_id::RenderError::UnexpectedPdfError;

pub fn render_pdf(html: &str) -> Result<Vec<u8>, RenderError> {
    let mut temp_html_file = Builder::new()
        .suffix(".html")
        .tempfile()
        .map_err(|err| UnexpectedPdfError(err.to_string()))?;

    temp_html_file
        .write_all(html.as_bytes())
        .map_err(|err| UnexpectedPdfError(err.to_string()))?;
    let temp_file_url = format!(
        "file://{}",
        temp_html_file
            .as_ref()
            .as_os_str()
            .to_str()
            .ok_or(RenderError::Unimplemented)?
    );

    let browser = Browser::new(LaunchOptions::default())
        .map_err(|err| UnexpectedPdfError(err.to_string()))?;
    let pdf_bytes = browser
        .new_tab()
        .map_err(|err| UnexpectedPdfError(err.to_string()))?
        .navigate_to(temp_file_url.as_str())
        .map_err(|err| UnexpectedPdfError(err.to_string()))?
        .wait_until_navigated()
        .map_err(|err| UnexpectedPdfError(err.to_string()))?
        .print_to_pdf(Some(PrintToPdfOptions::default()))
        .map_err(|err| UnexpectedPdfError(err.to_string()))?;

    Ok(pdf_bytes)
}
