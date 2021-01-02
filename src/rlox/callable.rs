use super::{
  interpreter::{ Interpreter },
  rlox_type::RloxType,
  rlox_errors::RloxError,
};

pub trait Callable: CallableClone {
  fn arity(&self) -> usize;
  fn call(&self, interpreter: &Interpreter, arguments: Vec<RloxType>) -> Result<RloxType, RloxError>;
  fn name(&self) -> String;
}

pub trait CallableClone {
  fn clone_box(&self) -> Box<dyn Callable>;
}

impl<T> CallableClone for T
where
  T: 'static + Callable + Clone,
{
  fn clone_box(&self) -> Box<dyn Callable> {
      Box::new(self.clone())
  }
}

impl Clone for Box<dyn Callable> {
  fn clone(&self) -> Box<dyn Callable> {
      self.clone_box()
  }
}

impl std::fmt::Debug for Box<dyn Callable> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
      write!(fmt, "callable");
      Ok(()) 
  }
}

impl PartialEq for Box<dyn Callable> {
  fn eq(&self, other: &Self) -> bool {
      self == other
  }
}
