use std::ops::Add;
use std::marker::PhantomData;
use num::Zero;

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

pub struct Sum<T>(PhantomData<fn() -> T>);

impl<T> Monoid for Sum<T>
where 
T: Zero + Add + Copy
{
    type S = T;
    fn e() -> Self::S {
        Self::S::zero()
    }
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S {
        *lhs + *rhs
    }
}