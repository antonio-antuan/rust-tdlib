use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddFavoriteSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker file to add

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddFavoriteSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddFavoriteSticker {}

impl AddFavoriteSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddFavoriteStickerBuilder {
        let mut inner = AddFavoriteSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addFavoriteSticker".to_string();

        AddFavoriteStickerBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct AddFavoriteStickerBuilder {
    inner: AddFavoriteSticker,
}

#[deprecated]
pub type RTDAddFavoriteStickerBuilder = AddFavoriteStickerBuilder;

impl AddFavoriteStickerBuilder {
    pub fn build(&self) -> AddFavoriteSticker {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<AddFavoriteSticker> for AddFavoriteSticker {
    fn as_ref(&self) -> &AddFavoriteSticker {
        self
    }
}

impl AsRef<AddFavoriteSticker> for AddFavoriteStickerBuilder {
    fn as_ref(&self) -> &AddFavoriteSticker {
        &self.inner
    }
}
