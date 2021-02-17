use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a chat event
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEvent {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat event identifier

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
    /// Point in time (Unix timestamp) when the event happened
    date: i32,
    /// Identifier of the user who performed the action that triggered the event
    user_id: i32,
    /// Action performed by the user

    #[serde(skip_serializing_if = "ChatEventAction::_is_default")]
    action: ChatEventAction,
}

impl RObject for ChatEvent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatEvent {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventBuilder {
        let mut inner = ChatEvent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn action(&self) -> &ChatEventAction {
        &self.action
    }
}

#[doc(hidden)]
pub struct RTDChatEventBuilder {
    inner: ChatEvent,
}

impl RTDChatEventBuilder {
    pub fn build(&self) -> ChatEvent {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn action<T: AsRef<ChatEventAction>>(&mut self, action: T) -> &mut Self {
        self.inner.action = action.as_ref().clone();
        self
    }
}

impl AsRef<ChatEvent> for ChatEvent {
    fn as_ref(&self) -> &ChatEvent {
        self
    }
}

impl AsRef<ChatEvent> for RTDChatEventBuilder {
    fn as_ref(&self) -> &ChatEvent {
        &self.inner
    }
}
