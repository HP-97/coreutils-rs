// use std::io::Write;

use crate::cli::Cli;

mod generate;
mod check;

pub fn run(args: &Cli) -> anyhow::Result<()> {
    if args.check {
        check::run()?;
    } else {
        generate::run(args)?;
    }
    Ok(())
}

