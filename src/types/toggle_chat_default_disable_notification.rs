use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the value of the default disable_notification parameter, used when a message is sent to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleChatDefaultDisableNotification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// New value of default_disable_notification
    default_disable_notification: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleChatDefaultDisableNotification {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleChatDefaultDisableNotification {}

impl ToggleChatDefaultDisableNotification {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleChatDefaultDisableNotificationBuilder {
        let mut inner = ToggleChatDefaultDisableNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleChatDefaultDisableNotification".to_string();

        RTDToggleChatDefaultDisableNotificationBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn default_disable_notification(&self) -> bool {
        self.default_disable_notification
    }
}

#[doc(hidden)]
pub struct RTDToggleChatDefaultDisableNotificationBuilder {
    inner: ToggleChatDefaultDisableNotification,
}

impl RTDToggleChatDefaultDisableNotificationBuilder {
    pub fn build(&self) -> ToggleChatDefaultDisableNotification {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn default_disable_notification(
        &mut self,
        default_disable_notification: bool,
    ) -> &mut Self {
        self.inner.default_disable_notification = default_disable_notification;
        self
    }
}

impl AsRef<ToggleChatDefaultDisableNotification> for ToggleChatDefaultDisableNotification {
    fn as_ref(&self) -> &ToggleChatDefaultDisableNotification {
        self
    }
}

impl AsRef<ToggleChatDefaultDisableNotification>
    for RTDToggleChatDefaultDisableNotificationBuilder
{
    fn as_ref(&self) -> &ToggleChatDefaultDisableNotification {
        &self.inner
    }
}
