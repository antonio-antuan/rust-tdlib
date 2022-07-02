use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes all pinned messages from a chat; requires can_pin_messages rights in the group or can_edit_messages rights in the channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnpinAllChatMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for UnpinAllChatMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for UnpinAllChatMessages {}

impl UnpinAllChatMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UnpinAllChatMessagesBuilder {
        let mut inner = UnpinAllChatMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "unpinAllChatMessages".to_string();

        UnpinAllChatMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct UnpinAllChatMessagesBuilder {
    inner: UnpinAllChatMessages,
}

#[deprecated]
pub type RTDUnpinAllChatMessagesBuilder = UnpinAllChatMessagesBuilder;

impl UnpinAllChatMessagesBuilder {
    pub fn build(&self) -> UnpinAllChatMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<UnpinAllChatMessages> for UnpinAllChatMessages {
    fn as_ref(&self) -> &UnpinAllChatMessages {
        self
    }
}

impl AsRef<UnpinAllChatMessages> for UnpinAllChatMessagesBuilder {
    fn as_ref(&self) -> &UnpinAllChatMessages {
        &self.inner
    }
}
