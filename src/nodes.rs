use super::errors::ResultError;

pub struct BasicNode {
    weights: Vec<f64>,
    bias: f64,
}

impl BasicNode {
    pub fn get_size(&self) -> usize {
        self.weights.len()
    }

    pub fn get_weights(&self) -> &Vec<f64> {
        &self.weights
    }

    pub fn get_result(&self, input: &[f64])-> Result<f64, ResultError> {
        if input.len() == self.get_size() {
            let mut results: f64 = 0.0;
            for (item1, item2) in (self.get_weights()).into_iter().zip(input) {
                results += *item1 * *item2;
            }
            Ok(results + self.bias)
    
        } else {
            Err(ResultError::DoesNotMatch)
        }
        
    }
}

pub fn node_constructor(weight: &[f64], bias: f64) -> BasicNode {
    BasicNode {
        weights: weight.to_vec(),
        bias: bias
    }
}