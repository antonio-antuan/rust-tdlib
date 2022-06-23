use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes chosen reaction for a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetMessageReaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the message belongs
    chat_id: i64,
    /// Identifier of the message
    message_id: i64,
    /// Text representation of the new chosen reaction. Can be an empty string or the currently chosen non-big reaction to remove the reaction
    reaction: String,
    /// Pass true if the reaction is added with a big animation
    is_big: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetMessageReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetMessageReaction {}

impl SetMessageReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetMessageReactionBuilder {
        let mut inner = SetMessageReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setMessageReaction".to_string();

        RTDSetMessageReactionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn reaction(&self) -> &String {
        &self.reaction
    }

    pub fn is_big(&self) -> bool {
        self.is_big
    }
}

#[doc(hidden)]
pub struct RTDSetMessageReactionBuilder {
    inner: SetMessageReaction,
}

impl RTDSetMessageReactionBuilder {
    pub fn build(&self) -> SetMessageReaction {
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

    pub fn reaction<T: AsRef<str>>(&mut self, reaction: T) -> &mut Self {
        self.inner.reaction = reaction.as_ref().to_string();
        self
    }

    pub fn is_big(&mut self, is_big: bool) -> &mut Self {
        self.inner.is_big = is_big;
        self
    }
}

impl AsRef<SetMessageReaction> for SetMessageReaction {
    fn as_ref(&self) -> &SetMessageReaction {
        self
    }
}

impl AsRef<SetMessageReaction> for RTDSetMessageReactionBuilder {
    fn as_ref(&self) -> &SetMessageReaction {
        &self.inner
    }
}
