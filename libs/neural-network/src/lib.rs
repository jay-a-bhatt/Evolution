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