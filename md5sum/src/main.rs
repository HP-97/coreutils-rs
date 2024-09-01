use std::io::Write;

use cli::parse_args;

mod cli;
mod cmd;

fn main() {
    // Parse arguments
    let args = parse_args();

    
    if let Err(err) = cmd::run(&args) {
        writeln!(std::io::stdout(), "{:?}", err).unwrap();
        std::process::exit(1)
    }
}
