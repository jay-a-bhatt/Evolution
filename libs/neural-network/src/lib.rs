pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Everything above was default example code

// TODO - Coding: new() section in https://pwy.io/posts/learning-to-fly-pt2/

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        return self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
        // Above is the same as
        // for layer in &self.layers {
        //     inputs = layer.propagate(inputs);
        // }
    }

    pub fn new(layers: Vec<Layer>) -> Self {
        return Self{ layers }
    }

    // Randomize network by taking the # of layers,
    // and the # of neurons per layer
    pub fn random(layers: Vec<LayerTopology>) -> Self {
        // Check if there is > 1 layer
        assert_eq!(layers.len() > 1);

        for adjacent_layers in layers.windows(2) { // .windows allows us to iterate by adjacent items
            let input_size = adjacent_layers[0].neurons;
            let output_size = adjacent_layers[1].neurons;
        }

        // Above is the same as
        // let mut built_layers = Vec::new();
        // for i in 0..(layers.len() - 1) {
        //     let input_size = layers[i].neurons;
        //     let output_size = layers[i + 1].neurons;

        // built_layers.push(Layer::random(
        //     input_size,
        //     output_size,
        // ))
        }
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
        // Above is the same as
        // let mut outputs = Vec::new();
        // for neuron in &self.neurons {
        //     let output = neuron.propagate(&inputs);
        //     outputs.push(output);
        // }
        // return outputs
    }
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        // Usually catching and handling the error would be better, but:
        // 1. If the assertion fails, the implementation is wrong and the 
        //    end user can't do anything on their side
        // 2. Just a little project and not in prod., so no need to waste time
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        return (self.bias + output).max(0.0)
        // Above let and return statement same as 
        // let mut output = 0.0;
        // 
        // for i in 0..inputs.len() {
        //     output += inputs[i] * self.weights[i];
        // }
        //     
        // output += self.bias;
        // 
        // if output > 0.0 {
        //     output
        // } else {
        //     0.0
        // }
    }
}