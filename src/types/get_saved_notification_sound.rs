use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns saved notification sound by its identifier. Returns a 404 error if there is no saved notification sound with the specified identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSavedNotificationSound {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the notification sound

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    notification_sound_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSavedNotificationSound {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSavedNotificationSound {}

impl GetSavedNotificationSound {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetSavedNotificationSoundBuilder {
        let mut inner = GetSavedNotificationSound::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSavedNotificationSound".to_string();

        GetSavedNotificationSoundBuilder { inner }
    }

    pub fn notification_sound_id(&self) -> i64 {
        self.notification_sound_id
    }
}

#[doc(hidden)]
pub struct GetSavedNotificationSoundBuilder {
    inner: GetSavedNotificationSound,
}

#[deprecated]
pub type RTDGetSavedNotificationSoundBuilder = GetSavedNotificationSoundBuilder;

impl GetSavedNotificationSoundBuilder {
    pub fn build(&self) -> GetSavedNotificationSound {
        self.inner.clone()
    }

    pub fn notification_sound_id(&mut self, notification_sound_id: i64) -> &mut Self {
        self.inner.notification_sound_id = notification_sound_id;
        self
    }
}

impl AsRef<GetSavedNotificationSound> for GetSavedNotificationSound {
    fn as_ref(&self) -> &GetSavedNotificationSound {
        self
    }
}

impl AsRef<GetSavedNotificationSound> for GetSavedNotificationSoundBuilder {
    fn as_ref(&self) -> &GetSavedNotificationSound {
        &self.inner
    }
}
