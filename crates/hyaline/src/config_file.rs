//! Loads and saves the settings file, `%APPDATA%\Hyaline\config.toml`.

use hyaline_core::config::Config;
use std::io;
use std::path::PathBuf;

fn path() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(|dir| PathBuf::from(dir).join("Hyaline").join("config.toml"))
}

/// Loads the settings. A missing file silently means defaults; an unreadable or invalid one
/// means defaults plus a notice for the user. The file itself is left untouched.
pub fn load() -> (Config, Option<String>) {
    let Some(path) = path() else {
        return (
            Config::default(),
            Some("APPDATA is not set, so settings cannot be saved.".into()),
        );
    };
    match std::fs::read_to_string(&path) {
        Ok(text) => match Config::parse(&text) {
            Ok(config) => (config, None),
            Err(error) => (
                Config::default(),
                Some(format!(
                    "Using default settings: config.toml is invalid ({error})."
                )),
            ),
        },
        Err(error) if error.kind() == io::ErrorKind::NotFound => (Config::default(), None),
        Err(error) => (
            Config::default(),
            Some(format!(
                "Using default settings: could not read config.toml ({error})."
            )),
        ),
    }
}

/// Saves the settings, replacing the file atomically so a crash never leaves half a file.
pub fn save(config: Config) -> io::Result<()> {
    let path = path().ok_or_else(|| io::Error::other("APPDATA is not set"))?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let temporary = path.with_extension("toml.tmp");
    std::fs::write(&temporary, config.to_toml())?;
    std::fs::rename(&temporary, &path)
}
