use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Notification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique persistent identifier of this notification
    id: i32,
    /// Notification date
    date: i32,
    /// True, if the notification was initially silent
    is_silent: bool,
    /// Notification type

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "NotificationType::_is_default")]
    type_: NotificationType,
}

impl RObject for Notification {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Notification {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationBuilder {
        let mut inner = Notification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDNotificationBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn is_silent(&self) -> bool {
        self.is_silent
    }

    pub fn type_(&self) -> &NotificationType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct RTDNotificationBuilder {
    inner: Notification,
}

impl RTDNotificationBuilder {
    pub fn build(&self) -> Notification {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn is_silent(&mut self, is_silent: bool) -> &mut Self {
        self.inner.is_silent = is_silent;
        self
    }

    pub fn type_<T: AsRef<NotificationType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<Notification> for Notification {
    fn as_ref(&self) -> &Notification {
        self
    }
}

impl AsRef<Notification> for RTDNotificationBuilder {
    fn as_ref(&self) -> &Notification {
        &self.inner
    }
}
