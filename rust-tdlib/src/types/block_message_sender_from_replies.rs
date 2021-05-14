use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Blocks an original sender of a message in the Replies chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockMessageSenderFromReplies {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of an incoming message in the Replies chat
    message_id: i64,
    /// Pass true if the message must be deleted
    delete_message: bool,
    /// Pass true if all messages from the same sender must be deleted
    delete_all_messages: bool,
    /// Pass true if the sender must be reported to the Telegram moderators
    report_spam: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for BlockMessageSenderFromReplies {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for BlockMessageSenderFromReplies {}

impl BlockMessageSenderFromReplies {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBlockMessageSenderFromRepliesBuilder {
        let mut inner = BlockMessageSenderFromReplies::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "blockMessageSenderFromReplies".to_string();

        RTDBlockMessageSenderFromRepliesBuilder { inner }
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn delete_message(&self) -> bool {
        self.delete_message
    }

    pub fn delete_all_messages(&self) -> bool {
        self.delete_all_messages
    }

    pub fn report_spam(&self) -> bool {
        self.report_spam
    }
}

#[doc(hidden)]
pub struct RTDBlockMessageSenderFromRepliesBuilder {
    inner: BlockMessageSenderFromReplies,
}

impl RTDBlockMessageSenderFromRepliesBuilder {
    pub fn build(&self) -> BlockMessageSenderFromReplies {
        self.inner.clone()
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn delete_message(&mut self, delete_message: bool) -> &mut Self {
        self.inner.delete_message = delete_message;
        self
    }

    pub fn delete_all_messages(&mut self, delete_all_messages: bool) -> &mut Self {
        self.inner.delete_all_messages = delete_all_messages;
        self
    }

    pub fn report_spam(&mut self, report_spam: bool) -> &mut Self {
        self.inner.report_spam = report_spam;
        self
    }
}

impl AsRef<BlockMessageSenderFromReplies> for BlockMessageSenderFromReplies {
    fn as_ref(&self) -> &BlockMessageSenderFromReplies {
        self
    }
}

impl AsRef<BlockMessageSenderFromReplies> for RTDBlockMessageSenderFromRepliesBuilder {
    fn as_ref(&self) -> &BlockMessageSenderFromReplies {
        &self.inner
    }
}
