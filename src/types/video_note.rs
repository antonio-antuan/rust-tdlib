use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a video note. The video must be equal in width and height, cropped to a circle, and stored in MPEG4 format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Duration of the video, in seconds; as defined by the sender
    duration: i32,
    /// Video width and height; as defined by the sender
    length: i32,
    /// Video minithumbnail; may be null
    minithumbnail: Option<Minithumbnail>,
    /// Video thumbnail in JPEG format; as defined by the sender; may be null
    thumbnail: Option<Thumbnail>,
    /// File containing the video
    video: File,
}

impl RObject for VideoNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl VideoNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDVideoNoteBuilder {
        let mut inner = VideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDVideoNoteBuilder { inner }
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
        &self.minithumbnail
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }

    pub fn video(&self) -> &File {
        &self.video
    }
}

#[doc(hidden)]
pub struct RTDVideoNoteBuilder {
    inner: VideoNote,
}

impl RTDVideoNoteBuilder {
    pub fn build(&self) -> VideoNote {
        self.inner.clone()
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
        self
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }

    pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
        self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
        self
    }

    pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = Some(thumbnail.as_ref().clone());
        self
    }

    pub fn video<T: AsRef<File>>(&mut self, video: T) -> &mut Self {
        self.inner.video = video.as_ref().clone();
        self
    }
}

impl AsRef<VideoNote> for VideoNote {
    fn as_ref(&self) -> &VideoNote {
        self
    }
}

impl AsRef<VideoNote> for RTDVideoNoteBuilder {
    fn as_ref(&self) -> &VideoNote {
        &self.inner
    }
}
