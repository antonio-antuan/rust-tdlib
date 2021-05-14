use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a message that is replied by a given message. Also returns the pinned message, the game message, and the invoice message for messages of the types messagePinMessage, messageGameScore, and messagePaymentSuccessful respectively
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRepliedMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat the message belongs to
    chat_id: i64,
    /// Identifier of the message reply to which to get
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetRepliedMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetRepliedMessage {}

impl GetRepliedMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetRepliedMessageBuilder {
        let mut inner = GetRepliedMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getRepliedMessage".to_string();

        RTDGetRepliedMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct RTDGetRepliedMessageBuilder {
    inner: GetRepliedMessage,
}

impl RTDGetRepliedMessageBuilder {
    pub fn build(&self) -> GetRepliedMessage {
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

impl AsRef<GetRepliedMessage> for GetRepliedMessage {
    fn as_ref(&self) -> &GetRepliedMessage {
        self
    }
}

impl AsRef<GetRepliedMessage> for RTDGetRepliedMessageBuilder {
    fn as_ref(&self) -> &GetRepliedMessage {
        &self.inner
    }
}
