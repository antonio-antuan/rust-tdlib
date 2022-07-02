use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns all scheduled messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatScheduledMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatScheduledMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatScheduledMessages {}

impl GetChatScheduledMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatScheduledMessagesBuilder {
        let mut inner = GetChatScheduledMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatScheduledMessages".to_string();

        GetChatScheduledMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetChatScheduledMessagesBuilder {
    inner: GetChatScheduledMessages,
}

#[deprecated]
pub type RTDGetChatScheduledMessagesBuilder = GetChatScheduledMessagesBuilder;

impl GetChatScheduledMessagesBuilder {
    pub fn build(&self) -> GetChatScheduledMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChatScheduledMessages> for GetChatScheduledMessages {
    fn as_ref(&self) -> &GetChatScheduledMessages {
        self
    }
}

impl AsRef<GetChatScheduledMessages> for GetChatScheduledMessagesBuilder {
    fn as_ref(&self) -> &GetChatScheduledMessages {
        &self.inner
    }
}
