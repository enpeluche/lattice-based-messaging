use crate::matrix::Matrix;
use crate::vector::Vector;

impl<const ROWS: usize, const COLS: usize> Matrix<f64, ROWS, COLS>
{
    pub fn gram_schmidt(&self) -> (Matrix<f64, ROWS, ROWS>,  Matrix<f64, ROWS, COLS>, Vector<f64, ROWS>){
        assert_ne!(ROWS, 0);

        let mut u = Matrix::<f64, ROWS, ROWS>::identity();
        let mut q = Matrix::<f64, ROWS, COLS>::new();
        let mut norms_squared = Vector::<f64, ROWS>::new();

        q.data[0] = self.data[0];

        norms_squared[0] = q.row(0).norm_squared();

        for i in 1..ROWS {
            let b_i = self.row(i);

            let mut q_i = b_i;

            for j in 0..i {
                if norms_squared[j].abs() > 1e-12 {
                    let q_j = q.row(j);

                    let mu = b_i.dot(&q_j) / norms_squared[j];
                    u.data[i][j] = mu;

                    q_i = q_i - (q_j * mu);
                }        
            }
        q.data[i] = q_i.data;
        norms_squared[i] = q_i.norm_squared();
        }

        (u, q, norms_squared)
    }
}