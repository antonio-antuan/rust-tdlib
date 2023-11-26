use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a reaction to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the reaction

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "ReactionType::_is_default")]
    type_: ReactionType,
    /// Number of times the reaction was added

    #[serde(default)]
    total_count: i32,
    /// True, if the reaction is chosen by the current user

    #[serde(default)]
    is_chosen: bool,
    /// Identifier of the message sender used by the current user to add the reaction; may be null if unknown or the reaction isn't chosen
    used_sender_id: Option<MessageSender>,
    /// Identifiers of at most 3 recent message senders, added the reaction; available in private, basic group and supergroup chats

    #[serde(default)]
    recent_sender_ids: Vec<MessageSender>,
}

impl RObject for MessageReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageReactionBuilder {
        let mut inner = MessageReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageReactionBuilder { inner }
    }

    pub fn type_(&self) -> &ReactionType {
        &self.type_
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn is_chosen(&self) -> bool {
        self.is_chosen
    }

    pub fn used_sender_id(&self) -> &Option<MessageSender> {
        &self.used_sender_id
    }

    pub fn recent_sender_ids(&self) -> &Vec<MessageSender> {
        &self.recent_sender_ids
    }
}

#[doc(hidden)]
pub struct MessageReactionBuilder {
    inner: MessageReaction,
}

#[deprecated]
pub type RTDMessageReactionBuilder = MessageReactionBuilder;

impl MessageReactionBuilder {
    pub fn build(&self) -> MessageReaction {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<ReactionType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn is_chosen(&mut self, is_chosen: bool) -> &mut Self {
        self.inner.is_chosen = is_chosen;
        self
    }

    pub fn used_sender_id<T: AsRef<MessageSender>>(&mut self, used_sender_id: T) -> &mut Self {
        self.inner.used_sender_id = Some(used_sender_id.as_ref().clone());
        self
    }

    pub fn recent_sender_ids(&mut self, recent_sender_ids: Vec<MessageSender>) -> &mut Self {
        self.inner.recent_sender_ids = recent_sender_ids;
        self
    }
}

impl AsRef<MessageReaction> for MessageReaction {
    fn as_ref(&self) -> &MessageReaction {
        self
    }
}

impl AsRef<MessageReaction> for MessageReactionBuilder {
    fn as_ref(&self) -> &MessageReaction {
        &self.inner
    }
}
