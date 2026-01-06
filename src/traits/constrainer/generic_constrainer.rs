//! Submodule defining a generic constrainer for SQL rules.

use sql_traits::traits::DatabaseLike;

use crate::traits::Constrainer;

/// A generic constrainer that holds and applies table rules.
pub struct GenericConstrainer<DB: DatabaseLike> {
    /// The registered table rules.
    tables: Vec<Box<dyn crate::traits::TableRule<Database = DB>>>,
    /// The registered column rules.
    columns: Vec<Box<dyn crate::traits::ColumnRule<Database = DB>>>,
    /// The registered foreign key rules.
    foreign_keys: Vec<Box<dyn crate::traits::ForeignKeyRule<Database = DB>>>,
}

impl<DB: DatabaseLike> Default for GenericConstrainer<DB> {
    fn default() -> Self {
        Self {
            tables: Vec::new(),
            columns: Vec::new(),
            foreign_keys: Vec::new(),
        }
    }
}

impl<DB: DatabaseLike> Constrainer for GenericConstrainer<DB> {
    type Database = DB;

    fn table_rules(
        &self,
    ) -> impl Iterator<Item = &dyn crate::traits::TableRule<Database = Self::Database>> {
        self.tables.iter().map(AsRef::as_ref)
    }

    fn column_rules(
        &self,
    ) -> impl Iterator<Item = &dyn crate::traits::ColumnRule<Database = Self::Database>> {
        self.columns.iter().map(AsRef::as_ref)
    }

    fn foreign_key_rules(
        &self,
    ) -> impl Iterator<Item = &dyn crate::traits::ForeignKeyRule<Database = Self::Database>> {
        self.foreign_keys.iter().map(AsRef::as_ref)
    }

    fn register_table_rule(
        &mut self,
        rule: Box<dyn crate::traits::TableRule<Database = Self::Database>>,
    ) {
        self.tables.push(rule);
    }

    fn register_column_rule(
        &mut self,
        rule: Box<dyn crate::traits::ColumnRule<Database = Self::Database>>,
    ) {
        self.columns.push(rule);
    }

    fn register_foreign_key_rule(
        &mut self,
        rule: Box<dyn crate::traits::ForeignKeyRule<Database = Self::Database>>,
    ) {
        self.foreign_keys.push(rule);
    }
}
