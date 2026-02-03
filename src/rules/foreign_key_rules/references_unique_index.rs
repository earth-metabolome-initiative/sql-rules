//! Submodule providing the `ReferencesUniqueIndex` constraint, which enforces
//! that foreign keys must reference columns that are covered by a unique index
//! in the referenced table.

use crate::{
    error::RuleErrorInfo,
    traits::{Constrainer, ForeignKeyRule, GenericConstrainer},
};
use sql_traits::traits::IndexLike;
use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike, TableLike};

/// Struct defining a constraint that enforces that foreign keys must reference
/// columns that are covered by a unique index in the referenced table.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `ReferencesUniqueIndex` constraint.
///
/// ```rust
/// use sql_rules::prelude::*;
/// use sqlparser::dialect::GenericDialect;
///
/// let constrainer: GenericConstrainer<ParserDB> = ReferencesUniqueIndex::default().into();
///
/// let invalid_schema = ParserDB::parse::<GenericDialect>(
///     r#"
/// CREATE TABLE parent (id INT, name TEXT);
/// CREATE TABLE child (id INT, FOREIGN KEY (id) REFERENCES parent (id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema_primary_key = ParserDB::parse::<GenericDialect>(
///     r#"
/// CREATE TABLE parent (id INT PRIMARY KEY);
/// CREATE TABLE child (id INT, FOREIGN KEY (id) REFERENCES parent (id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema_primary_key).is_ok());
///
/// let valid_schema_unique = ParserDB::parse::<GenericDialect>(
///     r#"
/// CREATE TABLE parent (id INT UNIQUE);
/// CREATE TABLE child (id INT, FOREIGN KEY (id) REFERENCES parent (id));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema_unique).is_ok());
///
/// let valid_schema_composite = ParserDB::parse::<GenericDialect>(
///     r#"
/// CREATE TABLE parent (id INT, code TEXT, UNIQUE (id, code));
/// CREATE TABLE child (id INT, code TEXT, FOREIGN KEY (id, code) REFERENCES parent (id, code));
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema_composite).is_ok());
/// ```
pub struct ReferencesUniqueIndex<C>(std::marker::PhantomData<C>);

impl<C> Default for ReferencesUniqueIndex<C> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<ReferencesUniqueIndex<DB>> for GenericConstrainer<DB> {
    fn from(constraint: ReferencesUniqueIndex<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_foreign_key_rule(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> ForeignKeyRule for ReferencesUniqueIndex<DB> {
    type Database = DB;

    fn validate_foreign_key(
        &self,
        database: &Self::Database,
        foreign_key: &<Self::Database as DatabaseLike>::ForeignKey,
    ) -> Result<(), crate::prelude::Error<DB>> {
        let host_table = foreign_key.host_table(database);
        let referenced_table = foreign_key.referenced_table(database);
        let referenced_columns: Vec<_> = foreign_key.referenced_columns(database).collect();

        // Check if there's a unique index that covers the referenced columns
        let has_matching_unique_index =
            referenced_table
                .unique_indices(database)
                .any(|unique_index| {
                    let index_columns: Vec<_> = unique_index.columns(database).collect();

                    index_columns == referenced_columns
                });

        if !has_matching_unique_index {
            let referenced_column_names: Vec<_> = referenced_columns
                .iter()
                .map(ColumnLike::column_name)
                .collect();

            let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("ReferencesUniqueIndex")
                .unwrap()
                .object(
                    foreign_key
                        .foreign_key_name()
                        .unwrap_or("Unnamed foreign key")
                        .to_owned(),
                )
                .unwrap()
                .message(format!(
                    "Foreign key from table '{}' references columns ({}) in table '{}' which are not covered by a unique index",
                    host_table.table_name(),
                    referenced_column_names.join(", "),
                    referenced_table.table_name(),
                ))
                .unwrap()
                .resolution(format!(
                    "Add a unique constraint or primary key on columns ({}) in table '{}', or remove the foreign key from table '{}'",
                    referenced_column_names.join(", "),
                    referenced_table.table_name(),
                    host_table.table_name(),
                ))
                .unwrap()
                .try_into()
                .unwrap();
            return Err(crate::error::Error::ForeignKey(
                Box::new(foreign_key.clone()),
                error.into(),
            ));
        }

        Ok(())
    }
}
