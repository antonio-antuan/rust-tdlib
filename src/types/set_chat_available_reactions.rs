use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes reactions, available in a chat. Available for basic groups, supergroups, and channels. Requires can_change_info administrator right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatAvailableReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Reactions available in the chat. All emoji reactions must be active

    #[serde(skip_serializing_if = "ChatAvailableReactions::_is_default")]
    available_reactions: ChatAvailableReactions,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatAvailableReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatAvailableReactions {}

impl SetChatAvailableReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetChatAvailableReactionsBuilder {
        let mut inner = SetChatAvailableReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatAvailableReactions".to_string();

        SetChatAvailableReactionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn available_reactions(&self) -> &ChatAvailableReactions {
        &self.available_reactions
    }
}

#[doc(hidden)]
pub struct SetChatAvailableReactionsBuilder {
    inner: SetChatAvailableReactions,
}

#[deprecated]
pub type RTDSetChatAvailableReactionsBuilder = SetChatAvailableReactionsBuilder;

impl SetChatAvailableReactionsBuilder {
    pub fn build(&self) -> SetChatAvailableReactions {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn available_reactions<T: AsRef<ChatAvailableReactions>>(
        &mut self,
        available_reactions: T,
    ) -> &mut Self {
        self.inner.available_reactions = available_reactions.as_ref().clone();
        self
    }
}

impl AsRef<SetChatAvailableReactions> for SetChatAvailableReactions {
    fn as_ref(&self) -> &SetChatAvailableReactions {
        self
    }
}

impl AsRef<SetChatAvailableReactions> for SetChatAvailableReactionsBuilder {
    fn as_ref(&self) -> &SetChatAvailableReactions {
        &self.inner
    }
}
