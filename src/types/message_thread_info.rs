use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a message thread
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageThreadInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the message thread belongs

    #[serde(default)]
    chat_id: i64,
    /// Message thread identifier, unique within the chat

    #[serde(default)]
    message_thread_id: i64,
    /// Information about the message thread
    reply_info: MessageReplyInfo,
    /// Approximate number of unread messages in the message thread

    #[serde(default)]
    unread_message_count: i32,
    /// The messages from which the thread starts. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id)

    #[serde(default)]
    messages: Vec<Message>,
    /// A draft of a message in the message thread; may be null
    draft_message: Option<DraftMessage>,
}

impl RObject for MessageThreadInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageThreadInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageThreadInfoBuilder {
        let mut inner = MessageThreadInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageThreadInfoBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn reply_info(&self) -> &MessageReplyInfo {
        &self.reply_info
    }

    pub fn unread_message_count(&self) -> i32 {
        self.unread_message_count
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn draft_message(&self) -> &Option<DraftMessage> {
        &self.draft_message
    }
}

#[doc(hidden)]
pub struct MessageThreadInfoBuilder {
    inner: MessageThreadInfo,
}

#[deprecated]
pub type RTDMessageThreadInfoBuilder = MessageThreadInfoBuilder;

impl MessageThreadInfoBuilder {
    pub fn build(&self) -> MessageThreadInfo {
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

    pub fn reply_info<T: AsRef<MessageReplyInfo>>(&mut self, reply_info: T) -> &mut Self {
        self.inner.reply_info = reply_info.as_ref().clone();
        self
    }

    pub fn unread_message_count(&mut self, unread_message_count: i32) -> &mut Self {
        self.inner.unread_message_count = unread_message_count;
        self
    }

    pub fn messages(&mut self, messages: Vec<Message>) -> &mut Self {
        self.inner.messages = messages;
        self
    }

    pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
        self.inner.draft_message = Some(draft_message.as_ref().clone());
        self
    }
}

impl AsRef<MessageThreadInfo> for MessageThreadInfo {
    fn as_ref(&self) -> &MessageThreadInfo {
        self
    }
}

impl AsRef<MessageThreadInfo> for MessageThreadInfoBuilder {
    fn as_ref(&self) -> &MessageThreadInfo {
        &self.inner
    }
}
