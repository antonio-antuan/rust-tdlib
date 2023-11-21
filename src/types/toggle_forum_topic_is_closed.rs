use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether a topic is closed in a forum supergroup chat; requires can_manage_topics administrator right in the supergroup unless the user is creator of the topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleForumTopicIsClosed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Message thread identifier of the forum topic

    #[serde(default)]
    message_thread_id: i64,
    /// Pass true to close the topic; pass false to reopen it

    #[serde(default)]
    is_closed: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleForumTopicIsClosed {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleForumTopicIsClosed {}

impl ToggleForumTopicIsClosed {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleForumTopicIsClosedBuilder {
        let mut inner = ToggleForumTopicIsClosed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleForumTopicIsClosed".to_string();

        ToggleForumTopicIsClosedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
}

#[doc(hidden)]
pub struct ToggleForumTopicIsClosedBuilder {
    inner: ToggleForumTopicIsClosed,
}

#[deprecated]
pub type RTDToggleForumTopicIsClosedBuilder = ToggleForumTopicIsClosedBuilder;

impl ToggleForumTopicIsClosedBuilder {
    pub fn build(&self) -> ToggleForumTopicIsClosed {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }

    pub fn is_closed(&mut self, is_closed: bool) -> &mut Self {
        self.inner.is_closed = is_closed;
        self
    }
}

impl AsRef<ToggleForumTopicIsClosed> for ToggleForumTopicIsClosed {
    fn as_ref(&self) -> &ToggleForumTopicIsClosed {
        self
    }
}

impl AsRef<ToggleForumTopicIsClosed> for ToggleForumTopicIsClosedBuilder {
    fn as_ref(&self) -> &ToggleForumTopicIsClosed {
        &self.inner
    }
}
