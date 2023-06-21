fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for ri in 0..3 {
        for ci in 0..3 {
            transposed[ri][ci] = matrix[ci][ri];
        }
    }
    transposed
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        for value in row {
            print!("{} ", value)
        }
        println!();
    }
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
