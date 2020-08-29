#[derive(Clone, Debug)]
pub enum Literal {
  StringType(String),
  NumberType(f64),
}
