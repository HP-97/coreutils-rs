use std::{
    borrow::Cow,
    fs::{read, read_to_string},
    io::Write,
};

use base64::{engine::general_purpose::URL_SAFE, Engine};

use crate::cli::Cli;

pub fn run(args: &Cli) -> anyhow::Result<()> {
    // TODO capture telemetry
    let mut successful_line_reads: usize = 0;
    let mut failed_line_reads: usize = 0;

    let mut err_occurred = false;

    // parse base64 files
    let base64_iter: Vec<Vec<u8>> = args
        .base64
        .iter()
        .map(|v| URL_SAFE.decode(v))
        .filter_map(|r| {
            // print any errors out to stderr
            r.map_err(|e| writeln!(std::io::stderr(), "base64: {}", e.to_string()).unwrap())
                .ok()
        })
        .collect();

    // parse FILEs
    let file_iter: Vec<Vec<u8>> = args
        .files
        .clone()
        .iter()
        .map(|v| (v, read(v)))
        .filter_map(|(file, r)| {
            // print any errors out to stderr
            r.map_err(|e| {
                writeln!(
                    std::io::stderr(),
                    "{}: {}",
                    file.to_string_lossy(),
                    e.to_string()
                )
                .unwrap()
            })
            .ok()
        })
        .collect();

    // Combine the two iterators using .chain() and then process everything
    for checksums_bytes in base64_iter.iter().chain(file_iter.iter()) {
        let checksums_str = String::from_utf8_lossy(&checksums_bytes);
        for line in checksums_str.lines() {
            let split: Vec<&str> = line.split("  ").collect();

            // validate the line
            if split.len() != 2 {
                failed_line_reads += 1;
                continue;
            }

            let Some(&curr_checksum) = split.get(0) else {
                continue;
            };

            let Some(&curr_filepath) = split.get(1) else {
                continue;
            };

            let content: Vec<u8>;
            // Calculate the file's checksum and compare against the FILE's stored checksum
            match read(curr_filepath) {
                Ok(v) => content = v,
                Err(e) => {
                    err_occurred = true;
                    writeln!(std::io::stdout(), "{}: {}", curr_filepath, e.to_string())?;
                    failed_line_reads += 1;
                    continue;
                }
            };

            let checksum = md5::compute(content);
            let checksum_str = format!("{:x}", checksum);

            // Check if the checksum matches
            if checksum_str == curr_checksum {
                writeln!(std::io::stdout(), "{}: OK", curr_filepath)?;
            } else {
                writeln!(std::io::stdout(), "{}: FAILED", curr_filepath)?;
                err_occurred = true;
                failed_line_reads += 1;
            }
        }
    }
    if err_occurred {
        writeln!(
            std::io::stderr(),
            "WARNING: {} listed file(s) failed to be processed",
            failed_line_reads
        )?;
        anyhow::bail!("");
    }
    Ok(())
}
