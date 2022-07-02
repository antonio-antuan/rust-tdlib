use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveNotification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of notification group to which the notification belongs

    #[serde(default)]
    notification_group_id: i32,
    /// Identifier of removed notification

    #[serde(default)]
    notification_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveNotification {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveNotification {}

impl RemoveNotification {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RemoveNotificationBuilder {
        let mut inner = RemoveNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeNotification".to_string();

        RemoveNotificationBuilder { inner }
    }

    pub fn notification_group_id(&self) -> i32 {
        self.notification_group_id
    }

    pub fn notification_id(&self) -> i32 {
        self.notification_id
    }
}

#[doc(hidden)]
pub struct RemoveNotificationBuilder {
    inner: RemoveNotification,
}

#[deprecated]
pub type RTDRemoveNotificationBuilder = RemoveNotificationBuilder;

impl RemoveNotificationBuilder {
    pub fn build(&self) -> RemoveNotification {
        self.inner.clone()
    }

    pub fn notification_group_id(&mut self, notification_group_id: i32) -> &mut Self {
        self.inner.notification_group_id = notification_group_id;
        self
    }

    pub fn notification_id(&mut self, notification_id: i32) -> &mut Self {
        self.inner.notification_id = notification_id;
        self
    }
}

impl AsRef<RemoveNotification> for RemoveNotification {
    fn as_ref(&self) -> &RemoveNotification {
        self
    }
}

impl AsRef<RemoveNotification> for RemoveNotificationBuilder {
    fn as_ref(&self) -> &RemoveNotification {
        &self.inner
    }
}
