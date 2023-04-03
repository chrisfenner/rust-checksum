mod checksums;

use std::env;
use anyhow::Result;

use crate::checksums::{Checksum, Luhn};

fn main() -> Result<()> {
    let mut args = env::args();
    args.next(); // skip the binary name
    let cmd: Command = get_command(args)?;

    let result: String = match cmd {
        Command::Generate(checksum, payload) => checksum.generate(payload.as_str()).map_err(anyhow::Error::msg)?,
        Command::Validate(checksum, message) => checksum.validate(message.as_str()).map_err(anyhow::Error::msg)?,
    };

    println!("{result}");

    Ok(())
}

enum Command {
    Generate(Box<dyn Checksum>, String),
    Validate(Box<dyn Checksum>, String),
}

fn get_command(mut args: env::Args) -> Result<Command> {
    match args.next() {
        None => Err(anyhow::format_err!("please provide a command ('generate' or 'validate')")),
        Some(str) => match str.to_ascii_lowercase().as_str() {
            "generate" => get_generate_command(&mut args),
            "validate" => get_validate_command(&mut args),
            _ => Err(anyhow::format_err!("command must be one of {{'generate', 'validate'}}")),
        }
    }
}

fn get_generate_command(args: &mut env::Args) -> Result<Command> {
    let checksum: Box<dyn Checksum> = get_checksum(args)?;
    let payload: String = match args.next() {
        Some(str) => str,
        _ => return Err(anyhow::format_err!("please provide a payload to checksum")),
    };
    Ok(Command::Generate(checksum, payload))
}

fn get_validate_command(args: &mut env::Args) -> Result<Command> {
    let checksum: Box<dyn Checksum> = get_checksum(args)?;
    let message: String = match args.next() {
        Some(str) => str,
        _ => return Err(anyhow::format_err!("please provide a message containing a checksummed payload")),
    };
    Ok(Command::Validate(checksum, message))
}

fn get_checksum(args: &mut env::Args) -> Result<Box<dyn Checksum>> {
    match args.next() {
        None => Err(anyhow::format_err!("please provide a checksum algorithm ('luhn')")),
        Some(str) => match str.to_ascii_lowercase().as_str() {
            "luhn" => Ok(Box::new(Luhn{})),
            _ => Err(anyhow::format_err!("checksum must be one of {{'luhn'}}")),
        }
    }
}