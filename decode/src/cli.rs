use clap::{ArgAction, ArgGroup, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["hex", "stdin"])
        .multiple(false)
))]
pub struct Args {
    /// Protocol you want to decode, either game or connection
    #[arg(value_parser = parse_protocol)]
    pub protocol: Protocol,

    /// The hex stream you want to decode
    pub hex: Option<String>,

    /// Read input hex stream from stdin instead of from arguments
    #[arg(long, short, action = ArgAction::SetTrue)]
    pub stdin: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Protocol {
    Game,
    Connection,
}

fn parse_protocol(s: &str) -> Result<Protocol, String> {
    match s.to_lowercase().as_str() {
        "game" | "g" => Ok(Protocol::Game),
        "connection" | "c" => Ok(Protocol::Connection),
        _ => Err(format!("Invalid protocol: {}", s)),
    }
}
