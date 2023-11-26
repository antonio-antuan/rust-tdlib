use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a video file sent in a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Duration of the video, in seconds

    #[serde(default)]
    duration: f32,
    /// Video width

    #[serde(default)]
    width: i32,
    /// Video height

    #[serde(default)]
    height: i32,
    /// True, if stickers were added to the video. The list of corresponding sticker sets can be received using getAttachedStickerSets

    #[serde(default)]
    has_stickers: bool,
    /// True, if the video has no sound

    #[serde(default)]
    is_animation: bool,
    /// Video minithumbnail; may be null
    minithumbnail: Option<Minithumbnail>,
    /// Video thumbnail in JPEG or MPEG4 format; may be null
    thumbnail: Option<Thumbnail>,
    /// Size of file prefix, which is supposed to be preloaded, in bytes

    #[serde(default)]
    preload_prefix_size: i32,
    /// File containing the video
    video: File,
}

impl RObject for StoryVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StoryVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryVideoBuilder {
        let mut inner = StoryVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryVideoBuilder { inner }
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn has_stickers(&self) -> bool {
        self.has_stickers
    }

    pub fn is_animation(&self) -> bool {
        self.is_animation
    }

    pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
        &self.minithumbnail
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }

    pub fn preload_prefix_size(&self) -> i32 {
        self.preload_prefix_size
    }

    pub fn video(&self) -> &File {
        &self.video
    }
}

#[doc(hidden)]
pub struct StoryVideoBuilder {
    inner: StoryVideo,
}

#[deprecated]
pub type RTDStoryVideoBuilder = StoryVideoBuilder;

impl StoryVideoBuilder {
    pub fn build(&self) -> StoryVideo {
        self.inner.clone()
    }

    pub fn duration(&mut self, duration: f32) -> &mut Self {
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

    pub fn has_stickers(&mut self, has_stickers: bool) -> &mut Self {
        self.inner.has_stickers = has_stickers;
        self
    }

    pub fn is_animation(&mut self, is_animation: bool) -> &mut Self {
        self.inner.is_animation = is_animation;
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

    pub fn preload_prefix_size(&mut self, preload_prefix_size: i32) -> &mut Self {
        self.inner.preload_prefix_size = preload_prefix_size;
        self
    }

    pub fn video<T: AsRef<File>>(&mut self, video: T) -> &mut Self {
        self.inner.video = video.as_ref().clone();
        self
    }
}

impl AsRef<StoryVideo> for StoryVideo {
    fn as_ref(&self) -> &StoryVideo {
        self
    }
}

impl AsRef<StoryVideo> for StoryVideoBuilder {
    fn as_ref(&self) -> &StoryVideo {
        &self.inner
    }
}
