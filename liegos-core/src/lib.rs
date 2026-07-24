pub mod codec;
pub mod protocol;

pub use codec::{decode_meaning, encode_meaning, CodecError};
pub use protocol::{MeaningPacket, MeaningVector};
