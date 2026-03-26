use rand::Rng;
use std::ops::{Add, Sub, Mul, Neg, AddAssign, MulAssign, SubAssign};
use num_traits::{Zero, One};
use rand_distr::{Normal, Distribution};

// TODO: faire la fonction indicatrice d'Euler, voir pour crypto RNG
// évaluer pour quel Q on utilise la crypto en générale

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

/// Implémente l'élément neutre de l'addition pour Z_q.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// # use num_traits::Zero;
/// let a = Zq::<7>::zero();
/// assert_eq!(a.value, 0);
/// assert!(a.is_zero());
/// ```
impl<const Q: u64> Zero for Zq<Q> {
    fn zero() -> Self {
        Self::new(0)
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }
}

/// Implémente l'élément neutre de la multiplication pour Z_q.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// # use num_traits::One;
/// let a = Zq::<7>::one();
/// assert_eq!(a.value, 1);
/// assert!(a.is_one());
/// ```
impl<const Q: u64> One for Zq<Q> {
    fn one() -> Self {
        Self::new(1)
    }
    fn is_one(&self) -> bool {
        self.value == 1
    }
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

    pub fn random_gaussian(sigma: f64) -> Self {
        let mut rng = rand::thread_rng();
        
        let normal = Normal::new(0.0, sigma).expect("Sigma doit être strictement positif");
        
        let sample = normal.sample(&mut rng);
        
        let discrete_sample = sample.round() as i64;
        
        Zq::new(discrete_sample)
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

/// Calcule l'addition de deux éléments dans $Z_q$.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// let a = Zq::<7>::new(4);
/// let b = Zq::<7>::new(5);
/// assert_eq!((a + b).value, 2); // 9 mod 7
/// ```
impl<const Q: u64> Add<Zq<Q>> for Zq<Q> {
    type Output = Zq<Q>;

    fn add(self, rhs: Zq<Q>) -> Self::Output {
        Zq::new((self.value as i128 + rhs.value as i128) as i64)
    }

}

/// Calcule l'opposé d'un élément dans $Z_q$ (opérateur unaire `-`).
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// let a = Zq::<7>::new(2);
/// assert_eq!((-a).value, 5); // L'opposé de 2 est 5 car 2 + 5 = 7 = 0 mod 7
/// ```
impl<const Q: u64> Neg for Zq<Q> {
    type Output = Zq<Q>;

    fn neg(self) -> Self::Output {
        Zq::new(-self.value)
    }
}


/// Calcule la soustraction de deux éléments dans $Z_q$.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// let a = Zq::<7>::new(2);
/// let b = Zq::<7>::new(5);
/// assert_eq!((a - b).value, 4); // -3 mod 7
/// ```
impl<const Q: u64> Sub<Zq<Q>> for Zq<Q> {
    type Output = Zq<Q>;

    fn sub(self, rhs: Zq<Q>) -> Self::Output {
        Zq::new(self.value - rhs.value)
    }
}

/// Calcule la multiplication de deux éléments dans $Z_q$.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// let a = Zq::<7>::new(3);
/// let b = Zq::<7>::new(4);
/// assert_eq!((a * b).value, 5); // 12 mod 7
/// ```
impl<const Q: u64> Mul<Zq<Q>> for Zq<Q> {
    type Output = Zq<Q>;

    fn mul(self, rhs: Zq<Q>) -> Self::Output {
        Zq::new((self.value as i128 * rhs.value as i128) as i64)
    }
}


/// Calcule la division dans $Z_q$.
/// 
impl<const Q: u64> std::ops::Div for Zq<Q> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let inv = rhs.inv().expect("Erreur : division par un élément non inversible dans Zq");
        self * inv
    }
}

/// Permet l'addition sur place avec l'opérateur `+=`.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// let mut a = Zq::<7>::new(4);
/// a += Zq::<7>::new(5); // 4 + 5 = 9 => 2 modulo 7
/// assert_eq!(a, Zq::<7>::new(2));
/// ```
impl<const Q: u64> AddAssign for Zq<Q> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Permet la soustraction sur place avec l'opérateur `-=`.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// let mut a = Zq::<7>::new(2);
/// a -= Zq::<7>::new(5); // 2 - 5 = -3 => 4 modulo 7
/// assert_eq!(a, Zq::<7>::new(4));
/// ```
impl<const Q: u64> SubAssign for Zq<Q> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Permet la multiplication sur place avec l'opérateur `*=`.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// let mut a = Zq::<7>::new(3);
/// a *= Zq::<7>::new(4); // 3 * 4 = 12 => 5 modulo 7
/// assert_eq!(a, Zq::<7>::new(5));
/// ```
impl<const Q: u64> MulAssign for Zq<Q> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

/// Permet de calculer la somme de tous les éléments d'un itérateur de `Zq<Q>`.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// // On travaille dans Z/7Z
/// let valeurs = vec![Zq::<7>::new(3), Zq::<7>::new(5), Zq::<7>::new(2)];
/// 
/// // 3 + 5 = 8 (=> 1). Puis 1 + 2 = 3.
/// let somme: Zq<7> = valeurs.into_iter().sum();
/// 
/// assert_eq!(somme, Zq::<7>::new(3));
/// ```
impl<const Q: u64> std::iter::Sum for Zq<Q> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

/// Permet de calculer le produit de tous les éléments d'un itérateur de `Zq<Q>`.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::zq::Zq;
/// // On travaille dans Z/7Z
/// let valeurs = vec![Zq::<7>::new(3), Zq::<7>::new(4), Zq::<7>::new(2)];
/// 
/// // 3 * 4 = 12 (=> 5). Puis 5 * 2 = 10 (=> 3).
/// let produit: Zq<7> = valeurs.into_iter().product();
/// 
/// assert_eq!(produit, Zq::<7>::new(3));
/// ```
impl<const Q: u64> std::iter::Product for Zq<Q> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |a, b| a * b)
    }
}