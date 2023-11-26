use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that messages are being viewed by the user. Sponsored messages must be marked as viewed only when the entire text of the message is shown on the screen (excluding the button). Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The identifiers of the messages being viewed

    #[serde(default)]
    message_ids: Vec<i64>,
    /// Source of the message view; pass null to guess the source based on chat open state

    #[serde(skip_serializing_if = "MessageSource::_is_default")]
    source: MessageSource,
    /// Pass true to mark as read the specified messages even the chat is closed

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ViewMessagesBuilder {
        let mut inner = ViewMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "viewMessages".to_string();

        ViewMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }

    pub fn source(&self) -> &MessageSource {
        &self.source
    }

    pub fn force_read(&self) -> bool {
        self.force_read
    }
}

#[doc(hidden)]
pub struct ViewMessagesBuilder {
    inner: ViewMessages,
}

#[deprecated]
pub type RTDViewMessagesBuilder = ViewMessagesBuilder;

impl ViewMessagesBuilder {
    pub fn build(&self) -> ViewMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
        self.inner.message_ids = message_ids;
        self
    }

    pub fn source<T: AsRef<MessageSource>>(&mut self, source: T) -> &mut Self {
        self.inner.source = source.as_ref().clone();
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

impl AsRef<ViewMessages> for ViewMessagesBuilder {
    fn as_ref(&self) -> &ViewMessages {
        &self.inner
    }
}
