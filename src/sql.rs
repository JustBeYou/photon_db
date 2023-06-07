use custom_error::custom_error;
use itertools::Itertools;

custom_error! {
    #[derive(Clone, PartialEq, Eq)]
    pub ParserError
    UnsupportedStatementError {statement: String} = "SQL statement not yet supported: {statement}",
    UnknownStatementError {statement: String} = "unknown SQL statement: {statement}",
    MissingValuesClauseError = "insert statement must contain a 'values' clause",
    MissingTableNameError {statement: String} = "'{statement}' statement must have a table name specified",
}

#[derive(Debug, PartialEq, Eq)]
pub struct Sql {
    statements: Vec<SqlStatement>,
}

impl Sql {
    pub fn parse(text: &str) -> Result<Sql, ParserError> {
        let (parsed, errors): (Vec<_>, Vec<_>) = text
            .to_lowercase()
            .split(';')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| parse_lowercase_line(line))
            .into_iter()
            .partition_result();

        match errors.first() {
            Some(error) => return Err(error.to_owned()),
            None => {}
        }

        return Ok(Sql { statements: parsed });
    }
}

fn parse_lowercase_line(line: &str) -> Result<SqlStatement, ParserError> {
    if line.starts_with("insert into") {
        let values_statement_idx = line
            .find("values")
            .ok_or(ParserError::MissingValuesClauseError)?;

        let table_name = line[12..values_statement_idx].trim();
        if table_name.len() == 0 {
            return Err(ParserError::MissingTableNameError {
                statement: String::from("insert"),
            });
        }

        return Ok(SqlStatement::Insert {
            table: String::from(table_name),
            values: std::collections::HashMap::new(),
        });
    } else if line.starts_with("select") {
        return Err(ParserError::UnsupportedStatementError {
            statement: String::from("select"),
        });
    } else if line.starts_with("drop") {
        return Err(ParserError::UnsupportedStatementError {
            statement: String::from("drop"),
        });
    } else if line.starts_with("create table") {
        return Err(ParserError::UnsupportedStatementError {
            statement: String::from("create table"),
        });
    }

    return Err(ParserError::UnknownStatementError {
        statement: String::from(line),
    });
}

#[derive(Debug, PartialEq, Eq)]
enum SqlStatement {
    CreateTable {
        name: String,
        columns: std::collections::HashMap<String, ColumnType>,
    },
    Select {
        table: String,
        columns: Vec<String>,
    },
    Insert {
        table: String,
        values: std::collections::HashMap<String, String>,
    },
    Drop {
        table: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
enum ColumnType {
    Id,

    Char255,

    Int8,
    Int16,
    Int32,
    Int64,

    Uint8,
    Uint16,
    Uint32,
    Uint64,
}
