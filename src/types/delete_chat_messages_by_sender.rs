use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes all messages sent by the specified message sender in a chat. Supported only for supergroups; requires can_delete_messages administrator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatMessagesBySender {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the sender of messages to delete

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteChatMessagesBySender {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteChatMessagesBySender {}

impl DeleteChatMessagesBySender {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteChatMessagesBySenderBuilder {
        let mut inner = DeleteChatMessagesBySender::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteChatMessagesBySender".to_string();

        DeleteChatMessagesBySenderBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }
}

#[doc(hidden)]
pub struct DeleteChatMessagesBySenderBuilder {
    inner: DeleteChatMessagesBySender,
}

#[deprecated]
pub type RTDDeleteChatMessagesBySenderBuilder = DeleteChatMessagesBySenderBuilder;

impl DeleteChatMessagesBySenderBuilder {
    pub fn build(&self) -> DeleteChatMessagesBySender {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
        self.inner.sender_id = sender_id.as_ref().clone();
        self
    }
}

impl AsRef<DeleteChatMessagesBySender> for DeleteChatMessagesBySender {
    fn as_ref(&self) -> &DeleteChatMessagesBySender {
        self
    }
}

impl AsRef<DeleteChatMessagesBySender> for DeleteChatMessagesBySenderBuilder {
    fn as_ref(&self) -> &DeleteChatMessagesBySender {
        &self.inner
    }
}
