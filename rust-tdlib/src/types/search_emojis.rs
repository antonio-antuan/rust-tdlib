use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for emojis by keywords. Supported only if the file database is enabled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchEmojis {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text to search for
    text: String,
    /// True, if only emojis, which exactly match text needs to be returned
    exact_match: bool,
    /// List of possible IETF language tags of the user's input language; may be empty if unknown
    input_language_codes: Vec<String>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchEmojis {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchEmojis {}

impl SearchEmojis {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchEmojisBuilder {
        let mut inner = SearchEmojis::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchEmojis".to_string();

        RTDSearchEmojisBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn exact_match(&self) -> bool {
        self.exact_match
    }

    pub fn input_language_codes(&self) -> &Vec<String> {
        &self.input_language_codes
    }
}

#[doc(hidden)]
pub struct RTDSearchEmojisBuilder {
    inner: SearchEmojis,
}

impl RTDSearchEmojisBuilder {
    pub fn build(&self) -> SearchEmojis {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn exact_match(&mut self, exact_match: bool) -> &mut Self {
        self.inner.exact_match = exact_match;
        self
    }

    pub fn input_language_codes(&mut self, input_language_codes: Vec<String>) -> &mut Self {
        self.inner.input_language_codes = input_language_codes;
        self
    }
}

impl AsRef<SearchEmojis> for SearchEmojis {
    fn as_ref(&self) -> &SearchEmojis {
        self
    }
}

impl AsRef<SearchEmojis> for RTDSearchEmojisBuilder {
    fn as_ref(&self) -> &SearchEmojis {
        &self.inner
    }
}
