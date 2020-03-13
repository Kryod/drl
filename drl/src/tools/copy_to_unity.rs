use std::fs;
use std::env;
use std::path::PathBuf;

pub fn copy_to_unity() {
    let debug_mode = cfg!(debug_assertions);
    let name = "drl";
    let (name, target_name) = if cfg!(target_os = "windows") {
        (
            format!("{}.dll", name),
            format!("{}.dll", name),
        )
    } else if cfg!(target_os = "macos") {
        (
            format!("lib{}.dylib", name),
            format!("{}.dylib", name),
        )
    } else { // Linux
        (
            format!("lib{}.so", name),
            format!("{}.so", name),
        )
    };

    let cwd = env::current_dir().expect("Could not get current working directory");

    let mut from = PathBuf::new();
    from.push(&cwd);
    from.push("target");
    if debug_mode {
        from.push("debug");
    } else {
        from.push("release");
    }
    from.push(&name);

    let mut to = PathBuf::new();
    to.push(&cwd);
    to.pop();
    to.push("unity-drl");
    to.push(&target_name);

    fs::copy(from, to).unwrap_or_else(|e| panic!("Could not copy {}: {}", name, e));
}
