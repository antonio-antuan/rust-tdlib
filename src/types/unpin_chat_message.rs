use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes a pinned message from a chat; requires can_pin_messages rights in the group or can_edit_messages rights in the channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnpinChatMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the removed pinned message

    #[serde(default)]
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for UnpinChatMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for UnpinChatMessage {}

impl UnpinChatMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UnpinChatMessageBuilder {
        let mut inner = UnpinChatMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "unpinChatMessage".to_string();

        UnpinChatMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct UnpinChatMessageBuilder {
    inner: UnpinChatMessage,
}

#[deprecated]
pub type RTDUnpinChatMessageBuilder = UnpinChatMessageBuilder;

impl UnpinChatMessageBuilder {
    pub fn build(&self) -> UnpinChatMessage {
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

impl AsRef<UnpinChatMessage> for UnpinChatMessage {
    fn as_ref(&self) -> &UnpinChatMessage {
        self
    }
}

impl AsRef<UnpinChatMessage> for UnpinChatMessageBuilder {
    fn as_ref(&self) -> &UnpinChatMessage {
        &self.inner
    }
}
