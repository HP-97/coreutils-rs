use std::path::PathBuf;

use clap::Parser;

const AFTER_HELP: &'static str = "\
Used https://www.maizure.org/projects/decoded-gnu-coreutils to
help determine the algorithm design.
";

const MISSING_OPERAND: &'static str = "\
missing operand.
Try 'dirname --help'  for more information.";

#[derive(Parser)]
#[command(version, verbatim_doc_comment, after_help=AFTER_HELP)]
/// Output each NAME with its last non-slash component and trailing slashes
/// removed; if NAME contains no slashes, output '.' (meaning the current directory).
///
/// NOTE: Unix systems use / while Windows uses \.
pub struct Cli {
    pub names: Vec<String>,
}

pub fn parse_args() -> Cli {
    let args = Cli::parse();
    args
}

pub fn validate_args(args: &Cli) -> anyhow::Result<()> {
    // ensure that at least one positional arg is provided by the user
    if args.names.len() == 0 {
        anyhow::bail!(MISSING_OPERAND);
    }
    Ok(())
}

