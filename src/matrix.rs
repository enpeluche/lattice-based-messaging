pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];

        Self{ rows, cols, data}
    }

    pub fn set(&mut self, r: usize, c: usize, val: f64) {
        if r < self.rows && c < self.cols {
            self.data[r][c] = val;
        } else {
            // En Rust, on évite les print de debug, 
            // mais pour l'instant on va juste ignorer les mauvais indices.
            eprintln!("Erreur : Index ({}, {}) hors limites", r, c);
        }
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