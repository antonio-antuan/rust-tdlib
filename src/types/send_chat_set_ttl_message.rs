use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the current TTL setting (sets a new self-destruct timer) in a secret chat and sends the corresponding message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendChatSetTtlMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// New TTL value, in seconds
    ttl: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendChatSetTtlMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendChatSetTtlMessage {}

impl SendChatSetTtlMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendChatSetTtlMessageBuilder {
        let mut inner = SendChatSetTtlMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendChatSetTtlMessage".to_string();

        RTDSendChatSetTtlMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn ttl(&self) -> i32 {
        self.ttl
    }
}

#[doc(hidden)]
pub struct RTDSendChatSetTtlMessageBuilder {
    inner: SendChatSetTtlMessage,
}

impl RTDSendChatSetTtlMessageBuilder {
    pub fn build(&self) -> SendChatSetTtlMessage {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn ttl(&mut self, ttl: i32) -> &mut Self {
        self.inner.ttl = ttl;
        self
    }
}

impl AsRef<SendChatSetTtlMessage> for SendChatSetTtlMessage {
    fn as_ref(&self) -> &SendChatSetTtlMessage {
        self
    }
}

impl AsRef<SendChatSetTtlMessage> for RTDSendChatSetTtlMessageBuilder {
    fn as_ref(&self) -> &SendChatSetTtlMessage {
        &self.inner
    }
}
