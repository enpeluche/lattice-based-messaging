use rand::Rng;
use std::ops::{Add, Sub, Mul, Neg, AddAssign, MulAssign, SubAssign};
use crate::utils::extended_gcd;

// L'algorithme de l'inverse (Euclide étendu).

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zq<const Q: i64>{
    pub value: i64,
}

impl<const Q: i64> Zq<Q> {
    pub fn new(value: i64) -> Self {
        assert!(Q > 0);

        Self{value:((value % Q) + Q) % Q}
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        let random_value = rng.gen_range(0..Q);

        Zq::new(random_value)
    }

    pub fn inv(&self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.value, Q);

        if g != 1 {
            return None;
        }

        Some(Zq::new(x))

    }

    pub fn display(&self) {
        print!("{} (mod {})", self.value, Q);
        
    }
}

impl<const Q: i64> Add<&Zq<Q>> for &Zq<Q> {
    type Output = Zq<Q>;

    fn add(self, rhs: &Zq<Q>) -> Self::Output {
        Zq::new(self.value + rhs.value)
    }

}

impl<const Q: i64> Sub<&Zq<Q>> for &Zq<Q> {
    type Output = Zq<Q>;

    fn sub(self, rhs: &Zq<Q>) -> Self::Output {
        Zq::new(self.value - rhs.value)
    }
}

impl<const Q: i64> Mul<&Zq<Q>> for &Zq<Q> {
    type Output = Zq<Q>;

    fn mul(self, rhs: &Zq<Q>) -> Self::Output {
        Zq::new(self.value * rhs.value)
    }
}

impl<const Q: i64> Neg for &Zq<Q> {
    type Output = Zq<Q>;

    fn neg(self) -> Self::Output {
        Zq::new(-self.value)
    }
}

impl<const Q: i64> AddAssign for Zq<Q> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Zq::new(self.value + rhs.value);
    }
}

impl<const Q: i64> SubAssign for Zq<Q> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Zq::new(self.value - rhs.value);
    }
}

impl<const Q: i64> MulAssign for Zq<Q> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Zq::new(self.value * rhs.value);
    }
}