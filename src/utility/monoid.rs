pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
}

pub struct SumMonoid;

impl Monoid for SumMonoid {
    type S = i64;
    fn e() -> Self::S {
        0
    }
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S {
        lhs + rhs
    }
}