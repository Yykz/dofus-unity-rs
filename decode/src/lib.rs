use std::marker::PhantomData;

use bytes::BytesMut;
use dofus_protocol::{connection, game, unpack_any};
use prost::{DecodeError, Message};

#[derive(Debug, Clone)]
pub struct Decoder<T> {
    data: BytesMut,
    err: bool,
    _marker: PhantomData<T>,
}

impl<T, B> From<B> for Decoder<T>
where
    B: AsRef<[u8]>,
{
    fn from(value: B) -> Self {
        Self {
            data: value.as_ref().into(),
            err: false,
            _marker: PhantomData,
        }
    }
}

impl<T> Iterator for Decoder<T>
where
    T: Message + std::default::Default,
{
    type Item = Result<T, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() || self.err {
            return None;
        }
        let r = T::decode_length_delimited(&mut self.data);
        if r.is_err() {
            self.err = true;
        }
        Some(r)
    }
}

pub fn decode_game(bytes: Vec<u8>) -> Vec<Result<String, String>> {
    let mut v = vec![];
    for message in Decoder::<game::Message>::from(bytes) {
        match message {
            Ok(m) => match m.content.unwrap() {
                game::message::Content::Request(game::Request { uid, content }) => {
                    v.push(Ok(format!(
                        "Request {{ uid: {uid}, {:?} }}",
                        unpack_any(&content.unwrap())
                    )))
                }
                game::message::Content::Response(game::Response { uid, content }) => {
                    v.push(Ok(format!(
                        "Response {{ uid: {uid}, {:?} }}",
                        unpack_any(&content.unwrap())
                    )))
                }
                game::message::Content::Event(game::Event { content }) => v.push(Ok(format!(
                    "Event {{ {:?} }}",
                    unpack_any(&content.unwrap())
                ))),
            },
            Err(e) => v.push(Err(format!("{e}"))),
        }
    }
    v
}

pub fn decode_connection(bytes: Vec<u8>) -> Vec<Result<String, String>> {
    let mut v = vec![];
    for message in Decoder::<connection::Message>::from(bytes) {
        match message {
            Ok(m) => v.push(Ok(format!("{m:?}"))),
            Err(e) => v.push(Err(format!("{e}"))),
        }
    }
    v
}
