
pub trait One {
    fn one() -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}

impl One for f32 {
    fn one() -> Self {
        1.0
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0
    }
}

macro_rules! impl_one_zero_int {
    ($_type:ty) => {
        impl One for $_type {
            fn one() -> Self { 1 }
        }

        impl Zero for $_type {
            fn zero() -> Self { 0 }
        }
    }
}

impl_one_zero_int!(i8);
impl_one_zero_int!(i16);
impl_one_zero_int!(i32);
impl_one_zero_int!(i64);