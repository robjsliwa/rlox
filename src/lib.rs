mod rlox;
mod scanners;

use rlox::*;
use scanners::Scanner;

use std::io::{stdin, stdout, Write};

pub fn run_file(filename: &str) -> Result<(), RloxError> {
    let interpreter = Interpreter::new();
    let data = scanners::read_source_code(filename)?;
    run(interpreter, data)
}

pub fn run_repl() -> Result<(), RloxError> {
    let interpreter = Interpreter::new();

    loop {
        let mut data = String::new();
        print!("> ");
        stdout().flush()?;
        stdin().read_line(&mut data)?;
        run(interpreter.clone(), data.chars().collect())?;
    }
}

fn repl_printer(result: Result<RloxType, RloxError>) {
    match result {
        Ok(r) => {
            if r != RloxType::NullType {
                println!("{}", r);
            }
        }
        Err(e) => {
            print_rlox_error(e);
        }
    }
}

fn print_rlox_error(e: RloxError) {
  match e {
    RloxError::ResolverError(r) => eprintln!("{}", r),
    RloxError::InterpreterError(i) => eprintln!("{}", i),
    RloxError::ParserError(p) => eprintln!("{}", p),
    _ => eprintln!("Unknown error."),
  }
}

fn run(interpreter: Interpreter, data: Vec<char>) -> Result<(), RloxError> {
    let mut scanner = Scanner::new(data);
    let tokens = scanner.scan_tokens();
    let parser = Parser::new(tokens);
    let statements = parser.parse();

    match statements {
        Ok(stmt) => {
          let resolver = Resolver::new(interpreter.clone());
          if let Err(e) = resolver.resolve_statements(stmt.clone()) {
            print_rlox_error(e);
            return Ok(());
          }
          interpreter.interpret(stmt, Some(repl_printer))
        }
        Err(e) => {
            print_rlox_error(e);
        }
    }

    Ok(())
}
