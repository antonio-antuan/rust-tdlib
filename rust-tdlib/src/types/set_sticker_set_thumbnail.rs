use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sets a sticker set thumbnail; for bots only. Returns the sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetStickerSetThumbnail {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker set owner
    user_id: i32,
    /// Sticker set name
    name: String,
    /// Thumbnail to set in PNG or TGS format. Animated thumbnail must be set for animated sticker sets and only for them. Pass a zero InputFileId to delete the thumbnail

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    thumbnail: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetStickerSetThumbnail {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetStickerSetThumbnail {}

impl SetStickerSetThumbnail {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetStickerSetThumbnailBuilder {
        let mut inner = SetStickerSetThumbnail::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setStickerSetThumbnail".to_string();

        RTDSetStickerSetThumbnailBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn thumbnail(&self) -> &InputFile {
        &self.thumbnail
    }
}

#[doc(hidden)]
pub struct RTDSetStickerSetThumbnailBuilder {
    inner: SetStickerSetThumbnail,
}

impl RTDSetStickerSetThumbnailBuilder {
    pub fn build(&self) -> SetStickerSetThumbnail {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn thumbnail<T: AsRef<InputFile>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = thumbnail.as_ref().clone();
        self
    }
}

impl AsRef<SetStickerSetThumbnail> for SetStickerSetThumbnail {
    fn as_ref(&self) -> &SetStickerSetThumbnail {
        self
    }
}

impl AsRef<SetStickerSetThumbnail> for RTDSetStickerSetThumbnailBuilder {
    fn as_ref(&self) -> &SetStickerSetThumbnail {
        &self.inner
    }
}
