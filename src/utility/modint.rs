use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::marker::PhantomData;

pub struct static_modint<M: Modulus> {
    val: u32,
    phantom: PhantomData<fn() -> M>,
}

impl<M> static_modint<M>
where
M: Modulus
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
    M:Modulus
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<M> AddAssign for static_modint<M> 
where
    M: Modulus
{
    fn add_assign(&mut self, other: Self) {
        self.val += other.val;
        if self.val >= M::MOD {
            self.val -= M::MOD;
        }
    }
}

pub trait Modulus {
    const MOD: u32;
    const IS_PRIME : bool;
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