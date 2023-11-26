use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of notification sounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSounds {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A list of notification sounds

    #[serde(default)]
    notification_sounds: Vec<NotificationSound>,
}

impl RObject for NotificationSounds {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl NotificationSounds {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NotificationSoundsBuilder {
        let mut inner = NotificationSounds::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NotificationSoundsBuilder { inner }
    }

    pub fn notification_sounds(&self) -> &Vec<NotificationSound> {
        &self.notification_sounds
    }
}

#[doc(hidden)]
pub struct NotificationSoundsBuilder {
    inner: NotificationSounds,
}

#[deprecated]
pub type RTDNotificationSoundsBuilder = NotificationSoundsBuilder;

impl NotificationSoundsBuilder {
    pub fn build(&self) -> NotificationSounds {
        self.inner.clone()
    }

    pub fn notification_sounds(
        &mut self,
        notification_sounds: Vec<NotificationSound>,
    ) -> &mut Self {
        self.inner.notification_sounds = notification_sounds;
        self
    }
}

impl AsRef<NotificationSounds> for NotificationSounds {
    fn as_ref(&self) -> &NotificationSounds {
        self
    }
}

impl AsRef<NotificationSounds> for NotificationSoundsBuilder {
    fn as_ref(&self) -> &NotificationSounds {
        &self.inner
    }
}
