use std::io::Write;

use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptions};
use tempfile::Builder;

use crate::log_id::RenderError;
use crate::log_id::RenderError::UnexpectedPdfError;

/// Returns PrintToPdfOptions following the recommended settings of:
/// https://pagedjs.org/documentation/2-getting-started-with-paged.js/#using-paged.js-as-a-polyfill-in-web-browsers
fn create_pdf_options() -> Option<PrintToPdfOptions> {
    Some(PrintToPdfOptions {
        margin_top: Some(0f64),
        margin_bottom: Some(0f64),
        margin_left: Some(0f64),
        margin_right: Some(0f64),
        header_template: None,
        footer_template: None,
        print_background: Some(false),
        ..PrintToPdfOptions::default()
    })
}

/// Renders the given html-string to a pdf represent as bytes.
/// It first writes the html-string to a temp-directory, because chrome needs a file to load as webpage.
/// Then it prints the rendered html as pdf. The result is returned and not written to disc.
///
/// # Arguments
/// * `html` - The rendered html as string
///
/// # Returns
/// The rendered PDF as bytes.
///
/// # Errors
/// * `UnexpectedPdfError` - in case something goes wrong with the underlying headless-chrome framework.
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
        .print_to_pdf(create_pdf_options())
        .map_err(|err| UnexpectedPdfError(err.to_string()))?;

    Ok(pdf_bytes)
}
