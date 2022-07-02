use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains statistics about interactions with a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsMessageInteractionInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
    /// Number of times the message was viewed

    #[serde(default)]
    view_count: i32,
    /// Number of times the message was forwarded

    #[serde(default)]
    forward_count: i32,
}

impl RObject for ChatStatisticsMessageInteractionInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatStatisticsMessageInteractionInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatStatisticsMessageInteractionInfoBuilder {
        let mut inner = ChatStatisticsMessageInteractionInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatStatisticsMessageInteractionInfoBuilder { inner }
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn view_count(&self) -> i32 {
        self.view_count
    }

    pub fn forward_count(&self) -> i32 {
        self.forward_count
    }
}

#[doc(hidden)]
pub struct ChatStatisticsMessageInteractionInfoBuilder {
    inner: ChatStatisticsMessageInteractionInfo,
}

#[deprecated]
pub type RTDChatStatisticsMessageInteractionInfoBuilder =
    ChatStatisticsMessageInteractionInfoBuilder;

impl ChatStatisticsMessageInteractionInfoBuilder {
    pub fn build(&self) -> ChatStatisticsMessageInteractionInfo {
        self.inner.clone()
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn view_count(&mut self, view_count: i32) -> &mut Self {
        self.inner.view_count = view_count;
        self
    }

    pub fn forward_count(&mut self, forward_count: i32) -> &mut Self {
        self.inner.forward_count = forward_count;
        self
    }
}

impl AsRef<ChatStatisticsMessageInteractionInfo> for ChatStatisticsMessageInteractionInfo {
    fn as_ref(&self) -> &ChatStatisticsMessageInteractionInfo {
        self
    }
}

impl AsRef<ChatStatisticsMessageInteractionInfo> for ChatStatisticsMessageInteractionInfoBuilder {
    fn as_ref(&self) -> &ChatStatisticsMessageInteractionInfo {
        &self.inner
    }
}
