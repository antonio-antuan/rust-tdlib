use crate::errors::*;
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
    chat_id: i64,
    /// Identifier of the chat from which to forward messages
    from_chat_id: i64,
    /// Identifiers of the messages to forward. Message identifiers must be in a strictly increasing order
    message_ids: Vec<i64>,
    /// Options to be used to send the messages
    options: MessageSendOptions,
    /// True, if content of the messages needs to be copied without links to the original messages. Always true if the messages are forwarded to a secret chat
    send_copy: bool,
    /// True, if media caption of message copies needs to be removed. Ignored if send_copy is false
    remove_caption: bool,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDForwardMessagesBuilder {
        let mut inner = ForwardMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "forwardMessages".to_string();

        RTDForwardMessagesBuilder { inner }
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
}

#[doc(hidden)]
pub struct RTDForwardMessagesBuilder {
    inner: ForwardMessages,
}

impl RTDForwardMessagesBuilder {
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
}

impl AsRef<ForwardMessages> for ForwardMessages {
    fn as_ref(&self) -> &ForwardMessages {
        self
    }
}

impl AsRef<ForwardMessages> for RTDForwardMessagesBuilder {
    fn as_ref(&self) -> &ForwardMessages {
        &self.inner
    }
}
