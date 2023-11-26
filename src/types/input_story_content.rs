use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// The content of a story to send
pub trait TDInputStoryContent: Debug + RObject {}

/// The content of a story to send
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum InputStoryContent {
    #[doc(hidden)]
    #[default]
    _Default,
    /// A photo story
    #[serde(rename = "inputStoryContentPhoto")]
    Photo(InputStoryContentPhoto),
    /// A video story
    #[serde(rename = "inputStoryContentVideo")]
    Video(InputStoryContentVideo),
}

impl RObject for InputStoryContent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputStoryContent::Photo(t) => t.extra(),
            InputStoryContent::Video(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputStoryContent::Photo(t) => t.client_id(),
            InputStoryContent::Video(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputStoryContent {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputStoryContent::_Default)
    }
}

impl AsRef<InputStoryContent> for InputStoryContent {
    fn as_ref(&self) -> &InputStoryContent {
        self
    }
}

/// A photo story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryContentPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Photo to send. The photo must be at most 10 MB in size. The photo size must be 1080x1920

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    photo: InputFile,
    /// File identifiers of the stickers added to the photo, if applicable

    #[serde(default)]
    added_sticker_file_ids: Vec<i32>,
}

impl RObject for InputStoryContentPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputStoryContent for InputStoryContentPhoto {}

impl InputStoryContentPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputStoryContentPhotoBuilder {
        let mut inner = InputStoryContentPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputStoryContentPhotoBuilder { inner }
    }

    pub fn photo(&self) -> &InputFile {
        &self.photo
    }

    pub fn added_sticker_file_ids(&self) -> &Vec<i32> {
        &self.added_sticker_file_ids
    }
}

#[doc(hidden)]
pub struct InputStoryContentPhotoBuilder {
    inner: InputStoryContentPhoto,
}

#[deprecated]
pub type RTDInputStoryContentPhotoBuilder = InputStoryContentPhotoBuilder;

impl InputStoryContentPhotoBuilder {
    pub fn build(&self) -> InputStoryContentPhoto {
        self.inner.clone()
    }

    pub fn photo<T: AsRef<InputFile>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }

    pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i32>) -> &mut Self {
        self.inner.added_sticker_file_ids = added_sticker_file_ids;
        self
    }
}

impl AsRef<InputStoryContentPhoto> for InputStoryContentPhoto {
    fn as_ref(&self) -> &InputStoryContentPhoto {
        self
    }
}

impl AsRef<InputStoryContentPhoto> for InputStoryContentPhotoBuilder {
    fn as_ref(&self) -> &InputStoryContentPhoto {
        &self.inner
    }
}

/// A video story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryContentVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Video to be sent. The video size must be 720x1280. The video must be streamable and stored in MPEG4 format, after encoding with x265 codec and key frames added each second

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    video: InputFile,
    /// File identifiers of the stickers added to the video, if applicable

    #[serde(default)]
    added_sticker_file_ids: Vec<i32>,
    /// Precise duration of the video, in seconds; 0-60

    #[serde(default)]
    duration: f32,
    /// True, if the video has no sound

    #[serde(default)]
    is_animation: bool,
}

impl RObject for InputStoryContentVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputStoryContent for InputStoryContentVideo {}

impl InputStoryContentVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputStoryContentVideoBuilder {
        let mut inner = InputStoryContentVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputStoryContentVideoBuilder { inner }
    }

    pub fn video(&self) -> &InputFile {
        &self.video
    }

    pub fn added_sticker_file_ids(&self) -> &Vec<i32> {
        &self.added_sticker_file_ids
    }

    pub fn duration(&self) -> f32 {
        self.duration
    }

    pub fn is_animation(&self) -> bool {
        self.is_animation
    }
}

#[doc(hidden)]
pub struct InputStoryContentVideoBuilder {
    inner: InputStoryContentVideo,
}

#[deprecated]
pub type RTDInputStoryContentVideoBuilder = InputStoryContentVideoBuilder;

impl InputStoryContentVideoBuilder {
    pub fn build(&self) -> InputStoryContentVideo {
        self.inner.clone()
    }

    pub fn video<T: AsRef<InputFile>>(&mut self, video: T) -> &mut Self {
        self.inner.video = video.as_ref().clone();
        self
    }

    pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i32>) -> &mut Self {
        self.inner.added_sticker_file_ids = added_sticker_file_ids;
        self
    }

    pub fn duration(&mut self, duration: f32) -> &mut Self {
        self.inner.duration = duration;
        self
    }

    pub fn is_animation(&mut self, is_animation: bool) -> &mut Self {
        self.inner.is_animation = is_animation;
        self
    }
}

impl AsRef<InputStoryContentVideo> for InputStoryContentVideo {
    fn as_ref(&self) -> &InputStoryContentVideo {
        self
    }
}

impl AsRef<InputStoryContentVideo> for InputStoryContentVideoBuilder {
    fn as_ref(&self) -> &InputStoryContentVideo {
        &self.inner
    }
}
