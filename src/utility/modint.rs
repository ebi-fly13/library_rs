use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct static_modint<M: Modulus> {
    val: u32,
    phantom: PhantomData<fn() -> M>,
}

impl<M> static_modint<M>
where
    M: Modulus,
{
    fn new() -> Self {
        Self {
            val: 0,
            phantom: PhantomData,
        }
    }

    fn modulus() -> u32 {
        M::MOD
    }

    fn val(&self) -> u32 {
        self.val
    }
}

impl<M> Add for static_modint<M>
where
    M: Modulus,
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<M> AddAssign for static_modint<M>
where
    M: Modulus,
{
    fn add_assign(&mut self, other: Self) {
        self.val += other.val;
        if self.val >= M::MOD {
            self.val -= M::MOD;
        }
    }
}

impl<M> Sub for static_modint<M>
where
    M: Modulus,
{
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl<M> SubAssign for static_modint<M>
where
    M: Modulus,
{
    fn sub_assign(&mut self, rhs: Self) {
        if self.val < rhs.val {
            self.val += M::MOD;
        }
        self.val -= rhs.val;
    }
}

impl<M> Mul for static_modint<M>
where
    M: Modulus,
{
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self *= rhs;
        self
    }
}

impl<M> MulAssign for static_modint<M>
where
    M: Modulus,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.val = ((self.val as u64 * rhs.val as u64) % M::MOD as u64) as u32;
    }
}

impl<M> Neg for static_modint<M>
where
    M: Modulus,
{
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            val: M::MOD - self.val,
            phantom: PhantomData,
        }
    }
}

pub trait Modulus {
    const MOD: u32;
    const IS_PRIME: bool;
}

pub struct Mod1000000007;

pub struct Mod998244353;

impl Modulus for Mod1000000007 {
    const MOD: u32 = 1_000_000_007;
    const IS_PRIME: bool = true;
}

impl Modulus for Mod998244353 {
    const MOD: u32 = 998_244_353;
    const IS_PRIME: bool = true;
}
