#[macro_use(c)]
extern crate cute;

use crate::neuron::Neuron;

mod neuron;


fn main() {

    // Weights
    let weights = [
        vec![ 0.2,   0.8, -0.5,   1.0],
        vec![ 0.5,  -0.91, 0.26, -0.5],
        vec![-0.26, -0.27, 0.17,  0.87]
    ];

    // Biases
    let biases = vec![2.0, 3.0, 0.5];

    // Neurons
    let n1 = Neuron {
                size: 4,
                weights: &weights[0],
                bias: biases[0]
    };
    let n2 = Neuron {
                size: 4,
                weights: &weights[1],
                bias: biases[1]
    };
    let n3 = Neuron {
                size: 4,
                weights: &weights[2],
                bias: biases[2]
    };

    // Inputs
    let inputs = vec![
        vec![1.0, 2.0, 3.0, 2.5],
        vec![2.0, 5.0, -1.0, 2.0],
        vec![-1.5, 2.7, 3.3, -0.8]
    ];

    // Outputs
    let outputs = vec![
        c![n1.output(&input), for input in &inputs],
        c![n2.output(&input), for input in &inputs],
        c![n3.output(&input), for input in &inputs]
    ];

    // Display
    println!("Output : {:?}", outputs);

    // NOTE : output is transposed, since we usually want one column for one neuron,
    // and one row for one input.

}
