//! Submodule defining a struct implementing `RuleFailureInformation` for
//! error reporting.

mod builder;

use std::fmt::Display;

use crate::traits::RuleFailureInformation;

#[derive(Debug)]
/// Struct implementing `RuleFailureInformation` for detailed error
/// reporting.
///
/// # Example
///
/// ```rust
/// use sql_rules::error::RuleErrorInfo;
/// use sql_rules::traits::RuleFailureInformation;
///
/// let error_info: RuleErrorInfo = RuleErrorInfo::builder()
///     .rule("TestRule").unwrap()
///     .object("test_table".to_string()).unwrap()
///     .message("Test message".to_string()).unwrap()
///     .resolution("Fix the issue".to_string()).unwrap()
///     .try_into()
///     .unwrap();
///
/// // Test Display formatting
/// let display = format!("{}", error_info);
/// assert!(display.contains("Rule: TestRule"));
/// assert!(display.contains("Object: test_table"));
/// assert!(display.contains("Message: Test message"));
/// assert!(display.contains("Resolution: Fix the issue"));
///
/// // Test getter methods
/// assert_eq!(error_info.rule(), "TestRule");
/// assert_eq!(error_info.object(), "test_table");
/// assert_eq!(error_info.message(), "Test message");
/// assert_eq!(error_info.resolution(), Some("Fix the issue"));
/// ```
pub struct RuleErrorInfo {
    /// Type of rule which failed.
    rule: &'static str,
    /// DB object which failed the rule.
    object: String,
    /// Error message describing the failure.
    message: String,
    /// What should be done to fix the failure.
    resolution: Option<String>,
}

impl RuleErrorInfo {
    /// Creates a new rule error info builder.
    #[must_use]
    pub fn builder() -> builder::RuleErrorInfoBuilder {
        builder::RuleErrorInfoBuilder::default()
    }
}

impl From<RuleErrorInfo> for Box<dyn RuleFailureInformation> {
    fn from(info: RuleErrorInfo) -> Self {
        Box::new(info)
    }
}

impl Display for RuleErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rule: {}\nObject: {}\nMessage: {}",
            self.rule, self.object, self.message
        )?;
        if let Some(resolution) = &self.resolution {
            write!(f, "\nResolution: {resolution}")?;
        }
        Ok(())
    }
}

impl RuleFailureInformation for RuleErrorInfo {
    fn rule(&self) -> &'static str {
        self.rule
    }

    fn object(&self) -> &str {
        &self.object
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn resolution(&self) -> Option<&str> {
        self.resolution.as_deref()
    }
}
