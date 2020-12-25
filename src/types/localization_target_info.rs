
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about the current localization target
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalizationTargetInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of available language packs for this application
  language_packs: Vec<LanguagePackInfo>,
  
}

impl RObject for LocalizationTargetInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "localizationTargetInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl LocalizationTargetInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLocalizationTargetInfoBuilder {
    let mut inner = LocalizationTargetInfo::default();
    inner.td_name = "localizationTargetInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLocalizationTargetInfoBuilder { inner }
  }

  pub fn language_packs(&self) -> &Vec<LanguagePackInfo> { &self.language_packs }

}

#[doc(hidden)]
pub struct RTDLocalizationTargetInfoBuilder {
  inner: LocalizationTargetInfo
}

impl RTDLocalizationTargetInfoBuilder {
  pub fn build(&self) -> LocalizationTargetInfo { self.inner.clone() }

   
  pub fn language_packs(&mut self, language_packs: Vec<LanguagePackInfo>) -> &mut Self {
    self.inner.language_packs = language_packs;
    self
  }

}

impl AsRef<LocalizationTargetInfo> for LocalizationTargetInfo {
  fn as_ref(&self) -> &LocalizationTargetInfo { self }
}

impl AsRef<LocalizationTargetInfo> for RTDLocalizationTargetInfoBuilder {
  fn as_ref(&self) -> &LocalizationTargetInfo { &self.inner }
}



