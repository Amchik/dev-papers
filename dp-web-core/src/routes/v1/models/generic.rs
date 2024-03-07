#[macro_export]
macro_rules! define_types {
    ($(#[$a:meta])* $v:vis enum $i:ident: $t:ty { $($(#[$av:meta])* $var:ident = $l:literal),+ $(,)? }) => {
        $(#[$a])*
        #[repr($t)]
        $v enum $i {
            $($(#[$av])* $var = $l),+
        }

        impl $i {
            /// Obrain type from integer value.
            /// # Panics
            /// This function panics if integer value is not variant of this enum.
            pub const fn from_bits(v: $t) -> Self {
                match v {
                    $($l => Self::$var),+
                    , _ => panic!(concat!("Invalid integer value for types ", stringify!($i))),
                }
            }

            /// Convert type to string
            #[allow(dead_code)]
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$var => stringify!($var)),+
                }
            }

            /// All possible values for
            #[doc = concat!(" [`", stringify!($i), "`]")]
            #[allow(dead_code)]
            pub const ALL_VALUES: &'static [Self] = &[$(Self::$var),+];
        }
    };
}
