use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains basic information about the photo of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhotoInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A small (160x160) chat photo variant in JPEG format. The file can be downloaded only before the photo is changed
    small: File,
    /// A big (640x640) chat photo variant in JPEG format. The file can be downloaded only before the photo is changed
    big: File,
    /// Chat photo minithumbnail; may be null
    minithumbnail: Option<Minithumbnail>,
    /// True, if the photo has animated variant

    #[serde(default)]
    has_animation: Option<bool>,
}

impl RObject for ChatPhotoInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatPhotoInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatPhotoInfoBuilder {
        let mut inner = ChatPhotoInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatPhotoInfoBuilder { inner }
    }

    pub fn small(&self) -> &File {
        &self.small
    }

    pub fn big(&self) -> &File {
        &self.big
    }

    pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
        &self.minithumbnail
    }

    pub fn has_animation(&self) -> &Option<bool> {
        &self.has_animation
    }
}

#[doc(hidden)]
pub struct ChatPhotoInfoBuilder {
    inner: ChatPhotoInfo,
}

#[deprecated]
pub type RTDChatPhotoInfoBuilder = ChatPhotoInfoBuilder;

impl ChatPhotoInfoBuilder {
    pub fn build(&self) -> ChatPhotoInfo {
        self.inner.clone()
    }

    pub fn small<T: AsRef<File>>(&mut self, small: T) -> &mut Self {
        self.inner.small = small.as_ref().clone();
        self
    }

    pub fn big<T: AsRef<File>>(&mut self, big: T) -> &mut Self {
        self.inner.big = big.as_ref().clone();
        self
    }

    pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
        self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
        self
    }

    pub fn has_animation(&mut self, has_animation: bool) -> &mut Self {
        self.inner.has_animation = Some(has_animation);
        self
    }
}

impl AsRef<ChatPhotoInfo> for ChatPhotoInfo {
    fn as_ref(&self) -> &ChatPhotoInfo {
        self
    }
}

impl AsRef<ChatPhotoInfo> for ChatPhotoInfoBuilder {
    fn as_ref(&self) -> &ChatPhotoInfo {
        &self.inner
    }
}
