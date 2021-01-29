use crate::types::*;
use crate::errors::*;
use uuid::Uuid;


/// Describes an image in JPEG format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhotoSize {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Image type (see https://core.telegram.org/constructor/photoSize)
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: String,
  /// Information about the image file
  photo: File,
  /// Image width
  width: i64,
  /// Image height
  height: i64,
  /// Sizes of progressive JPEG file prefixes, which can be used to preliminarily show the image
  progressive_sizes: Vec<i64>,
  
}

impl RObject for PhotoSize {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "photoSize" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PhotoSize {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPhotoSizeBuilder {
    let mut inner = PhotoSize::default();
    inner.td_name = "photoSize".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPhotoSizeBuilder { inner }
  }

  pub fn type_(&self) -> &String { &self.type_ }

  pub fn photo(&self) -> &File { &self.photo }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

  pub fn progressive_sizes(&self) -> &Vec<i64> { &self.progressive_sizes }

}

#[doc(hidden)]
pub struct RTDPhotoSizeBuilder {
  inner: PhotoSize
}

impl RTDPhotoSizeBuilder {
  pub fn build(&self) -> PhotoSize { self.inner.clone() }

   
  pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().to_string();
    self
  }

   
  pub fn photo<T: AsRef<File>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

   
  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

   
  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

   
  pub fn progressive_sizes(&mut self, progressive_sizes: Vec<i64>) -> &mut Self {
    self.inner.progressive_sizes = progressive_sizes;
    self
  }

}

impl AsRef<PhotoSize> for PhotoSize {
  fn as_ref(&self) -> &PhotoSize { self }
}

impl AsRef<PhotoSize> for RTDPhotoSizeBuilder {
  fn as_ref(&self) -> &PhotoSize { &self.inner }
}



