//! Submodule providing the `UniqueForeignKey` constraint, which enforces
//! that foreign keys are unique per table, meaning that no two foreign keys
//! have the same columns in a single table.

use std::hash::{DefaultHasher, Hash, Hasher};

use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that table names are lowercase.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `UniqueForeignKey` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = UniqueForeignKey::default().into();
///
/// let invalid_schema =
///     ParserDB::try_from("CREATE TABLE MyTable (id INT PRIMARY KEY REFERENCES MyTable (id), FOREIGN KEY (id) REFERENCES MyTable (id));")
///         .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err(), "1) Foreign keys must be unique per table");
///
/// let invalid_schema2 =
///     ParserDB::try_from("CREATE TABLE MyTable (id INT PRIMARY KEY, FOREIGN KEY (id) REFERENCES MyTable (id), FOREIGN KEY (id) REFERENCES MyTable (id));").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema2).is_err(), "2) Foreign keys must be unique per table");
///
/// let valid_schema =
///     ParserDB::try_from("CREATE TABLE mytable (id INT PRIMARY KEY, FOREIGN KEY (id) REFERENCES mytable (id));").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
/// ```
pub struct UniqueForeignKey<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for UniqueForeignKey<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<UniqueForeignKey<DB>> for GenericConstrainer<DB> {
    fn from(constraint: UniqueForeignKey<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for UniqueForeignKey<DB> {
    type Database = DB;

    fn validate_table(
        &self,
        database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), crate::error::Error> {
        // Find the duplicate foreign keys to provide detailed error information
        let constraints = table.foreign_keys(database).collect::<Vec<_>>();
        let mut signatures_with_fks: Vec<_> = constraints
            .iter()
            .map(|fk| {
                let mut hasher = DefaultHasher::new();
                for host_col in fk.host_columns(database) {
                    host_col.hash(&mut hasher);
                }
                let referenced_table = fk.referenced_table(database);
                referenced_table.hash(&mut hasher);
                for referenced_col in fk.referenced_columns(database) {
                    referenced_col.hash(&mut hasher);
                }
                (hasher.finish(), *fk)
            })
            .collect();

        signatures_with_fks.sort_unstable_by_key(|(sig, _)| *sig);

        // Find the first duplicate
        let mut duplicate_fks = Vec::new();
        for window in signatures_with_fks.windows(2) {
            if let [(sig1, fk1), (sig2, fk2)] = window
                && sig1 == sig2
            {
                duplicate_fks.push(*fk1);
                duplicate_fks.push(*fk2);
                break;
            }
        }

        if !duplicate_fks.is_empty() {
            let table_name = table.table_name();

            let error: ConstraintErrorInfo = if duplicate_fks.is_empty() {
                // Fallback for generic case
                ConstraintErrorInfo::builder()
                    .constraint("UniqueForeignKey")
                    .unwrap()
                    .object(table_name.to_owned())
                    .unwrap()
                    .message(format!("Table '{}' has duplicate foreign keys", table_name))
                    .unwrap()
                    .resolution("Ensure all foreign keys in the table are unique".to_string())
                    .unwrap()
                    .try_into()
                    .unwrap()
            } else {
                // Build detailed error message with the duplicate foreign keys
                let mut fk_details = Vec::new();
                for fk in &duplicate_fks {
                    let host_cols: Vec<_> =
                        fk.host_columns(database).map(|c| c.column_name()).collect();
                    let referenced_table = fk.referenced_table(database);
                    let referenced_cols: Vec<_> = fk
                        .referenced_columns(database)
                        .map(|c| c.column_name())
                        .collect();

                    fk_details.push(format!(
                        "FOREIGN KEY ({}) REFERENCES {} ({})",
                        host_cols.join(", "),
                        referenced_table.table_name(),
                        referenced_cols.join(", ")
                    ));
                }

                let message = format!(
                    "Table '{}' has {} duplicate foreign key definitions:\n  - {}\nBoth foreign keys reference the same columns and target table",
                    table_name,
                    duplicate_fks.len(),
                    fk_details.join("\n  - ")
                );

                let resolution = format!(
                    "Remove one of the duplicate foreign key constraints from table '{}'. Keep only one: {}",
                    table_name,
                    fk_details.first().unwrap()
                );

                ConstraintErrorInfo::builder()
                    .constraint("UniqueForeignKey")
                    .unwrap()
                    .object(table_name.to_owned())
                    .unwrap()
                    .message(message)
                    .unwrap()
                    .resolution(resolution)
                    .unwrap()
                    .try_into()
                    .unwrap()
            };

            return Err(crate::error::Error::Table(error.into()));
        }

        Ok(())
    }
}
