use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about the current localization target
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalizationTargetInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of available language packs for this application
    language_packs: Vec<LanguagePackInfo>,
}

impl RObject for LocalizationTargetInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl LocalizationTargetInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLocalizationTargetInfoBuilder {
        let mut inner = LocalizationTargetInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDLocalizationTargetInfoBuilder { inner }
    }

    pub fn language_packs(&self) -> &Vec<LanguagePackInfo> {
        &self.language_packs
    }
}

#[doc(hidden)]
pub struct RTDLocalizationTargetInfoBuilder {
    inner: LocalizationTargetInfo,
}

impl RTDLocalizationTargetInfoBuilder {
    pub fn build(&self) -> LocalizationTargetInfo {
        self.inner.clone()
    }

    pub fn language_packs(&mut self, language_packs: Vec<LanguagePackInfo>) -> &mut Self {
        self.inner.language_packs = language_packs;
        self
    }
}

impl AsRef<LocalizationTargetInfo> for LocalizationTargetInfo {
    fn as_ref(&self) -> &LocalizationTargetInfo {
        self
    }
}

impl AsRef<LocalizationTargetInfo> for RTDLocalizationTargetInfoBuilder {
    fn as_ref(&self) -> &LocalizationTargetInfo {
        &self.inner
    }
}
