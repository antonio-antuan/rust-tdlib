use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns list of chats with non-default notification settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatNotificationSettingsExceptions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If specified, only chats from the specified scope will be returned

    #[serde(skip_serializing_if = "NotificationSettingsScope::_is_default")]
    scope: NotificationSettingsScope,
    /// If true, also chats with non-default sound will be returned
    compare_sound: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatNotificationSettingsExceptions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatNotificationSettingsExceptions {}

impl GetChatNotificationSettingsExceptions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetChatNotificationSettingsExceptionsBuilder {
        let mut inner = GetChatNotificationSettingsExceptions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatNotificationSettingsExceptions".to_string();

        RTDGetChatNotificationSettingsExceptionsBuilder { inner }
    }

    pub fn scope(&self) -> &NotificationSettingsScope {
        &self.scope
    }

    pub fn compare_sound(&self) -> bool {
        self.compare_sound
    }
}

#[doc(hidden)]
pub struct RTDGetChatNotificationSettingsExceptionsBuilder {
    inner: GetChatNotificationSettingsExceptions,
}

impl RTDGetChatNotificationSettingsExceptionsBuilder {
    pub fn build(&self) -> GetChatNotificationSettingsExceptions {
        self.inner.clone()
    }

    pub fn scope<T: AsRef<NotificationSettingsScope>>(&mut self, scope: T) -> &mut Self {
        self.inner.scope = scope.as_ref().clone();
        self
    }

    pub fn compare_sound(&mut self, compare_sound: bool) -> &mut Self {
        self.inner.compare_sound = compare_sound;
        self
    }
}

impl AsRef<GetChatNotificationSettingsExceptions> for GetChatNotificationSettingsExceptions {
    fn as_ref(&self) -> &GetChatNotificationSettingsExceptions {
        self
    }
}

impl AsRef<GetChatNotificationSettingsExceptions>
    for RTDGetChatNotificationSettingsExceptionsBuilder
{
    fn as_ref(&self) -> &GetChatNotificationSettingsExceptions {
        &self.inner
    }
}
