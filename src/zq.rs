use rand::Rng;
use std::ops::{Add, Sub, Mul, Neg};

// L'algorithme de l'inverse (Euclide étendu).

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zq{
    pub value: i64,
    pub q: i64,
}

impl Zq {
    pub fn new(value: i64, q: i64) -> Self {
        assert!(q > 0);

        Self{value:((value % q) + q) % q, q}
    }

    pub fn random(q: i64) -> Self {
        assert!(q > 0);

        let mut rng = rand::thread_rng();

        let random_value = rng.gen_range(0..q);

        Zq::new(random_value, q)
    }

    pub fn display(&self) {
        print!("{} (mod {})", self.value, self.q);
        
    }
}

impl Add<&Zq> for &Zq {
    type Output = Zq;

    fn add(self, rhs: &Zq) -> Zq {
        assert_eq!(self.q, rhs.q);

        Zq::new(self.value + rhs.value, self.q)
    }

}

impl Sub<&Zq> for &Zq {
    type Output = Zq;

    fn sub(self, rhs: &Zq) -> Zq {
        assert_eq!(self.q, rhs.q);

        Zq::new(self.value - rhs.value, self.q)
    }
}

impl Mul<&Zq> for &Zq {
    type Output = Zq;

    fn mul(self, rhs: &Zq) -> Zq {
        assert_eq!(self.q, rhs.q);

        Zq::new(self.value * rhs.value, self.q)
    }
}

impl Neg for &Zq {
    type Output = Zq;

    fn neg(self) -> Zq {
        Zq::new(-self.value, self.q)
    }
}