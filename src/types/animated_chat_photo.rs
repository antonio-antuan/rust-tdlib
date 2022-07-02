use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Animated variant of a chat photo in MPEG4 format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnimatedChatPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Animation width and height

    #[serde(default)]
    length: i32,
    /// Information about the animation file
    file: File,
    /// Timestamp of the frame, used as a static chat photo

    #[serde(default)]
    main_frame_timestamp: f32,
}

impl RObject for AnimatedChatPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AnimatedChatPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AnimatedChatPhotoBuilder {
        let mut inner = AnimatedChatPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AnimatedChatPhotoBuilder { inner }
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn file(&self) -> &File {
        &self.file
    }

    pub fn main_frame_timestamp(&self) -> f32 {
        self.main_frame_timestamp
    }
}

#[doc(hidden)]
pub struct AnimatedChatPhotoBuilder {
    inner: AnimatedChatPhoto,
}

#[deprecated]
pub type RTDAnimatedChatPhotoBuilder = AnimatedChatPhotoBuilder;

impl AnimatedChatPhotoBuilder {
    pub fn build(&self) -> AnimatedChatPhoto {
        self.inner.clone()
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }

    pub fn file<T: AsRef<File>>(&mut self, file: T) -> &mut Self {
        self.inner.file = file.as_ref().clone();
        self
    }

    pub fn main_frame_timestamp(&mut self, main_frame_timestamp: f32) -> &mut Self {
        self.inner.main_frame_timestamp = main_frame_timestamp;
        self
    }
}

impl AsRef<AnimatedChatPhoto> for AnimatedChatPhoto {
    fn as_ref(&self) -> &AnimatedChatPhoto {
        self
    }
}

impl AsRef<AnimatedChatPhoto> for AnimatedChatPhotoBuilder {
    fn as_ref(&self) -> &AnimatedChatPhoto {
        &self.inner
    }
}
