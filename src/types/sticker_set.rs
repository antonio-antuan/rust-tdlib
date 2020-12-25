
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the sticker set
  id: isize,
  /// Title of the sticker set
  title: String,
  /// Name of the sticker set
  name: String,
  /// Sticker set thumbnail in WEBP format with width and height 100; may be null. The file can be downloaded only before the thumbnail is changed
  thumbnail: Option<PhotoSize>,
  /// True, if the sticker set has been installed by the current user
  is_installed: bool,
  /// True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously
  is_archived: bool,
  /// True, if the sticker set is official
  is_official: bool,
  /// True, is the stickers in the set are animated
  is_animated: bool,
  /// True, if the stickers in the set are masks
  is_masks: bool,
  /// True for already viewed trending sticker sets
  is_viewed: bool,
  /// List of stickers in this set
  stickers: Vec<Sticker>,
  /// A list of emoji corresponding to the stickers in the same order. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
  emojis: Vec<Emojis>,
  
}

impl RObject for StickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickerSet" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl StickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStickerSetBuilder {
    let mut inner = StickerSet::default();
    inner.td_name = "stickerSet".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDStickerSetBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn title(&self) -> &String { &self.title }

  pub fn name(&self) -> &String { &self.name }

  pub fn thumbnail(&self) -> &Option<PhotoSize> { &self.thumbnail }

  pub fn is_installed(&self) -> bool { self.is_installed }

  pub fn is_archived(&self) -> bool { self.is_archived }

  pub fn is_official(&self) -> bool { self.is_official }

  pub fn is_animated(&self) -> bool { self.is_animated }

  pub fn is_masks(&self) -> bool { self.is_masks }

  pub fn is_viewed(&self) -> bool { self.is_viewed }

  pub fn stickers(&self) -> &Vec<Sticker> { &self.stickers }

  pub fn emojis(&self) -> &Vec<Emojis> { &self.emojis }

}

#[doc(hidden)]
pub struct RTDStickerSetBuilder {
  inner: StickerSet
}

impl RTDStickerSetBuilder {
  pub fn build(&self) -> StickerSet { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn thumbnail<T: AsRef<PhotoSize>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

   
  pub fn is_installed(&mut self, is_installed: bool) -> &mut Self {
    self.inner.is_installed = is_installed;
    self
  }

   
  pub fn is_archived(&mut self, is_archived: bool) -> &mut Self {
    self.inner.is_archived = is_archived;
    self
  }

   
  pub fn is_official(&mut self, is_official: bool) -> &mut Self {
    self.inner.is_official = is_official;
    self
  }

   
  pub fn is_animated(&mut self, is_animated: bool) -> &mut Self {
    self.inner.is_animated = is_animated;
    self
  }

   
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.is_masks = is_masks;
    self
  }

   
  pub fn is_viewed(&mut self, is_viewed: bool) -> &mut Self {
    self.inner.is_viewed = is_viewed;
    self
  }

   
  pub fn stickers(&mut self, stickers: Vec<Sticker>) -> &mut Self {
    self.inner.stickers = stickers;
    self
  }

   
  pub fn emojis(&mut self, emojis: Vec<Emojis>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }

}

impl AsRef<StickerSet> for StickerSet {
  fn as_ref(&self) -> &StickerSet { self }
}

impl AsRef<StickerSet> for RTDStickerSetBuilder {
  fn as_ref(&self) -> &StickerSet { &self.inner }
}



