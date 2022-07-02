use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SynchronizeLanguagePack {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Language pack identifier

    #[serde(default)]
    language_pack_id: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SynchronizeLanguagePack {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SynchronizeLanguagePack {}

impl SynchronizeLanguagePack {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SynchronizeLanguagePackBuilder {
        let mut inner = SynchronizeLanguagePack::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "synchronizeLanguagePack".to_string();

        SynchronizeLanguagePackBuilder { inner }
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }
}

#[doc(hidden)]
pub struct SynchronizeLanguagePackBuilder {
    inner: SynchronizeLanguagePack,
}

#[deprecated]
pub type RTDSynchronizeLanguagePackBuilder = SynchronizeLanguagePackBuilder;

impl SynchronizeLanguagePackBuilder {
    pub fn build(&self) -> SynchronizeLanguagePack {
        self.inner.clone()
    }

    pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
        self.inner.language_pack_id = language_pack_id.as_ref().to_string();
        self
    }
}

impl AsRef<SynchronizeLanguagePack> for SynchronizeLanguagePack {
    fn as_ref(&self) -> &SynchronizeLanguagePack {
        self
    }
}

impl AsRef<SynchronizeLanguagePack> for SynchronizeLanguagePackBuilder {
    fn as_ref(&self) -> &SynchronizeLanguagePack {
        &self.inner
    }
}
