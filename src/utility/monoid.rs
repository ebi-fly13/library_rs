use crate::utility::internal_type_trait::{Zero, One};
use std::marker::PhantomData;
use std::ops::{Add, Mul};

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
    T: Zero + Add<Output = T> + Copy,
{
    type S = T;
    fn e() -> Self::S {
        Self::S::zero()
    }
    fn op(&lhs: &Self::S, &rhs: &Self::S) -> Self::S {
        lhs + rhs
    }
}

pub struct MapAddSumMonoid<S>(PhantomData<fn() -> S>);

impl<S> Monoid for MapAddSumMonoid<S>
where
    S: Zero + Add<Output = S> + Copy,
{
    type S = (S, S);
    fn e() -> Self::S {
        (S::zero(), S::zero())
    }
    fn op(&lhs: &Self::S, &rhs: &Self::S) -> Self::S {
        (lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
}

impl<T> MapMonoid for MapAddSumMonoid<T>
where
    T: Zero + Add<Output = T> + Mul<Output = T> + Copy,
{
    type S = MapAddSumMonoid<T>;
    type F = T;
    fn id() -> Self::F {
        Self::F::zero()
    }
    fn mapping(&f: &Self::F, &x: &<Self::S as Monoid>::S) -> <Self::S as Monoid>::S {
        (x.0 + f * x.1, x.1)
    }
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

pub struct MapAffineSumMonoid<S>(PhantomData<fn() -> S>);

impl<S> Monoid for MapAffineSumMonoid<S>
where
    S: Zero + Add<Output = S> + Copy,
{
    type S = (S, S);
    fn e() -> Self::S {
        (S::zero(), S::zero())
    }
    fn op(&lhs: &Self::S, &rhs: &Self::S) -> Self::S {
        (lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
}

impl<T> MapMonoid for MapAffineSumMonoid<T>
where
    T: Zero + One + Add<Output = T> + Mul<Output = T> + Copy,
{
    type S = MapAddSumMonoid<T>;
    type F = (T, T);
    fn id() -> Self::F {
        (T::one(), T::zero())
    }
    fn mapping(&f: &Self::F, &x: &<Self::S as Monoid>::S) -> <Self::S as Monoid>::S {
        (x.0 * f.0 + f.1, x.1)
    }
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        (f.0 * g.0, f.0 * g.1 + f.1)
    }
}