use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Resends messages which failed to send. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed. If a message is re-sent, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be re-sent, null will be returned instead of the message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to send messages
    chat_id: i64,
    /// Identifiers of the messages to resend. Message identifiers must be in a strictly increasing order
    message_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResendMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResendMessages {}

impl ResendMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResendMessagesBuilder {
        let mut inner = ResendMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resendMessages".to_string();

        RTDResendMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }
}

#[doc(hidden)]
pub struct RTDResendMessagesBuilder {
    inner: ResendMessages,
}

impl RTDResendMessagesBuilder {
    pub fn build(&self) -> ResendMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
        self.inner.message_ids = message_ids;
        self
    }
}

impl AsRef<ResendMessages> for ResendMessages {
    fn as_ref(&self) -> &ResendMessages {
        self
    }
}

impl AsRef<ResendMessages> for RTDResendMessagesBuilder {
    fn as_ref(&self) -> &ResendMessages {
        &self.inner
    }
}
