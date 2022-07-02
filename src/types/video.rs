use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a video file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Video {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Duration of the video, in seconds; as defined by the sender

    #[serde(default)]
    duration: i32,
    /// Video width; as defined by the sender

    #[serde(default)]
    width: i32,
    /// Video height; as defined by the sender

    #[serde(default)]
    height: i32,
    /// Original name of the file; as defined by the sender

    #[serde(default)]
    file_name: String,
    /// MIME type of the file; as defined by the sender

    #[serde(default)]
    mime_type: String,
    /// True, if stickers were added to the video. The list of corresponding sticker sets can be received using getAttachedStickerSets

    #[serde(default)]
    has_stickers: bool,
    /// True, if the video is supposed to be streamed

    #[serde(default)]
    supports_streaming: bool,
    /// Video minithumbnail; may be null
    minithumbnail: Option<Minithumbnail>,
    /// Video thumbnail in JPEG or MPEG4 format; as defined by the sender; may be null
    thumbnail: Option<Thumbnail>,
    /// File containing the video
    video: File,
}

impl RObject for Video {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Video {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> VideoBuilder {
        let mut inner = Video::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        VideoBuilder { inner }
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }

    pub fn has_stickers(&self) -> bool {
        self.has_stickers
    }

    pub fn supports_streaming(&self) -> bool {
        self.supports_streaming
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
pub struct VideoBuilder {
    inner: Video,
}

#[deprecated]
pub type RTDVideoBuilder = VideoBuilder;

impl VideoBuilder {
    pub fn build(&self) -> Video {
        self.inner.clone()
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
        self
    }

    pub fn width(&mut self, width: i32) -> &mut Self {
        self.inner.width = width;
        self
    }

    pub fn height(&mut self, height: i32) -> &mut Self {
        self.inner.height = height;
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

    pub fn has_stickers(&mut self, has_stickers: bool) -> &mut Self {
        self.inner.has_stickers = has_stickers;
        self
    }

    pub fn supports_streaming(&mut self, supports_streaming: bool) -> &mut Self {
        self.inner.supports_streaming = supports_streaming;
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

impl AsRef<Video> for Video {
    fn as_ref(&self) -> &Video {
        self
    }
}

impl AsRef<Video> for VideoBuilder {
    fn as_ref(&self) -> &Video {
        &self.inner
    }
}
