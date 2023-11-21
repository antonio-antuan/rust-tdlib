use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a notification sound in MP3 format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSound {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the notification sound

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Duration of the sound, in seconds

    #[serde(default)]
    duration: i32,
    /// Point in time (Unix timestamp) when the sound was created

    #[serde(default)]
    date: i32,
    /// Title of the notification sound

    #[serde(default)]
    title: String,
    /// Arbitrary data, defined while the sound was uploaded

    #[serde(default)]
    data: String,
    /// File containing the sound
    sound: File,
}

impl RObject for NotificationSound {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl NotificationSound {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> NotificationSoundBuilder {
        let mut inner = NotificationSound::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        NotificationSoundBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn sound(&self) -> &File {
        &self.sound
    }
}

#[doc(hidden)]
pub struct NotificationSoundBuilder {
    inner: NotificationSound,
}

#[deprecated]
pub type RTDNotificationSoundBuilder = NotificationSoundBuilder;

impl NotificationSoundBuilder {
    pub fn build(&self) -> NotificationSound {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }

    pub fn sound<T: AsRef<File>>(&mut self, sound: T) -> &mut Self {
        self.inner.sound = sound.as_ref().clone();
        self
    }
}

impl AsRef<NotificationSound> for NotificationSound {
    fn as_ref(&self) -> &NotificationSound {
        self
    }
}

impl AsRef<NotificationSound> for NotificationSoundBuilder {
    fn as_ref(&self) -> &NotificationSound {
        &self.inner
    }
}
