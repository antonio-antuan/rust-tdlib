use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the notification settings for chats of a given type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetScopeNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Types of chats for which to return the notification settings information

    #[serde(skip_serializing_if = "NotificationSettingsScope::_is_default")]
    scope: NotificationSettingsScope,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetScopeNotificationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetScopeNotificationSettings {}

impl GetScopeNotificationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetScopeNotificationSettingsBuilder {
        let mut inner = GetScopeNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getScopeNotificationSettings".to_string();

        RTDGetScopeNotificationSettingsBuilder { inner }
    }

    pub fn scope(&self) -> &NotificationSettingsScope {
        &self.scope
    }
}

#[doc(hidden)]
pub struct RTDGetScopeNotificationSettingsBuilder {
    inner: GetScopeNotificationSettings,
}

impl RTDGetScopeNotificationSettingsBuilder {
    pub fn build(&self) -> GetScopeNotificationSettings {
        self.inner.clone()
    }

    pub fn scope<T: AsRef<NotificationSettingsScope>>(&mut self, scope: T) -> &mut Self {
        self.inner.scope = scope.as_ref().clone();
        self
    }
}

impl AsRef<GetScopeNotificationSettings> for GetScopeNotificationSettings {
    fn as_ref(&self) -> &GetScopeNotificationSettings {
        self
    }
}

impl AsRef<GetScopeNotificationSettings> for RTDGetScopeNotificationSettingsBuilder {
    fn as_ref(&self) -> &GetScopeNotificationSettings {
        &self.inner
    }
}
