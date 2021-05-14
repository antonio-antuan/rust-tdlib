use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns strings from a language pack in the current localization target by their keys. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLanguagePackStrings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Language pack identifier of the strings to be returned
    language_pack_id: String,
    /// Language pack keys of the strings to be returned; leave empty to request all available strings
    keys: Vec<String>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetLanguagePackStrings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetLanguagePackStrings {}

impl GetLanguagePackStrings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetLanguagePackStringsBuilder {
        let mut inner = GetLanguagePackStrings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLanguagePackStrings".to_string();

        RTDGetLanguagePackStringsBuilder { inner }
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }

    pub fn keys(&self) -> &Vec<String> {
        &self.keys
    }
}

#[doc(hidden)]
pub struct RTDGetLanguagePackStringsBuilder {
    inner: GetLanguagePackStrings,
}

impl RTDGetLanguagePackStringsBuilder {
    pub fn build(&self) -> GetLanguagePackStrings {
        self.inner.clone()
    }

    pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
        self.inner.language_pack_id = language_pack_id.as_ref().to_string();
        self
    }

    pub fn keys(&mut self, keys: Vec<String>) -> &mut Self {
        self.inner.keys = keys;
        self
    }
}

impl AsRef<GetLanguagePackStrings> for GetLanguagePackStrings {
    fn as_ref(&self) -> &GetLanguagePackStrings {
        self
    }
}

impl AsRef<GetLanguagePackStrings> for RTDGetLanguagePackStringsBuilder {
    fn as_ref(&self) -> &GetLanguagePackStrings {
        &self.inner
    }
}
