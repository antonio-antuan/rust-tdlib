use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLanguagePackString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Path to the language pack database in which strings are stored
    language_pack_database_path: String,
    /// Localization target to which the language pack belongs
    localization_target: String,
    /// Language pack identifier
    language_pack_id: String,
    /// Language pack key of the string to be returned
    key: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetLanguagePackString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLanguagePackStringValue for GetLanguagePackString {}

impl RFunction for GetLanguagePackString {}

impl GetLanguagePackString {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetLanguagePackStringBuilder {
        let mut inner = GetLanguagePackString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLanguagePackString".to_string();

        RTDGetLanguagePackStringBuilder { inner }
    }

    pub fn language_pack_database_path(&self) -> &String {
        &self.language_pack_database_path
    }

    pub fn localization_target(&self) -> &String {
        &self.localization_target
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }

    pub fn key(&self) -> &String {
        &self.key
    }
}

#[doc(hidden)]
pub struct RTDGetLanguagePackStringBuilder {
    inner: GetLanguagePackString,
}

impl RTDGetLanguagePackStringBuilder {
    pub fn build(&self) -> GetLanguagePackString {
        self.inner.clone()
    }

    pub fn language_pack_database_path<T: AsRef<str>>(
        &mut self,
        language_pack_database_path: T,
    ) -> &mut Self {
        self.inner.language_pack_database_path = language_pack_database_path.as_ref().to_string();
        self
    }

    pub fn localization_target<T: AsRef<str>>(&mut self, localization_target: T) -> &mut Self {
        self.inner.localization_target = localization_target.as_ref().to_string();
        self
    }

    pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
        self.inner.language_pack_id = language_pack_id.as_ref().to_string();
        self
    }

    pub fn key<T: AsRef<str>>(&mut self, key: T) -> &mut Self {
        self.inner.key = key.as_ref().to_string();
        self
    }
}

impl AsRef<GetLanguagePackString> for GetLanguagePackString {
    fn as_ref(&self) -> &GetLanguagePackString {
        self
    }
}

impl AsRef<GetLanguagePackString> for RTDGetLanguagePackStringBuilder {
    fn as_ref(&self) -> &GetLanguagePackString {
        &self.inner
    }
}
