use crate::validate_starsector_core_dir;
use std::path::{Path, PathBuf};

pub fn locate_core_dirs() -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = candidate_paths()
        .into_iter()
        .filter(|p| validate_starsector_core_dir(p).is_ok())
        .collect();

    found.sort();
    found.dedup();
    found
}

const CORE_DIR_SUBPATHS: &[&str] = &[
    "starsector-core",
    "Contents/Resources/Java",
    "Starsector.app/Contents/Resources/Java",
    "Fractal Softworks/Starsector/starsector-core",
    "Starsector/starsector-core",
];

pub fn resolve_core_dir(picked: &Path) -> Option<PathBuf> {
    if validate_starsector_core_dir(picked).is_ok() {
        return Some(picked.to_owned());
    }

    for rel in CORE_DIR_SUBPATHS {
        let candidate = picked.join(rel);
        if validate_starsector_core_dir(&candidate).is_ok() {
            return Some(candidate);
        }
    }

    find_core_dir(picked, 4)
}

fn find_core_dir(root: &Path, depth: usize) -> Option<PathBuf> {
    if validate_starsector_core_dir(root).is_ok() {
        return Some(root.to_owned());
    }
    if depth == 0 {
        return None;
    }
    let mut subdirs: Vec<PathBuf> = std::fs::read_dir(root)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    subdirs.sort();
    subdirs.iter().find_map(|dir| find_core_dir(dir, depth - 1))
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

#[cfg(test)]
mod tests {
    use super::*;

    struct TempTree(PathBuf);

    impl TempTree {
        fn new(name: &str) -> Self {
            let dir = std::env::temp_dir().join(format!("sslab_{}_{}", std::process::id(), name));
            let _ = std::fs::remove_dir_all(&dir);
            Self(dir)
        }

        fn make_core(&self, rel: &str) -> PathBuf {
            let core = self.0.join(rel);
            for sub in ["data", "graphics", "sounds"] {
                std::fs::create_dir_all(core.join(sub)).unwrap();
            }
            core
        }
    }

    impl Drop for TempTree {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn resolves_the_core_dir_itself() {
        let tree = TempTree::new("itself");
        let core = tree.make_core("starsector-core");
        assert_eq!(resolve_core_dir(&core), Some(core));
    }

    #[test]
    fn resolves_install_root_via_known_subpath() {
        let tree = TempTree::new("install");
        let core = tree.make_core("starsector-core");
        assert_eq!(resolve_core_dir(&tree.0), Some(core));
    }

    #[test]
    fn resolves_macos_app_bundle() {
        let tree = TempTree::new("app");
        let core = tree.make_core("Starsector.app/Contents/Resources/Java");
        assert_eq!(resolve_core_dir(&tree.0.join("Starsector.app")), Some(core));
    }

    #[test]
    fn resolves_by_shallow_search_when_nested() {
        let tree = TempTree::new("nested");
        let core = tree.make_core("games/Starsector/starsector-core");
        assert_eq!(resolve_core_dir(&tree.0), Some(core));
    }

    #[test]
    fn returns_none_when_no_data_present() {
        let tree = TempTree::new("empty");
        std::fs::create_dir_all(tree.0.join("nothing/here")).unwrap();
        assert_eq!(resolve_core_dir(&tree.0), None);
    }
}
