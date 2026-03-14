use rand::Rng;
use std::ops::{Add, Sub, Mul, Neg, AddAssign, MulAssign, SubAssign};

// TODO : implement one, zero, Display, Sum, Prod

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

impl<const Q: u64> Zq<Q> {
    pub fn new(value: i64) -> Self {
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

    pub fn display(&self) {
        print!("{} (mod {})", self.value, Q);
        
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