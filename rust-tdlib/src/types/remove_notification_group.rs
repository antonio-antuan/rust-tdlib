use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveNotificationGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Notification group identifier
    notification_group_id: i32,
    /// The maximum identifier of removed notifications
    max_notification_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveNotificationGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveNotificationGroup {}

impl RemoveNotificationGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveNotificationGroupBuilder {
        let mut inner = RemoveNotificationGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeNotificationGroup".to_string();

        RTDRemoveNotificationGroupBuilder { inner }
    }

    pub fn notification_group_id(&self) -> i32 {
        self.notification_group_id
    }

    pub fn max_notification_id(&self) -> i32 {
        self.max_notification_id
    }
}

#[doc(hidden)]
pub struct RTDRemoveNotificationGroupBuilder {
    inner: RemoveNotificationGroup,
}

impl RTDRemoveNotificationGroupBuilder {
    pub fn build(&self) -> RemoveNotificationGroup {
        self.inner.clone()
    }

    pub fn notification_group_id(&mut self, notification_group_id: i32) -> &mut Self {
        self.inner.notification_group_id = notification_group_id;
        self
    }

    pub fn max_notification_id(&mut self, max_notification_id: i32) -> &mut Self {
        self.inner.max_notification_id = max_notification_id;
        self
    }
}

impl AsRef<RemoveNotificationGroup> for RemoveNotificationGroup {
    fn as_ref(&self) -> &RemoveNotificationGroup {
        self
    }
}

impl AsRef<RemoveNotificationGroup> for RTDRemoveNotificationGroupBuilder {
    fn as_ref(&self) -> &RemoveNotificationGroup {
        &self.inner
    }
}
