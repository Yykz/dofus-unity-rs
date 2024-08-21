use prost::{DecodeError, Message};
use prost_types::Any;

pub fn unpack_any(any: Any) -> Option<Result<Box<dyn Message>, DecodeError>> {
	let Any { type_url, value } = any;    
	DECODE_MAP.get(&type_url).map(move |func| func(&value))
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

include!(concat!(env!("OUT_DIR"), "/_include.rs"));

pub use com::ankama::dofus::server::game::protocol as game;
pub use com::ankama::dofus::server::connection::protocol as connection;



