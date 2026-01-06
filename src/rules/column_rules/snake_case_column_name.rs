//! Submodule providing the `SnakeCaseColumnName` constraint, which enforces
//! that column names follow `snake_case` style.

use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    error::RuleErrorInfo,
    traits::{ColumnRule, Constrainer, GenericConstrainer},
};

use heck::ToSnakeCase;

/// Struct defining a constraint that enforces that column names follow
/// `snake_case` style.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `SnakeCaseColumnName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = SnakeCaseColumnName::default().into();
///
/// // Invalid: PascalCase
/// let invalid_schema = ParserDB::try_from("CREATE TABLE mytable (MyColumn INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Invalid: double underscore
/// let invalid_schema2 = ParserDB::try_from("CREATE TABLE mytable (my__column INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema2).is_err());
///
/// // Invalid: camelCase
/// let invalid_schema3 = ParserDB::try_from("CREATE TABLE mytable (myColumn INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema3).is_err());
///
/// // Valid: proper snake_case
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (my_column INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// let valid_schema2 = ParserDB::try_from("CREATE TABLE mytable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema2).is_ok());
///
/// let valid_schema3 = ParserDB::try_from("CREATE TABLE mytable (first_name TEXT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema3).is_ok());
/// ```
pub struct SnakeCaseColumnName<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for SnakeCaseColumnName<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<SnakeCaseColumnName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: SnakeCaseColumnName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_rule(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> ColumnRule for SnakeCaseColumnName<DB> {
    type Database = DB;

    fn validate_column(
        &self,
        database: &Self::Database,
        column: &<Self::Database as DatabaseLike>::Column,
    ) -> Result<(), crate::error::Error<Self::Database>> {
        let column_name = column.column_name();
        let snake_cased = column_name.to_snake_case();

        // Check if the name matches its snake_case conversion
        if snake_cased == column_name {
            Ok(())
        } else {
            let table = column.table(database);
            let table_name = table.table_name();
            let expected_name = snake_cased;

            let issue = if column_name.contains("__") {
                "contains double underscores"
            } else if column_name.chars().any(|c| c.is_ascii_uppercase()) {
                "contains uppercase letters"
            } else if column_name != expected_name {
                "does not follow snake_case convention"
            } else {
                "is not valid snake_case"
            };

            let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("SnakeCaseColumnName")
                .unwrap()
                .object(format!("{}.{}", table_name, column_name))
                .unwrap()
                .message(format!(
                    "Column '{column_name}' in table '{table_name}' violates snake_case naming convention: {issue}"
                ))
                .unwrap()
                .resolution(format!(
                    "Change '{column_name}' to '{expected_name}' in table '{table_name}' (use lowercase letters and single underscores only)"
                ))
                .unwrap()
                .try_into()
                .unwrap();
            Err(crate::error::Error::Column(
                Box::new(column.clone()),
                error.into(),
            ))
        }
    }
}
