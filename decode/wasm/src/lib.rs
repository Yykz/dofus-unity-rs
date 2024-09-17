use decode::{decode_connection, decode_game};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decode(protocol: &str, input: &str) -> String {
    let bytes = hex::decode(input);

    match (protocol, bytes) {
        ("connection", Ok(bytes)) => {
            let s: Vec<String> = decode_connection(bytes)
                .into_iter()
                .map(|r| match r {
                    Ok(m) => m,
                    Err(e) => format!("Error: {e}"),
                })
                .collect();
            s.join("\n")
        }
        ("game", Ok(bytes)) => {
            let s: Vec<String> = decode_game(bytes)
                .into_iter()
                .map(|r| match r {
                    Ok(m) => m,
                    Err(e) => format!("Error: {e}"),
                })
                .collect();
            s.join("\n")
        }
        (_, Err(_e)) => {
            format!("Invalid hexstream")
        }
        _ => format!("protocol dont exist"),
    }
}
