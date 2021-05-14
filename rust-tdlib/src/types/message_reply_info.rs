use crate::errors::*;
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
    reply_count: i32,
    /// Recent repliers to the message; available in channels with a discussion supergroup
    recent_repliers: Vec<MessageSender>,
    /// Identifier of the last read incoming reply to the message
    last_read_inbox_message_id: i64,
    /// Identifier of the last read outgoing reply to the message
    last_read_outbox_message_id: i64,
    /// Identifier of the last reply to the message
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageReplyInfoBuilder {
        let mut inner = MessageReplyInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageReplyInfoBuilder { inner }
    }

    pub fn reply_count(&self) -> i32 {
        self.reply_count
    }

    pub fn recent_repliers(&self) -> &Vec<MessageSender> {
        &self.recent_repliers
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
pub struct RTDMessageReplyInfoBuilder {
    inner: MessageReplyInfo,
}

impl RTDMessageReplyInfoBuilder {
    pub fn build(&self) -> MessageReplyInfo {
        self.inner.clone()
    }

    pub fn reply_count(&mut self, reply_count: i32) -> &mut Self {
        self.inner.reply_count = reply_count;
        self
    }

    pub fn recent_repliers(&mut self, recent_repliers: Vec<MessageSender>) -> &mut Self {
        self.inner.recent_repliers = recent_repliers;
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

impl AsRef<MessageReplyInfo> for RTDMessageReplyInfoBuilder {
    fn as_ref(&self) -> &MessageReplyInfo {
        &self.inner
    }
}
