use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes reactions available in the chat
pub trait TDChatAvailableReactions: Debug + RObject {}

/// Describes reactions available in the chat
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum ChatAvailableReactions {
    #[doc(hidden)]
    #[default]
    _Default,
    /// All reactions are available in the chat
    #[serde(rename = "chatAvailableReactionsAll")]
    All(ChatAvailableReactionsAll),
    /// Only specific reactions are available in the chat
    #[serde(rename = "chatAvailableReactionsSome")]
    Some(ChatAvailableReactionsSome),
}

impl RObject for ChatAvailableReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatAvailableReactions::All(t) => t.extra(),
            ChatAvailableReactions::Some(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatAvailableReactions::All(t) => t.client_id(),
            ChatAvailableReactions::Some(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatAvailableReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatAvailableReactions::_Default)
    }
}

impl AsRef<ChatAvailableReactions> for ChatAvailableReactions {
    fn as_ref(&self) -> &ChatAvailableReactions {
        self
    }
}

/// All reactions are available in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatAvailableReactionsAll {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatAvailableReactionsAll {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAvailableReactions for ChatAvailableReactionsAll {}

impl ChatAvailableReactionsAll {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatAvailableReactionsAllBuilder {
        let mut inner = ChatAvailableReactionsAll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatAvailableReactionsAllBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatAvailableReactionsAllBuilder {
    inner: ChatAvailableReactionsAll,
}

#[deprecated]
pub type RTDChatAvailableReactionsAllBuilder = ChatAvailableReactionsAllBuilder;

impl ChatAvailableReactionsAllBuilder {
    pub fn build(&self) -> ChatAvailableReactionsAll {
        self.inner.clone()
    }
}

impl AsRef<ChatAvailableReactionsAll> for ChatAvailableReactionsAll {
    fn as_ref(&self) -> &ChatAvailableReactionsAll {
        self
    }
}

impl AsRef<ChatAvailableReactionsAll> for ChatAvailableReactionsAllBuilder {
    fn as_ref(&self) -> &ChatAvailableReactionsAll {
        &self.inner
    }
}

/// Only specific reactions are available in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatAvailableReactionsSome {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The list of reactions

    #[serde(default)]
    reactions: Vec<ReactionType>,
}

impl RObject for ChatAvailableReactionsSome {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAvailableReactions for ChatAvailableReactionsSome {}

impl ChatAvailableReactionsSome {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatAvailableReactionsSomeBuilder {
        let mut inner = ChatAvailableReactionsSome::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatAvailableReactionsSomeBuilder { inner }
    }

    pub fn reactions(&self) -> &Vec<ReactionType> {
        &self.reactions
    }
}

#[doc(hidden)]
pub struct ChatAvailableReactionsSomeBuilder {
    inner: ChatAvailableReactionsSome,
}

#[deprecated]
pub type RTDChatAvailableReactionsSomeBuilder = ChatAvailableReactionsSomeBuilder;

impl ChatAvailableReactionsSomeBuilder {
    pub fn build(&self) -> ChatAvailableReactionsSome {
        self.inner.clone()
    }

    pub fn reactions(&mut self, reactions: Vec<ReactionType>) -> &mut Self {
        self.inner.reactions = reactions;
        self
    }
}

impl AsRef<ChatAvailableReactionsSome> for ChatAvailableReactionsSome {
    fn as_ref(&self) -> &ChatAvailableReactionsSome {
        self
    }
}

impl AsRef<ChatAvailableReactionsSome> for ChatAvailableReactionsSomeBuilder {
    fn as_ref(&self) -> &ChatAvailableReactionsSome {
        &self.inner
    }
}
