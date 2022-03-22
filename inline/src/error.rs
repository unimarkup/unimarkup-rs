
/// Error enum for possible inline errors
/// 
/// Note: Temporary solution until log_id is separated from core
#[derive(Debug)]
pub enum InlineError{
  /// Set if either text group, uri or attribute block is not closed properly
  ClosingViolation,
}

