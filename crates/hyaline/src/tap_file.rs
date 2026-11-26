//! Where Explorer loads the helper from: a content-named copy under `%LOCALAPPDATA%\Hyaline`,
//! never the install folder. Explorer keeps a loaded helper locked until it restarts, and
//! upgrades or uninstalls replace the install folder while it does.

use hyaline_core::tap::{is_tap_copy, tap_file_name};
use std::io;
use std::path::{Path, PathBuf};

const SHIPPED: &str = "hyaline_tap.dll";

/// The helper shipped next to this exe, also when the exe was started through a symlink
/// (winget's command alias).
pub fn shipped() -> io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let beside = exe.with_file_name(SHIPPED);
    if beside.is_file() {
        return Ok(beside);
    }
    let real = std::fs::canonicalize(&exe)?.with_file_name(SHIPPED);
    if real.is_file() {
        Ok(real)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} not found", beside.display()),
        ))
    }
}

/// The file Explorer should load: the copy, or `shipped` itself if the copy cannot be made.
pub fn loadable(shipped: &Path) -> PathBuf {
    copy(shipped).unwrap_or_else(|error| {
        eprintln!("hyaline: copying the helper failed, using it in place: {error}");
        shipped.to_path_buf()
    })
}

/// Makes sure the copy matches `shipped` and removes copies of other builds that are not in
/// use (a copy Explorer has loaded cannot be deleted, so it stays until the next start).
fn copy(shipped: &Path) -> io::Result<PathBuf> {
    let dir = std::env::var_os("LOCALAPPDATA")
        .map(|dir| PathBuf::from(dir).join("Hyaline"))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "LOCALAPPDATA is not set"))?;
    std::fs::create_dir_all(&dir)?;
    let contents = std::fs::read(shipped)?;
    let name = tap_file_name(&contents);
    let copy = dir.join(&name);
    if std::fs::read(&copy).ok().as_deref() != Some(contents.as_slice()) {
        let temp = dir.join(format!("{name}.tmp"));
        std::fs::write(&temp, &contents)?;
        std::fs::rename(&temp, &copy)?;
    }
    for entry in std::fs::read_dir(&dir)?.flatten() {
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if is_tap_copy(&file_name) && !file_name.eq_ignore_ascii_case(&name) {
            let _ = std::fs::remove_file(entry.path());
        }
    }
    Ok(copy)
}
