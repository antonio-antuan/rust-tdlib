
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Thumbnail image of a very poor quality and low resolution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Minithumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Thumbnail width, usually doesn't exceed 40
  width: i64,
  /// Thumbnail height, usually doesn't exceed 40
  height: i64,
  /// The thumbnail in JPEG format
  data: String,
  
}

impl RObject for Minithumbnail {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "minithumbnail" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Minithumbnail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMinithumbnailBuilder {
    let mut inner = Minithumbnail::default();
    inner.td_name = "minithumbnail".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMinithumbnailBuilder { inner }
  }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

  pub fn data(&self) -> &String { &self.data }

}

#[doc(hidden)]
pub struct RTDMinithumbnailBuilder {
  inner: Minithumbnail
}

impl RTDMinithumbnailBuilder {
  pub fn build(&self) -> Minithumbnail { self.inner.clone() }

   
  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

   
  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

}

impl AsRef<Minithumbnail> for Minithumbnail {
  fn as_ref(&self) -> &Minithumbnail { self }
}

impl AsRef<Minithumbnail> for RTDMinithumbnailBuilder {
  fn as_ref(&self) -> &Minithumbnail { &self.inner }
}



