use std::io;
use proto_gen::Generator;

fn main() -> io::Result<()> {
    Generator::from_source("./Ankama.Dofus.Protocol.Game/", "protos/".into());
    Ok(())
}
