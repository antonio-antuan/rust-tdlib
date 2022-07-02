use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForwardMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which to forward messages

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the chat from which to forward messages

    #[serde(default)]
    from_chat_id: i64,
    /// Identifiers of the messages to forward. Message identifiers must be in a strictly increasing order. At most 100 messages can be forwarded simultaneously

    #[serde(default)]
    message_ids: Vec<i64>,
    /// Options to be used to send the messages; pass null to use default options
    options: MessageSendOptions,
    /// If true, content of the messages will be copied without reference to the original sender. Always true if the messages are forwarded to a secret chat or are local

    #[serde(default)]
    send_copy: bool,
    /// If true, media caption of message copies will be removed. Ignored if send_copy is false

    #[serde(default)]
    remove_caption: bool,
    /// If true, messages will not be forwarded and instead fake messages will be returned

    #[serde(default)]
    only_preview: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ForwardMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ForwardMessages {}

impl ForwardMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ForwardMessagesBuilder {
        let mut inner = ForwardMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "forwardMessages".to_string();

        ForwardMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn from_chat_id(&self) -> i64 {
        self.from_chat_id
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }

    pub fn options(&self) -> &MessageSendOptions {
        &self.options
    }

    pub fn send_copy(&self) -> bool {
        self.send_copy
    }

    pub fn remove_caption(&self) -> bool {
        self.remove_caption
    }

    pub fn only_preview(&self) -> bool {
        self.only_preview
    }
}

#[doc(hidden)]
pub struct ForwardMessagesBuilder {
    inner: ForwardMessages,
}

#[deprecated]
pub type RTDForwardMessagesBuilder = ForwardMessagesBuilder;

impl ForwardMessagesBuilder {
    pub fn build(&self) -> ForwardMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn from_chat_id(&mut self, from_chat_id: i64) -> &mut Self {
        self.inner.from_chat_id = from_chat_id;
        self
    }

    pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
        self.inner.message_ids = message_ids;
        self
    }

    pub fn options<T: AsRef<MessageSendOptions>>(&mut self, options: T) -> &mut Self {
        self.inner.options = options.as_ref().clone();
        self
    }

    pub fn send_copy(&mut self, send_copy: bool) -> &mut Self {
        self.inner.send_copy = send_copy;
        self
    }

    pub fn remove_caption(&mut self, remove_caption: bool) -> &mut Self {
        self.inner.remove_caption = remove_caption;
        self
    }

    pub fn only_preview(&mut self, only_preview: bool) -> &mut Self {
        self.inner.only_preview = only_preview;
        self
    }
}

impl AsRef<ForwardMessages> for ForwardMessages {
    fn as_ref(&self) -> &ForwardMessages {
        self
    }
}

impl AsRef<ForwardMessages> for ForwardMessagesBuilder {
    fn as_ref(&self) -> &ForwardMessages {
        &self.inner
    }
}
