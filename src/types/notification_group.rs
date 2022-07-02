use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a group of notifications
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique persistent auto-incremented from 1 identifier of the notification group

    #[serde(default)]
    id: i32,
    /// Type of the group

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "NotificationGroupType::_is_default")]
    type_: NotificationGroupType,
    /// Identifier of a chat to which all notifications in the group belong

    #[serde(default)]
    chat_id: i64,
    /// Total number of active notifications in the group

    #[serde(default)]
    total_count: i32,
    /// The list of active notifications

    #[serde(default)]
    notifications: Vec<Notification>,
}

impl RObject for NotificationGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl NotificationGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NotificationGroupBuilder {
        let mut inner = NotificationGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NotificationGroupBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn type_(&self) -> &NotificationGroupType {
        &self.type_
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn notifications(&self) -> &Vec<Notification> {
        &self.notifications
    }
}

#[doc(hidden)]
pub struct NotificationGroupBuilder {
    inner: NotificationGroup,
}

#[deprecated]
pub type RTDNotificationGroupBuilder = NotificationGroupBuilder;

impl NotificationGroupBuilder {
    pub fn build(&self) -> NotificationGroup {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn type_<T: AsRef<NotificationGroupType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn notifications(&mut self, notifications: Vec<Notification>) -> &mut Self {
        self.inner.notifications = notifications;
        self
    }
}

impl AsRef<NotificationGroup> for NotificationGroup {
    fn as_ref(&self) -> &NotificationGroup {
        self
    }
}

impl AsRef<NotificationGroup> for NotificationGroupBuilder {
    fn as_ref(&self) -> &NotificationGroup {
        &self.inner
    }
}
