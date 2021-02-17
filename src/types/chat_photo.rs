use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat or user profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique photo identifier

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
    /// Point in time (Unix timestamp) when the photo has been added
    added_date: i32,
    /// Photo minithumbnail; may be null
    minithumbnail: Option<Minithumbnail>,
    /// Available variants of the photo in JPEG format, in different size
    sizes: Vec<PhotoSize>,
    /// Animated variant of the photo in MPEG4 format; may be null
    animation: Option<AnimatedChatPhoto>,
}

impl RObject for ChatPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatPhotoBuilder {
        let mut inner = ChatPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatPhotoBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn added_date(&self) -> i32 {
        self.added_date
    }

    pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
        &self.minithumbnail
    }

    pub fn sizes(&self) -> &Vec<PhotoSize> {
        &self.sizes
    }

    pub fn animation(&self) -> &Option<AnimatedChatPhoto> {
        &self.animation
    }
}

#[doc(hidden)]
pub struct RTDChatPhotoBuilder {
    inner: ChatPhoto,
}

impl RTDChatPhotoBuilder {
    pub fn build(&self) -> ChatPhoto {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn added_date(&mut self, added_date: i32) -> &mut Self {
        self.inner.added_date = added_date;
        self
    }

    pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
        self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
        self
    }

    pub fn sizes(&mut self, sizes: Vec<PhotoSize>) -> &mut Self {
        self.inner.sizes = sizes;
        self
    }

    pub fn animation<T: AsRef<AnimatedChatPhoto>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = Some(animation.as_ref().clone());
        self
    }
}

impl AsRef<ChatPhoto> for ChatPhoto {
    fn as_ref(&self) -> &ChatPhoto {
        self
    }
}

impl AsRef<ChatPhoto> for RTDChatPhotoBuilder {
    fn as_ref(&self) -> &ChatPhoto {
        &self.inner
    }
}
