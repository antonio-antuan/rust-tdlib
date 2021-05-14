use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes notification settings for chats of a given type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetScopeNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Types of chats for which to change the notification settings

    #[serde(skip_serializing_if = "NotificationSettingsScope::_is_default")]
    scope: NotificationSettingsScope,
    /// The new notification settings for the given scope
    notification_settings: ScopeNotificationSettings,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetScopeNotificationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetScopeNotificationSettings {}

impl SetScopeNotificationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetScopeNotificationSettingsBuilder {
        let mut inner = SetScopeNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setScopeNotificationSettings".to_string();

        RTDSetScopeNotificationSettingsBuilder { inner }
    }

    pub fn scope(&self) -> &NotificationSettingsScope {
        &self.scope
    }

    pub fn notification_settings(&self) -> &ScopeNotificationSettings {
        &self.notification_settings
    }
}

#[doc(hidden)]
pub struct RTDSetScopeNotificationSettingsBuilder {
    inner: SetScopeNotificationSettings,
}

impl RTDSetScopeNotificationSettingsBuilder {
    pub fn build(&self) -> SetScopeNotificationSettings {
        self.inner.clone()
    }

    pub fn scope<T: AsRef<NotificationSettingsScope>>(&mut self, scope: T) -> &mut Self {
        self.inner.scope = scope.as_ref().clone();
        self
    }

    pub fn notification_settings<T: AsRef<ScopeNotificationSettings>>(
        &mut self,
        notification_settings: T,
    ) -> &mut Self {
        self.inner.notification_settings = notification_settings.as_ref().clone();
        self
    }
}

impl AsRef<SetScopeNotificationSettings> for SetScopeNotificationSettings {
    fn as_ref(&self) -> &SetScopeNotificationSettings {
        self
    }
}

impl AsRef<SetScopeNotificationSettings> for RTDSetScopeNotificationSettingsBuilder {
    fn as_ref(&self) -> &SetScopeNotificationSettings {
        &self.inner
    }
}
