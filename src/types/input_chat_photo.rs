use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a photo to be set as a user profile or chat photo
pub trait TDInputChatPhoto: Debug + RObject {}

/// Describes a photo to be set as a user profile or chat photo
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputChatPhoto {
    #[doc(hidden)]
    _Default,
    /// An animation in MPEG4 format; must be square, at most 10 seconds long, have width between 160 and 800 and be at most 2MB in size
    #[serde(rename(
        serialize = "inputChatPhotoAnimation",
        deserialize = "inputChatPhotoAnimation"
    ))]
    Animation(InputChatPhotoAnimation),
    /// A previously used profile photo of the current user
    #[serde(rename(
        serialize = "inputChatPhotoPrevious",
        deserialize = "inputChatPhotoPrevious"
    ))]
    Previous(InputChatPhotoPrevious),
    /// A static photo in JPEG format
    #[serde(rename(
        serialize = "inputChatPhotoStatic",
        deserialize = "inputChatPhotoStatic"
    ))]
    Static(InputChatPhotoStatic),
}

impl Default for InputChatPhoto {
    fn default() -> Self {
        InputChatPhoto::_Default
    }
}

impl RObject for InputChatPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputChatPhoto::Animation(t) => t.extra(),
            InputChatPhoto::Previous(t) => t.extra(),
            InputChatPhoto::Static(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputChatPhoto::Animation(t) => t.client_id(),
            InputChatPhoto::Previous(t) => t.client_id(),
            InputChatPhoto::Static(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputChatPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputChatPhoto::_Default)
    }
}

impl AsRef<InputChatPhoto> for InputChatPhoto {
    fn as_ref(&self) -> &InputChatPhoto {
        self
    }
}

/// An animation in MPEG4 format; must be square, at most 10 seconds long, have width between 160 and 800 and be at most 2MB in size
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputChatPhotoAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Animation to be set as profile photo. Only inputFileLocal and inputFileGenerated are allowed

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    animation: InputFile,
    /// Timestamp of the frame, which will be used as static chat photo
    main_frame_timestamp: f32,
}

impl RObject for InputChatPhotoAnimation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputChatPhoto for InputChatPhotoAnimation {}

impl InputChatPhotoAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputChatPhotoAnimationBuilder {
        let mut inner = InputChatPhotoAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputChatPhotoAnimationBuilder { inner }
    }

    pub fn animation(&self) -> &InputFile {
        &self.animation
    }

    pub fn main_frame_timestamp(&self) -> f32 {
        self.main_frame_timestamp
    }
}

#[doc(hidden)]
pub struct RTDInputChatPhotoAnimationBuilder {
    inner: InputChatPhotoAnimation,
}

impl RTDInputChatPhotoAnimationBuilder {
    pub fn build(&self) -> InputChatPhotoAnimation {
        self.inner.clone()
    }

    pub fn animation<T: AsRef<InputFile>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = animation.as_ref().clone();
        self
    }

    pub fn main_frame_timestamp(&mut self, main_frame_timestamp: f32) -> &mut Self {
        self.inner.main_frame_timestamp = main_frame_timestamp;
        self
    }
}

impl AsRef<InputChatPhotoAnimation> for InputChatPhotoAnimation {
    fn as_ref(&self) -> &InputChatPhotoAnimation {
        self
    }
}

impl AsRef<InputChatPhotoAnimation> for RTDInputChatPhotoAnimationBuilder {
    fn as_ref(&self) -> &InputChatPhotoAnimation {
        &self.inner
    }
}

/// A previously used profile photo of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputChatPhotoPrevious {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the profile photo to reuse

    #[serde(deserialize_with = "super::_common::number_from_string")]
    chat_photo_id: i64,
}

impl RObject for InputChatPhotoPrevious {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputChatPhoto for InputChatPhotoPrevious {}

impl InputChatPhotoPrevious {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputChatPhotoPreviousBuilder {
        let mut inner = InputChatPhotoPrevious::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputChatPhotoPreviousBuilder { inner }
    }

    pub fn chat_photo_id(&self) -> i64 {
        self.chat_photo_id
    }
}

#[doc(hidden)]
pub struct RTDInputChatPhotoPreviousBuilder {
    inner: InputChatPhotoPrevious,
}

impl RTDInputChatPhotoPreviousBuilder {
    pub fn build(&self) -> InputChatPhotoPrevious {
        self.inner.clone()
    }

    pub fn chat_photo_id(&mut self, chat_photo_id: i64) -> &mut Self {
        self.inner.chat_photo_id = chat_photo_id;
        self
    }
}

impl AsRef<InputChatPhotoPrevious> for InputChatPhotoPrevious {
    fn as_ref(&self) -> &InputChatPhotoPrevious {
        self
    }
}

impl AsRef<InputChatPhotoPrevious> for RTDInputChatPhotoPreviousBuilder {
    fn as_ref(&self) -> &InputChatPhotoPrevious {
        &self.inner
    }
}

/// A static photo in JPEG format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputChatPhotoStatic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Photo to be set as profile photo. Only inputFileLocal and inputFileGenerated are allowed

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    photo: InputFile,
}

impl RObject for InputChatPhotoStatic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputChatPhoto for InputChatPhotoStatic {}

impl InputChatPhotoStatic {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputChatPhotoStaticBuilder {
        let mut inner = InputChatPhotoStatic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputChatPhotoStaticBuilder { inner }
    }

    pub fn photo(&self) -> &InputFile {
        &self.photo
    }
}

#[doc(hidden)]
pub struct RTDInputChatPhotoStaticBuilder {
    inner: InputChatPhotoStatic,
}

impl RTDInputChatPhotoStaticBuilder {
    pub fn build(&self) -> InputChatPhotoStatic {
        self.inner.clone()
    }

    pub fn photo<T: AsRef<InputFile>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }
}

impl AsRef<InputChatPhotoStatic> for InputChatPhotoStatic {
    fn as_ref(&self) -> &InputChatPhotoStatic {
        self
    }
}

impl AsRef<InputChatPhotoStatic> for RTDInputChatPhotoStaticBuilder {
    fn as_ref(&self) -> &InputChatPhotoStatic {
        &self.inner
    }
}
