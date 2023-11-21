use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a media, which is attached to an invoice
pub trait TDMessageExtendedMedia: Debug + RObject {}

/// Describes a media, which is attached to an invoice
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum MessageExtendedMedia {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The media is a photo
    #[serde(rename = "messageExtendedMediaPhoto")]
    Photo(MessageExtendedMediaPhoto),
    /// The media is hidden until the invoice is paid
    #[serde(rename = "messageExtendedMediaPreview")]
    Preview(MessageExtendedMediaPreview),
    /// The media is unsupported
    #[serde(rename = "messageExtendedMediaUnsupported")]
    Unsupported(MessageExtendedMediaUnsupported),
    /// The media is a video
    #[serde(rename = "messageExtendedMediaVideo")]
    Video(MessageExtendedMediaVideo),
}

impl RObject for MessageExtendedMedia {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageExtendedMedia::Photo(t) => t.extra(),
            MessageExtendedMedia::Preview(t) => t.extra(),
            MessageExtendedMedia::Unsupported(t) => t.extra(),
            MessageExtendedMedia::Video(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageExtendedMedia::Photo(t) => t.client_id(),
            MessageExtendedMedia::Preview(t) => t.client_id(),
            MessageExtendedMedia::Unsupported(t) => t.client_id(),
            MessageExtendedMedia::Video(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageExtendedMedia {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageExtendedMedia::_Default)
    }
}

impl AsRef<MessageExtendedMedia> for MessageExtendedMedia {
    fn as_ref(&self) -> &MessageExtendedMedia {
        self
    }
}

/// The media is a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageExtendedMediaPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The photo
    photo: Photo,
    /// Photo caption
    caption: FormattedText,
}

impl RObject for MessageExtendedMediaPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageExtendedMedia for MessageExtendedMediaPhoto {}

impl MessageExtendedMediaPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageExtendedMediaPhotoBuilder {
        let mut inner = MessageExtendedMediaPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageExtendedMediaPhotoBuilder { inner }
    }

    pub fn photo(&self) -> &Photo {
        &self.photo
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct MessageExtendedMediaPhotoBuilder {
    inner: MessageExtendedMediaPhoto,
}

#[deprecated]
pub type RTDMessageExtendedMediaPhotoBuilder = MessageExtendedMediaPhotoBuilder;

impl MessageExtendedMediaPhotoBuilder {
    pub fn build(&self) -> MessageExtendedMediaPhoto {
        self.inner.clone()
    }

    pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<MessageExtendedMediaPhoto> for MessageExtendedMediaPhoto {
    fn as_ref(&self) -> &MessageExtendedMediaPhoto {
        self
    }
}

impl AsRef<MessageExtendedMediaPhoto> for MessageExtendedMediaPhotoBuilder {
    fn as_ref(&self) -> &MessageExtendedMediaPhoto {
        &self.inner
    }
}

/// The media is hidden until the invoice is paid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageExtendedMediaPreview {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Media width; 0 if unknown

    #[serde(default)]
    width: i32,
    /// Media height; 0 if unknown

    #[serde(default)]
    height: i32,
    /// Media duration; 0 if unknown

    #[serde(default)]
    duration: i32,
    /// Media minithumbnail; may be null
    minithumbnail: Option<Minithumbnail>,
    /// Media caption
    caption: FormattedText,
}

impl RObject for MessageExtendedMediaPreview {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageExtendedMedia for MessageExtendedMediaPreview {}

impl MessageExtendedMediaPreview {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageExtendedMediaPreviewBuilder {
        let mut inner = MessageExtendedMediaPreview::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageExtendedMediaPreviewBuilder { inner }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
        &self.minithumbnail
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct MessageExtendedMediaPreviewBuilder {
    inner: MessageExtendedMediaPreview,
}

#[deprecated]
pub type RTDMessageExtendedMediaPreviewBuilder = MessageExtendedMediaPreviewBuilder;

impl MessageExtendedMediaPreviewBuilder {
    pub fn build(&self) -> MessageExtendedMediaPreview {
        self.inner.clone()
    }

    pub fn width(&mut self, width: i32) -> &mut Self {
        self.inner.width = width;
        self
    }

    pub fn height(&mut self, height: i32) -> &mut Self {
        self.inner.height = height;
        self
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
        self
    }

    pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
        self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
        self
    }

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<MessageExtendedMediaPreview> for MessageExtendedMediaPreview {
    fn as_ref(&self) -> &MessageExtendedMediaPreview {
        self
    }
}

impl AsRef<MessageExtendedMediaPreview> for MessageExtendedMediaPreviewBuilder {
    fn as_ref(&self) -> &MessageExtendedMediaPreview {
        &self.inner
    }
}

/// The media is unsupported
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageExtendedMediaUnsupported {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Media caption
    caption: FormattedText,
}

impl RObject for MessageExtendedMediaUnsupported {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageExtendedMedia for MessageExtendedMediaUnsupported {}

impl MessageExtendedMediaUnsupported {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageExtendedMediaUnsupportedBuilder {
        let mut inner = MessageExtendedMediaUnsupported::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageExtendedMediaUnsupportedBuilder { inner }
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct MessageExtendedMediaUnsupportedBuilder {
    inner: MessageExtendedMediaUnsupported,
}

#[deprecated]
pub type RTDMessageExtendedMediaUnsupportedBuilder = MessageExtendedMediaUnsupportedBuilder;

impl MessageExtendedMediaUnsupportedBuilder {
    pub fn build(&self) -> MessageExtendedMediaUnsupported {
        self.inner.clone()
    }

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<MessageExtendedMediaUnsupported> for MessageExtendedMediaUnsupported {
    fn as_ref(&self) -> &MessageExtendedMediaUnsupported {
        self
    }
}

impl AsRef<MessageExtendedMediaUnsupported> for MessageExtendedMediaUnsupportedBuilder {
    fn as_ref(&self) -> &MessageExtendedMediaUnsupported {
        &self.inner
    }
}

/// The media is a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageExtendedMediaVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The video
    video: Video,
    /// Photo caption
    caption: FormattedText,
}

impl RObject for MessageExtendedMediaVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageExtendedMedia for MessageExtendedMediaVideo {}

impl MessageExtendedMediaVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageExtendedMediaVideoBuilder {
        let mut inner = MessageExtendedMediaVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageExtendedMediaVideoBuilder { inner }
    }

    pub fn video(&self) -> &Video {
        &self.video
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct MessageExtendedMediaVideoBuilder {
    inner: MessageExtendedMediaVideo,
}

#[deprecated]
pub type RTDMessageExtendedMediaVideoBuilder = MessageExtendedMediaVideoBuilder;

impl MessageExtendedMediaVideoBuilder {
    pub fn build(&self) -> MessageExtendedMediaVideo {
        self.inner.clone()
    }

    pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
        self.inner.video = video.as_ref().clone();
        self
    }

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<MessageExtendedMediaVideo> for MessageExtendedMediaVideo {
    fn as_ref(&self) -> &MessageExtendedMediaVideo {
        self
    }
}

impl AsRef<MessageExtendedMediaVideo> for MessageExtendedMediaVideoBuilder {
    fn as_ref(&self) -> &MessageExtendedMediaVideo {
        &self.inner
    }
}
