//! Kande — the Swa compiler.
//!
//! A systems programming language with Kiswahili syntax.
//! Swa replaces C as a machine contract layer.

pub mod abi;
pub mod ast;
pub mod codegen;
pub mod diagnostics;
pub mod driver;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod sema;

// Re-export key types for convenience.
pub use diagnostics::{Diagnostic, DiagnosticBag, Severity, SourceLocation, SourceSpan};
