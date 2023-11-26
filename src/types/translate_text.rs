use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Translates a text to the given language. If the current user is a Telegram Premium user, then text formatting is preserved
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TranslateText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text to translate
    text: FormattedText,
    /// Language code of the language to which the message is translated. Must be one of "af", "sq", "am", "ar", "hy", "az", "eu", "be", "bn", "bs", "bg", "ca", "ceb", "zh-CN", "zh", "zh-Hans", "zh-TW", "zh-Hant", "co", "hr", "cs", "da", "nl", "en", "eo", "et", "fi", "fr", "fy", "gl", "ka", "de", "el", "gu", "ht", "ha", "haw", "he", "iw", "hi", "hmn", "hu", "is", "ig", "id", "in", "ga", "it", "ja", "jv", "kn", "kk", "km", "rw", "ko", "ku", "ky", "lo", "la", "lv", "lt", "lb", "mk", "mg", "ms", "ml", "mt", "mi", "mr", "mn", "my", "ne", "no", "ny", "or", "ps", "fa", "pl", "pt", "pa", "ro", "ru", "sm", "gd", "sr", "st", "sn", "sd", "si", "sk", "sl", "so", "es", "su", "sw", "sv", "tl", "tg", "ta", "tt", "te", "th", "tr", "tk", "uk", "ur", "ug", "uz", "vi", "cy", "xh", "yi", "ji", "yo", "zu"

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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TranslateTextBuilder {
        let mut inner = TranslateText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "translateText".to_string();

        TranslateTextBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    pub fn to_language_code(&self) -> &String {
        &self.to_language_code
    }
}

#[doc(hidden)]
pub struct TranslateTextBuilder {
    inner: TranslateText,
}

#[deprecated]
pub type RTDTranslateTextBuilder = TranslateTextBuilder;

impl TranslateTextBuilder {
    pub fn build(&self) -> TranslateText {
        self.inner.clone()
    }

    pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
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

impl AsRef<TranslateText> for TranslateTextBuilder {
    fn as_ref(&self) -> &TranslateText {
        &self.inner
    }
}
