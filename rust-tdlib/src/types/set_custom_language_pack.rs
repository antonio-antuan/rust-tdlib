use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds or changes a custom local language pack to the current localization target
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetCustomLanguagePack {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Information about the language pack. Language pack ID must start with 'X', consist only of English letters, digits and hyphens, and must not exceed 64 characters. Can be called before authorization
    info: LanguagePackInfo,
    /// Strings of the new language pack
    strings: Vec<LanguagePackString>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetCustomLanguagePack {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetCustomLanguagePack {}

impl SetCustomLanguagePack {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetCustomLanguagePackBuilder {
        let mut inner = SetCustomLanguagePack::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setCustomLanguagePack".to_string();

        RTDSetCustomLanguagePackBuilder { inner }
    }

    pub fn info(&self) -> &LanguagePackInfo {
        &self.info
    }

    pub fn strings(&self) -> &Vec<LanguagePackString> {
        &self.strings
    }
}

#[doc(hidden)]
pub struct RTDSetCustomLanguagePackBuilder {
    inner: SetCustomLanguagePack,
}

impl RTDSetCustomLanguagePackBuilder {
    pub fn build(&self) -> SetCustomLanguagePack {
        self.inner.clone()
    }

    pub fn info<T: AsRef<LanguagePackInfo>>(&mut self, info: T) -> &mut Self {
        self.inner.info = info.as_ref().clone();
        self
    }

    pub fn strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self {
        self.inner.strings = strings;
        self
    }
}

impl AsRef<SetCustomLanguagePack> for SetCustomLanguagePack {
    fn as_ref(&self) -> &SetCustomLanguagePack {
        self
    }
}

impl AsRef<SetCustomLanguagePack> for RTDSetCustomLanguagePackBuilder {
    fn as_ref(&self) -> &SetCustomLanguagePack {
        &self.inner
    }
}
