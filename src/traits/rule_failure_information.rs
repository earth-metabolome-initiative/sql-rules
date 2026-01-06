//! Submodule providing the `RuleFailureInformation` trait for error
//! reporting.

use std::fmt::{Debug, Display};

/// Trait for types that provide information about a rule failure.
pub trait RuleFailureInformation: Display + Debug {
    /// Type of rule which failed.
    fn rule(&self) -> &'static str;

    /// DB object which failed the rule.
    fn object(&self) -> &str;

    /// Error message describing the failure.
    fn message(&self) -> &str;

    /// What should be done to fix the failure.
    fn resolution(&self) -> Option<&str>;
}
