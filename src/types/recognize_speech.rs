use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Recognizes speech in a voice note message. The message must be successfully sent and must not be scheduled. May return an error with a message "MSG_VOICE_TOO_LONG" if the voice note is too long to be recognized
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecognizeSpeech {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the message belongs
    chat_id: i64,
    /// Identifier of the message
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RecognizeSpeech {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RecognizeSpeech {}

impl RecognizeSpeech {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRecognizeSpeechBuilder {
        let mut inner = RecognizeSpeech::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "recognizeSpeech".to_string();

        RTDRecognizeSpeechBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct RTDRecognizeSpeechBuilder {
    inner: RecognizeSpeech,
}

impl RTDRecognizeSpeechBuilder {
    pub fn build(&self) -> RecognizeSpeech {
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
}

impl AsRef<RecognizeSpeech> for RecognizeSpeech {
    fn as_ref(&self) -> &RecognizeSpeech {
        self
    }
}

impl AsRef<RecognizeSpeech> for RTDRecognizeSpeechBuilder {
    fn as_ref(&self) -> &RecognizeSpeech {
        &self.inner
    }
}
