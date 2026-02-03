//! Submodule providing the `SingularColumnName` constraint, which enforces
//! that the last segment of column names is singular.

use inflection_rs::inflection::singularize;
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    error::RuleErrorInfo,
    traits::{ColumnRule, Constrainer, GenericConstrainer},
};

/// Struct defining a constraint that enforces that the last segment of column
/// names is singular.
///
/// For column names with underscores (e.g., `user_account`), only the last
/// segment after the final underscore is checked for singularity.
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `SingularColumnName` constraint.
///
/// ```rust
/// use sql_rules::prelude::*;
/// use sqlparser::dialect::GenericDialect;
///
/// let constrainer: GenericConstrainer<ParserDB> = SingularColumnName::default().into();
///
/// // Invalid plural column names
/// let invalid_schema = ParserDB::parse::<GenericDialect>("CREATE TABLE mytable (users INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let invalid_schema2 = ParserDB::parse::<GenericDialect>("CREATE TABLE mytable (user_accounts INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema2).is_err());
///
/// // Valid singular column names
/// let valid_schema = ParserDB::parse::<GenericDialect>("CREATE TABLE mytable (user INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// let valid_schema2 = ParserDB::parse::<GenericDialect>("CREATE TABLE mytable (user_account INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_schema2).is_ok());
///
/// // Edge cases with Latin singulars
/// let valid_spectrum = ParserDB::parse::<GenericDialect>("CREATE TABLE mytable (spectrum INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_spectrum).is_ok());
///
/// let invalid_spectra = ParserDB::parse::<GenericDialect>("CREATE TABLE mytable (spectra INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_spectra).is_err());
///
/// let valid_matrix = ParserDB::parse::<GenericDialect>("CREATE TABLE mytable (matrix INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_matrix).is_ok());
///
/// let invalid_matrices = ParserDB::parse::<GenericDialect>("CREATE TABLE mytable (matrices INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_matrices).is_err());
///
/// let valid_taxon = ParserDB::parse::<GenericDialect>("CREATE TABLE mytable (taxon INT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_taxon).is_ok());
///
/// let invalid_taxa = ParserDB::parse::<GenericDialect>("CREATE TABLE mytable (taxa INT);").unwrap();
/// assert!(constrainer.validate_schema(&invalid_taxa).is_err());
/// ```
pub struct SingularColumnName<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for SingularColumnName<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<SingularColumnName<DB>> for GenericConstrainer<DB> {
    fn from(constraint: SingularColumnName<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_column_rule(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> ColumnRule for SingularColumnName<DB> {
    type Database = DB;

    fn validate_column(
        &self,
        database: &Self::Database,
        column: &<Self::Database as DatabaseLike>::Column,
    ) -> Result<(), crate::error::Error<Self::Database>> {
        let column_name = column.column_name();
        let last_segment = column_name.split('_').next_back().unwrap_or(column_name);

        // Check if the last segment is singular by verifying that singularizing it
        // doesn't change it
        let singularized = singularize(last_segment);

        if singularized == last_segment {
            Ok(())
        } else {
            let table = column.table(database);
            let table_name = table.table_name();
            let expected_name = if column_name.contains('_') {
                let prefix = &column_name[..column_name.rfind('_').unwrap()];
                format!("{}_{}", prefix, &singularized)
            } else {
                singularized.clone()
            };

            let error: RuleErrorInfo = RuleErrorInfo::builder()
                .rule("SingularColumnName")
                .unwrap()
                .object(format!("{table_name}.{column_name}"))
                .unwrap()
                .message(format!(
                    "Column '{column_name}' in table '{table_name}' violates singular naming convention: the last segment '{last_segment}' is plural, not singular"
                ))
                .unwrap()
                .resolution(format!(
                    "Change '{column_name}' to '{expected_name}' in table '{table_name}' (singularize the last segment from '{last_segment}' to '{singularized}')"
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
