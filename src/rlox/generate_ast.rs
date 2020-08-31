#[macro_export]
macro_rules! generate_ast {
  // entry point
  ($(#[$attr:meta])* struct $name:ident { $($field:ident : $ftype:ty),* $(,)? }) => {
    $(#[$attr])*
    pub struct $name {
      $( $field: $ftype, )*
    }

    impl $name {
      pub fn new(
        $( $field:$ftype, )*
      ) -> $name {
        $name {
          $( $field, )*
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  // use super::*;

  generate_ast!(
    #[derive(Debug)]
    struct Stmt {
      a: String,
      b: bool,
      c: u64,
    }
  );

  #[test]
  fn create_via_new() {
    let stmt = Stmt::new(String::from("Howdy"), true, 10);
    println!("stmt {:?}", stmt);

    assert_eq!(stmt.a, String::from("Howdy"));
    assert_eq!(stmt.b, true);
    assert_eq!(stmt.c, 10);
  }
}
