use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLanguagePackInfo {
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

impl RObject for GetLanguagePackInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetLanguagePackInfo {}

impl GetLanguagePackInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetLanguagePackInfoBuilder {
        let mut inner = GetLanguagePackInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLanguagePackInfo".to_string();

        GetLanguagePackInfoBuilder { inner }
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }
}

#[doc(hidden)]
pub struct GetLanguagePackInfoBuilder {
    inner: GetLanguagePackInfo,
}

#[deprecated]
pub type RTDGetLanguagePackInfoBuilder = GetLanguagePackInfoBuilder;

impl GetLanguagePackInfoBuilder {
    pub fn build(&self) -> GetLanguagePackInfo {
        self.inner.clone()
    }

    pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
        self.inner.language_pack_id = language_pack_id.as_ref().to_string();
        self
    }
}

impl AsRef<GetLanguagePackInfo> for GetLanguagePackInfo {
    fn as_ref(&self) -> &GetLanguagePackInfo {
        self
    }
}

impl AsRef<GetLanguagePackInfo> for GetLanguagePackInfoBuilder {
    fn as_ref(&self) -> &GetLanguagePackInfo {
        &self.inner
    }
}
