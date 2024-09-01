use std::io::Write;
use cli::{parse_args, validate_args};

mod cli;
mod cmd;

fn main() -> anyhow::Result<()>{
    // Parse and validate arguments
    let args = parse_args();
    validate_args(&args)?;

    if let Err(err) = cmd::run(&args) {
        writeln!(std::io::stdout(), "{:?}", err).unwrap();
        std::process::exit(1);
    }
    Ok(())
}

