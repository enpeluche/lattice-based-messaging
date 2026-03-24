use std::f64;
use rand::Rng;
use std::ops::{Add, Sub, Mul, Neg};
use num_traits::{Zero, One};
use std::array;

use crate::vector::Vector;

// TODO: Display
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    pub data: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where T: Default + Copy
{
    pub fn new() -> Self {
        Self{data: [[T::default(); COLS]; ROWS]}
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        
        let mut mat = Self::new();

        for r in 0..ROWS {
            for c in 0..COLS {
                mat.data[r][c] = rng.r#gen::<f64>();
            }
        }

        mat
    }

    pub fn set(&mut self, r: usize, c: usize, val: f64) {
        if r < self.rows && c < self.cols {
            self.data[r][c] = val;
        } else {
            eprintln!("Erreur : Index ({}, {}) hors limites", r, c);
        }
    }

    pub fn transpose(&self) -> Matrix<T, COLS, ROWS> {
        let mut mat = Matrix::<T, COLS, ROWS>::new();

        for r in 0..ROWS {
            for c in 0..COLS{
                mat.data[c][r] = self.data[r][c];
            }
        }

        mat
    }

}


impl<T, const N: usize> Matrix<T, N, N>
where 
    T: Copy + Zero + One 
{
    pub fn identity() -> Self {
        Self {
            data: array::from_fn(|r| {
                array::from_fn(|c| {
                    if r == c {
                        T::one()
                    } else {
                        T::zero()
                    }
                })
            })
        }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Add<Matrix<T, ROWS, COLS>> for Matrix<T, ROWS, COLS>{
    type Output = Matrix;
    fn add(self, other: &Matrix) -> Matrix {
        let mut res = Matrix::new(self.rows, self.cols);

        for r in 0..self.rows {
            for c in 0..self.cols {
                res.data[r][c] = self.data[r][c] + other.data[r][c]
            }
        }

        res
    }
}

impl<T, const ROWS: usize, const COLS: usize> Neg for &Matrix{
    type Output = Matrix;
    fn neg(self) -> Matrix {
        let mut res = Matrix::new(self.rows, self.cols);

        for r in 0..self.rows {
            for c in 0..self.cols {
                res.data[r][c] = -self.data[r][c]
            }
        }

        res
    }
}

impl<T, const ROWS: usize, const COLS: usize> Sub<&Matrix> for &Matrix{
    type Output = Matrix;
    fn sub(self, other: &Matrix) -> Matrix {
        let mut res = Matrix::new(self.rows, self.cols);

        for r in 0..self.rows {
            for c in 0..self.cols {
                res.data[r][c] = self.data[r][c] - other.data[r][c]
            }
        }

        res
    }
}

impl<T, const ROWS: usize, const COLS: usize> Mul<&Vector> for &Matrix{
    type Output = Vector;

    fn mul(self, other: &Vector) -> Vector {
        assert_eq!(self.cols, other.size);

        let mut res = Vector::new(self.rows);
        
        for i in 0..self.rows {
            for k in 0..other.size {
                res.data[i] += self.data[i][k] * other.data[k];
            }
            
        }

        res
    }
}

impl<T, const ROWS: usize, const COLS: usize> Mul<Matrix<T, I, J>> for Matrix<T, J, K>{
    type Output = Matrix<T, I, K>;

    fn mul(self, other: Matrix) -> Matrix<T, I, K> {

        let mut res = Self::Output::new();
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    res.data[i][j] = res.data[i][j] + self.data[i][k] * other.data[k][j];
                }
            }
        }

        res
    }
}