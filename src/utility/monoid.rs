pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
}

pub trait MapMonoid: Monoid {
    type F: Clone;
    fn id() -> Self::F;
    fn mapping(f: &Self::F, s: &Self::S) -> Self::S;
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
}
