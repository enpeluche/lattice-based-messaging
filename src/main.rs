mod matrix;
use matrix::Matrix;

fn main() {
    let mut m = Matrix::new(3, 3);
    
    m.set(0, 0, 1.0);
    m.set(1, 1, 1.0);
    m.set(2, 2, 1.0);
    
    println!("Ma matrice identité :");
    m.display();
}
// je suis un commentaire !