use std::path::PathBuf;

/// Finds the legnth of the end of the directory name
pub fn get_dir_length(path: &PathBuf) -> usize {

    let dir_sep;

    #[cfg(unix)]
    {
        dir_sep = r"\/";
    }

    #[cfg(windows)]
    {
        dir_sep =  r"\\";
    }

    path.to_string_lossy().chars().rev()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linux_paths() {
        let test1 = PathBuf::from("asdf");
        assert_eq!(get_dir_length(&test1), 0)
    }
} 
