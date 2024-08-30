use clap::Parser;

const AFTER_HELP: &'static str = "\
Used https://www.maizure.org/projects/decoded-gnu-coreutils to
determine algorithm design
";

#[derive(Parser)]
#[command(version, verbatim_doc_comment, after_help=AFTER_HELP)]
pub struct Cli {}
