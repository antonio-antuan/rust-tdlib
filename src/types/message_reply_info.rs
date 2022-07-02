use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about replies to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageReplyInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of times the message was directly or indirectly replied

    #[serde(default)]
    reply_count: i32,
    /// Identifiers of at most 3 recent repliers to the message; available in channels with a discussion supergroup. The users and chats are expected to be inaccessible: only their photo and name will be available

    #[serde(default)]
    recent_replier_ids: Vec<MessageSender>,
    /// Identifier of the last read incoming reply to the message

    #[serde(default)]
    last_read_inbox_message_id: i64,
    /// Identifier of the last read outgoing reply to the message

    #[serde(default)]
    last_read_outbox_message_id: i64,
    /// Identifier of the last reply to the message

    #[serde(default)]
    last_message_id: i64,
}

impl RObject for MessageReplyInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageReplyInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageReplyInfoBuilder {
        let mut inner = MessageReplyInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageReplyInfoBuilder { inner }
    }

    pub fn reply_count(&self) -> i32 {
        self.reply_count
    }

    pub fn recent_replier_ids(&self) -> &Vec<MessageSender> {
        &self.recent_replier_ids
    }

    pub fn last_read_inbox_message_id(&self) -> i64 {
        self.last_read_inbox_message_id
    }

    pub fn last_read_outbox_message_id(&self) -> i64 {
        self.last_read_outbox_message_id
    }

    pub fn last_message_id(&self) -> i64 {
        self.last_message_id
    }
}

#[doc(hidden)]
pub struct MessageReplyInfoBuilder {
    inner: MessageReplyInfo,
}

#[deprecated]
pub type RTDMessageReplyInfoBuilder = MessageReplyInfoBuilder;

impl MessageReplyInfoBuilder {
    pub fn build(&self) -> MessageReplyInfo {
        self.inner.clone()
    }

    pub fn reply_count(&mut self, reply_count: i32) -> &mut Self {
        self.inner.reply_count = reply_count;
        self
    }

    pub fn recent_replier_ids(&mut self, recent_replier_ids: Vec<MessageSender>) -> &mut Self {
        self.inner.recent_replier_ids = recent_replier_ids;
        self
    }

    pub fn last_read_inbox_message_id(&mut self, last_read_inbox_message_id: i64) -> &mut Self {
        self.inner.last_read_inbox_message_id = last_read_inbox_message_id;
        self
    }

    pub fn last_read_outbox_message_id(&mut self, last_read_outbox_message_id: i64) -> &mut Self {
        self.inner.last_read_outbox_message_id = last_read_outbox_message_id;
        self
    }

    pub fn last_message_id(&mut self, last_message_id: i64) -> &mut Self {
        self.inner.last_message_id = last_message_id;
        self
    }
}

impl AsRef<MessageReplyInfo> for MessageReplyInfo {
    fn as_ref(&self) -> &MessageReplyInfo {
        self
    }
}

impl AsRef<MessageReplyInfo> for MessageReplyInfoBuilder {
    fn as_ref(&self) -> &MessageReplyInfo {
        &self.inner
    }
}
