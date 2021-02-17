use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Thumbnail image of a very poor quality and low resolution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Minithumbnail {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Thumbnail width, usually doesn't exceed 40
    width: i32,
    /// Thumbnail height, usually doesn't exceed 40
    height: i32,
    /// The thumbnail in JPEG format
    data: String,
}

impl RObject for Minithumbnail {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Minithumbnail {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMinithumbnailBuilder {
        let mut inner = Minithumbnail::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMinithumbnailBuilder { inner }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDMinithumbnailBuilder {
    inner: Minithumbnail,
}

impl RTDMinithumbnailBuilder {
    pub fn build(&self) -> Minithumbnail {
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

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<Minithumbnail> for Minithumbnail {
    fn as_ref(&self) -> &Minithumbnail {
        self
    }
}

impl AsRef<Minithumbnail> for RTDMinithumbnailBuilder {
    fn as_ref(&self) -> &Minithumbnail {
        &self.inner
    }
}
