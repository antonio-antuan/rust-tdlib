use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends a notification about a screenshot taken in a chat. Supported only in private and secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendChatScreenshotTakenNotification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendChatScreenshotTakenNotification {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendChatScreenshotTakenNotification {}

impl SendChatScreenshotTakenNotification {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendChatScreenshotTakenNotificationBuilder {
        let mut inner = SendChatScreenshotTakenNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendChatScreenshotTakenNotification".to_string();

        RTDSendChatScreenshotTakenNotificationBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDSendChatScreenshotTakenNotificationBuilder {
    inner: SendChatScreenshotTakenNotification,
}

impl RTDSendChatScreenshotTakenNotificationBuilder {
    pub fn build(&self) -> SendChatScreenshotTakenNotification {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<SendChatScreenshotTakenNotification> for SendChatScreenshotTakenNotification {
    fn as_ref(&self) -> &SendChatScreenshotTakenNotification {
        self
    }
}

impl AsRef<SendChatScreenshotTakenNotification> for RTDSendChatScreenshotTakenNotificationBuilder {
    fn as_ref(&self) -> &SendChatScreenshotTakenNotification {
        &self.inner
    }
}
