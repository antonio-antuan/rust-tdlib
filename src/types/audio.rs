use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes an audio file. Audio is usually in MP3 or M4A format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Audio {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Duration of the audio, in seconds; as defined by the sender

    #[serde(default)]
    duration: i32,
    /// Title of the audio; as defined by the sender

    #[serde(default)]
    title: String,
    /// Performer of the audio; as defined by the sender

    #[serde(default)]
    performer: String,
    /// Original name of the file; as defined by the sender

    #[serde(default)]
    file_name: String,
    /// The MIME type of the file; as defined by the sender

    #[serde(default)]
    mime_type: String,
    /// The minithumbnail of the album cover; may be null
    album_cover_minithumbnail: Option<Minithumbnail>,
    /// The thumbnail of the album cover in JPEG format; as defined by the sender. The full size thumbnail is supposed to be extracted from the downloaded file; may be null
    album_cover_thumbnail: Option<Thumbnail>,
    /// File containing the audio
    audio: File,
}

impl RObject for Audio {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Audio {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AudioBuilder {
        let mut inner = Audio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AudioBuilder { inner }
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn performer(&self) -> &String {
        &self.performer
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }

    pub fn album_cover_minithumbnail(&self) -> &Option<Minithumbnail> {
        &self.album_cover_minithumbnail
    }

    pub fn album_cover_thumbnail(&self) -> &Option<Thumbnail> {
        &self.album_cover_thumbnail
    }

    pub fn audio(&self) -> &File {
        &self.audio
    }
}

#[doc(hidden)]
pub struct AudioBuilder {
    inner: Audio,
}

#[deprecated]
pub type RTDAudioBuilder = AudioBuilder;

impl AudioBuilder {
    pub fn build(&self) -> Audio {
        self.inner.clone()
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn performer<T: AsRef<str>>(&mut self, performer: T) -> &mut Self {
        self.inner.performer = performer.as_ref().to_string();
        self
    }

    pub fn file_name<T: AsRef<str>>(&mut self, file_name: T) -> &mut Self {
        self.inner.file_name = file_name.as_ref().to_string();
        self
    }

    pub fn mime_type<T: AsRef<str>>(&mut self, mime_type: T) -> &mut Self {
        self.inner.mime_type = mime_type.as_ref().to_string();
        self
    }

    pub fn album_cover_minithumbnail<T: AsRef<Minithumbnail>>(
        &mut self,
        album_cover_minithumbnail: T,
    ) -> &mut Self {
        self.inner.album_cover_minithumbnail = Some(album_cover_minithumbnail.as_ref().clone());
        self
    }

    pub fn album_cover_thumbnail<T: AsRef<Thumbnail>>(
        &mut self,
        album_cover_thumbnail: T,
    ) -> &mut Self {
        self.inner.album_cover_thumbnail = Some(album_cover_thumbnail.as_ref().clone());
        self
    }

    pub fn audio<T: AsRef<File>>(&mut self, audio: T) -> &mut Self {
        self.inner.audio = audio.as_ref().clone();
        self
    }
}

impl AsRef<Audio> for Audio {
    fn as_ref(&self) -> &Audio {
        self
    }
}

impl AsRef<Audio> for AudioBuilder {
    fn as_ref(&self) -> &Audio {
        &self.inner
    }
}
