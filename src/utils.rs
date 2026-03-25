/// Calcule le PGCD étendu de `a` et `b`.
/// 
/// Retourne un tuple `(g, u, v)` tel que `a * u + b * v = g`, où `g` est le 
/// plus grand commun diviseur (PGCD) de `a` et `b`.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::utils::extended_gcd;
/// let (g, u, v) = extended_gcd(35, 15);
/// 
/// assert_eq!(g, 5); // Le PGCD de 35 et 15 est 5
/// assert_eq!(u, 1);
/// assert_eq!(v, -2);
/// 
/// // Vérification de l'identité de Bézout : 35*1 + 15*(-2) = 5
/// assert_eq!(35 * u + 15 * v, g);
/// ```
pub fn extended_gcd(mut a: i64, mut b:i64) -> (i64, i64, i64){
    if a==0 { return (b, 0, 1);}

    let (mut ua , mut va) = (1, 0);
    let (mut ub , mut vb) = (0, 1);

    while b != 0 {
        let q = a / b;
        
        (a, b) = (b, a % b);

        (ua, ub) = (ub, ua - q * ub);
        (va, vb) = (vb, va - q * vb);
    }

    (a, ua, va)
}

/// Calcule le PGCD (Plus Grand Commun Diviseur) de `a` et `b`.
/// 
/// # Examples
/// 
/// ```
/// # use lattice_based_messaging::utils::gcd;
/// // Cas classique
/// let result = gcd(48, 18);
/// assert_eq!(result, 6);
/// 
/// // Nombres premiers entre eux
/// let primes = gcd(17, 23);
/// assert_eq!(primes, 1);
/// ```
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}