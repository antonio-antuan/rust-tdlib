use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTP URL which can be used to automatically log in to the translation platform and suggest new emoji replacements. The URL will be valid for 30 seconds after generation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetEmojiSuggestionsUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Language code for which the emoji replacements will be suggested
    language_code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetEmojiSuggestionsUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetEmojiSuggestionsUrl {}

impl GetEmojiSuggestionsUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetEmojiSuggestionsUrlBuilder {
        let mut inner = GetEmojiSuggestionsUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getEmojiSuggestionsUrl".to_string();

        RTDGetEmojiSuggestionsUrlBuilder { inner }
    }

    pub fn language_code(&self) -> &String {
        &self.language_code
    }
}

#[doc(hidden)]
pub struct RTDGetEmojiSuggestionsUrlBuilder {
    inner: GetEmojiSuggestionsUrl,
}

impl RTDGetEmojiSuggestionsUrlBuilder {
    pub fn build(&self) -> GetEmojiSuggestionsUrl {
        self.inner.clone()
    }

    pub fn language_code<T: AsRef<str>>(&mut self, language_code: T) -> &mut Self {
        self.inner.language_code = language_code.as_ref().to_string();
        self
    }
}

impl AsRef<GetEmojiSuggestionsUrl> for GetEmojiSuggestionsUrl {
    fn as_ref(&self) -> &GetEmojiSuggestionsUrl {
        self
    }
}

impl AsRef<GetEmojiSuggestionsUrl> for RTDGetEmojiSuggestionsUrlBuilder {
    fn as_ref(&self) -> &GetEmojiSuggestionsUrl {
        &self.inner
    }
}
