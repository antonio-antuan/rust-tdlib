
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Photo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if stickers were added to the photo
  has_stickers: bool,
  /// Photo minithumbnail; may be null
  minithumbnail: Option<Minithumbnail>,
  /// Available variants of the photo, in different sizes
  sizes: Vec<PhotoSize>,
  
}

impl RObject for Photo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "photo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Photo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPhotoBuilder {
    let mut inner = Photo::default();
    inner.td_name = "photo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPhotoBuilder { inner }
  }

  pub fn has_stickers(&self) -> bool { self.has_stickers }

  pub fn minithumbnail(&self) -> &Option<Minithumbnail> { &self.minithumbnail }

  pub fn sizes(&self) -> &Vec<PhotoSize> { &self.sizes }

}

#[doc(hidden)]
pub struct RTDPhotoBuilder {
  inner: Photo
}

impl RTDPhotoBuilder {
  pub fn build(&self) -> Photo { self.inner.clone() }

   
  pub fn has_stickers(&mut self, has_stickers: bool) -> &mut Self {
    self.inner.has_stickers = has_stickers;
    self
  }

   
  pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
    self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
    self
  }

   
  pub fn sizes(&mut self, sizes: Vec<PhotoSize>) -> &mut Self {
    self.inner.sizes = sizes;
    self
  }

}

impl AsRef<Photo> for Photo {
  fn as_ref(&self) -> &Photo { self }
}

impl AsRef<Photo> for RTDPhotoBuilder {
  fn as_ref(&self) -> &Photo { &self.inner }
}



