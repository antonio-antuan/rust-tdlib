use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes an image in JPEG format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhotoSize {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Image type (see https://core.telegram.org/constructor/photoSize)

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(default)]
    type_: String,
    /// Information about the image file
    photo: File,
    /// Image width

    #[serde(default)]
    width: i32,
    /// Image height

    #[serde(default)]
    height: i32,
    /// Sizes of progressive JPEG file prefixes, which can be used to preliminarily show the image; in bytes

    #[serde(default)]
    progressive_sizes: Vec<i32>,
}

impl RObject for PhotoSize {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PhotoSize {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PhotoSizeBuilder {
        let mut inner = PhotoSize::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PhotoSizeBuilder { inner }
    }

    pub fn type_(&self) -> &String {
        &self.type_
    }

    pub fn photo(&self) -> &File {
        &self.photo
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn progressive_sizes(&self) -> &Vec<i32> {
        &self.progressive_sizes
    }
}

#[doc(hidden)]
pub struct PhotoSizeBuilder {
    inner: PhotoSize,
}

#[deprecated]
pub type RTDPhotoSizeBuilder = PhotoSizeBuilder;

impl PhotoSizeBuilder {
    pub fn build(&self) -> PhotoSize {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().to_string();
        self
    }

    pub fn photo<T: AsRef<File>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
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

    pub fn progressive_sizes(&mut self, progressive_sizes: Vec<i32>) -> &mut Self {
        self.inner.progressive_sizes = progressive_sizes;
        self
    }
}

impl AsRef<PhotoSize> for PhotoSize {
    fn as_ref(&self) -> &PhotoSize {
        self
    }
}

impl AsRef<PhotoSize> for PhotoSizeBuilder {
    fn as_ref(&self) -> &PhotoSize {
        &self.inner
    }
}
