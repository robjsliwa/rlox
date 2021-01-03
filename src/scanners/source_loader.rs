use std::fs::File;
use std::io::Read;
use crate::RloxError;

pub fn read_source_code(filename: &str) -> Result<Vec<char>, RloxError> {
  let mut source_code = File::open(filename)?;

  let mut data = String::from("");
  source_code.read_to_string(&mut data)?;

  //.read_to_end(&mut data)?;

  Ok(data.chars().collect())
}
