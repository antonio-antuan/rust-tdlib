use crate::errors::*;
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
    /// Text representation of the reaction
    reaction: String,
    /// Number of times the reaction was added
    total_count: i32,
    /// True, if the reaction is chosen by the current user
    is_chosen: bool,
    /// Identifiers of at most 3 recent message senders, added the reaction; available in private, basic group and supergroup chats
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageReactionBuilder {
        let mut inner = MessageReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageReactionBuilder { inner }
    }

    pub fn reaction(&self) -> &String {
        &self.reaction
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn is_chosen(&self) -> bool {
        self.is_chosen
    }

    pub fn recent_sender_ids(&self) -> &Vec<MessageSender> {
        &self.recent_sender_ids
    }
}

#[doc(hidden)]
pub struct RTDMessageReactionBuilder {
    inner: MessageReaction,
}

impl RTDMessageReactionBuilder {
    pub fn build(&self) -> MessageReaction {
        self.inner.clone()
    }

    pub fn reaction<T: AsRef<str>>(&mut self, reaction: T) -> &mut Self {
        self.inner.reaction = reaction.as_ref().to_string();
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

impl AsRef<MessageReaction> for RTDMessageReactionBuilder {
    fn as_ref(&self) -> &MessageReaction {
        &self.inner
    }
}
