use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Information about the message sent by answerWebAppQuery
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the sent inline message, if known

    #[serde(default)]
    inline_message_id: String,
}

impl RObject for SentWebAppMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl SentWebAppMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SentWebAppMessageBuilder {
        let mut inner = SentWebAppMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SentWebAppMessageBuilder { inner }
    }

    pub fn inline_message_id(&self) -> &String {
        &self.inline_message_id
    }
}

#[doc(hidden)]
pub struct SentWebAppMessageBuilder {
    inner: SentWebAppMessage,
}

#[deprecated]
pub type RTDSentWebAppMessageBuilder = SentWebAppMessageBuilder;

impl SentWebAppMessageBuilder {
    pub fn build(&self) -> SentWebAppMessage {
        self.inner.clone()
    }

    pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
        self.inner.inline_message_id = inline_message_id.as_ref().to_string();
        self
    }
}

impl AsRef<SentWebAppMessage> for SentWebAppMessage {
    fn as_ref(&self) -> &SentWebAppMessage {
        self
    }
}

impl AsRef<SentWebAppMessage> for SentWebAppMessageBuilder {
    fn as_ref(&self) -> &SentWebAppMessage {
        &self.inner
    }
}
