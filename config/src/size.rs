macro_rules! size {
    ($name:ident, $prefix:ident, $($size: expr),*) => {
        paste::item! {
            pub enum $name {
                $(
                    #[doc = concat!(stringify!($prefix), stringify!($size))]
                    [<$prefix $size>],
                )*
            }
            impl $name {
                pub fn class(&self) -> &'static str {
                    use $name::*;
                    match *self {
                        $(
                            [<$prefix $size>] =>
                                const_str::concat!(casey::lower!(stringify!($prefix)), "-", const_str::replace!(const_str::replace!(stringify!($prefix), "s", "/"), "_", ".")),
                        )*
                    }
                }
            }
        }
    };
}

size!(
    Height, H, 0, px, 1, 1_5, 2, 2_5, 3, 3_5, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 20, 24, 28, 32,
    36, 40, 44, 48, 52, 56, 60, 64, 80, 96, auto, "1s2", "1s3", "1s4", "2s4", "3s4", "1s5", "2s5",
    "3s5", "4s5", "1s6", "2s6", "3s6", "4s6", "5s6", full, screen, min, max, fit
);
size!(
    Width, W, 0, px, 1, 1_5, 2, 2_5, 3, 3_5, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 20, 24, 28, 32,
    36, 40, 44, 48, 52, 56, 60, 64, 80, 96, auto, "1s2", "1s3", "1s4", "2s4", "3s4", "1s5", "2s5",
    "3s5", "4s5", "1s6", "2s6", "3s6", "4s6", "5s6", full, screen, min, max, fit
);
