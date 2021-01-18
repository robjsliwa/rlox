#[derive(Clone, Debug, PartialEq)]
pub struct RloxClass {
  name: String,
}

impl RloxClass {
  pub fn new(name: &str) -> RloxClass {
    RloxClass {
      name: name.to_string(),
    }
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }
}
