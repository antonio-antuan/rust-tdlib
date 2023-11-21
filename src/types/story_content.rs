use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains the content of a story
pub trait TDStoryContent: Debug + RObject {}

/// Contains the content of a story
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum StoryContent {
    #[doc(hidden)]
    #[default]
    _Default,
    /// A photo story
    #[serde(rename = "storyContentPhoto")]
    Photo(StoryContentPhoto),
    /// A story content that is not supported in the current TDLib version
    #[serde(rename = "storyContentUnsupported")]
    Unsupported(StoryContentUnsupported),
    /// A video story
    #[serde(rename = "storyContentVideo")]
    Video(StoryContentVideo),
}

impl RObject for StoryContent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            StoryContent::Photo(t) => t.extra(),
            StoryContent::Unsupported(t) => t.extra(),
            StoryContent::Video(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            StoryContent::Photo(t) => t.client_id(),
            StoryContent::Unsupported(t) => t.client_id(),
            StoryContent::Video(t) => t.client_id(),

            _ => None,
        }
    }
}

impl StoryContent {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, StoryContent::_Default)
    }
}

impl AsRef<StoryContent> for StoryContent {
    fn as_ref(&self) -> &StoryContent {
        self
    }
}

/// A photo story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The photo
    photo: Photo,
}

impl RObject for StoryContentPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryContent for StoryContentPhoto {}

impl StoryContentPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryContentPhotoBuilder {
        let mut inner = StoryContentPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryContentPhotoBuilder { inner }
    }

    pub fn photo(&self) -> &Photo {
        &self.photo
    }
}

#[doc(hidden)]
pub struct StoryContentPhotoBuilder {
    inner: StoryContentPhoto,
}

#[deprecated]
pub type RTDStoryContentPhotoBuilder = StoryContentPhotoBuilder;

impl StoryContentPhotoBuilder {
    pub fn build(&self) -> StoryContentPhoto {
        self.inner.clone()
    }

    pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }
}

impl AsRef<StoryContentPhoto> for StoryContentPhoto {
    fn as_ref(&self) -> &StoryContentPhoto {
        self
    }
}

impl AsRef<StoryContentPhoto> for StoryContentPhotoBuilder {
    fn as_ref(&self) -> &StoryContentPhoto {
        &self.inner
    }
}

/// A story content that is not supported in the current TDLib version
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentUnsupported {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StoryContentUnsupported {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryContent for StoryContentUnsupported {}

impl StoryContentUnsupported {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryContentUnsupportedBuilder {
        let mut inner = StoryContentUnsupported::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryContentUnsupportedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct StoryContentUnsupportedBuilder {
    inner: StoryContentUnsupported,
}

#[deprecated]
pub type RTDStoryContentUnsupportedBuilder = StoryContentUnsupportedBuilder;

impl StoryContentUnsupportedBuilder {
    pub fn build(&self) -> StoryContentUnsupported {
        self.inner.clone()
    }
}

impl AsRef<StoryContentUnsupported> for StoryContentUnsupported {
    fn as_ref(&self) -> &StoryContentUnsupported {
        self
    }
}

impl AsRef<StoryContentUnsupported> for StoryContentUnsupportedBuilder {
    fn as_ref(&self) -> &StoryContentUnsupported {
        &self.inner
    }
}

/// A video story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryContentVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The video in MPEG4 format
    video: StoryVideo,
    /// Alternative version of the video in MPEG4 format, encoded by x264 codec; may be null
    alternative_video: Option<StoryVideo>,
}

impl RObject for StoryContentVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryContent for StoryContentVideo {}

impl StoryContentVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryContentVideoBuilder {
        let mut inner = StoryContentVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryContentVideoBuilder { inner }
    }

    pub fn video(&self) -> &StoryVideo {
        &self.video
    }

    pub fn alternative_video(&self) -> &Option<StoryVideo> {
        &self.alternative_video
    }
}

#[doc(hidden)]
pub struct StoryContentVideoBuilder {
    inner: StoryContentVideo,
}

#[deprecated]
pub type RTDStoryContentVideoBuilder = StoryContentVideoBuilder;

impl StoryContentVideoBuilder {
    pub fn build(&self) -> StoryContentVideo {
        self.inner.clone()
    }

    pub fn video<T: AsRef<StoryVideo>>(&mut self, video: T) -> &mut Self {
        self.inner.video = video.as_ref().clone();
        self
    }

    pub fn alternative_video<T: AsRef<StoryVideo>>(&mut self, alternative_video: T) -> &mut Self {
        self.inner.alternative_video = Some(alternative_video.as_ref().clone());
        self
    }
}

impl AsRef<StoryContentVideo> for StoryContentVideo {
    fn as_ref(&self) -> &StoryContentVideo {
        self
    }
}

impl AsRef<StoryContentVideo> for StoryContentVideoBuilder {
    fn as_ref(&self) -> &StoryContentVideo {
        &self.inner
    }
}
