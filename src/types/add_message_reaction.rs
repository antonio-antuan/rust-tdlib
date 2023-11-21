use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds a reaction to a message. Use getMessageAvailableReactions to receive the list of available reactions for the message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddMessageReaction {
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
    /// Type of the reaction to add

    #[serde(skip_serializing_if = "ReactionType::_is_default")]
    reaction_type: ReactionType,
    /// Pass true if the reaction is added with a big animation

    #[serde(default)]
    is_big: bool,
    /// Pass true if the reaction needs to be added to recent reactions

    #[serde(default)]
    update_recent_reactions: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddMessageReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddMessageReaction {}

impl AddMessageReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddMessageReactionBuilder {
        let mut inner = AddMessageReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addMessageReaction".to_string();

        AddMessageReactionBuilder { inner }
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

    pub fn is_big(&self) -> bool {
        self.is_big
    }

    pub fn update_recent_reactions(&self) -> bool {
        self.update_recent_reactions
    }
}

#[doc(hidden)]
pub struct AddMessageReactionBuilder {
    inner: AddMessageReaction,
}

#[deprecated]
pub type RTDAddMessageReactionBuilder = AddMessageReactionBuilder;

impl AddMessageReactionBuilder {
    pub fn build(&self) -> AddMessageReaction {
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

    pub fn is_big(&mut self, is_big: bool) -> &mut Self {
        self.inner.is_big = is_big;
        self
    }

    pub fn update_recent_reactions(&mut self, update_recent_reactions: bool) -> &mut Self {
        self.inner.update_recent_reactions = update_recent_reactions;
        self
    }
}

impl AsRef<AddMessageReaction> for AddMessageReaction {
    fn as_ref(&self) -> &AddMessageReaction {
        self
    }
}

impl AsRef<AddMessageReaction> for AddMessageReactionBuilder {
    fn as_ref(&self) -> &AddMessageReaction {
        &self.inner
    }
}
