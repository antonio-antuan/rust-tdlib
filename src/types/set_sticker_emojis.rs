use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the list of emoji corresponding to a sticker; for bots only. The sticker must belong to a regular or custom emoji sticker set created by the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetStickerEmojis {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,
    /// New string with 1-20 emoji corresponding to the sticker

    #[serde(default)]
    emojis: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetStickerEmojis {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetStickerEmojis {}

impl SetStickerEmojis {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetStickerEmojisBuilder {
        let mut inner = SetStickerEmojis::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setStickerEmojis".to_string();

        SetStickerEmojisBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }

    pub fn emojis(&self) -> &String {
        &self.emojis
    }
}

#[doc(hidden)]
pub struct SetStickerEmojisBuilder {
    inner: SetStickerEmojis,
}

#[deprecated]
pub type RTDSetStickerEmojisBuilder = SetStickerEmojisBuilder;

impl SetStickerEmojisBuilder {
    pub fn build(&self) -> SetStickerEmojis {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }

    pub fn emojis<T: AsRef<str>>(&mut self, emojis: T) -> &mut Self {
        self.inner.emojis = emojis.as_ref().to_string();
        self
    }
}

impl AsRef<SetStickerEmojis> for SetStickerEmojis {
    fn as_ref(&self) -> &SetStickerEmojis {
        self
    }
}

impl AsRef<SetStickerEmojis> for SetStickerEmojisBuilder {
    fn as_ref(&self) -> &SetStickerEmojis {
        &self.inner
    }
}
