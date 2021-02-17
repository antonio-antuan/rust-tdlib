use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method shouldn't be called explicitly for the current used/base language packs. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SynchronizeLanguagePack {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Language pack identifier
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSynchronizeLanguagePackBuilder {
        let mut inner = SynchronizeLanguagePack::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "synchronizeLanguagePack".to_string();

        RTDSynchronizeLanguagePackBuilder { inner }
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }
}

#[doc(hidden)]
pub struct RTDSynchronizeLanguagePackBuilder {
    inner: SynchronizeLanguagePack,
}

impl RTDSynchronizeLanguagePackBuilder {
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

impl AsRef<SynchronizeLanguagePack> for RTDSynchronizeLanguagePackBuilder {
    fn as_ref(&self) -> &SynchronizeLanguagePack {
        &self.inner
    }
}
