//! This file contains all of the logic in relation to the CLI configuration
use std::path::PathBuf;

use clap::Parser;

/// Print or check MD5 checksums.
///
/// Unix and Windows is supported.
#[derive(Parser)]
#[command(version, verbatim_doc_comment)]
pub struct Cli {
    pub files: Vec<PathBuf>,
    /// read checksums from the FILEs and check them
    #[arg(short, long)]
    pub check: bool,
}

/// Parses args
pub fn parse_args() -> Cli {
    let args = Cli::parse();
    args
}
