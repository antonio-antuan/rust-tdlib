
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// A thumbnail to be sent along with a file; should be in JPEG or WEBP format for stickers, and less than 200 kB in size
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputThumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Thumbnail file to send. Sending thumbnails by file_id is currently not supported
  thumbnail: InputFile,
  /// Thumbnail width, usually shouldn't exceed 320. Use 0 if unknown
  width: i64,
  /// Thumbnail height, usually shouldn't exceed 320. Use 0 if unknown
  height: i64,
  
}

impl RObject for InputThumbnail {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputThumbnail" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl InputThumbnail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputThumbnailBuilder {
    let mut inner = InputThumbnail::default();
    inner.td_name = "inputThumbnail".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInputThumbnailBuilder { inner }
  }

  pub fn thumbnail(&self) -> &InputFile { &self.thumbnail }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

}

#[doc(hidden)]
pub struct RTDInputThumbnailBuilder {
  inner: InputThumbnail
}

impl RTDInputThumbnailBuilder {
  pub fn build(&self) -> InputThumbnail { self.inner.clone() }

   
  pub fn thumbnail<T: AsRef<InputFile>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
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

}

impl AsRef<InputThumbnail> for InputThumbnail {
  fn as_ref(&self) -> &InputThumbnail { self }
}

impl AsRef<InputThumbnail> for RTDInputThumbnailBuilder {
  fn as_ref(&self) -> &InputThumbnail { &self.inner }
}



