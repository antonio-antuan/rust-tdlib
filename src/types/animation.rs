use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes an animation file. The animation must be encoded in GIF or MPEG4 format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Animation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Duration of the animation, in seconds; as defined by the sender

    #[serde(default)]
    duration: i32,
    /// Width of the animation

    #[serde(default)]
    width: i32,
    /// Height of the animation

    #[serde(default)]
    height: i32,
    /// Original name of the file; as defined by the sender

    #[serde(default)]
    file_name: String,
    /// MIME type of the file, usually "image/gif" or "video/mp4"

    #[serde(default)]
    mime_type: String,
    /// True, if stickers were added to the animation. The list of corresponding sticker set can be received using getAttachedStickerSets

    #[serde(default)]
    has_stickers: bool,
    /// Animation minithumbnail; may be null
    minithumbnail: Option<Minithumbnail>,
    /// Animation thumbnail in JPEG or MPEG4 format; may be null
    thumbnail: Option<Thumbnail>,
    /// File containing the animation
    animation: File,
}

impl RObject for Animation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Animation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AnimationBuilder {
        let mut inner = Animation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AnimationBuilder { inner }
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

    pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
        &self.minithumbnail
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }

    pub fn animation(&self) -> &File {
        &self.animation
    }
}

#[doc(hidden)]
pub struct AnimationBuilder {
    inner: Animation,
}

#[deprecated]
pub type RTDAnimationBuilder = AnimationBuilder;

impl AnimationBuilder {
    pub fn build(&self) -> Animation {
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

    pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
        self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
        self
    }

    pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = Some(thumbnail.as_ref().clone());
        self
    }

    pub fn animation<T: AsRef<File>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = animation.as_ref().clone();
        self
    }
}

impl AsRef<Animation> for Animation {
    fn as_ref(&self) -> &Animation {
        self
    }
}

impl AsRef<Animation> for AnimationBuilder {
    fn as_ref(&self) -> &Animation {
        &self.inner
    }
}
