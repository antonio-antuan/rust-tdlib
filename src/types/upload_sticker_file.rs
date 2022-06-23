use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Uploads a file with a sticker; returns the uploaded file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadStickerFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker file owner; ignored for regular users
    user_id: i64,
    /// Sticker file to upload
    sticker: InputSticker,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for UploadStickerFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for UploadStickerFile {}

impl UploadStickerFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUploadStickerFileBuilder {
        let mut inner = UploadStickerFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "uploadStickerFile".to_string();

        RTDUploadStickerFileBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn sticker(&self) -> &InputSticker {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct RTDUploadStickerFileBuilder {
    inner: UploadStickerFile,
}

impl RTDUploadStickerFileBuilder {
    pub fn build(&self) -> UploadStickerFile {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn sticker<T: AsRef<InputSticker>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<UploadStickerFile> for UploadStickerFile {
    fn as_ref(&self) -> &UploadStickerFile {
        self
    }
}

impl AsRef<UploadStickerFile> for RTDUploadStickerFileBuilder {
    fn as_ref(&self) -> &UploadStickerFile {
        &self.inner
    }
}
