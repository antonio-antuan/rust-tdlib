use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Rates recognized speech in a voice note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RateSpeechRecognition {
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
    /// Pass true if the speech recognition is good

    #[serde(default)]
    is_good: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RateSpeechRecognition {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RateSpeechRecognition {}

impl RateSpeechRecognition {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRateSpeechRecognitionBuilder {
        let mut inner = RateSpeechRecognition::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "rateSpeechRecognition".to_string();

        RTDRateSpeechRecognitionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn is_good(&self) -> bool {
        self.is_good
    }
}

#[doc(hidden)]
pub struct RTDRateSpeechRecognitionBuilder {
    inner: RateSpeechRecognition,
}

impl RTDRateSpeechRecognitionBuilder {
    pub fn build(&self) -> RateSpeechRecognition {
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

    pub fn is_good(&mut self, is_good: bool) -> &mut Self {
        self.inner.is_good = is_good;
        self
    }
}

impl AsRef<RateSpeechRecognition> for RateSpeechRecognition {
    fn as_ref(&self) -> &RateSpeechRecognition {
        self
    }
}

impl AsRef<RateSpeechRecognition> for RTDRateSpeechRecognitionBuilder {
    fn as_ref(&self) -> &RateSpeechRecognition {
        &self.inner
    }
}
