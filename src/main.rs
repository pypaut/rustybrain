use crate::neuron::Neuron;

mod neuron;


fn main() {

    // Weights
    let weights1 = vec![ 0.2,   0.8, -0.5,   1.0];
    let weights2 = vec![ 0.5,  -0.91, 0.26, -0.5];
    let weights3 = vec![-0.26, -0.27, 0.17,  0.87];

    // Biases
    let b1 = 2f32;
    let b2 = 3f32;
    let b3 = 0.5f32;

    // Neurons
    let n1 = Neuron {
                size: 4,
                weights: weights1,
                bias: b1
    };
    let n2 = Neuron {
                size: 4,
                weights: weights2,
                bias: b2
    };
    let n3 = Neuron {
                size: 4,
                weights: weights3,
                bias: b3
    };

    // Inputs
    let input = vec![1f32, 2f32, 3f32, 2.5f32];

    // Outputs
    let outputs = vec![
        n1.output(&input),
        n2.output(&input),
        n3.output(&input)
    ];

    // Display
    println!("Output : {:?}", outputs);

}
