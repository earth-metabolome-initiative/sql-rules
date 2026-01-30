//! Submodule providing the `TextualColumnRule` rule.

use crate::{
    error::RuleErrorInfo,
    traits::{ColumnRule, Constrainer, GenericConstrainer},
};
use sql_traits::traits::IndexLike;
use sql_traits::traits::{CheckConstraintLike, ColumnLike, DatabaseLike, TableLike};

/// Struct defining a rule that enforces constraints on textual columns.
///
/// 1. If a column is textual (method `is_textual` returns true), it must have a check constraint that verifies it is not empty.
/// 2. All textual columns should have also an upper bound check constraint length.
///    - If they appear in an index, they cannot be longer than 255 characters.
///    - If they do not appear in an index, they cannot be longer than 8K characters.
///
/// # Example
///
/// ```rust
/// use sql_rules::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = TextualColumnRule::default().into();
///
/// // Invalid: Textual column without constraints
/// let invalid_schema = ParserDB::try_from("CREATE TABLE users (name TEXT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Invalid: Textual column with only not-empty constraint
/// let invalid_schema2 = ParserDB::try_from("CREATE TABLE users (name TEXT CHECK (name <> ''));").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema2).is_err());
///
/// // Valid: Textual column with both constraints
/// let valid_schema = ParserDB::try_from("CREATE TABLE users (name TEXT CHECK (name <> ''), CHECK (LENGTH(name) <= 255));").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct TextualColumnRule<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for TextualColumnRule<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<TextualColumnRule<DB>> for GenericConstrainer<DB> {
    fn from(rule: TextualColumnRule<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_rule(Box::new(rule));
        constrainer
    }
}

impl<DB: DatabaseLike> ColumnRule for TextualColumnRule<DB> {
    type Database = DB;

    fn validate_column(
        &self,
        database: &Self::Database,
        column: &<Self::Database as DatabaseLike>::Column,
    ) -> Result<(), crate::error::Error<DB>> {
        // If column is not textual, we don't care.
        if !column.is_textual(database) {
            return Ok(());
        }

        let table = column.table(database);
        let column_name = column.column_name();
        let table_name = table.table_name();

        // 1. Check for not-empty constraint using ColumnLike check_constraints and CheckConstraintLike is_not_empty_text_constraint
        let has_not_empty_check = column
            .check_constraints(database)
            .any(|cc| cc.is_not_empty_text_constraint(database));

        if !has_not_empty_check {
            let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("TextualColumnRule")
                .unwrap()
                .object(format!("{}.{}", table_name, column_name))
                .unwrap()
                .message(format!(
                    "Textual column '{}' must have a check constraint verifying it is not empty.",
                    column_name
                ))
                .unwrap()
                .resolution("Add a check constraint verifying the column is not empty (e.g. `CHECK (col <> '')`).".to_string())
                .unwrap()
                .try_into()
                .unwrap();
            return Err(crate::error::Error::Column(
                Box::new(column.clone()),
                error.into(),
            ));
        }

        // 2. Check for upper bound length constraint
        let mut max_length: Option<usize> = None;

        for cc in column.check_constraints(database) {
            if let Some(limit) = cc.is_upper_bounded_text_constraint(database) {
                match max_length {
                    Some(current) => {
                        if limit < current {
                            max_length = Some(limit);
                        }
                    }
                    None => max_length = Some(limit),
                }
            }
        }

        if max_length.is_none() {
            let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("TextualColumnRule")
                .unwrap()
                .object(format!("{}.{}", table_name, column_name))
                .unwrap()
                .message(format!(
                    "Textual column '{}' must have an upper bound length check constraint.",
                    column_name
                ))
                .unwrap()
                .resolution(
                    "Add a length check constraint (e.g. `CHECK (LENGTH(col) <= 255)`)."
                        .to_string(),
                )
                .unwrap()
                .try_into()
                .unwrap();
            return Err(crate::error::Error::Column(
                Box::new(column.clone()),
                error.into(),
            ));
        }

        let limit = max_length.unwrap();

        // Check if appears in an index
        // We check if it is part of an index.
        let in_unique_index = table.indices(database).any(|idx| {
            idx.columns(database)
                .any(|c| c.column_name() == column_name)
        });
        let in_primary_key = column.is_primary_key(database);

        let in_index = in_unique_index || in_primary_key;

        if in_index {
            if limit > 255 {
                let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("TextualColumnRule")
                .unwrap()
                .object(format!("{}.{}", table_name, column_name))
                .unwrap()
                .message(format!(
                    "Textual column '{}' appears in an index but has length limit {} which is greater than 255.",
                    column_name, limit
                ))
                .unwrap()
                .resolution("Reduce the length limit to 255 or less, or remove the column from the index.".to_string())
                .unwrap()
                .try_into()
                .unwrap();
                return Err(crate::error::Error::Column(
                    Box::new(column.clone()),
                    error.into(),
                ));
            }
        } else {
            if limit > 8192 {
                // Warn user (Error with resolution explaining warning)
                let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("TextualColumnRule")
                .unwrap()
                .object(format!("{}.{}", table_name, column_name))
                .unwrap()
                .message(format!(
                    "Textual column '{}' has length limit {} which is greater than 8192 (8K). This column likely stores a document.",
                    column_name, limit
                ))
                .unwrap()
                .resolution("If you intend to store large text documents, this might be better suited for a document store or Blob storage. Consider reducing the size if not necessary.".to_string())
                .unwrap()
                .try_into()
                .unwrap();
                return Err(crate::error::Error::Column(
                    Box::new(column.clone()),
                    error.into(),
                ));
            }
        }

        Ok(())
    }
}
