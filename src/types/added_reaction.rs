use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a reaction applied to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddedReaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the reaction

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "ReactionType::_is_default")]
    type_: ReactionType,
    /// Identifier of the chat member, applied the reaction

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,
    /// True, if the reaction was added by the current user

    #[serde(default)]
    is_outgoing: bool,
    /// Point in time (Unix timestamp) when the reaction was added

    #[serde(default)]
    date: i32,
}

impl RObject for AddedReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AddedReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddedReactionBuilder {
        let mut inner = AddedReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AddedReactionBuilder { inner }
    }

    pub fn type_(&self) -> &ReactionType {
        &self.type_
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }

    pub fn is_outgoing(&self) -> bool {
        self.is_outgoing
    }

    pub fn date(&self) -> i32 {
        self.date
    }
}

#[doc(hidden)]
pub struct AddedReactionBuilder {
    inner: AddedReaction,
}

#[deprecated]
pub type RTDAddedReactionBuilder = AddedReactionBuilder;

impl AddedReactionBuilder {
    pub fn build(&self) -> AddedReaction {
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

    pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
        self.inner.is_outgoing = is_outgoing;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }
}

impl AsRef<AddedReaction> for AddedReaction {
    fn as_ref(&self) -> &AddedReaction {
        self
    }
}

impl AsRef<AddedReaction> for AddedReactionBuilder {
    fn as_ref(&self) -> &AddedReaction {
        &self.inner
    }
}
