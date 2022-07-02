use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns list of message sender identifiers, which can be used to send messages in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatAvailableMessageSenders {
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

impl RObject for GetChatAvailableMessageSenders {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatAvailableMessageSenders {}

impl GetChatAvailableMessageSenders {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatAvailableMessageSendersBuilder {
        let mut inner = GetChatAvailableMessageSenders::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatAvailableMessageSenders".to_string();

        GetChatAvailableMessageSendersBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetChatAvailableMessageSendersBuilder {
    inner: GetChatAvailableMessageSenders,
}

#[deprecated]
pub type RTDGetChatAvailableMessageSendersBuilder = GetChatAvailableMessageSendersBuilder;

impl GetChatAvailableMessageSendersBuilder {
    pub fn build(&self) -> GetChatAvailableMessageSenders {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChatAvailableMessageSenders> for GetChatAvailableMessageSenders {
    fn as_ref(&self) -> &GetChatAvailableMessageSenders {
        self
    }
}

impl AsRef<GetChatAvailableMessageSenders> for GetChatAvailableMessageSendersBuilder {
    fn as_ref(&self) -> &GetChatAvailableMessageSenders {
        &self.inner
    }
}
