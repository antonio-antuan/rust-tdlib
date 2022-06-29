use crate::errors::*;
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
    /// Text representation of the reaction

    #[serde(default)]
    reaction: String,
    /// Identifier of the chat member, applied the reaction

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAddedReactionBuilder {
        let mut inner = AddedReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDAddedReactionBuilder { inner }
    }

    pub fn reaction(&self) -> &String {
        &self.reaction
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }
}

#[doc(hidden)]
pub struct RTDAddedReactionBuilder {
    inner: AddedReaction,
}

impl RTDAddedReactionBuilder {
    pub fn build(&self) -> AddedReaction {
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
}

impl AsRef<AddedReaction> for AddedReaction {
    fn as_ref(&self) -> &AddedReaction {
        self
    }
}

impl AsRef<AddedReaction> for RTDAddedReactionBuilder {
    fn as_ref(&self) -> &AddedReaction {
        &self.inner
    }
}
