use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes a sticker from the list of favorite stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveFavoriteSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker file to delete from the list

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveFavoriteSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveFavoriteSticker {}

impl RemoveFavoriteSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveFavoriteStickerBuilder {
        let mut inner = RemoveFavoriteSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeFavoriteSticker".to_string();

        RTDRemoveFavoriteStickerBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct RTDRemoveFavoriteStickerBuilder {
    inner: RemoveFavoriteSticker,
}

impl RTDRemoveFavoriteStickerBuilder {
    pub fn build(&self) -> RemoveFavoriteSticker {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<RemoveFavoriteSticker> for RemoveFavoriteSticker {
    fn as_ref(&self) -> &RemoveFavoriteSticker {
        self
    }
}

impl AsRef<RemoveFavoriteSticker> for RTDRemoveFavoriteStickerBuilder {
    fn as_ref(&self) -> &RemoveFavoriteSticker {
        &self.inner
    }
}
