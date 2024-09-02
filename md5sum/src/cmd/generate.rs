use std::{fs::read, io::Write};

use crate::cli::Cli;

pub fn run(args: &Cli) -> anyhow::Result<()> {
    let mut err_occurred = false;
    for file in args.files.iter() {
        let content: Vec<u8>;

        match read(file.clone()) {
            Ok(v) => content = v,
            Err(e) => {
                err_occurred = true;
                writeln!(
                    std::io::stderr(),
                    "{}: {}",
                    file.to_string_lossy(),
                    e.to_string()
                )?;
                continue;
            }
        };

        let checksum = md5::compute(content);
        // ensure we have a 2-length whitespace gap between the checksum and path
        writeln!(
            std::io::stdout(),
            "{:x}  {}",
            checksum,
            file.to_string_lossy()
        )?;
    }

    // Return an empty Err to allow the main fn to handle the exit code
    if err_occurred {
        anyhow::bail!("");
    }
    Ok(())
}
