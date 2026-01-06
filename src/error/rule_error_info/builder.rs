//! Submodule providing the builder for `RuleErrorInfo`.

use crate::error::RuleErrorInfo;

#[derive(Default)]
/// Builder for `RuleErrorInfo`.
pub struct RuleErrorInfoBuilder {
    rule: Option<&'static str>,
    object: Option<String>,
    message: Option<String>,
    resolution: Option<String>,
}

impl RuleErrorInfoBuilder {
    /// Set the `rule` attribute.
    pub fn rule(mut self, rule: &'static str) -> Result<Self, RuleErrorInfoBuilderError> {
        if rule.trim().is_empty() {
            return Err(RuleErrorInfoBuilderError::EmptyRule);
        }
        self.rule = Some(rule);
        Ok(self)
    }

    /// Set the `object` attribute.
    pub fn object(mut self, object: String) -> Result<Self, RuleErrorInfoBuilderError> {
        if object.trim().is_empty() {
            return Err(RuleErrorInfoBuilderError::EmptyObject);
        }
        self.object = Some(object);
        Ok(self)
    }

    /// Set the `message` attribute.
    pub fn message(mut self, message: String) -> Result<Self, RuleErrorInfoBuilderError> {
        if message.trim().is_empty() {
            return Err(RuleErrorInfoBuilderError::EmptyMessage);
        }
        self.message = Some(message);
        Ok(self)
    }

    /// Set the `resolution` attribute.
    pub fn resolution(mut self, resolution: String) -> Result<Self, RuleErrorInfoBuilderError> {
        if resolution.trim().is_empty() {
            return Err(RuleErrorInfoBuilderError::EmptyResolution);
        }
        self.resolution = Some(resolution);
        Ok(self)
    }
}

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when building a `RuleErrorInfo`.
pub enum RuleErrorInfoBuilderError {
    #[error("missing attribute: {0}")]
    MissingAttribute(&'static str),
    #[error("attribute 'rule' cannot be empty")]
    EmptyRule,
    #[error("attribute 'message' cannot be empty")]
    EmptyMessage,
    #[error("attribute 'object' cannot be empty")]
    EmptyObject,
    #[error("attribute 'resolution' cannot be empty")]
    EmptyResolution,
}

impl TryFrom<RuleErrorInfoBuilder> for RuleErrorInfo {
    type Error = RuleErrorInfoBuilderError;

    fn try_from(builder: RuleErrorInfoBuilder) -> Result<Self, Self::Error> {
        Ok(RuleErrorInfo {
            rule: builder
                .rule
                .ok_or(RuleErrorInfoBuilderError::MissingAttribute("rule"))?,
            object: builder
                .object
                .ok_or(RuleErrorInfoBuilderError::MissingAttribute("object"))?,
            message: builder
                .message
                .ok_or(RuleErrorInfoBuilderError::MissingAttribute("message"))?,
            resolution: builder.resolution,
        })
    }
}
