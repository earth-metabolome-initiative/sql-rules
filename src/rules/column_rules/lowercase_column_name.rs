//! Submodule providing the `LowercaseColumnName` constraint, which enforces
//! that column names are lowercase.

use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    error::RuleErrorInfo,
    traits::{ColumnRule, Constrainer, GenericConstrainer},
};

/// Struct defining a constraint that enforces that column names are lowercase.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `LowercaseColumnName` constraint.
///
/// ```rust
/// use sql_rules::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = LowercaseColumnName::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE mytable (Id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct LowercaseColumnName<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for LowercaseColumnName<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<LowercaseColumnName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: LowercaseColumnName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_rule(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> ColumnRule for LowercaseColumnName<DB> {
    type Database = DB;

    fn validate_column(
        &self,
        database: &Self::Database,
        column: &<Self::Database as DatabaseLike>::Column,
    ) -> Result<(), crate::error::Error<Self::Database>> {
        if column
            .column_name()
            .chars()
            .all(|c| !c.is_alphabetic() || c.is_lowercase())
        {
            Ok(())
        } else {
            let table = column.table(database);
            let table_name = table.table_name();
            let column_name = column.column_name();

            let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("LowercaseColumnName")
                .unwrap()
                .object(format!("{table_name}.{column_name}"))
                .unwrap()
                .message(format!(
                    "Column '{column_name}' in table '{table_name}' is not lowercase"
                ))
                .unwrap()
                .resolution(format!(
                    "Rename column '{column_name}' in table '{table_name}' to be all lowercase"
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
