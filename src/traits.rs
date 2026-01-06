//! Submodule defining traits used for SQL schema constraints.

mod table_rule;
pub use table_rule::TableRule;
mod column_rule;
pub use column_rule::ColumnRule;
pub mod constrainer;
pub use constrainer::{Constrainer, DefaultConstrainer, GenericConstrainer};
mod constraint_failure_information;
pub use constraint_failure_information::ConstraintFailureInformation;
mod foreign_key_rule;
pub use foreign_key_rule::ForeignKeyRule;
