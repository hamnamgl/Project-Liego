use crate::protocol::{MeaningPacket, MeaningVector};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("input must not be empty")]
    EmptyInput,
    #[error("packet json is invalid: {0}")]
    InvalidJson(#[from] serde_json::Error),
}

pub fn encode_meaning(ontology: impl Into<String>, input: &str) -> Result<MeaningPacket, CodecError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CodecError::EmptyInput);
    }

    let vector = MeaningVector::new(text_vector(trimmed));
    let payload = Value::String(trimmed.to_owned());

    Ok(MeaningPacket::new(ontology, vector, payload))
}

pub fn decode_meaning(json: &str) -> Result<MeaningPacket, CodecError> {
    Ok(serde_json::from_str(json)?)
}

fn text_vector(input: &str) -> Vec<f32> {
    let bytes = input.as_bytes();
    let len = bytes.len() as f32;
    let sum = bytes.iter().map(|byte| *byte as f32).sum::<f32>();
    let vowels = input
        .chars()
        .filter(|ch| matches!(ch.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count() as f32;

    vec![len, sum / len, vowels / len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_non_empty_text() {
        let packet = encode_meaning("default", "hello").unwrap();

        assert_eq!(packet.ontology, "default");
        assert_eq!(packet.payload, Value::String("hello".to_owned()));
        assert_eq!(packet.vector.dimensions.len(), 3);
    }

    #[test]
    fn rejects_empty_text() {
        assert!(matches!(encode_meaning("default", " "), Err(CodecError::EmptyInput)));
    }
}
