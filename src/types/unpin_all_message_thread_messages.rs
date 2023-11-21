use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes all pinned messages from a forum topic; requires can_pin_messages rights in the supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnpinAllMessageThreadMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Message thread identifier in which messages will be unpinned

    #[serde(default)]
    message_thread_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for UnpinAllMessageThreadMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for UnpinAllMessageThreadMessages {}

impl UnpinAllMessageThreadMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UnpinAllMessageThreadMessagesBuilder {
        let mut inner = UnpinAllMessageThreadMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "unpinAllMessageThreadMessages".to_string();

        UnpinAllMessageThreadMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }
}

#[doc(hidden)]
pub struct UnpinAllMessageThreadMessagesBuilder {
    inner: UnpinAllMessageThreadMessages,
}

#[deprecated]
pub type RTDUnpinAllMessageThreadMessagesBuilder = UnpinAllMessageThreadMessagesBuilder;

impl UnpinAllMessageThreadMessagesBuilder {
    pub fn build(&self) -> UnpinAllMessageThreadMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }
}

impl AsRef<UnpinAllMessageThreadMessages> for UnpinAllMessageThreadMessages {
    fn as_ref(&self) -> &UnpinAllMessageThreadMessages {
        self
    }
}

impl AsRef<UnpinAllMessageThreadMessages> for UnpinAllMessageThreadMessagesBuilder {
    fn as_ref(&self) -> &UnpinAllMessageThreadMessages {
        &self.inner
    }
}
