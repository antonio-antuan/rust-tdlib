use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of message senders, which can be used to send messages in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMessageSenders {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of available message senders

    #[serde(default)]
    senders: Vec<ChatMessageSender>,
}

impl RObject for ChatMessageSenders {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatMessageSenders {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMessageSendersBuilder {
        let mut inner = ChatMessageSenders::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMessageSendersBuilder { inner }
    }

    pub fn senders(&self) -> &Vec<ChatMessageSender> {
        &self.senders
    }
}

#[doc(hidden)]
pub struct ChatMessageSendersBuilder {
    inner: ChatMessageSenders,
}

#[deprecated]
pub type RTDChatMessageSendersBuilder = ChatMessageSendersBuilder;

impl ChatMessageSendersBuilder {
    pub fn build(&self) -> ChatMessageSenders {
        self.inner.clone()
    }

    pub fn senders(&mut self, senders: Vec<ChatMessageSender>) -> &mut Self {
        self.inner.senders = senders;
        self
    }
}

impl AsRef<ChatMessageSenders> for ChatMessageSenders {
    fn as_ref(&self) -> &ChatMessageSenders {
        self
    }
}

impl AsRef<ChatMessageSenders> for ChatMessageSendersBuilder {
    fn as_ref(&self) -> &ChatMessageSenders {
        &self.inner
    }
}
