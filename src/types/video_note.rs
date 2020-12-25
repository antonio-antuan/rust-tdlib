
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a video note. The video must be equal in width and height, cropped to a circle, and stored in MPEG4 format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Duration of the video, in seconds; as defined by the sender
  duration: i64,
  /// Video width and height; as defined by the sender
  length: i64,
  /// Video minithumbnail; may be null
  minithumbnail: Option<Minithumbnail>,
  /// Video thumbnail; as defined by the sender; may be null
  thumbnail: Option<PhotoSize>,
  /// File containing the video
  video: File,
  
}

impl RObject for VideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "videoNote" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl VideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDVideoNoteBuilder {
    let mut inner = VideoNote::default();
    inner.td_name = "videoNote".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDVideoNoteBuilder { inner }
  }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn length(&self) -> i64 { self.length }

  pub fn minithumbnail(&self) -> &Option<Minithumbnail> { &self.minithumbnail }

  pub fn thumbnail(&self) -> &Option<PhotoSize> { &self.thumbnail }

  pub fn video(&self) -> &File { &self.video }

}

#[doc(hidden)]
pub struct RTDVideoNoteBuilder {
  inner: VideoNote
}

impl RTDVideoNoteBuilder {
  pub fn build(&self) -> VideoNote { self.inner.clone() }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

   
  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
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

   
  pub fn video<T: AsRef<File>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

}

impl AsRef<VideoNote> for VideoNote {
  fn as_ref(&self) -> &VideoNote { self }
}

impl AsRef<VideoNote> for RTDVideoNoteBuilder {
  fn as_ref(&self) -> &VideoNote { &self.inner }
}



