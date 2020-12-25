
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains auto-download settings presets for the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoDownloadSettingsPresets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Preset with lowest settings; supposed to be used by default when roaming
  low: AutoDownloadSettings,
  /// Preset with medium settings; supposed to be used by default when using mobile data
  medium: AutoDownloadSettings,
  /// Preset with highest settings; supposed to be used by default when connected on Wi-Fi
  high: AutoDownloadSettings,
  
}

impl RObject for AutoDownloadSettingsPresets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "autoDownloadSettingsPresets" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl AutoDownloadSettingsPresets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAutoDownloadSettingsPresetsBuilder {
    let mut inner = AutoDownloadSettingsPresets::default();
    inner.td_name = "autoDownloadSettingsPresets".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAutoDownloadSettingsPresetsBuilder { inner }
  }

  pub fn low(&self) -> &AutoDownloadSettings { &self.low }

  pub fn medium(&self) -> &AutoDownloadSettings { &self.medium }

  pub fn high(&self) -> &AutoDownloadSettings { &self.high }

}

#[doc(hidden)]
pub struct RTDAutoDownloadSettingsPresetsBuilder {
  inner: AutoDownloadSettingsPresets
}

impl RTDAutoDownloadSettingsPresetsBuilder {
  pub fn build(&self) -> AutoDownloadSettingsPresets { self.inner.clone() }

   
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
  fn as_ref(&self) -> &AutoDownloadSettingsPresets { self }
}

impl AsRef<AutoDownloadSettingsPresets> for RTDAutoDownloadSettingsPresetsBuilder {
  fn as_ref(&self) -> &AutoDownloadSettingsPresets { &self.inner }
}



