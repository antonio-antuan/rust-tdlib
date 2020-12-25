
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a sticker that should be added to a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// PNG image with the sticker; must be up to 512 kB in size and fit in a 512x512 square
  png_sticker: InputFile,
  /// Emoji corresponding to the sticker
  emojis: String,
  /// For masks, position where the mask should be placed; may be null
  mask_position: Option<MaskPosition>,
  
}

impl RObject for InputSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputSticker" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl InputSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputStickerBuilder {
    let mut inner = InputSticker::default();
    inner.td_name = "inputSticker".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInputStickerBuilder { inner }
  }

  pub fn png_sticker(&self) -> &InputFile { &self.png_sticker }

  pub fn emojis(&self) -> &String { &self.emojis }

  pub fn mask_position(&self) -> &Option<MaskPosition> { &self.mask_position }

}

#[doc(hidden)]
pub struct RTDInputStickerBuilder {
  inner: InputSticker
}

impl RTDInputStickerBuilder {
  pub fn build(&self) -> InputSticker { self.inner.clone() }

   
  pub fn png_sticker<T: AsRef<InputFile>>(&mut self, png_sticker: T) -> &mut Self {
    self.inner.png_sticker = png_sticker.as_ref().clone();
    self
  }

   
  pub fn emojis<T: AsRef<str>>(&mut self, emojis: T) -> &mut Self {
    self.inner.emojis = emojis.as_ref().to_string();
    self
  }

   
  pub fn mask_position<T: AsRef<MaskPosition>>(&mut self, mask_position: T) -> &mut Self {
    self.inner.mask_position = Some(mask_position.as_ref().clone());
    self
  }

}

impl AsRef<InputSticker> for InputSticker {
  fn as_ref(&self) -> &InputSticker { self }
}

impl AsRef<InputSticker> for RTDInputStickerBuilder {
  fn as_ref(&self) -> &InputSticker { &self.inner }
}



