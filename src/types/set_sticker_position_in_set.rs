use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the position of a sticker in the set to which it belongs; for bots only. The sticker set must have been created by the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetStickerPositionInSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,
    /// New position of the sticker in the set, zero-based

    #[serde(default)]
    position: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetStickerPositionInSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetStickerPositionInSet {}

impl SetStickerPositionInSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetStickerPositionInSetBuilder {
        let mut inner = SetStickerPositionInSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setStickerPositionInSet".to_string();

        SetStickerPositionInSetBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }

    pub fn position(&self) -> i32 {
        self.position
    }
}

#[doc(hidden)]
pub struct SetStickerPositionInSetBuilder {
    inner: SetStickerPositionInSet,
}

#[deprecated]
pub type RTDSetStickerPositionInSetBuilder = SetStickerPositionInSetBuilder;

impl SetStickerPositionInSetBuilder {
    pub fn build(&self) -> SetStickerPositionInSet {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }

    pub fn position(&mut self, position: i32) -> &mut Self {
        self.inner.position = position;
        self
    }
}

impl AsRef<SetStickerPositionInSet> for SetStickerPositionInSet {
    fn as_ref(&self) -> &SetStickerPositionInSet {
        self
    }
}

impl AsRef<SetStickerPositionInSet> for SetStickerPositionInSetBuilder {
    fn as_ref(&self) -> &SetStickerPositionInSet {
        &self.inner
    }
}
