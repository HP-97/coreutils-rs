// use std::io::Write;

use crate::cli::Cli;

mod check;
mod generate;

pub fn run(args: &Cli) -> anyhow::Result<()> {
    if args.check {
        check::run()?;
    } else {
        generate::run(args)?;
    }
    Ok(())
}
