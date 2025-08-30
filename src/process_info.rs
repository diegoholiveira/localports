use std::path::PathBuf;
use sysinfo::Process;

#[derive(Debug, Clone)]
pub struct ProcessDetails {
    pub bin_path: Option<PathBuf>,
    pub directory: String,
}

pub fn get_process_details(process: &Process) -> ProcessDetails {
    let bin_path = process.exe().map(|p| p.to_path_buf());
    let cwd = process.cwd().map(|p| p.to_path_buf());

    let resolved_path = get_resolved_bin_path(&bin_path, &cwd);
    let directory = format_directory_path(&resolved_path);

    ProcessDetails {
        bin_path,
        directory: directory,
    }
}

pub(crate) fn get_resolved_bin_path(
    bin_path: &Option<PathBuf>,
    cwd: &Option<PathBuf>,
) -> Option<PathBuf> {
    match (bin_path, cwd) {
        (Some(bin_path), Some(cwd)) => {
            // If bin_path starts with ./, resolve it against cwd
            if bin_path.starts_with("./") {
                Some(cwd.join(bin_path.strip_prefix("./").unwrap()))
            } else if bin_path.is_relative() {
                Some(cwd.join(bin_path))
            } else {
                Some(bin_path.clone())
            }
        }
        (Some(bin_path), None) => Some(bin_path.clone()),
        _ => None,
    }
}

fn format_directory_path(path: &Option<PathBuf>) -> String {
    match path {
        Some(p) => {
            // Convert absolute path to use ~ if it's in home directory
            if let Some(home) = dirs::home_dir() {
                if p.starts_with(&home) {
                    format!("~/{}", p.strip_prefix(&home).unwrap().display())
                } else {
                    p.display().to_string()
                }
            } else {
                p.display().to_string()
            }
        }
        None => "unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_resolved_bin_path_relative_with_dot_slash() {
        let bin_path = Some(PathBuf::from("./my-binary"));
        let cwd = Some(PathBuf::from("/home/user/project"));

        let result = get_resolved_bin_path(&bin_path, &cwd);
        assert_eq!(result, Some(PathBuf::from("/home/user/project/my-binary")));
    }

    #[test]
    fn test_get_resolved_bin_path_relative_without_dot_slash() {
        let bin_path = Some(PathBuf::from("my-binary"));
        let cwd = Some(PathBuf::from("/home/user/project"));

        let result = get_resolved_bin_path(&bin_path, &cwd);
        assert_eq!(result, Some(PathBuf::from("/home/user/project/my-binary")));
    }

    #[test]
    fn test_get_resolved_bin_path_absolute() {
        let bin_path = Some(PathBuf::from("/usr/bin/my-binary"));
        let cwd = Some(PathBuf::from("/home/user/project"));

        let result = get_resolved_bin_path(&bin_path, &cwd);
        assert_eq!(result, Some(PathBuf::from("/usr/bin/my-binary")));
    }

    #[test]
    fn test_get_resolved_bin_path_no_cwd() {
        let bin_path = Some(PathBuf::from("./my-binary"));
        let cwd = None;

        let result = get_resolved_bin_path(&bin_path, &cwd);
        assert_eq!(result, Some(PathBuf::from("./my-binary")));
    }

    #[test]
    fn test_get_resolved_bin_path_no_bin_path() {
        let bin_path = None;
        let cwd = Some(PathBuf::from("/home/user/project"));

        let result = get_resolved_bin_path(&bin_path, &cwd);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_resolved_bin_path_both_none() {
        let bin_path = None;
        let cwd = None;

        let result = get_resolved_bin_path(&bin_path, &cwd);
        assert_eq!(result, None);
    }
}
