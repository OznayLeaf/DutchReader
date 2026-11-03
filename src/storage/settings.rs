//! User settings, stored in `settings.json`.

use super::atomic::{load_json, write_json_atomic};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    /// Follow the Android system setting.
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MeaningsSource {
    /// Use only the glossary bundled with each story.
    #[default]
    Offline,
    /// Use the online dictionary, with an automatic offline fallback.
    Online,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub theme: Theme,
    /// Android TextToSpeech rate. 1.0 is normal; the app starts slightly slower
    /// so beginners can follow the narration.
    #[serde(default = "default_tts_speed")]
    pub tts_speed: f32,
    #[serde(default)]
    pub meanings_source: MeaningsSource,
}

/// Default narration speed: 12% below Android's normal rate.
pub const DEFAULT_TTS_SPEED: f32 = 0.88;

fn default_tts_speed() -> f32 {
    DEFAULT_TTS_SPEED
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: Theme::System,
            tts_speed: DEFAULT_TTS_SPEED,
            meanings_source: MeaningsSource::Offline,
        }
    }
}

impl Settings {
    /// Keep values inside the ranges the UI exposes. Called after loading and
    /// before saving so a hand-edited file cannot break the app.
    pub fn sanitize(&mut self) {
        if !self.tts_speed.is_finite() {
            self.tts_speed = DEFAULT_TTS_SPEED;
        }
        self.tts_speed = self.tts_speed.clamp(0.5, 2.0);
    }

    pub fn load(path: &Path) -> Self {
        let loaded = load_json::<Settings>(path);
        if let Some(warning) = &loaded.warning {
            eprintln!("[settings] {warning}");
        }
        let mut settings = loaded.value;
        settings.sanitize();
        settings
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let mut settings = self.clone();
        settings.sanitize();
        write_json_atomic(path, &settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_offline_and_system_theme() {
        let settings = Settings::default();
        assert_eq!(settings.theme, Theme::System);
        assert_eq!(settings.meanings_source, MeaningsSource::Offline);
        assert_eq!(settings.tts_speed, DEFAULT_TTS_SPEED);
        assert_eq!(settings.tts_speed, 0.88);
    }

    #[test]
    fn sanitize_clamps_speed_and_repairs_nan() {
        let mut settings = Settings {
            tts_speed: 99.0,
            ..Settings::default()
        };
        settings.sanitize();
        assert_eq!(settings.tts_speed, 2.0);

        settings.tts_speed = f32::NAN;
        settings.sanitize();
        assert_eq!(settings.tts_speed, DEFAULT_TTS_SPEED);
    }

    #[test]
    fn missing_fields_use_defaults() {
        let settings: Settings = serde_json::from_str(r#"{"theme":"dark"}"#).unwrap();
        assert_eq!(settings.theme, Theme::Dark);
        assert_eq!(settings.tts_speed, DEFAULT_TTS_SPEED);
        assert_eq!(settings.meanings_source, MeaningsSource::Offline);
    }

    #[test]
    fn saved_speed_is_kept() {
        let settings: Settings = serde_json::from_str(r#"{"tts_speed":1.25}"#).unwrap();
        assert_eq!(settings.tts_speed, 1.25);
    }
}
