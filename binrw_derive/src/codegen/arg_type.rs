use proc_macro2::TokenStream;
use super::TopLevelAttrs;
use crate::CompileError;
use quote::quote;

/// Generate the argument type for the derived impl
pub fn generate(tla: &TopLevelAttrs) -> Result<TokenStream, CompileError> {
    Ok(tla.import.types())
}

