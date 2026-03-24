use rand::Rng;
use std::ops::{Add, Sub, Mul, Neg, AddAssign, MulAssign, SubAssign};
use num_traits::{Zero, One};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zq<const Q: u64>{
    pub value: i64,
}

impl<const Q: u64> Default for Zq<Q>  {
    fn default() -> Self { Self::new(0)}
}

impl<const Q: u64> From<i64> for Zq<Q> {
    fn from(v: i64) -> Self { Self::new(v)}
}

impl<const Q: u64> Zero for Zq<Q> {
    fn zero() -> Self {
        Self::new(0)
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }
}

impl<const Q: u64> One for Zq<Q> {
    fn one() -> Self {
        Self::new(1)
    }    
}

impl<const Q: u64> Zq<Q> {
    pub fn zero() -> Self {Self::new(0)}
    pub fn one() -> Self {Self::new(1)}

    pub fn is_zero(&self) -> bool {self.value==0}
    pub fn is_one(&self) -> bool {self.value==1}
}

impl<const Q: u64> std::fmt::Display for Zq<Q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const Q: u64> Zq<Q> {
    pub fn new(value: i64) -> Self {
        assert!(Q > 0, "Modulo Q doit être > 0");

        let q = Q as i64;

        Self{value:((value % q) + q) % q}
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        let random_value = rng.gen_range(0..Q);

        Zq::new(random_value as i64)
    }

    pub fn inv(&self) -> Option<Self> {
        let (g, x, _) = crate::utils::extended_gcd(self.value, Q as i64);

        if g != 1 {
            return None;
        }

        Some(Zq::new(x))

    }

    pub fn is_unit(&self) -> bool {
        crate::utils::gcd(self.value, Q as i64) == 1
    }
}

impl<const Q: u64> Add<Zq<Q>> for Zq<Q> {
    type Output = Zq<Q>;

    fn add(self, rhs: Zq<Q>) -> Self::Output {
        Zq::new(self.value + rhs.value)
    }

}

impl<const Q: u64> Sub<Zq<Q>> for Zq<Q> {
    type Output = Zq<Q>;

    fn sub(self, rhs: Zq<Q>) -> Self::Output {
        Zq::new(self.value - rhs.value)
    }
}

impl<const Q: u64> Mul<Zq<Q>> for Zq<Q> {
    type Output = Zq<Q>;

    fn mul(self, rhs: Zq<Q>) -> Self::Output {
        Zq::new(self.value * rhs.value)
    }
}

impl<const Q: u64> Neg for Zq<Q> {
    type Output = Zq<Q>;

    fn neg(self) -> Self::Output {
        Zq::new(-self.value)
    }
}

impl<const Q: u64> AddAssign for Zq<Q> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const Q: u64> SubAssign for Zq<Q> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const Q: u64> MulAssign for Zq<Q> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const Q: u64> std::iter::Sum for Zq<Q> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl<const Q: u64> std::iter::Product for Zq<Q> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |a, b| a * b)
    }
}

impl<const Q: u64> std::ops::Div for Zq<Q> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let inv = rhs.inv().expect("Erreur : division par un élément non inversible dans Zq");
        self * inv
    }
}