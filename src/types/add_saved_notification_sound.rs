use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds a new notification sound to the list of saved notification sounds. The new notification sound is added to the top of the list. If it is already in the list, its position isn't changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddSavedNotificationSound {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Notification sound file to add

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sound: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddSavedNotificationSound {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddSavedNotificationSound {}

impl AddSavedNotificationSound {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddSavedNotificationSoundBuilder {
        let mut inner = AddSavedNotificationSound::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addSavedNotificationSound".to_string();

        AddSavedNotificationSoundBuilder { inner }
    }

    pub fn sound(&self) -> &InputFile {
        &self.sound
    }
}

#[doc(hidden)]
pub struct AddSavedNotificationSoundBuilder {
    inner: AddSavedNotificationSound,
}

#[deprecated]
pub type RTDAddSavedNotificationSoundBuilder = AddSavedNotificationSoundBuilder;

impl AddSavedNotificationSoundBuilder {
    pub fn build(&self) -> AddSavedNotificationSound {
        self.inner.clone()
    }

    pub fn sound<T: AsRef<InputFile>>(&mut self, sound: T) -> &mut Self {
        self.inner.sound = sound.as_ref().clone();
        self
    }
}

impl AsRef<AddSavedNotificationSound> for AddSavedNotificationSound {
    fn as_ref(&self) -> &AddSavedNotificationSound {
        self
    }
}

impl AsRef<AddSavedNotificationSound> for AddSavedNotificationSoundBuilder {
    fn as_ref(&self) -> &AddSavedNotificationSound {
        &self.inner
    }
}
