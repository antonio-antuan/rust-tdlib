use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the notification settings of a forum topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetForumTopicNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message thread identifier of the forum topic

    #[serde(default)]
    message_thread_id: i64,
    /// New notification settings for the forum topic. If the topic is muted for more than 366 days, it is considered to be muted forever
    notification_settings: ChatNotificationSettings,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetForumTopicNotificationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetForumTopicNotificationSettings {}

impl SetForumTopicNotificationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetForumTopicNotificationSettingsBuilder {
        let mut inner = SetForumTopicNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setForumTopicNotificationSettings".to_string();

        SetForumTopicNotificationSettingsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn notification_settings(&self) -> &ChatNotificationSettings {
        &self.notification_settings
    }
}

#[doc(hidden)]
pub struct SetForumTopicNotificationSettingsBuilder {
    inner: SetForumTopicNotificationSettings,
}

#[deprecated]
pub type RTDSetForumTopicNotificationSettingsBuilder = SetForumTopicNotificationSettingsBuilder;

impl SetForumTopicNotificationSettingsBuilder {
    pub fn build(&self) -> SetForumTopicNotificationSettings {
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

    pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(
        &mut self,
        notification_settings: T,
    ) -> &mut Self {
        self.inner.notification_settings = notification_settings.as_ref().clone();
        self
    }
}

impl AsRef<SetForumTopicNotificationSettings> for SetForumTopicNotificationSettings {
    fn as_ref(&self) -> &SetForumTopicNotificationSettings {
        self
    }
}

impl AsRef<SetForumTopicNotificationSettings> for SetForumTopicNotificationSettingsBuilder {
    fn as_ref(&self) -> &SetForumTopicNotificationSettings {
        &self.inner
    }
}
