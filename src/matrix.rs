use std::f64;
use rand::Rng;
use std::ops::{Add, Sub, Mul, Neg};

use crate::vector::Vector;



pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];

        Self{rows, cols, data}
    }

    pub fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];

        for i in 0..size {
            data[i][i] = 1.0; // et là on aurait une méthode pour prendre le 1 du type sur lequel on travail ? genre on a un ::ONE pour i64 ou f64 ??
        }
        Self{rows: size, cols: size, data}
    }

    pub fn random(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();
        
        let mut mat = Self::new(rows, cols);

        for r in 0..rows {
            for c in 0..cols {
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

    pub fn transpose(&self) -> Self {
        let mut mat = Matrix::new(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols{
                mat.set(j,i, self.data[i][j]);
            }
        }

        mat
    }

    pub fn display(&self) {
        for row in &self.data {
            print!("[ ");
            for val in row {
                print!("{:>6.2} ", val);
            }
            println!("]");
        }
    }
}

impl Add<&Matrix> for &Matrix{
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

impl Neg for &Matrix{
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

impl Sub<&Matrix> for &Matrix{
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

impl Mul<&Vector> for &Matrix{
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

impl Mul<&Matrix> for &Matrix{
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);

        let mut res = Matrix::new(self.rows, other.cols);
        
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