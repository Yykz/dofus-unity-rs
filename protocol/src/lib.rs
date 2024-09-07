use prost::{DecodeError, Message};
use prost_types::Any;
/*
#[derive(Debug)]
pub enum AnyUnpackError {
    InvalidTypeUrl,
    DecodeError(DecodeError),
}

impl From<DecodeError> for AnyUnpackError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

pub fn unpack_any(any: &Any) -> Result<AnyMessage, AnyUnpackError> {
    let (type_url, value) = (&any.type_url, &any.value);
    match ANYMESSAGE_MAP.get(type_url).map(move |func| func(value)) {
        None => Err(AnyUnpackError::InvalidTypeUrl),
        Some(r) => r.map_err(|e| AnyUnpackError::DecodeError(e)),
    }
}*/

include!(concat!(env!("OUT_DIR"), "/_include.rs"));

#[doc(inline)]
pub use com::ankama::dofus::server::connection::protocol as connection;
#[doc(inline)]
pub use com::ankama::dofus::server::game::protocol as game;
