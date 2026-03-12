use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

pub struct Vector {
    pub size: usize,
    pub data: Vec<f64>,
}

impl Vector {
    pub fn new(size: usize) -> Self {
        let data = vec![0.0; size];

        Self{size, data}
    }

    pub fn dot(&self, other: &Vector) -> f64{
        assert_eq!(self.size, other.size);

        let mut s = 0.0;

        for i in 0..self.size {
            s = s + self.data[i] * other.data[i];
        }
        s
    }

    pub fn norm(&self) -> f64{
        self.dot(self).sqrt()
    }

    pub fn display(&self) {
        print!("[ ");
        for val in &self.data {
            print!("{:>6.2} ", val);
        }
        println!("]");
    }
}


impl Add<&Vector> for &Vector{
    type Output = Vector;
    fn add(self, other: &Vector) -> Vector {
        assert_eq!(self.size, other.size);

        let mut res = Vector::new(self.size);

        for i in 0..self.size {
            res.data[i] = self.data[i] + other.data[i];
        }

        res
    }
}

impl Sub<&Vector> for &Vector{
    type Output = Vector;
    fn sub(self, other: &Vector) -> Vector {
        assert_eq!(self.size, other.size);

        let mut res = Vector::new(self.size);

        for i in 0..self.size {
            res.data[i] = self.data[i] - other.data[i];
        }

        res
    }
}

impl Mul<f64> for &Vector{
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        let mut res = Vector::new(self.size);

        for i in 0..self.size {
            res.data[i] = self.data[i] * scalar;
        }

        res
    }
}