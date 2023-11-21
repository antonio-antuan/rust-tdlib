use crate::errors::Result;
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
    /// Type of the reaction

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "ReactionType::_is_default")]
    type_: ReactionType,
    /// Identifier of the sender, added the reaction

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,
    /// True, if the reaction was added with a big animation

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UnreadReactionBuilder {
        let mut inner = UnreadReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UnreadReactionBuilder { inner }
    }

    pub fn type_(&self) -> &ReactionType {
        &self.type_
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }

    pub fn is_big(&self) -> bool {
        self.is_big
    }
}

#[doc(hidden)]
pub struct UnreadReactionBuilder {
    inner: UnreadReaction,
}

#[deprecated]
pub type RTDUnreadReactionBuilder = UnreadReactionBuilder;

impl UnreadReactionBuilder {
    pub fn build(&self) -> UnreadReaction {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<ReactionType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
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

impl AsRef<UnreadReaction> for UnreadReactionBuilder {
    fn as_ref(&self) -> &UnreadReaction {
        &self.inner
    }
}
