use anyhow::Result;
use fs::Fs;
use serde_json::{Map, Value};

use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VsCodeSettingsSource {
    VsCode,
    Cursor,
}

impl std::fmt::Display for VsCodeSettingsSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VsCodeSettingsSource::VsCode => write!(f, "VS Code"),
            VsCodeSettingsSource::Cursor => write!(f, "Cursor"),
        }
    }
}

pub struct VsCodeSettings {
    pub source: VsCodeSettingsSource,
    content: Map<String, Value>,
}

impl VsCodeSettings {
    pub fn from_str(content: &str, source: VsCodeSettingsSource) -> Result<Self> {
        Ok(Self {
            source,
            content: serde_json_lenient::from_str(content)?,
        })
    }

    pub async fn load_user_settings(source: VsCodeSettingsSource, fs: Arc<dyn Fs>) -> Result<Self> {
        let path = match source {
            VsCodeSettingsSource::VsCode => paths::vscode_settings_file(),
            VsCodeSettingsSource::Cursor => paths::cursor_settings_file(),
        };
        let content = fs.load(path).await?;
        Ok(Self {
            source,
            content: serde_json_lenient::from_str(&content)?,
        })
    }

    pub fn read_value(&self, setting: &str) -> Option<&Value> {
        if let Some(value) = self.content.get(setting) {
            return Some(value);
        }
        // TODO: maybe check if it's in [platform] settings for current platform as a fallback
        // TODO: deal with language specific settings
        None
    }

    pub fn read_string(&self, setting: &str) -> Option<&str> {
        self.read_value(setting).and_then(|v| v.as_str())
    }

    pub fn read_bool(&self, setting: &str) -> Option<bool> {
        self.read_value(setting).and_then(|v| v.as_bool())
    }

    pub fn string_setting(&self, key: &str, setting: &mut Option<String>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_str) {
            *setting = Some(s.to_owned())
        }
    }

    pub fn bool_setting(&self, key: &str, setting: &mut Option<bool>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_bool) {
            *setting = Some(s)
        }
    }

    pub fn u32_setting(&self, key: &str, setting: &mut Option<u32>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s as u32)
        }
    }

    pub fn u64_setting(&self, key: &str, setting: &mut Option<u64>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s)
        }
    }

    pub fn usize_setting(&self, key: &str, setting: &mut Option<usize>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_u64) {
            *setting = Some(s.try_into().unwrap())
        }
    }

    pub fn f32_setting(&self, key: &str, setting: &mut Option<f32>) {
        if let Some(s) = self.content.get(key).and_then(Value::as_f64) {
            *setting = Some(s as f32)
        }
    }

    pub fn enum_setting<T>(
        &self,
        key: &str,
        setting: &mut Option<T>,
        f: impl FnOnce(&str) -> Option<T>,
    ) {
        if let Some(s) = self.content.get(key).and_then(Value::as_str).and_then(f) {
            *setting = Some(s)
        }
    }
}
