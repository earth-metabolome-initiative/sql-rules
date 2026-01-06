//! Submodule providing the `LowercaseTableName` constraint, which enforces that
//! table names are lowercase.

use sql_traits::traits::{DatabaseLike, TableLike};

use crate::{
    error::RuleErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableRule},
};

/// Struct defining a constraint that enforces that table names are lowercase.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `LowercaseTableName` constraint.
///
/// ```rust
/// use sql_rules::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = LowercaseTableName::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE MyTable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from("CREATE TABLE mytable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct LowercaseTableName<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for LowercaseTableName<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<LowercaseTableName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: LowercaseTableName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_rule(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableRule for LowercaseTableName<DB> {
    type Database = DB;

    fn validate_table(
        &self,
        _database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), crate::error::Error<DB>> {
        if table
            .table_name()
            .chars()
            .all(|c| !c.is_alphabetic() || c.is_lowercase())
        {
            Ok(())
        } else {
            let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("LowercaseTableName")
                .unwrap()
                .object(table.table_name().to_owned())
                .unwrap()
                .message(format!(
                    "Table name '{}' is not lowercase",
                    table.table_name()
                ))
                .unwrap()
                .resolution("Rename the table to be all lowercase".to_string())
                .unwrap()
                .try_into()
                .unwrap();
            Err(crate::error::Error::Table(
                Box::new(table.clone()),
                error.into(),
            ))
        }
    }
}
