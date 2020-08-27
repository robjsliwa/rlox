use clap::{App, Arg};
use rloxlib::{run_file, run_repl};

fn main() {
  let matches = App::new("rlox programming language")
    .arg(
      Arg::new("input")
        .short('i')
        .long("input")
        .value_name("INPUT")
        .about("Specifiy source code input file"),
    )
    .get_matches();

  if let Some(source_file_name) = matches.value_of("input") {
    if let Err(err) = run_file(source_file_name) {
      println!("Error {}", err);
    }
  } else {
    if let Err(err) = run_repl() {
      println!("Error {}", err);
    }
  }
}
