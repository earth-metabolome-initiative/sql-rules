//! Submodule providing the `NoRustKeywordTableName` constraint, which enforces
//! that table names are not Rust keywords.

use sql_traits::traits::{DatabaseLike, TableLike};

use crate::{
    error::RuleErrorInfo,
    rules::rust_keywords::is_rust_keyword,
    traits::{Constrainer, GenericConstrainer, TableRule},
};

/// Struct defining a constraint that enforces that table names are not Rust
/// keywords.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `NoRustKeywordTableName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = NoRustKeywordTableName::default().into();
///
/// let invalid_schema = ParserDB::try_from("CREATE TABLE struct (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from("CREATE TABLE my_struct (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct NoRustKeywordTableName<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for NoRustKeywordTableName<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<NoRustKeywordTableName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: NoRustKeywordTableName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_rule(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableRule for NoRustKeywordTableName<DB> {
    type Database = DB;

    fn validate_table(
        &self,
        _database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), crate::error::Error<DB>> {
        let table_name = table.table_name();
        if is_rust_keyword(table_name) {
            let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("NoRustKeywordTableName")
                .unwrap()
                .object(table_name.to_owned())
                .unwrap()
                .message(format!("Table name '{}' is a Rust keyword.", table_name))
                .unwrap()
                .resolution(format!(
                    "Rename the table '{}' to something that is not a Rust keyword.",
                    table_name
                ))
                .unwrap()
                .try_into()
                .unwrap();
            return Err(crate::error::Error::Table(
                Box::new(table.clone()),
                error.into(),
            ));
        }
        Ok(())
    }
}
