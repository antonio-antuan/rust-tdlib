use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the mask position of a mask sticker; for bots only. The sticker must belong to a mask sticker set created by the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetStickerMaskPosition {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,
    /// Position where the mask is placed; pass null to remove mask position
    mask_position: MaskPosition,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetStickerMaskPosition {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetStickerMaskPosition {}

impl SetStickerMaskPosition {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetStickerMaskPositionBuilder {
        let mut inner = SetStickerMaskPosition::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setStickerMaskPosition".to_string();

        SetStickerMaskPositionBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }

    pub fn mask_position(&self) -> &MaskPosition {
        &self.mask_position
    }
}

#[doc(hidden)]
pub struct SetStickerMaskPositionBuilder {
    inner: SetStickerMaskPosition,
}

#[deprecated]
pub type RTDSetStickerMaskPositionBuilder = SetStickerMaskPositionBuilder;

impl SetStickerMaskPositionBuilder {
    pub fn build(&self) -> SetStickerMaskPosition {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }

    pub fn mask_position<T: AsRef<MaskPosition>>(&mut self, mask_position: T) -> &mut Self {
        self.inner.mask_position = mask_position.as_ref().clone();
        self
    }
}

impl AsRef<SetStickerMaskPosition> for SetStickerMaskPosition {
    fn as_ref(&self) -> &SetStickerMaskPosition {
        self
    }
}

impl AsRef<SetStickerMaskPosition> for SetStickerMaskPositionBuilder {
    fn as_ref(&self) -> &SetStickerMaskPosition {
        &self.inner
    }
}
