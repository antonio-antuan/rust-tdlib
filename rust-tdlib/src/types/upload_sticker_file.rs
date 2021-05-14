use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Uploads a PNG image with a sticker; for bots only; returns the uploaded file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadStickerFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker file owner
    user_id: i32,
    /// PNG image with the sticker; must be up to 512 KB in size and fit in 512x512 square

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    png_sticker: InputFile,

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

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn png_sticker(&self) -> &InputFile {
        &self.png_sticker
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

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn png_sticker<T: AsRef<InputFile>>(&mut self, png_sticker: T) -> &mut Self {
        self.inner.png_sticker = png_sticker.as_ref().clone();
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
