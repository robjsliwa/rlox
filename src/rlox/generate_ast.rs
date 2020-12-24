#[macro_export]
macro_rules! parse_ast_visitor_entry {
  ($visitor_name:ident $t:ident $g:ident) => {
    fn $visitor_name(&self, expr: &$t<$g>) -> Result<T, Error>;
  };
  ($visitor_name:ident $t:ident) => {
    fn $visitor_name(&self, expr: &$t) -> Result<T, Error>;
  };
}
  
#[macro_export]
macro_rules! generate_ast_visitor {
  ($name: ident {
    $($visitor_name:ident $t:ident $($g:ident)?),*,
  }) => {
    pub trait $name<T> {
      fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error>;
    }
      
    pub trait Visitor<T> {
      $(crate::parse_ast_visitor_entry!($visitor_name $t $($g)?);)*
    }
  };
}
  
#[macro_export]
macro_rules! parse_grammar_entry {
  ($visitor_name:ident $name:ident $g:ident {
    $($var_name:ident: $t:ty),+;
  }) => {
    pub struct $name<$g> {
      $(pub $var_name: $t),*,
    }
    
    impl<$g> $name<$g> {
      pub fn new(
        $($var_name: $t),*,
      ) -> $name<$g> {
        $name {
          $($var_name),*,
        }
      }
    }
    
    impl<T> Expr<T> for $name<$g> {
      fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
        visitor.borrow().$visitor_name(self)
      }
    }
  };
  ($visitor_name:ident $name:ident {
    $($var_name:ident: $t:ty),+;
  }) => {
    pub struct $name {
      $(pub $var_name: $t),*,
    }
    
    impl $name {
      pub fn new(
        $($var_name: $t),*,
      ) -> $name {
        $name {
          $($var_name),*,
        }
      }
    }
    
    impl<T> Expr<T> for $name {
      fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
        visitor.borrow().$visitor_name(self)
      }
    }
  };
}
  
#[macro_export]
macro_rules! generate_ast {
  ($root_name: ident {
    $($visitor_name:ident $name:ident $($g:ident)* => $($var_name:ident: $t:ty),+;)+
  }) => {
    crate::generate_ast_visitor!($root_name {
      $($visitor_name $name $($g)*),*,
    });
    $(crate::parse_grammar_entry!($visitor_name $name $($g)* {
      $($var_name: $t),+;
    });)+
  };
}
  
  
  
