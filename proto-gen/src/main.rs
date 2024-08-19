use proto_gen::Generator;
use std::io;

fn main() -> io::Result<()> {
    if let Err(e) = Generator::from_source("./prototest/Ankama.Dofus.Protocol.Game/", "game_protos/", "game") {
        panic!("{e}")
    }
    Ok(())
}
