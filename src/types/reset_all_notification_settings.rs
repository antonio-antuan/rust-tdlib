use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Resets all notification settings to their default values. By default, all chats are unmuted, the sound is set to "default" and message previews are shown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetAllNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResetAllNotificationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResetAllNotificationSettings {}

impl ResetAllNotificationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ResetAllNotificationSettingsBuilder {
        let mut inner = ResetAllNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resetAllNotificationSettings".to_string();

        ResetAllNotificationSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ResetAllNotificationSettingsBuilder {
    inner: ResetAllNotificationSettings,
}

#[deprecated]
pub type RTDResetAllNotificationSettingsBuilder = ResetAllNotificationSettingsBuilder;

impl ResetAllNotificationSettingsBuilder {
    pub fn build(&self) -> ResetAllNotificationSettings {
        self.inner.clone()
    }
}

impl AsRef<ResetAllNotificationSettings> for ResetAllNotificationSettings {
    fn as_ref(&self) -> &ResetAllNotificationSettings {
        self
    }
}

impl AsRef<ResetAllNotificationSettings> for ResetAllNotificationSettingsBuilder {
    fn as_ref(&self) -> &ResetAllNotificationSettings {
        &self.inner
    }
}
