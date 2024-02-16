// TODO: Have different error types like runtime, parsing etc.

/// Custom runtime error for our interpreter
///
/// # Examples
///
/// ```
/// use rustigo::errors::RuntimeError;
///
/// fn execute() -> Result<String, RuntimeError> {
///   let reason = String::from("Error occured while executing");
///
///   // return the error
///   Err(RuntimeError::Why(reason))
/// }
/// ```
pub enum RuntimeError {
  /// The reason why the error occurred
  Why(String),
}
