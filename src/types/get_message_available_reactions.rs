use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns reactions, which can be added to a message. The list can change after updateActiveEmojiReactions, updateChatAvailableReactions for the chat, or updateMessageInteractionInfo for the message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageAvailableReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the message belongs

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message

    #[serde(default)]
    message_id: i64,
    /// Number of reaction per row, 5-25

    #[serde(default)]
    row_size: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageAvailableReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageAvailableReactions {}

impl GetMessageAvailableReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMessageAvailableReactionsBuilder {
        let mut inner = GetMessageAvailableReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageAvailableReactions".to_string();

        GetMessageAvailableReactionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn row_size(&self) -> i32 {
        self.row_size
    }
}

#[doc(hidden)]
pub struct GetMessageAvailableReactionsBuilder {
    inner: GetMessageAvailableReactions,
}

#[deprecated]
pub type RTDGetMessageAvailableReactionsBuilder = GetMessageAvailableReactionsBuilder;

impl GetMessageAvailableReactionsBuilder {
    pub fn build(&self) -> GetMessageAvailableReactions {
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

    pub fn row_size(&mut self, row_size: i32) -> &mut Self {
        self.inner.row_size = row_size;
        self
    }
}

impl AsRef<GetMessageAvailableReactions> for GetMessageAvailableReactions {
    fn as_ref(&self) -> &GetMessageAvailableReactions {
        self
    }
}

impl AsRef<GetMessageAvailableReactions> for GetMessageAvailableReactionsBuilder {
    fn as_ref(&self) -> &GetMessageAvailableReactions {
        &self.inner
    }
}
