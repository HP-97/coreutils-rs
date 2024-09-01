use std::{fs::read, io::Write};

use crate::cli::Cli;

pub fn run(args: &Cli) -> anyhow::Result<()> {
    for file in args.files.iter() {
        let content: Vec<u8>;
        match read(file.clone()) {
            Ok(v) => content = v,
            Err(e) => {
                writeln!(std::io::stderr(), "{}: {}", file.to_string_lossy(), e.to_string())?;
                continue;
            },
        };

        let checksum = md5::compute(content);
        writeln!(std::io::stdout(), "{:x}  {}", checksum, file.to_string_lossy())?;
    }
    Ok(())
}

