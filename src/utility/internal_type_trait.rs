pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}

pub trait One {
    fn one() -> Self;
}