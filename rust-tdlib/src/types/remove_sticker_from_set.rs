use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes a sticker from the set to which it belongs; for bots only. The sticker set must have been created by the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveStickerFromSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveStickerFromSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveStickerFromSet {}

impl RemoveStickerFromSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveStickerFromSetBuilder {
        let mut inner = RemoveStickerFromSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeStickerFromSet".to_string();

        RTDRemoveStickerFromSetBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct RTDRemoveStickerFromSetBuilder {
    inner: RemoveStickerFromSet,
}

impl RTDRemoveStickerFromSetBuilder {
    pub fn build(&self) -> RemoveStickerFromSet {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<RemoveStickerFromSet> for RemoveStickerFromSet {
    fn as_ref(&self) -> &RemoveStickerFromSet {
        self
    }
}

impl AsRef<RemoveStickerFromSet> for RTDRemoveStickerFromSetBuilder {
    fn as_ref(&self) -> &RemoveStickerFromSet {
        &self.inner
    }
}
