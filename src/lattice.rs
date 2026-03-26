use crate::matrix::Matrix;

pub struct Lattice<const N: usize>  {
    pub basis: Matrix<i64, N, N>,
    pub q: Option<i64>
}