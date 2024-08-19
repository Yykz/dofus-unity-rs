use proto_gen::Generator;
use std::io;

fn main() -> io::Result<()> {
    Generator::from_source("./Ankama.Dofus.Protocol.Game/", "game_protos/".into());
    Ok(())
}
