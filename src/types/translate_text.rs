use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Translates a text to the given language. Returns a 404 error if the translation can't be performed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TranslateText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text to translate

    #[serde(default)]
    text: String,
    /// A two-letter ISO 639-1 language code of the language from which the message is translated. If empty, the language will be detected automatically

    #[serde(default)]
    from_language_code: String,
    /// A two-letter ISO 639-1 language code of the language to which the message is translated

    #[serde(default)]
    to_language_code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TranslateText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TranslateText {}

impl TranslateText {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTranslateTextBuilder {
        let mut inner = TranslateText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "translateText".to_string();

        RTDTranslateTextBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn from_language_code(&self) -> &String {
        &self.from_language_code
    }

    pub fn to_language_code(&self) -> &String {
        &self.to_language_code
    }
}

#[doc(hidden)]
pub struct RTDTranslateTextBuilder {
    inner: TranslateText,
}

impl RTDTranslateTextBuilder {
    pub fn build(&self) -> TranslateText {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn from_language_code<T: AsRef<str>>(&mut self, from_language_code: T) -> &mut Self {
        self.inner.from_language_code = from_language_code.as_ref().to_string();
        self
    }

    pub fn to_language_code<T: AsRef<str>>(&mut self, to_language_code: T) -> &mut Self {
        self.inner.to_language_code = to_language_code.as_ref().to_string();
        self
    }
}

impl AsRef<TranslateText> for TranslateText {
    fn as_ref(&self) -> &TranslateText {
        self
    }
}

impl AsRef<TranslateText> for RTDTranslateTextBuilder {
    fn as_ref(&self) -> &TranslateText {
        &self.inner
    }
}
