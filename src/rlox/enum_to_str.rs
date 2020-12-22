#[macro_export]
macro_rules! enum_to_str {
    (#[$m:meta] $vis:vis enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        #[$m]
        $vis enum $name {
            $($variant = $val),*,
        }

        impl $name {
            $vis fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
    (#[$m:meta] $vis:vis enum $name:ident {
        $($variant:ident),*,
    }) => {
        #[$m]
        $vis enum $name {
            $($variant),*,
        }

        impl $name {
            $vis fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}