//! Corruption-safe file storage.
//!
//! Every write goes to `<path>.tmp`, is flushed and fsynced, then renamed over
//! the target. Before replacing the target, the previous good copy is saved as
//! `<path>.bak`. A reader that finds a corrupt or empty main file falls back to
//! the backup, then to a default value.
//!
//! This is deliberately plain `std` so it works identically on Android and in
//! desktop unit tests.

use serde::Serialize;
use serde::de::DeserializeOwned;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadSource {
    Main,
    Backup,
    Default,
}

#[derive(Debug)]
pub struct Loaded<T> {
    pub value: T,
    pub source: LoadSource,
    /// Set when the main file was unusable and a fallback was used.
    pub warning: Option<String>,
}

/// Write `bytes` to `path` atomically, keeping the previous version as `.bak`.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }

    let tmp = sibling(path, ".tmp");
    {
        let mut file = File::create(&tmp)?;
        file.write_all(bytes)?;
        file.flush()?;
        // fsync the data before the rename so a crash cannot leave a renamed
        // but empty/partial file.
        file.sync_all()?;
    }

    // Keep the last good version. A crash during this copy leaves the main file
    // untouched, so the worst case is a stale backup.
    if path.exists() {
        let backup = sibling(path, ".bak");
        let mut source = File::open(path)?;
        let mut destination = File::create(&backup)?;
        io::copy(&mut source, &mut destination)?;
        destination.flush()?;
        destination.sync_all()?;
    }

    // On Windows and Android, rename replaces an existing destination.
    fs::rename(&tmp, path)?;

    // Best-effort directory fsync on Unix (including Android) so the rename is
    // durable too. Windows does not allow opening a directory as a File.
    #[cfg(unix)]
    if let Some(parent) = path.parent() {
        if let Ok(dir) = File::open(parent) {
            let _ = dir.sync_all();
        }
    }

    Ok(())
}

/// Serialize a value as pretty JSON and write it atomically.
pub fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let mut json = serde_json::to_vec_pretty(value)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    json.push(b'\n');
    write_atomic(path, &json)
}

/// Read and deserialize a JSON file, recovering from `.bak` when necessary.
///
/// A missing file is not a warning on first run; a present but corrupt file is.
pub fn load_json<T>(path: &Path) -> Loaded<T>
where
    T: DeserializeOwned + Default,
{
    let main = read_and_parse(path);
    match main {
        Ok(value) => Loaded {
            value,
            source: LoadSource::Main,
            warning: None,
        },
        Err(main_error) => {
            let main_existed = path.exists();
            let backup_path = sibling(path, ".bak");
            let backup = read_and_parse(&backup_path);
            match backup {
                Ok(value) => Loaded {
                    value,
                    source: LoadSource::Backup,
                    warning: Some(format!(
                        "{} was unusable ({main_error}); recovered from {}",
                        path.display(),
                        backup_path.display()
                    )),
                },
                Err(backup_error) => {
                    let warning = if main_existed {
                        Some(format!(
                            "{} was unusable ({main_error}) and its backup was unusable too ({backup_error}); starting with defaults",
                            path.display()
                        ))
                    } else {
                        None
                    };
                    Loaded {
                        value: T::default(),
                        source: LoadSource::Default,
                        warning,
                    }
                }
            }
        }
    }
}

fn read_and_parse<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let mut file = File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    if bytes.iter().all(|b| b.is_ascii_whitespace()) {
        return Err(format!("{}: file is empty", path.display()));
    }
    serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))
}

/// Remove leftover temporary files. Called at startup; harmless if none exist.
pub fn clean_temporary_files(path: &Path) {
    let _ = fs::remove_file(sibling(path, ".tmp"));
}

fn sibling(path: &Path, suffix: &str) -> PathBuf {
    let mut name: OsString = path.as_os_str().to_os_string();
    name.push(suffix);
    PathBuf::from(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::sync::atomic::{AtomicU64, Ordering};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    struct TestData {
        value: u32,
        name: String,
    }

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let n = COUNTER.fetch_add(1, Ordering::Relaxed);
            let dir =
                std::env::temp_dir().join(format!("dutchreader-atomic-{}-{n}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            TestDir(dir)
        }

        fn file(&self, name: &str) -> PathBuf {
            self.0.join(name)
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn writes_and_reads_json() {
        let dir = TestDir::new();
        let path = dir.file("data.json");
        let data = TestData {
            value: 7,
            name: "fiets".into(),
        };
        write_json_atomic(&path, &data).unwrap();
        let loaded = load_json::<TestData>(&path);
        assert_eq!(loaded.value, data);
        assert_eq!(loaded.source, LoadSource::Main);
        assert!(loaded.warning.is_none());
    }

    #[test]
    fn second_write_keeps_the_previous_version_as_backup() {
        let dir = TestDir::new();
        let path = dir.file("data.json");
        write_json_atomic(
            &path,
            &TestData {
                value: 1,
                name: "old".into(),
            },
        )
        .unwrap();
        write_json_atomic(
            &path,
            &TestData {
                value: 2,
                name: "new".into(),
            },
        )
        .unwrap();

        assert_eq!(load_json::<TestData>(&path).value.value, 2);
        let backup = sibling(&path, ".bak");
        let backup_json: TestData =
            serde_json::from_str(&fs::read_to_string(backup).unwrap()).unwrap();
        assert_eq!(backup_json.value, 1);
    }

    #[test]
    fn corrupt_main_falls_back_to_backup() {
        let dir = TestDir::new();
        let path = dir.file("data.json");
        write_json_atomic(
            &path,
            &TestData {
                value: 1,
                name: "old".into(),
            },
        )
        .unwrap();
        write_json_atomic(
            &path,
            &TestData {
                value: 2,
                name: "new".into(),
            },
        )
        .unwrap();

        fs::write(&path, b"{ this is not json").unwrap();
        let loaded = load_json::<TestData>(&path);
        assert_eq!(loaded.value.value, 1);
        assert_eq!(loaded.source, LoadSource::Backup);
        assert!(loaded.warning.unwrap().contains("recovered"));
    }

    #[test]
    fn corrupt_main_and_backup_start_with_defaults() {
        let dir = TestDir::new();
        let path = dir.file("data.json");
        fs::write(&path, b"not json").unwrap();
        fs::write(sibling(&path, ".bak"), b"also not json").unwrap();
        let loaded = load_json::<TestData>(&path);
        assert_eq!(loaded.value, TestData::default());
        assert_eq!(loaded.source, LoadSource::Default);
        assert!(loaded.warning.is_some());
    }

    #[test]
    fn interrupted_write_does_not_corrupt_the_previous_file() {
        let dir = TestDir::new();
        let path = dir.file("data.json");
        let original = TestData {
            value: 41,
            name: "intact".into(),
        };
        write_json_atomic(&path, &original).unwrap();

        // Simulate a process killed after creating the temp file but before
        // renaming it: a partial .tmp exists, the target is untouched.
        fs::write(sibling(&path, ".tmp"), b"{\"value\": 9, \"na").unwrap();

        let loaded = load_json::<TestData>(&path);
        assert_eq!(loaded.value, original);
        assert_eq!(loaded.source, LoadSource::Main);

        // The next successful write must cleanly replace the stale temp file.
        write_json_atomic(
            &path,
            &TestData {
                value: 42,
                name: "next".into(),
            },
        )
        .unwrap();
        assert_eq!(load_json::<TestData>(&path).value.value, 42);
    }

    #[test]
    fn empty_main_is_treated_as_corrupt() {
        let dir = TestDir::new();
        let path = dir.file("data.json");
        fs::write(&path, b"").unwrap();
        let loaded = load_json::<TestData>(&path);
        assert_eq!(loaded.source, LoadSource::Default);
        assert!(loaded.warning.is_some());
    }

    #[test]
    fn missing_file_is_not_a_warning() {
        let dir = TestDir::new();
        let path = dir.file("never-written.json");
        let loaded = load_json::<TestData>(&path);
        assert_eq!(loaded.source, LoadSource::Default);
        assert!(loaded.warning.is_none());
    }
}
