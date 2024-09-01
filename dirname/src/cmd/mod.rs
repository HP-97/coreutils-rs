use std::io::Write;

use crate::cli::Cli;

pub fn run(args: &Cli) -> anyhow::Result<()> {
    args.names
        .clone()
        .into_iter()
        .map(|name| (name.clone(), get_dir_length(&name)))
        .for_each(|(name, dir_length)| {
            let substr: String;
            if dir_length == 0 {
                substr = ".".to_string();
            } else {
                substr = name.chars().take(dir_length - 1).collect();
            }
            writeln!(std::io::stdout(), "{}", substr).unwrap();
        });
    Ok(())
}

/// Finds the length of the end of the directory name
pub fn get_dir_length(path: &str) -> usize {
    let dir_sep;

    #[cfg(unix)]
    {
        dir_sep = r"/";
    }

    #[cfg(windows)]
    {
        dir_sep = r"\";
    }

    let path_str_len = path.len();

    match path
        .chars()
        .rev()
        .enumerate()
        .find(|&(_, x)| x.to_string() == dir_sep)
    {
        Some((idx, _)) => return path_str_len - idx,
        None => return 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_linux_paths() {
        let test1 = "asdf";
        assert_eq!(get_dir_length(test1), 0);

        let test2 = "bin/echo";
        assert_eq!(get_dir_length(&test2), 4);

        let test3 = "/usr/bin/hello";
        assert_eq!(get_dir_length(&test3), 9);
    }
}
