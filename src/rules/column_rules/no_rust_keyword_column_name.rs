//! Submodule providing the `NoRustKeywordColumnName` constraint, which enforces
//! that column names are not Rust keywords.

use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    error::RuleErrorInfo,
    rules::rust_keywords::is_rust_keyword,
    traits::{ColumnRule, Constrainer, GenericConstrainer},
};

/// Struct defining a constraint that enforces that column names are not Rust
/// keywords.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `NoRustKeywordColumnName` constraint.
///
/// ```rust
/// use sql_rules::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = NoRustKeywordColumnName::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE mytable (struct INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (my_struct INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct NoRustKeywordColumnName<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for NoRustKeywordColumnName<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<NoRustKeywordColumnName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: NoRustKeywordColumnName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_rule(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> ColumnRule for NoRustKeywordColumnName<DB> {
    type Database = DB;

    fn validate_column(
        &self,
        database: &Self::Database,
        column: &<Self::Database as DatabaseLike>::Column,
    ) -> Result<(), crate::error::Error<Self::Database>> {
        let column_name = column.column_name();
        if is_rust_keyword(column_name) {
            let table_name = column.table(database).table_name();
            let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("NoRustKeywordColumnName")
                .unwrap()
                .object(format!("{table_name}.{column_name}"))
                .unwrap()
                .message(format!(
                    "Column name '{column_name}' in table '{table_name}' is a Rust keyword."
                ))
                .unwrap()
                .resolution(format!(
                    "Rename the column '{column_name}' to something that is not a Rust keyword."
                ))
                .unwrap()
                .try_into()
                .unwrap();
            return Err(crate::error::Error::Column(
                Box::new(column.clone()),
                error.into(),
            ));
        }
        Ok(())
    }
}
