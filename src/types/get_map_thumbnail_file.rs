use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMapThumbnailFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Location of the map center
    location: Location,
    /// Map zoom level; 13-20

    #[serde(default)]
    zoom: i32,
    /// Map width in pixels before applying scale; 16-1024

    #[serde(default)]
    width: i32,
    /// Map height in pixels before applying scale; 16-1024

    #[serde(default)]
    height: i32,
    /// Map scale; 1-3

    #[serde(default)]
    scale: i32,
    /// Identifier of a chat, in which the thumbnail will be shown. Use 0 if unknown

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMapThumbnailFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMapThumbnailFile {}

impl GetMapThumbnailFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMapThumbnailFileBuilder {
        let mut inner = GetMapThumbnailFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMapThumbnailFile".to_string();

        GetMapThumbnailFileBuilder { inner }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn zoom(&self) -> i32 {
        self.zoom
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn scale(&self) -> i32 {
        self.scale
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetMapThumbnailFileBuilder {
    inner: GetMapThumbnailFile,
}

#[deprecated]
pub type RTDGetMapThumbnailFileBuilder = GetMapThumbnailFileBuilder;

impl GetMapThumbnailFileBuilder {
    pub fn build(&self) -> GetMapThumbnailFile {
        self.inner.clone()
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }

    pub fn zoom(&mut self, zoom: i32) -> &mut Self {
        self.inner.zoom = zoom;
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

    pub fn scale(&mut self, scale: i32) -> &mut Self {
        self.inner.scale = scale;
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetMapThumbnailFile> for GetMapThumbnailFile {
    fn as_ref(&self) -> &GetMapThumbnailFile {
        self
    }
}

impl AsRef<GetMapThumbnailFile> for GetMapThumbnailFileBuilder {
    fn as_ref(&self) -> &GetMapThumbnailFile {
        &self.inner
    }
}
