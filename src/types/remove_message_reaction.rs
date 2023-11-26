use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes a reaction from a message. A chosen reaction can always be removed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveMessageReaction {
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
    /// Type of the reaction to remove

    #[serde(skip_serializing_if = "ReactionType::_is_default")]
    reaction_type: ReactionType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveMessageReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveMessageReaction {}

impl RemoveMessageReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RemoveMessageReactionBuilder {
        let mut inner = RemoveMessageReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeMessageReaction".to_string();

        RemoveMessageReactionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn reaction_type(&self) -> &ReactionType {
        &self.reaction_type
    }
}

#[doc(hidden)]
pub struct RemoveMessageReactionBuilder {
    inner: RemoveMessageReaction,
}

#[deprecated]
pub type RTDRemoveMessageReactionBuilder = RemoveMessageReactionBuilder;

impl RemoveMessageReactionBuilder {
    pub fn build(&self) -> RemoveMessageReaction {
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

    pub fn reaction_type<T: AsRef<ReactionType>>(&mut self, reaction_type: T) -> &mut Self {
        self.inner.reaction_type = reaction_type.as_ref().clone();
        self
    }
}

impl AsRef<RemoveMessageReaction> for RemoveMessageReaction {
    fn as_ref(&self) -> &RemoveMessageReaction {
        self
    }
}

impl AsRef<RemoveMessageReaction> for RemoveMessageReactionBuilder {
    fn as_ref(&self) -> &RemoveMessageReaction {
        &self.inner
    }
}
