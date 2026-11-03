//! Loading stories from the embedded index or from a directory.

use super::model::Story;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// A story plus the path it came from, used by validation messages.
#[derive(Debug, Clone)]
pub struct LoadedStory {
    /// Path relative to `assets/stories`, e.g. `a1/01-de-fiets.json`.
    pub source: String,
    pub story: Story,
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("cannot read {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid story JSON in {story}: {message}")]
    Json { story: String, message: String },
}

impl LoadedStory {
    pub fn from_json(source: impl Into<String>, json: &str) -> Result<Self, LoadError> {
        let source = source.into();
        let mut story: Story = serde_json::from_str(json).map_err(|e| LoadError::Json {
            story: source.clone(),
            message: e.to_string(),
        })?;
        story.merge_starter_glossary();
        Ok(LoadedStory { source, story })
    }
}

/// Load every story compiled into the binary by `build.rs`.
pub fn load_embedded() -> Result<Vec<LoadedStory>, LoadError> {
    let mut stories = Vec::new();
    for embedded in super::index::STORIES {
        stories.push(LoadedStory::from_json(embedded.source, embedded.json)?);
    }
    sort_stories(&mut stories);
    Ok(stories)
}

/// Load every `*.json` under `<root>/<level>/`.
///
/// Used by tests and by the validator's `--stories` option. The returned source
/// labels are relative to `root` with forward slashes.
pub fn load_dir(root: &Path) -> Result<Vec<LoadedStory>, LoadError> {
    let mut stories = Vec::new();
    for level in ["a1", "a2", "b1", "b2"] {
        let dir = root.join(level);
        if !dir.is_dir() {
            continue;
        }
        let mut files = match std::fs::read_dir(&dir) {
            Ok(iter) => iter
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("json"))
                .collect::<Vec<_>>(),
            Err(e) => {
                return Err(LoadError::Io {
                    path: dir,
                    source: e,
                });
            }
        };
        files.sort();
        for path in files {
            let raw = std::fs::read_to_string(&path).map_err(|e| LoadError::Io {
                path: path.clone(),
                source: e,
            })?;
            let source = format!("{}/{}", level, path.file_name().unwrap().to_string_lossy());
            stories.push(LoadedStory::from_json(source, &raw)?);
        }
    }
    sort_stories(&mut stories);
    Ok(stories)
}

/// Sort globally by (level, order, id).
pub fn sort_stories(stories: &mut [LoadedStory]) {
    stories.sort_by(|a, b| {
        a.story
            .level
            .cmp(&b.story.level)
            .then(a.story.order.cmp(&b.story.order))
            .then(a.story.id.cmp(&b.story.id))
    });
}

/// Extract the level folder and file name from a source label.
///
/// Works with both `/` and `\` separators so Windows test paths behave the same
/// as Android asset paths.
pub fn split_source(source: &str) -> (Option<&str>, Option<&str>) {
    let mut parts = source.rsplit(['/', '\\']);
    let file = parts.next();
    let folder = parts.next();
    (folder, file)
}
