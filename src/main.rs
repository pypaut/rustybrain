use crate::matrix::Matrix;

mod matrix;


fn main() {

    // Weights
    let weights = Matrix {
        n: 3,
        p: 4,
        val: vec![
            vec![0.2, 0.8, -0.5, 1.0],
            vec![0.5, -0.91, 0.26, -0.5],
            vec![-0.26, -0.27, 0.17, 0.87]
        ]
    };

    // Biases
    let biases = Matrix {
        n: 3,
        p: 3,
        val: vec![
            vec![2.0, 3.0, 0.5],
            vec![2.0, 3.0, 0.5],
            vec![2.0, 3.0, 0.5]
        ]
    };

    // Inputs
    let inputs = Matrix {
        n: 3,
        p: 4,
        val: vec![
            vec![1.0, 2.0, 3.0, 2.5],
            vec![2.0, 5.0, -1.0, 2.0],
            vec![-1.5, 2.7, 3.3, -0.8]
        ]
    };

    // Outputs
    let outputs = inputs.mult(&weights.transposed()) + biases;

    // Display
    println!("Output : {:?}", outputs.val);

}
