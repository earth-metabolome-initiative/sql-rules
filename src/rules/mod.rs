//! Submodule providing structs to add custom rules on SQL schemas.

mod table_rules;
pub use table_rules::*;
mod column_rules;
pub use column_rules::*;
mod foreign_key_rules;
pub use foreign_key_rules::*;
pub mod rust_keywords;
