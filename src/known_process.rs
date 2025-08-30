use std::path::PathBuf;

static SYSTEM_BINARIES: &[&str] = &[
    "/System/Library/CoreServices/ControlCenter.app/Contents/MacOS/ControlCenter",
    "/usr/libexec/rapportd",
];

pub fn is_system_process(bin_path: &Option<PathBuf>) -> bool {
    match bin_path {
        Some(path) => {
            let path_str = path.to_string_lossy();
            SYSTEM_BINARIES.contains(&path_str.as_ref())
        }
        None => false,
    }
}
