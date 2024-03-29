use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of chat events
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEvents {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of events

    #[serde(default)]
    events: Vec<ChatEvent>,
}

impl RObject for ChatEvents {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatEvents {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatEventsBuilder {
        let mut inner = ChatEvents::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatEventsBuilder { inner }
    }

    pub fn events(&self) -> &Vec<ChatEvent> {
        &self.events
    }
}

#[doc(hidden)]
pub struct ChatEventsBuilder {
    inner: ChatEvents,
}

#[deprecated]
pub type RTDChatEventsBuilder = ChatEventsBuilder;

impl ChatEventsBuilder {
    pub fn build(&self) -> ChatEvents {
        self.inner.clone()
    }

    pub fn events(&mut self, events: Vec<ChatEvent>) -> &mut Self {
        self.inner.events = events;
        self
    }
}

impl AsRef<ChatEvents> for ChatEvents {
    fn as_ref(&self) -> &ChatEvents {
        self
    }
}

impl AsRef<ChatEvents> for ChatEventsBuilder {
    fn as_ref(&self) -> &ChatEvents {
        &self.inner
    }
}
