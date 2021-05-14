use crate::errors::*;
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
    total_count: i32,
    /// List of photos
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatPhotosBuilder {
        let mut inner = ChatPhotos::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatPhotosBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn photos(&self) -> &Vec<ChatPhoto> {
        &self.photos
    }
}

#[doc(hidden)]
pub struct RTDChatPhotosBuilder {
    inner: ChatPhotos,
}

impl RTDChatPhotosBuilder {
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

impl AsRef<ChatPhotos> for RTDChatPhotosBuilder {
    fn as_ref(&self) -> &ChatPhotos {
        &self.inner
    }
}
