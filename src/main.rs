mod matrix;
use matrix::Matrix;

mod vector;
use vector::Vector;

fn main() {
    let m = Matrix::random(3, 3);
    
    println!("Ma matrice :");
    m.display();
    println!("somme");
    (&m+&m.transpose()).display();
}
// je suis un commentaire !

// on adopte quelle convention ? 