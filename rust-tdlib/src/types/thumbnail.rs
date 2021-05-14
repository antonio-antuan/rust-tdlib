use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a thumbnail
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Thumbnail {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Thumbnail format

    #[serde(skip_serializing_if = "ThumbnailFormat::_is_default")]
    format: ThumbnailFormat,
    /// Thumbnail width
    width: i32,
    /// Thumbnail height
    height: i32,
    /// The thumbnail
    file: File,
}

impl RObject for Thumbnail {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Thumbnail {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDThumbnailBuilder {
        let mut inner = Thumbnail::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDThumbnailBuilder { inner }
    }

    pub fn format(&self) -> &ThumbnailFormat {
        &self.format
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

#[doc(hidden)]
pub struct RTDThumbnailBuilder {
    inner: Thumbnail,
}

impl RTDThumbnailBuilder {
    pub fn build(&self) -> Thumbnail {
        self.inner.clone()
    }

    pub fn format<T: AsRef<ThumbnailFormat>>(&mut self, format: T) -> &mut Self {
        self.inner.format = format.as_ref().clone();
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

    pub fn file<T: AsRef<File>>(&mut self, file: T) -> &mut Self {
        self.inner.file = file.as_ref().clone();
        self
    }
}

impl AsRef<Thumbnail> for Thumbnail {
    fn as_ref(&self) -> &Thumbnail {
        self
    }
}

impl AsRef<Thumbnail> for RTDThumbnailBuilder {
    fn as_ref(&self) -> &Thumbnail {
        &self.inner
    }
}
