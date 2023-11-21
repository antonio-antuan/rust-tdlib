use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Extracts text or caption of the given message and translates it to the given language. If the current user is a Telegram Premium user, then text formatting is preserved
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TranslateMessageText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the message belongs

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message

    #[serde(default)]
    message_id: i64,
    /// Language code of the language to which the message is translated. Must be one of "af", "sq", "am", "ar", "hy", "az", "eu", "be", "bn", "bs", "bg", "ca", "ceb", "zh-CN", "zh", "zh-Hans", "zh-TW", "zh-Hant", "co", "hr", "cs", "da", "nl", "en", "eo", "et", "fi", "fr", "fy", "gl", "ka", "de", "el", "gu", "ht", "ha", "haw", "he", "iw", "hi", "hmn", "hu", "is", "ig", "id", "in", "ga", "it", "ja", "jv", "kn", "kk", "km", "rw", "ko", "ku", "ky", "lo", "la", "lv", "lt", "lb", "mk", "mg", "ms", "ml", "mt", "mi", "mr", "mn", "my", "ne", "no", "ny", "or", "ps", "fa", "pl", "pt", "pa", "ro", "ru", "sm", "gd", "sr", "st", "sn", "sd", "si", "sk", "sl", "so", "es", "su", "sw", "sv", "tl", "tg", "ta", "tt", "te", "th", "tr", "tk", "uk", "ur", "ug", "uz", "vi", "cy", "xh", "yi", "ji", "yo", "zu"

    #[serde(default)]
    to_language_code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TranslateMessageText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TranslateMessageText {}

impl TranslateMessageText {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TranslateMessageTextBuilder {
        let mut inner = TranslateMessageText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "translateMessageText".to_string();

        TranslateMessageTextBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn to_language_code(&self) -> &String {
        &self.to_language_code
    }
}

#[doc(hidden)]
pub struct TranslateMessageTextBuilder {
    inner: TranslateMessageText,
}

#[deprecated]
pub type RTDTranslateMessageTextBuilder = TranslateMessageTextBuilder;

impl TranslateMessageTextBuilder {
    pub fn build(&self) -> TranslateMessageText {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn to_language_code<T: AsRef<str>>(&mut self, to_language_code: T) -> &mut Self {
        self.inner.to_language_code = to_language_code.as_ref().to_string();
        self
    }
}

impl AsRef<TranslateMessageText> for TranslateMessageText {
    fn as_ref(&self) -> &TranslateMessageText {
        self
    }
}

impl AsRef<TranslateMessageText> for TranslateMessageTextBuilder {
    fn as_ref(&self) -> &TranslateMessageText {
        &self.inner
    }
}
