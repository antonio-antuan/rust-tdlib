use crate::errors::*;
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
    chat_id: i64,
    /// Message thread identifier, unique within the chat
    message_thread_id: i64,
    /// Contains information about the message thread
    reply_info: MessageReplyInfo,
    /// The messages from which the thread starts. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id)
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageThreadInfoBuilder {
        let mut inner = MessageThreadInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageThreadInfoBuilder { inner }
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

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn draft_message(&self) -> &Option<DraftMessage> {
        &self.draft_message
    }
}

#[doc(hidden)]
pub struct RTDMessageThreadInfoBuilder {
    inner: MessageThreadInfo,
}

impl RTDMessageThreadInfoBuilder {
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

impl AsRef<MessageThreadInfo> for RTDMessageThreadInfoBuilder {
    fn as_ref(&self) -> &MessageThreadInfo {
        &self.inner
    }
}
