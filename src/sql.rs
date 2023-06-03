use custom_error::custom_error;

custom_error! {
    #[derive(PartialEq, Eq)]
    pub ParserError
    UnknownStatement {statement: String} = "unknown SQL statement: {statement}"
}

#[derive(Debug, PartialEq, Eq)]
pub struct Sql {
    statements: Vec<SqlStatement>,
}

impl Sql {
    pub fn parse(line: &str) -> Result<Sql, ParserError> {
        Err(ParserError::UnknownStatement {
            statement: line.to_owned(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SqlStatement {
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
