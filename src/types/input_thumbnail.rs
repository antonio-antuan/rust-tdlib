use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A thumbnail to be sent along with a file; must be in JPEG or WEBP format for stickers, and less than 200 KB in size
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputThumbnail {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Thumbnail file to send. Sending thumbnails by file_id is currently not supported

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    thumbnail: InputFile,
    /// Thumbnail width, usually shouldn't exceed 320. Use 0 if unknown
    width: i32,
    /// Thumbnail height, usually shouldn't exceed 320. Use 0 if unknown
    height: i32,
}

impl RObject for InputThumbnail {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl InputThumbnail {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputThumbnailBuilder {
        let mut inner = InputThumbnail::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputThumbnailBuilder { inner }
    }

    pub fn thumbnail(&self) -> &InputFile {
        &self.thumbnail
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

#[doc(hidden)]
pub struct RTDInputThumbnailBuilder {
    inner: InputThumbnail,
}

impl RTDInputThumbnailBuilder {
    pub fn build(&self) -> InputThumbnail {
        self.inner.clone()
    }

    pub fn thumbnail<T: AsRef<InputFile>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = thumbnail.as_ref().clone();
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
}

impl AsRef<InputThumbnail> for InputThumbnail {
    fn as_ref(&self) -> &InputThumbnail {
        self
    }
}

impl AsRef<InputThumbnail> for RTDInputThumbnailBuilder {
    fn as_ref(&self) -> &InputThumbnail {
        &self.inner
    }
}
