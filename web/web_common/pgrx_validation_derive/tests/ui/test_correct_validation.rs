//! Submodule for testing compile success of the `validation` attribute on functions which
//! return the correct return type.
use pgrx_validation_derive::validation;

#[validation]
pub fn must_not_be_another_empty(_arg: &str) -> Result<(), validation_errors::Error> {
    Ok(())
}

#[validation]
/// A simple validation function which returns the correct error type should pass.
pub fn must_not_be_empty(arg: &str) -> Result<(), validation_errors::Error> {
    must_not_be_another_empty(arg)?;
    Ok(())
}

/// The main for this test.
fn main() {}
