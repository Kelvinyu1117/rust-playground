// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed_matrix = matrix;

    for i in 0..3 {
        for j in 0..3{
            transposed_matrix[i][j] = matrix[j][i];
        }
    }

    transposed_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    println!("[");
    for i in 0..3 {
        print!("  [ ");
        for j in 0..3{
            print!("{} ", matrix[i][j]);
        }
        println!("]")
    }
    println!("]");
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
