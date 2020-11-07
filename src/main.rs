use crate::layer::Layer;
use crate::matrix::Matrix;

mod layer;
mod matrix;


fn main() {

    // Inputs
    let X = Matrix {
        n: 3,
        p: 4,
        val: vec![
            vec![1.0, 2.0, 3.0, 2.5],
            vec![2.0, 5.0, -1.0, 2.0],
            vec![-1.5, 2.7, 3.3, -0.8]
        ]
    };

    // Initialize layer
    let l = Layer::new(3, 4);

    // Compute outputs
    let outputs = l.forward(X);

    // Display
    println!("Outputs: {?}", outputs.val);

}
