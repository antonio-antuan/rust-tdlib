
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes an animation file. The animation must be encoded in GIF or MPEG4 format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Animation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Duration of the animation, in seconds; as defined by the sender
  duration: i64,
  /// Width of the animation
  width: i64,
  /// Height of the animation
  height: i64,
  /// Original name of the file; as defined by the sender
  file_name: String,
  /// MIME type of the file, usually "image/gif" or "video/mp4"
  mime_type: String,
  /// Animation minithumbnail; may be null
  minithumbnail: Option<Minithumbnail>,
  /// Animation thumbnail; may be null
  thumbnail: Option<PhotoSize>,
  /// File containing the animation
  animation: File,
  
}

impl RObject for Animation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "animation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Animation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAnimationBuilder {
    let mut inner = Animation::default();
    inner.td_name = "animation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAnimationBuilder { inner }
  }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

  pub fn file_name(&self) -> &String { &self.file_name }

  pub fn mime_type(&self) -> &String { &self.mime_type }

  pub fn minithumbnail(&self) -> &Option<Minithumbnail> { &self.minithumbnail }

  pub fn thumbnail(&self) -> &Option<PhotoSize> { &self.thumbnail }

  pub fn animation(&self) -> &File { &self.animation }

}

#[doc(hidden)]
pub struct RTDAnimationBuilder {
  inner: Animation
}

impl RTDAnimationBuilder {
  pub fn build(&self) -> Animation { self.inner.clone() }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
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

   
  pub fn file_name<T: AsRef<str>>(&mut self, file_name: T) -> &mut Self {
    self.inner.file_name = file_name.as_ref().to_string();
    self
  }

   
  pub fn mime_type<T: AsRef<str>>(&mut self, mime_type: T) -> &mut Self {
    self.inner.mime_type = mime_type.as_ref().to_string();
    self
  }

   
  pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
    self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
    self
  }

   
  pub fn thumbnail<T: AsRef<PhotoSize>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

   
  pub fn animation<T: AsRef<File>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }

}

impl AsRef<Animation> for Animation {
  fn as_ref(&self) -> &Animation { self }
}

impl AsRef<Animation> for RTDAnimationBuilder {
  fn as_ref(&self) -> &Animation { &self.inner }
}



