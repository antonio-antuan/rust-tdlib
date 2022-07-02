use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains auto-download settings presets for the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoDownloadSettingsPresets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Preset with lowest settings; supposed to be used by default when roaming
    low: AutoDownloadSettings,
    /// Preset with medium settings; supposed to be used by default when using mobile data
    medium: AutoDownloadSettings,
    /// Preset with highest settings; supposed to be used by default when connected on Wi-Fi
    high: AutoDownloadSettings,
}

impl RObject for AutoDownloadSettingsPresets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AutoDownloadSettingsPresets {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AutoDownloadSettingsPresetsBuilder {
        let mut inner = AutoDownloadSettingsPresets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AutoDownloadSettingsPresetsBuilder { inner }
    }

    pub fn low(&self) -> &AutoDownloadSettings {
        &self.low
    }

    pub fn medium(&self) -> &AutoDownloadSettings {
        &self.medium
    }

    pub fn high(&self) -> &AutoDownloadSettings {
        &self.high
    }
}

#[doc(hidden)]
pub struct AutoDownloadSettingsPresetsBuilder {
    inner: AutoDownloadSettingsPresets,
}

#[deprecated]
pub type RTDAutoDownloadSettingsPresetsBuilder = AutoDownloadSettingsPresetsBuilder;

impl AutoDownloadSettingsPresetsBuilder {
    pub fn build(&self) -> AutoDownloadSettingsPresets {
        self.inner.clone()
    }

    pub fn low<T: AsRef<AutoDownloadSettings>>(&mut self, low: T) -> &mut Self {
        self.inner.low = low.as_ref().clone();
        self
    }

    pub fn medium<T: AsRef<AutoDownloadSettings>>(&mut self, medium: T) -> &mut Self {
        self.inner.medium = medium.as_ref().clone();
        self
    }

    pub fn high<T: AsRef<AutoDownloadSettings>>(&mut self, high: T) -> &mut Self {
        self.inner.high = high.as_ref().clone();
        self
    }
}

impl AsRef<AutoDownloadSettingsPresets> for AutoDownloadSettingsPresets {
    fn as_ref(&self) -> &AutoDownloadSettingsPresets {
        self
    }
}

impl AsRef<AutoDownloadSettingsPresets> for AutoDownloadSettingsPresetsBuilder {
    fn as_ref(&self) -> &AutoDownloadSettingsPresets {
        &self.inner
    }
}
