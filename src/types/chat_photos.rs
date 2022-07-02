use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of chat or user profile photos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhotos {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Total number of photos

    #[serde(default)]
    total_count: i32,
    /// List of photos

    #[serde(default)]
    photos: Vec<ChatPhoto>,
}

impl RObject for ChatPhotos {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatPhotos {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatPhotosBuilder {
        let mut inner = ChatPhotos::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatPhotosBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn photos(&self) -> &Vec<ChatPhoto> {
        &self.photos
    }
}

#[doc(hidden)]
pub struct ChatPhotosBuilder {
    inner: ChatPhotos,
}

#[deprecated]
pub type RTDChatPhotosBuilder = ChatPhotosBuilder;

impl ChatPhotosBuilder {
    pub fn build(&self) -> ChatPhotos {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn photos(&mut self, photos: Vec<ChatPhoto>) -> &mut Self {
        self.inner.photos = photos;
        self
    }
}

impl AsRef<ChatPhotos> for ChatPhotos {
    fn as_ref(&self) -> &ChatPhotos {
        self
    }
}

impl AsRef<ChatPhotos> for ChatPhotosBuilder {
    fn as_ref(&self) -> &ChatPhotos {
        &self.inner
    }
}
