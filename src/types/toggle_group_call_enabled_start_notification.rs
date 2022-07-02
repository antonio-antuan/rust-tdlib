use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether the current user will receive a notification when the group call will start; scheduled group calls only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleGroupCallEnabledStartNotification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// New value of the enabled_start_notification setting

    #[serde(default)]
    enabled_start_notification: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleGroupCallEnabledStartNotification {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleGroupCallEnabledStartNotification {}

impl ToggleGroupCallEnabledStartNotification {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleGroupCallEnabledStartNotificationBuilder {
        let mut inner = ToggleGroupCallEnabledStartNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleGroupCallEnabledStartNotification".to_string();

        ToggleGroupCallEnabledStartNotificationBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn enabled_start_notification(&self) -> bool {
        self.enabled_start_notification
    }
}

#[doc(hidden)]
pub struct ToggleGroupCallEnabledStartNotificationBuilder {
    inner: ToggleGroupCallEnabledStartNotification,
}

#[deprecated]
pub type RTDToggleGroupCallEnabledStartNotificationBuilder =
    ToggleGroupCallEnabledStartNotificationBuilder;

impl ToggleGroupCallEnabledStartNotificationBuilder {
    pub fn build(&self) -> ToggleGroupCallEnabledStartNotification {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn enabled_start_notification(&mut self, enabled_start_notification: bool) -> &mut Self {
        self.inner.enabled_start_notification = enabled_start_notification;
        self
    }
}

impl AsRef<ToggleGroupCallEnabledStartNotification> for ToggleGroupCallEnabledStartNotification {
    fn as_ref(&self) -> &ToggleGroupCallEnabledStartNotification {
        self
    }
}

impl AsRef<ToggleGroupCallEnabledStartNotification>
    for ToggleGroupCallEnabledStartNotificationBuilder
{
    fn as_ref(&self) -> &ToggleGroupCallEnabledStartNotification {
        &self.inner
    }
}
