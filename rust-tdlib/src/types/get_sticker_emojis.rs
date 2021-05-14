use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns emoji corresponding to a sticker. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStickerEmojis {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker file identifier

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetStickerEmojis {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetStickerEmojis {}

impl GetStickerEmojis {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetStickerEmojisBuilder {
        let mut inner = GetStickerEmojis::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStickerEmojis".to_string();

        RTDGetStickerEmojisBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct RTDGetStickerEmojisBuilder {
    inner: GetStickerEmojis,
}

impl RTDGetStickerEmojisBuilder {
    pub fn build(&self) -> GetStickerEmojis {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<GetStickerEmojis> for GetStickerEmojis {
    fn as_ref(&self) -> &GetStickerEmojis {
        self
    }
}

impl AsRef<GetStickerEmojis> for RTDGetStickerEmojisBuilder {
    fn as_ref(&self) -> &GetStickerEmojis {
        &self.inner
    }
}
