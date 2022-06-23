use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an unread reaction to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnreadReaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text representation of the reaction
    reaction: String,
    /// Identifier of the sender, added the reaction

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,
    /// True, if the reaction was added with a big animation
    is_big: bool,
}

impl RObject for UnreadReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl UnreadReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUnreadReactionBuilder {
        let mut inner = UnreadReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUnreadReactionBuilder { inner }
    }

    pub fn reaction(&self) -> &String {
        &self.reaction
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }

    pub fn is_big(&self) -> bool {
        self.is_big
    }
}

#[doc(hidden)]
pub struct RTDUnreadReactionBuilder {
    inner: UnreadReaction,
}

impl RTDUnreadReactionBuilder {
    pub fn build(&self) -> UnreadReaction {
        self.inner.clone()
    }

    pub fn reaction<T: AsRef<str>>(&mut self, reaction: T) -> &mut Self {
        self.inner.reaction = reaction.as_ref().to_string();
        self
    }

    pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
        self.inner.sender_id = sender_id.as_ref().clone();
        self
    }

    pub fn is_big(&mut self, is_big: bool) -> &mut Self {
        self.inner.is_big = is_big;
        self
    }
}

impl AsRef<UnreadReaction> for UnreadReaction {
    fn as_ref(&self) -> &UnreadReaction {
        self
    }
}

impl AsRef<UnreadReaction> for RTDUnreadReactionBuilder {
    fn as_ref(&self) -> &UnreadReaction {
        &self.inner
    }
}
