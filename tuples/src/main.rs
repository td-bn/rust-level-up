use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})\n({} {})\n", self.0, self.1, self.2, self.3)
    }
}

// Returns a new Matrix that is the transpose of the given Matrix
fn transpose(mat: Matrix) -> Matrix {
    Matrix(mat.0, mat.2, mat.1, mat.3)
}

fn main() {
    let mat = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", mat);

    println!("{}", transpose(mat));
}
