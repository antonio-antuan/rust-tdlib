use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the notification settings of a chat. Notification settings of a chat with the current user (Saved Messages) can't be changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// New notification settings for the chat. If the chat is muted for more than 1 week, it is considered to be muted forever
    notification_settings: ChatNotificationSettings,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatNotificationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatNotificationSettings {}

impl SetChatNotificationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatNotificationSettingsBuilder {
        let mut inner = SetChatNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatNotificationSettings".to_string();

        RTDSetChatNotificationSettingsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn notification_settings(&self) -> &ChatNotificationSettings {
        &self.notification_settings
    }
}

#[doc(hidden)]
pub struct RTDSetChatNotificationSettingsBuilder {
    inner: SetChatNotificationSettings,
}

impl RTDSetChatNotificationSettingsBuilder {
    pub fn build(&self) -> SetChatNotificationSettings {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(
        &mut self,
        notification_settings: T,
    ) -> &mut Self {
        self.inner.notification_settings = notification_settings.as_ref().clone();
        self
    }
}

impl AsRef<SetChatNotificationSettings> for SetChatNotificationSettings {
    fn as_ref(&self) -> &SetChatNotificationSettings {
        self
    }
}

impl AsRef<SetChatNotificationSettings> for RTDSetChatNotificationSettingsBuilder {
    fn as_ref(&self) -> &SetChatNotificationSettings {
        &self.inner
    }
}
