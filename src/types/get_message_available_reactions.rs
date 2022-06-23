use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns reactions, which can be added to a message. The list can change after updateReactions, updateChatAvailableReactions for the chat, or updateMessageInteractionInfo for the message. The method will return Premium reactions, even the current user has no Premium subscription
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageAvailableReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the message belongs
    chat_id: i64,
    /// Identifier of the message
    message_id: i64,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetMessageAvailableReactionsBuilder {
        let mut inner = GetMessageAvailableReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageAvailableReactions".to_string();

        RTDGetMessageAvailableReactionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct RTDGetMessageAvailableReactionsBuilder {
    inner: GetMessageAvailableReactions,
}

impl RTDGetMessageAvailableReactionsBuilder {
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
}

impl AsRef<GetMessageAvailableReactions> for GetMessageAvailableReactions {
    fn as_ref(&self) -> &GetMessageAvailableReactions {
        self
    }
}

impl AsRef<GetMessageAvailableReactions> for RTDGetMessageAvailableReactionsBuilder {
    fn as_ref(&self) -> &GetMessageAvailableReactions {
        &self.inner
    }
}
