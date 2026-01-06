//! Submodule defining the `ColumnRule` trait, which defines a rule
//! which applies to an object that implements the `ConstrainableColumn` trait.

use sql_traits::traits::ColumnLike;

use crate::error::Error;

/// Trait for types that define a column rule object.
pub trait ColumnRule {
    /// The column type that this rule applies to.
    type Column: ColumnLike;

    /// Validates that the given column satisfies the rule.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to query additional
    ///   column information from.
    /// * `column` - The column to validate.
    ///
    /// # Errors
    ///
    /// Returns an error if the column violates this rule.
    fn validate_column(
        &self,
        database: &<Self::Column as ColumnLike>::DB,
        column: &Self::Column,
    ) -> Result<(), Error>;
}
