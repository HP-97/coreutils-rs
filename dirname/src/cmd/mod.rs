use std::path::PathBuf;

/// Finds the legnth of the end of the directory name
pub fn get_dir_length(path: &PathBuf) -> usize {
    let dir_sep;

    #[cfg(unix)]
    {
        dir_sep = r"/";
    }

    #[cfg(windows)]
    {
        dir_sep = r"\";
    }

    let path_str = path.to_string_lossy();
    let path_str_len = path_str.len();

    match path_str
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
        let test1 = PathBuf::from("asdf");
        assert_eq!(get_dir_length(&test1), 0);

        let test2 = PathBuf::from("bin/echo");
        assert_eq!(get_dir_length(&test2), 4);

        let test3 = PathBuf::from("/usr/bin/hello");
        assert_eq!(get_dir_length(&test3), 9);
    }
}
