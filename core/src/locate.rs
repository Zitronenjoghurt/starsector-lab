use crate::validate_starsector_core_dir;
use std::path::PathBuf;

pub fn locate_core_dirs() -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = candidate_paths()
        .into_iter()
        .filter(|p| validate_starsector_core_dir(p).is_ok())
        .collect();

    found.sort();
    found.dedup();
    found
}

#[cfg(target_os = "windows")]
fn candidate_paths() -> Vec<PathBuf> {
    let mut v = Vec::new();
    for var in ["ProgramFiles(x86)", "ProgramFiles"] {
        if let Some(base) = std::env::var_os(var) {
            v.push(
                PathBuf::from(base)
                    .join("Fractal Softworks")
                    .join("Starsector")
                    .join("starsector-core"),
            );
        }
    }
    v
}

#[cfg(target_os = "linux")]
fn candidate_paths() -> Vec<PathBuf> {
    let mut v = vec![PathBuf::from("/opt/starsector/starsector-core")];
    if let Some(home) = dirs::home_dir() {
        for rel in [
            "starsector/starsector-core",
            ".local/share/starsector/starsector-core",
            "games/starsector/starsector-core",
        ] {
            v.push(home.join(rel));
        }
    }
    v
}

#[cfg(target_os = "macos")]
fn candidate_paths() -> Vec<PathBuf> {
    let mut apps = vec![PathBuf::from("/Applications/Starsector.app")];
    if let Some(home) = dirs::home_dir() {
        apps.push(home.join("Applications/Starsector.app"));
    }

    apps.into_iter()
        .map(|app| app.join("Contents/Resources/Java"))
        .collect()
}
