use std::{array, iter::Sum, ops::{Add, Mul, Neg, Sub}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

impl<T, const N: usize> Vector<T, N>
where T: Copy + Default + Add<Output = T> + Mul<Output = T> + Sum
{
    pub fn new() -> Self {Self{data: [T::default(); N]}}

    pub fn dot(&self, other: &Self) -> T{
        self.data.iter()
                 .zip(other.data.iter())
                 .map(|(a,b)| *a * *b)
                 .sum()
    }
}

impl<T, const N: usize> std::fmt::Display for Vector<T, N>
where 
    T: std::fmt::Display 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        
        for (i, val) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", val)?;
        }
        
        write!(f, "]")?;
        Ok(())
    }
}

impl<T, const N: usize> Add<Vector<T, N>> for Vector<T, N>
where T: Copy + Add<Output = T>
{
    type Output = Vector<T, N>;
    fn add(self, other: Vector<T, N>) -> Vector<T, N> {
        Self::Output {
            data: array::from_fn(|i| self.data[i] + other.data[i])
        }
    }
}

impl<T, const N: usize> Sub<Vector<T, N>> for Vector<T, N>
where T: Copy + Sub<Output = T>
{
    type Output = Vector<T, N>;
    fn sub(self, other: Vector<T, N>) -> Vector<T, N> {
        Self::Output {
            data: array::from_fn(|i| self.data[i] - other.data[i])
        }
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where T: Copy + Mul<Output = T>
{
    type Output = Vector<T, N>;
    fn mul(self, scalar: T) -> Vector<T, N> {
        Self::Output {
            data: array::from_fn(|i| self.data[i] * scalar)
        }
    }
}

impl<T, const N: usize> Neg for Vector<T, N>
where T: Copy + Neg<Output = T>
{
    type Output = Vector<T, N>;

    fn neg(self) -> Vector<T, N> {
        Self::Output {
            data: array::from_fn(|i| -self.data[i])
        }
    }
}