use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of message senders
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSenders {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total count of messages senders found

    #[serde(default)]
    total_count: i32,
    /// List of message senders

    #[serde(default)]
    senders: Vec<MessageSender>,
}

impl RObject for MessageSenders {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageSenders {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageSendersBuilder {
        let mut inner = MessageSenders::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageSendersBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn senders(&self) -> &Vec<MessageSender> {
        &self.senders
    }
}

#[doc(hidden)]
pub struct MessageSendersBuilder {
    inner: MessageSenders,
}

#[deprecated]
pub type RTDMessageSendersBuilder = MessageSendersBuilder;

impl MessageSendersBuilder {
    pub fn build(&self) -> MessageSenders {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn senders(&mut self, senders: Vec<MessageSender>) -> &mut Self {
        self.inner.senders = senders;
        self
    }
}

impl AsRef<MessageSenders> for MessageSenders {
    fn as_ref(&self) -> &MessageSenders {
        self
    }
}

impl AsRef<MessageSenders> for MessageSendersBuilder {
    fn as_ref(&self) -> &MessageSenders {
        &self.inner
    }
}
