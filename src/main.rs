#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod zq;
mod utils;
mod vector;
use std::iter;

use zq::Zq;

fn main() {
    // Exercice 1 : le filtre inversible
    const Q: u64 = 10;
    let numbers = vec![Zq::<Q>::new(0), Zq::<Q>::new(1),Zq::<Q>::new(2),Zq::<Q>::new(3),Zq::<Q>::new(4),
                                    Zq::<Q>::new(5),Zq::<Q>::new(6),Zq::<Q>::new(7),Zq::<Q>::new(8),Zq::<Q>::new(9)];

    let units: Vec<_> = numbers.iter()
                                           .filter(|x| x.is_unit())
                                           .cloned()
                                           .collect();

    // Exercice 2 : Le produit scalaire
    
    let v1 = vec![Zq::<17>::new(2), Zq::<17>::new(5), Zq::<17>::new(3)];
    let v2 = vec![Zq::<17>::new(4), Zq::<17>::new(1), Zq::<17>::new(6)];

    let dot_product: Zq<17> = v1.iter()
                                   .zip(v2.iter())
                                   .map(|(a, b)| *a * *b)
                                   .sum();

    // Exercice 3 : La recherche d'index

    let data = vec![Zq::<5>::new(1), Zq::<5>::new(3), Zq::<5>::new(0), Zq::<5>::new(2)];

    let zero_index = data.iter()
                                         .position(|x| x.value == 0);

    // Exercice 4 : La matrice à plat

    let matrix_data = (0..9).map(|x| Zq::<100>::new(x)).collect::<Vec<_>>();
    let cols = 3;
    let row_to_get = 1; // La deuxième ligne (index 1)
    
    let row: Vec<_> = matrix_data.iter()
                                           .skip(cols * row_to_get)
                                           .take(cols)
                                           .cloned()
                                           .collect();

    // Exercice 5 : Le challenge "Polynomial"

    let coeffs = vec![Zq::<13>::new(1), Zq::<13>::new(2), Zq::<13>::new(1)]; // 1 + 2x + x^2
    let x = Zq::<13>::new(3);

    let result: Zq<13> = coeffs.iter()
                                  .rev()
                                  .fold(Zq::new(0), |acc, c| acc * x + *c);
}