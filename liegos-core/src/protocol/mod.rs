use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeaningPacket {
    pub ontology: String,
    pub vector: MeaningVector,
    pub payload: serde_json::Value,
}

impl MeaningPacket {
    pub fn new(
        ontology: impl Into<String>,
        vector: MeaningVector,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            ontology: ontology.into(),
            vector,
            payload,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeaningVector {
    pub dimensions: Vec<f32>,
}

impl MeaningVector {
    pub fn new(dimensions: Vec<f32>) -> Self {
        Self { dimensions }
    }

    pub fn magnitude(&self) -> f32 {
        self.dimensions.iter().map(|value| value * value).sum::<f32>().sqrt()
    }
}
