mod rlox;
mod scanners;

use scanners::Scanner;

use failure::Error;
use std::io::{stdin, stdout, Write};

pub fn run_file(filename: &str) -> Result<(), Error> {
    let data = scanners::read_source_code(filename)?;
    run(data)
}

pub fn run_repl() -> Result<(), Error> {
    loop {
        let mut data = String::new();
        print!("> ");
        stdout().flush()?;
        stdin().read_line(&mut data)?;
        run(data.chars().collect())?;
    }
}

fn run(data: Vec<char>) -> Result<(), Error> {
    let mut scanner = Scanner::new(data);
    let tokens = scanner.scan_tokens();
    println!("{:?}", tokens);
    Ok(())
}
