// ─── < Imports > ────────────────────────────────────────────────────

use std::path::{Component, Path, PathBuf};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn normalize_path_like_resource(resource: &str) -> String {
    if resource.is_empty() {
        return String::new();
    }

    let normalized_path = normalize_path_components(Path::new(resource));

    normalized_path.to_string_lossy().to_string()
}

pub fn matches_resource_name(resource: &str, protected_resource: &str) -> bool {
    let normalized_resource = normalize_path_like_resource(resource);
    let normalized_protected_resource = normalize_path_like_resource(protected_resource);

    if normalized_resource == normalized_protected_resource {
        return true;
    }

    let protected_resource_inside_folder = format!("/{normalized_protected_resource}");

    normalized_resource.ends_with(&protected_resource_inside_folder)
}

pub fn matches_path_prefix(resource: &str, blocked_prefix: &str) -> bool {
    let normalized_resource = normalize_path_like_resource(resource);
    let normalized_blocked_prefix = normalize_path_like_resource(blocked_prefix);

    if normalized_blocked_prefix.is_empty() {
        return false;
    }

    if normalized_resource == normalized_blocked_prefix {
        return true;
    }

    let blocked_prefix_with_separator = format!("{normalized_blocked_prefix}/");

    normalized_resource.starts_with(&blocked_prefix_with_separator)
}

// ─── < Private Functions > ──────────────────────────────────────────

fn normalize_path_components(path: &Path) -> PathBuf {
    let mut normalized_path = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized_path.push(prefix.as_os_str()),
            Component::RootDir => normalized_path.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => apply_parent_dir(&mut normalized_path),
            Component::Normal(part) => normalized_path.push(part),
        }
    }

    normalized_path
}

fn apply_parent_dir(path: &mut PathBuf) {
    if path.as_os_str().is_empty() {
        path.push("..");
        return;
    }

    if path_ends_with_parent_dir(path) {
        path.push("..");
        return;
    }

    if path_is_root(path) {
        return;
    }

    if !path.pop() {
        path.push("..");
    }
}

fn path_ends_with_parent_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|file_name| file_name.to_str())
        .is_some_and(|file_name| file_name == "..")
}

fn path_is_root(path: &Path) -> bool {
    path.parent().is_none() && path.has_root()
}
