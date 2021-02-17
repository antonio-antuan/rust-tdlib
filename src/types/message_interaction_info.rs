use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about interactions with a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageInteractionInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of times the message was viewed
    view_count: i32,
    /// Number of times the message was forwarded
    forward_count: i32,
    /// Contains information about direct or indirect replies to the message; may be null. Currently, available only in channels with a discussion supergroup and discussion supergroups for messages, which are not replies itself
    reply_info: Option<MessageReplyInfo>,
}

impl RObject for MessageInteractionInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageInteractionInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageInteractionInfoBuilder {
        let mut inner = MessageInteractionInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageInteractionInfoBuilder { inner }
    }

    pub fn view_count(&self) -> i32 {
        self.view_count
    }

    pub fn forward_count(&self) -> i32 {
        self.forward_count
    }

    pub fn reply_info(&self) -> &Option<MessageReplyInfo> {
        &self.reply_info
    }
}

#[doc(hidden)]
pub struct RTDMessageInteractionInfoBuilder {
    inner: MessageInteractionInfo,
}

impl RTDMessageInteractionInfoBuilder {
    pub fn build(&self) -> MessageInteractionInfo {
        self.inner.clone()
    }

    pub fn view_count(&mut self, view_count: i32) -> &mut Self {
        self.inner.view_count = view_count;
        self
    }

    pub fn forward_count(&mut self, forward_count: i32) -> &mut Self {
        self.inner.forward_count = forward_count;
        self
    }

    pub fn reply_info<T: AsRef<MessageReplyInfo>>(&mut self, reply_info: T) -> &mut Self {
        self.inner.reply_info = Some(reply_info.as_ref().clone());
        self
    }
}

impl AsRef<MessageInteractionInfo> for MessageInteractionInfo {
    fn as_ref(&self) -> &MessageInteractionInfo {
        self
    }
}

impl AsRef<MessageInteractionInfo> for RTDMessageInteractionInfoBuilder {
    fn as_ref(&self) -> &MessageInteractionInfo {
        &self.inner
    }
}
