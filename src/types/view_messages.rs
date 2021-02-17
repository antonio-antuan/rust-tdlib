use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that messages are being viewed by the user. Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// If not 0, a message thread identifier in which the messages are being viewed
    message_thread_id: i64,
    /// The identifiers of the messages being viewed
    message_ids: Vec<i64>,
    /// True, if messages in closed chats should be marked as read by the request
    force_read: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ViewMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ViewMessages {}

impl ViewMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDViewMessagesBuilder {
        let mut inner = ViewMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "viewMessages".to_string();

        RTDViewMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }

    pub fn force_read(&self) -> bool {
        self.force_read
    }
}

#[doc(hidden)]
pub struct RTDViewMessagesBuilder {
    inner: ViewMessages,
}

impl RTDViewMessagesBuilder {
    pub fn build(&self) -> ViewMessages {
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

    pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
        self.inner.message_ids = message_ids;
        self
    }

    pub fn force_read(&mut self, force_read: bool) -> &mut Self {
        self.inner.force_read = force_read;
        self
    }
}

impl AsRef<ViewMessages> for ViewMessages {
    fn as_ref(&self) -> &ViewMessages {
        self
    }
}

impl AsRef<ViewMessages> for RTDViewMessagesBuilder {
    fn as_ref(&self) -> &ViewMessages {
        &self.inner
    }
}
