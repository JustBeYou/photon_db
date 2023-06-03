use std::io::{self, BufRead, Write};

use custom_error::custom_error;

use crate::sql::{ParserError, Sql};

#[cfg(not(tarpaulin_include))]
pub fn main_loop() {
    let mut lines = io::stdin().lock().lines();
    let mut stdout = io::stdout();
    let mut context = Context::new();

    while context.running {
        print!("> ");
        stdout.flush().unwrap();

        let line = lines.next().unwrap().unwrap();
        println!("{}", parse_and_execute(line.as_str(), &mut context))
    }
}

fn parse_and_execute(line: &str, context: &mut Context) -> ReplResult {
    let parsed = parse(&line);

    match parsed {
        Err(error) => error.to_string(),
        Ok(command) => execute(&command, context),
    }
}

custom_error! {
    #[derive(PartialEq, Eq)]
    ReplError
    UnknownCommandError {command: String} = "unknown command {command}",
    SqlParserError {error: ParserError} = "sql parsing failed: {error}"
}

struct Context {
    running: bool,
}

impl Context {
    fn new() -> Context {
        Context { running: true }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ReplCommand {
    Exit,
    DoNothing,
    SqlCommand(Sql),
}

fn parse(line: &str) -> Result<ReplCommand, ReplError> {
    match line.chars().next() {
        Some('.') => match line {
            ".exit" => Ok(ReplCommand::Exit),
            _ => Err(ReplError::UnknownCommandError {
                command: line.to_owned(),
            }),
        },
        Some(_) => Sql::parse(line)
            .map(|sql| ReplCommand::SqlCommand(sql))
            .map_err(|sql_error| ReplError::SqlParserError { error: sql_error }),
        None => Ok(ReplCommand::DoNothing),
    }
}

type ReplResult = String;

fn execute(command: &ReplCommand, context: &mut Context) -> ReplResult {
    match command {
        ReplCommand::Exit => {
            context.running = false;
            String::from("Bye bye")
        }
        ReplCommand::DoNothing => String::new(),
        ReplCommand::SqlCommand(_statement) => String::from("SQL support not yet implemented"),
    }
}

#[cfg(test)]
mod tests {
    use crate::repl::{parse, ReplCommand};

    use super::{execute, parse_and_execute, Context};

    #[test]
    fn test_repl_parse_commands() {
        assert_eq!(parse(".exit"), Ok(ReplCommand::Exit));
        assert_eq!(
            parse(".something"),
            Err(crate::repl::ReplError::UnknownCommandError {
                command: String::from(".something")
            })
        );
        assert_eq!(parse(""), Ok(ReplCommand::DoNothing))
    }

    #[test]
    fn test_repl_execute_commands() {
        let mut context = Context::new();
        let result = execute(&ReplCommand::Exit, &mut context);
        assert_eq!(result, "Bye bye");
        assert_eq!(context.running, false);

        let result = execute(&ReplCommand::DoNothing, &mut context);
        assert_eq!(result, "");
    }

    #[test]
    fn test_repl_returns_message() {
        let mut context = Context::new();
        let result = parse_and_execute(".something", &mut context);
        assert_eq!(result, "unknown command .something");

        let result = parse_and_execute(".exit", &mut context);
        assert_eq!(result, "Bye bye");
    }
}
