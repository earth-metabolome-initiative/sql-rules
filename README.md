# SQL Rules

[![CI](https://github.com/earth-metabolome-initiative/sql-rules/workflows/Rust%20CI/badge.svg)](https://github.com/earth-metabolome-initiative/sql-rules/actions)
[![Security Audit](https://github.com/earth-metabolome-initiative/sql-rules/workflows/Security%20Audit/badge.svg)](https://github.com/earth-metabolome-initiative/sql-rules/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/earth-metabolome-initiative/sql-rules/branch/main/graph/badge.svg)](https://codecov.io/gh/earth-metabolome-initiative/sql-rules)

[SQL-traits](https://github.com/earth-metabolome-initiative/sql-traits)-based higher-level rules for validating SQL schemas. These are general-purpose rules that are often desirable in SQL projects to enforce common standards (naming conventions, best practices) which are not strict SQL requirements but improve project quality and consistency.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sql_rules = "0.1.0"
```

## Usage

The library provides a `Constrainer` trait which applies registered `TableRule`, `ColumnRule`, and `ForeignKeyRule` implementations to a database schema.

### Using the Default Constrainer

The `DefaultConstrainer` comes pre-configured with a comprehensive set of common-sense rules.

```rust
use sql_rules::prelude::*;
use sqlparser::dialect::GenericDialect;
// ParserDB is available from sql-traits for testing and examples

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load or define your database schema
    let database = ParserDB::parse::<GenericDialect>("CREATE TABLE users (id INT PRIMARY KEY, name TEXT);")?;

    // 2. Create the default constrainer
    let constrainer = DefaultConstrainer::<ParserDB>::default();

    // 3. Validate the schema
    match constrainer.validate_schema(&database) {
        Ok(_) => println!("Schema is valid!"),
        Err(e) => eprintln!("Schema validation failed: {}", e),
    }

    Ok(())
}
```

### Custom Rules Configuration

You can select specific rules using `GenericConstrainer`.

```rust
use sql_rules::prelude::*;

let mut constrainer = GenericConstrainer::<ParserDB>::default();

// Register only specific rules
constrainer.register_table_rule(Box::new(SnakeCaseTableName::default()));
constrainer.register_column_rule(Box::new(LowercaseColumnName::default()));

// Use the constrainer...
```

## Available Rules

### Table Rules

- **HasPrimaryKey**: Ensures everyone table has a primary key.
- **Naming Conventions**: `LowercaseTableName`, `SnakeCaseTableName`, `PluralTableName`.
- **Reserved Words**: `NoRustKeywordTableName`.
- **Checks**: `NoTautologicalCheckRule`, `NoNegationCheckRule`, `UniqueCheckRule`.
- **Extensions**: `NoForbiddenColumnInExtension`, `NonRedundantExtensionDag`.
- **Uniqueness**: `UniqueForeignKey`, `UniqueUniqueIndex`.

### Column Rules

- **Naming Conventions**: `LowercaseColumnName`, `SnakeCaseColumnName`, `SingularColumnName`.
- **Reserved Words**: `NoRustKeywordColumnName`.
- **Primary Keys**: `NonCompositePrimaryKeyNamedId`.

### Foreign Key Rules

- **Naming Conventions**: `LowercaseForeignKeyName`, `NoRustKeywordForeignKeyName`.
- **Structure**: `CompatibleForeignKey`, `ReferencesUniqueIndex`.
- **Best Practices**: `PrimaryKeyReferenceEndsWithId`, `ExtensionForeignKeyOnDeleteCascade`.

## Contributing

If you can think of any rule that most SQL databases should enforce (or that represents a common best practice), please consider contributing it!
