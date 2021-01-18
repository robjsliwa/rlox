use super::rlox_class::RloxClass;

#[derive(Clone, Debug, PartialEq)]
pub struct RloxInstance {
  klass: RloxClass,
}

impl RloxInstance {
  pub fn new(klass: RloxClass) -> RloxInstance {
    RloxInstance {
      klass,
    }
  }

  pub fn as_string(&self) -> String {
    format!("{} instance", self.klass.class_name())
  }
}
