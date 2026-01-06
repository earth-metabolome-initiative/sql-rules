//! Submodule providing the builder for `ConstraintErrorInfo`.

use crate::error::ConstraintErrorInfo;

#[derive(Default)]
/// Builder for `ConstraintErrorInfo`.
pub struct ConstraintErrorInfoBuilder {
    constraint: Option<&'static str>,
    object: Option<String>,
    message: Option<String>,
    resolution: Option<String>,
}

impl ConstraintErrorInfoBuilder {
    /// Set the `constraint` attribute.
    pub fn constraint(
        mut self,
        constraint: &'static str,
    ) -> Result<Self, ConstraintErrorInfoBuilderError> {
        if constraint.trim().is_empty() {
            return Err(ConstraintErrorInfoBuilderError::EmptyConstraint);
        }
        self.constraint = Some(constraint);
        Ok(self)
    }

    /// Set the `object` attribute.
    pub fn object(mut self, object: String) -> Result<Self, ConstraintErrorInfoBuilderError> {
        if object.trim().is_empty() {
            return Err(ConstraintErrorInfoBuilderError::EmptyObject);
        }
        self.object = Some(object);
        Ok(self)
    }

    /// Set the `message` attribute.
    pub fn message(mut self, message: String) -> Result<Self, ConstraintErrorInfoBuilderError> {
        if message.trim().is_empty() {
            return Err(ConstraintErrorInfoBuilderError::EmptyMessage);
        }
        self.message = Some(message);
        Ok(self)
    }

    /// Set the `resolution` attribute.
    pub fn resolution(
        mut self,
        resolution: String,
    ) -> Result<Self, ConstraintErrorInfoBuilderError> {
        if resolution.trim().is_empty() {
            return Err(ConstraintErrorInfoBuilderError::EmptyResolution);
        }
        self.resolution = Some(resolution);
        Ok(self)
    }
}

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when building a `ConstraintErrorInfo`.
pub enum ConstraintErrorInfoBuilderError {
    #[error("missing attribute: {0}")]
    MissingAttribute(&'static str),
    #[error("attribute 'constraint' cannot be empty")]
    EmptyConstraint,
    #[error("attribute 'message' cannot be empty")]
    EmptyMessage,
    #[error("attribute 'object' cannot be empty")]
    EmptyObject,
    #[error("attribute 'resolution' cannot be empty")]
    EmptyResolution,
}

impl TryFrom<ConstraintErrorInfoBuilder> for ConstraintErrorInfo {
    type Error = ConstraintErrorInfoBuilderError;

    fn try_from(builder: ConstraintErrorInfoBuilder) -> Result<Self, Self::Error> {
        Ok(ConstraintErrorInfo {
            constraint: builder.constraint.ok_or(
                ConstraintErrorInfoBuilderError::MissingAttribute("constraint"),
            )?,
            object: builder
                .object
                .ok_or(ConstraintErrorInfoBuilderError::MissingAttribute("object"))?,
            message: builder
                .message
                .ok_or(ConstraintErrorInfoBuilderError::MissingAttribute("message"))?,
            resolution: builder.resolution,
        })
    }
}
