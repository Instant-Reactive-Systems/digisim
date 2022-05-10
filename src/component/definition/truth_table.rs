use std::iter::Zip;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct TruthTable {
    pub inputs: Vec<Vec<bool>>,
    pub outputs: Vec<Vec<bool>>,
}

impl TruthTable {
    pub fn iter(&self) -> Zip<std::slice::Iter<Vec<bool>>, std::slice::Iter<Vec<bool>>> {
        self.inputs.iter().zip(self.outputs.iter())
    }
    
    pub fn iter_mut(&mut self) -> Zip<std::slice::IterMut<Vec<bool>>, std::slice::IterMut<Vec<bool>>> {
        self.inputs.iter_mut().zip(self.outputs.iter_mut())
    }
}

