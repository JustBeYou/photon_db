use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    io::{self, BufRead, Write},
};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut stdout = io::stdout();
    let mut context = Context::new();

    while context.repl_running {
        print!("> ");
        stdout.flush().unwrap();

        let line = lines.next().unwrap().unwrap();
        let parsed = repl_parse_line(&line);

        match parsed {
            Err(error) => println!("! {}", error),
            Ok(command) => repl_execute(&command, &mut context),
        };
    }
}
struct Context {
    repl_running: bool,
}

impl Context {
    fn new() -> Context {
        Context { repl_running: true }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ReplCommand {
    Exit,
    SqlCommand(SqlStatement),
}

#[derive(Debug, PartialEq, Eq)]
enum SqlStatement {
    Select {
        table: String,
        columns: Vec<String>,
    },
    Insert {
        table: String,
        values: HashMap<String, String>,
    },
    Drop {
        table: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
enum ParserError {
    EmptyCommand,
    UnknownCommand(String),
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnknownCommand(command) => write!(f, "Unknown command '{}'.", command),
            ParserError::EmptyCommand => write!(f, "Empty command provided."),
        }
    }
}

fn repl_parse_line(line: &str) -> Result<ReplCommand, ParserError> {
    let first_char = line.chars().next().ok_or(ParserError::EmptyCommand)?;

    if first_char == '.' {
        return match line {
            ".exit" => Ok(ReplCommand::Exit),
            _ => Err(ParserError::UnknownCommand(line.to_owned())),
        };
    }

    sql_parse_line(line).map(|statement| ReplCommand::SqlCommand(statement))
}

fn sql_parse_line(line: &str) -> Result<SqlStatement, ParserError> {
    // TODO: implement this
    Err(ParserError::UnknownCommand(line.to_owned()))
}

fn repl_execute(command: &ReplCommand, context: &mut Context) {
    match command {
        ReplCommand::Exit => context.repl_running = false,
        ReplCommand::SqlCommand(statement) => sql_execute(statement, context),
    };
}

fn sql_execute(statement: &SqlStatement, context: &mut Context) {
    match statement {
        SqlStatement::Select { table, columns } => todo!(),
        SqlStatement::Insert { table, values } => todo!(),
        SqlStatement::Drop { table } => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{repl_parse_line, ParserError, ReplCommand};

    #[test]
    fn test_repl_parse_commands() {
        assert_eq!(repl_parse_line(".exit"), Ok(ReplCommand::Exit));
        assert_eq!(
            repl_parse_line("something"),
            Err(ParserError::UnknownCommand(String::from("something")))
        );
        assert_eq!(repl_parse_line(""), Err(ParserError::EmptyCommand))
    }
}
