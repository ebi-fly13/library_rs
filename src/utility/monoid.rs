use num::Zero;
use std::marker::PhantomData;
use std::ops::Add;

pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
}

pub trait MapMonoid {
    type S: Monoid;
    type F: Clone;
    fn e() -> <Self::S as Monoid>::S {
        Self::S::e()
    }
    fn op(lhs: &<Self::S as Monoid>::S, rhs: &<Self::S as Monoid>::S) -> <Self::S as Monoid>::S {
        <Self::S as Monoid>::op(lhs, rhs)
    }
    fn id() -> Self::F;
    fn mapping(f: &Self::F, x: &<Self::S as Monoid>::S) -> <Self::S as Monoid>::S;
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
}

pub struct Sum<T>(PhantomData<fn() -> T>);

impl<T> Monoid for Sum<T>
where
    T: Zero + Add + Copy,
{
    type S = T;
    fn e() -> Self::S {
        Self::S::zero()
    }
    fn op(&lhs: &Self::S, &rhs: &Self::S) -> Self::S {
        lhs + rhs
    }
}
