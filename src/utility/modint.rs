use std::marker::PhantomData;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Div, DivAssign, Neg, Sub, SubAssign};
use crate::utility::internal_type_trait::Zero;

pub type ModInt1000000007 = StaticModInt<Mod1000000007>;
pub type ModInt998244353 = StaticModInt<Mod998244353>;

pub struct StaticModInt<M: Modulus> {
    val: u32,
    phantom: PhantomData<fn() -> M>,
}

impl<M> StaticModInt<M>
where
    M: Modulus,
{
    fn val(&self) -> u32 {
        self.val
    }

    fn pow(self, mut n: u32) -> Self {
        let mut ret = Self {
            val: 1,
            phantom: PhantomData,
        };
        let mut x = self;
        while n > 0 {
            if n & 1 == 1 {
                ret *= x;
            }
            x *= x;
            n >>= 1;
        }
        ret
    }
}

impl<M> Clone for StaticModInt<M>
where
    M: Modulus
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for StaticModInt<M>
where
    M: Modulus
{

}

impl<M> Zero for StaticModInt<M>
where
    M: Modulus
{
    fn zero() -> Self {
        Default::default()
    }
}

impl<M> Add for StaticModInt<M>
where
    M: Modulus,
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<M> AddAssign for StaticModInt<M>
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

impl<M> Sub for StaticModInt<M>
where
    M: Modulus,
{
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl<M> SubAssign for StaticModInt<M>
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

impl<M> Mul for StaticModInt<M>
where
    M: Modulus,
{
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        self *= rhs;
        self
    }
}

impl<M> MulAssign for StaticModInt<M>
where
    M: Modulus,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.val = ((self.val as u64 * rhs.val as u64) % M::MOD as u64) as u32;
    }
}

impl<M> Neg for StaticModInt<M>
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

impl<M> Div for StaticModInt<M>
where
M: Modulus
{
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self {
        self /= rhs;
        self
    }
}

impl<M> DivAssign for StaticModInt<M>
where
M: Modulus
{
    fn div_assign(&mut self, rhs: Self) {
        assert!(M::IS_PRIME);
        assert_ne!(rhs.val, 0);
        *self *= rhs.pow(M::MOD - 2);
    }
}

impl<M> Default for StaticModInt<M>
where
    M: Modulus,
{
    fn default() -> Self {
        Self {
            val: 0,
            phantom: PhantomData,
        }
    }
}

impl<M> fmt::Display for StaticModInt<M>
where
M: Modulus
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val())
    }
}

impl<M: Modulus> From<u32> for StaticModInt<M> {
    fn from(mut val: u32) -> Self {
        if M::MOD <= val {
            val -= M::MOD;
        }
        Self {
            val,
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

#[cfg(test)]
mod tests {
    use crate::utility::modint::{ModInt1000000007, ModInt998244353};
    #[test]
    fn static_modint_add() {
        fn add_998(lhs: u32, rhs: u32) -> u32 {
            (ModInt998244353::from(lhs) + ModInt998244353::from(rhs)).val()
        }

        fn add_1e9(lhs: u32, rhs: u32) -> u32 {
            (ModInt1000000007::from(lhs) + ModInt1000000007::from(rhs)).val()
        }

        assert_eq!(2, add_998(1, 1));
        assert_eq!(1, add_998(998_244_352, 2));

        assert_eq!(2, add_1e9(1,1));
        assert_eq!(1, add_1e9(1_000_000_006, 2));
    }
}