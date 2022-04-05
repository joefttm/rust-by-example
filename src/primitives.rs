use std::fmt;
use std::mem;

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: &mut Matrix) -> &Matrix {
    let tmp = matrix.1;
    matrix.1 = matrix.2;
    matrix.2 = tmp;
    matrix
}

pub fn joe_test_primitive() {
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("One million is written as {}", 1_000_000u32);
    let mut matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(&mut matrix));
    // array and slice
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array occupies {} bytes", mem::size_of_val(&xs));
}