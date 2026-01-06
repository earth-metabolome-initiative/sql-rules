//! Submodule providing the `SnakeCaseTableName` constraint, which enforces that
//! table names follow `snake_case` style.

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};
use heck::ToSnakeCase;
use sql_traits::traits::{DatabaseLike, TableLike};

/// Struct defining a constraint that enforces that table names follow
/// `snake_case` style.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `SnakeCaseTableName` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = SnakeCaseTableName::default().into();
///
/// // Invalid: PascalCase
/// let invalid_schema = ParserDB::try_from("CREATE TABLE MyTable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Invalid: double underscore
/// let invalid_schema2 = ParserDB::try_from("CREATE TABLE my__table (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema2).is_err());
///
/// // Invalid: camelCase
/// let invalid_schema3 = ParserDB::try_from("CREATE TABLE myTable (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema3).is_err());
///
/// // Valid: proper snake_case
/// let valid_schema = ParserDB::try_from("CREATE TABLE my_table (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// let valid_schema2 = ParserDB::try_from("CREATE TABLE users (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema2).is_ok());
///
/// let valid_schema3 = ParserDB::try_from("CREATE TABLE user_accounts (id INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema3).is_ok());
/// ```
pub struct SnakeCaseTableName<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for SnakeCaseTableName<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<SnakeCaseTableName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: SnakeCaseTableName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for SnakeCaseTableName<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        _database: &Self::Database,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        let table_name = context.table_name();
        let expected_name = table_name.to_snake_case();

        let issue = if table_name.contains("__") {
            "contains double underscores"
        } else if table_name.chars().any(|c| c.is_ascii_uppercase()) {
            "contains uppercase letters"
        } else if table_name != expected_name {
            "does not follow snake_case convention"
        } else {
            "is not valid snake_case"
        };

        let error: ConstraintErrorInfo = ConstraintErrorInfo::new()
            .constraint("SnakeCaseTableName")
            .unwrap()
            .object(table_name.to_owned())
            .unwrap()
            .message(format!(
                "Table '{table_name}' violates snake_case naming convention: {issue}"
            ))
            .unwrap()
            .resolution(format!(
                "Change '{table_name}' to '{expected_name}' (use lowercase letters and single underscores only)"
            ))
            .unwrap()
            .try_into()
            .unwrap();
        error.into()
    }

    fn validate_table(
        &self,
        database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), crate::error::Error> {
        let table_name = table.table_name();
        let expected_name = table_name.to_snake_case();

        // Check if the name matches its snake_case conversion
        if expected_name == table_name {
            Ok(())
        } else {
            Err(crate::error::Error::Table(
                self.table_error_information(database, table),
            ))
        }
    }
}
