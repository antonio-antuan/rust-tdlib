use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes a notification sound from the list of saved notification sounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveSavedNotificationSound {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the notification sound

    #[serde(deserialize_with = "super::_common::number_from_string")]
    notification_sound_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveSavedNotificationSound {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveSavedNotificationSound {}

impl RemoveSavedNotificationSound {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveSavedNotificationSoundBuilder {
        let mut inner = RemoveSavedNotificationSound::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeSavedNotificationSound".to_string();

        RTDRemoveSavedNotificationSoundBuilder { inner }
    }

    pub fn notification_sound_id(&self) -> i64 {
        self.notification_sound_id
    }
}

#[doc(hidden)]
pub struct RTDRemoveSavedNotificationSoundBuilder {
    inner: RemoveSavedNotificationSound,
}

impl RTDRemoveSavedNotificationSoundBuilder {
    pub fn build(&self) -> RemoveSavedNotificationSound {
        self.inner.clone()
    }

    pub fn notification_sound_id(&mut self, notification_sound_id: i64) -> &mut Self {
        self.inner.notification_sound_id = notification_sound_id;
        self
    }
}

impl AsRef<RemoveSavedNotificationSound> for RemoveSavedNotificationSound {
    fn as_ref(&self) -> &RemoveSavedNotificationSound {
        self
    }
}

impl AsRef<RemoveSavedNotificationSound> for RTDRemoveSavedNotificationSoundBuilder {
    fn as_ref(&self) -> &RemoveSavedNotificationSound {
        &self.inner
    }
}
