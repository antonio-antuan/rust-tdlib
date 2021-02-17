use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Photo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if stickers were added to the photo. The list of corresponding sticker sets can be received using getAttachedStickerSets
    has_stickers: bool,
    /// Photo minithumbnail; may be null
    minithumbnail: Option<Minithumbnail>,
    /// Available variants of the photo, in different sizes
    sizes: Vec<PhotoSize>,
}

impl RObject for Photo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Photo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPhotoBuilder {
        let mut inner = Photo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPhotoBuilder { inner }
    }

    pub fn has_stickers(&self) -> bool {
        self.has_stickers
    }

    pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
        &self.minithumbnail
    }

    pub fn sizes(&self) -> &Vec<PhotoSize> {
        &self.sizes
    }
}

#[doc(hidden)]
pub struct RTDPhotoBuilder {
    inner: Photo,
}

impl RTDPhotoBuilder {
    pub fn build(&self) -> Photo {
        self.inner.clone()
    }

    pub fn has_stickers(&mut self, has_stickers: bool) -> &mut Self {
        self.inner.has_stickers = has_stickers;
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
}

impl AsRef<Photo> for Photo {
    fn as_ref(&self) -> &Photo {
        self
    }
}

impl AsRef<Photo> for RTDPhotoBuilder {
    fn as_ref(&self) -> &Photo {
        &self.inner
    }
}
