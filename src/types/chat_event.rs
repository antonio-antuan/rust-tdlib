use crate::errors::Result;
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

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Point in time (Unix timestamp) when the event happened

    #[serde(default)]
    date: i32,
    /// Identifier of the user or chat who performed the action

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    member_id: MessageSender,
    /// The action

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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatEventBuilder {
        let mut inner = ChatEvent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatEventBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn member_id(&self) -> &MessageSender {
        &self.member_id
    }

    pub fn action(&self) -> &ChatEventAction {
        &self.action
    }
}

#[doc(hidden)]
pub struct ChatEventBuilder {
    inner: ChatEvent,
}

#[deprecated]
pub type RTDChatEventBuilder = ChatEventBuilder;

impl ChatEventBuilder {
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

    pub fn member_id<T: AsRef<MessageSender>>(&mut self, member_id: T) -> &mut Self {
        self.inner.member_id = member_id.as_ref().clone();
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

impl AsRef<ChatEvent> for ChatEventBuilder {
    fn as_ref(&self) -> &ChatEvent {
        &self.inner
    }
}
