use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a message sender, which can be used to send messages in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMessageSender {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The message sender

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender: MessageSender,
    /// True, if Telegram Premium is needed to use the message sender

    #[serde(default)]
    needs_premium: bool,
}

impl RObject for ChatMessageSender {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatMessageSender {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMessageSenderBuilder {
        let mut inner = ChatMessageSender::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMessageSenderBuilder { inner }
    }

    pub fn sender(&self) -> &MessageSender {
        &self.sender
    }

    pub fn needs_premium(&self) -> bool {
        self.needs_premium
    }
}

#[doc(hidden)]
pub struct ChatMessageSenderBuilder {
    inner: ChatMessageSender,
}

#[deprecated]
pub type RTDChatMessageSenderBuilder = ChatMessageSenderBuilder;

impl ChatMessageSenderBuilder {
    pub fn build(&self) -> ChatMessageSender {
        self.inner.clone()
    }

    pub fn sender<T: AsRef<MessageSender>>(&mut self, sender: T) -> &mut Self {
        self.inner.sender = sender.as_ref().clone();
        self
    }

    pub fn needs_premium(&mut self, needs_premium: bool) -> &mut Self {
        self.inner.needs_premium = needs_premium;
        self
    }
}

impl AsRef<ChatMessageSender> for ChatMessageSender {
    fn as_ref(&self) -> &ChatMessageSender {
        self
    }
}

impl AsRef<ChatMessageSender> for ChatMessageSenderBuilder {
    fn as_ref(&self) -> &ChatMessageSender {
        &self.inner
    }
}
