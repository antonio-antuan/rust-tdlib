use crate::errors::Result;
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LocalizationTargetInfoBuilder {
        let mut inner = LocalizationTargetInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        LocalizationTargetInfoBuilder { inner }
    }

    pub fn language_packs(&self) -> &Vec<LanguagePackInfo> {
        &self.language_packs
    }
}

#[doc(hidden)]
pub struct LocalizationTargetInfoBuilder {
    inner: LocalizationTargetInfo,
}

#[deprecated]
pub type RTDLocalizationTargetInfoBuilder = LocalizationTargetInfoBuilder;

impl LocalizationTargetInfoBuilder {
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

impl AsRef<LocalizationTargetInfo> for LocalizationTargetInfoBuilder {
    fn as_ref(&self) -> &LocalizationTargetInfo {
        &self.inner
    }
}
