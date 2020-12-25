
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes an audio file. Audio is usually in MP3 or M4A format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Audio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Duration of the audio, in seconds; as defined by the sender
  duration: i64,
  /// Title of the audio; as defined by the sender
  title: String,
  /// Performer of the audio; as defined by the sender
  performer: String,
  /// Original name of the file; as defined by the sender
  file_name: String,
  /// The MIME type of the file; as defined by the sender
  mime_type: String,
  /// The minithumbnail of the album cover; may be null
  album_cover_minithumbnail: Option<Minithumbnail>,
  /// The thumbnail of the album cover; as defined by the sender. The full size thumbnail should be extracted from the downloaded file; may be null
  album_cover_thumbnail: Option<PhotoSize>,
  /// File containing the audio
  audio: File,
  
}

impl RObject for Audio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "audio" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Audio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAudioBuilder {
    let mut inner = Audio::default();
    inner.td_name = "audio".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAudioBuilder { inner }
  }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn title(&self) -> &String { &self.title }

  pub fn performer(&self) -> &String { &self.performer }

  pub fn file_name(&self) -> &String { &self.file_name }

  pub fn mime_type(&self) -> &String { &self.mime_type }

  pub fn album_cover_minithumbnail(&self) -> &Option<Minithumbnail> { &self.album_cover_minithumbnail }

  pub fn album_cover_thumbnail(&self) -> &Option<PhotoSize> { &self.album_cover_thumbnail }

  pub fn audio(&self) -> &File { &self.audio }

}

#[doc(hidden)]
pub struct RTDAudioBuilder {
  inner: Audio
}

impl RTDAudioBuilder {
  pub fn build(&self) -> Audio { self.inner.clone() }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn performer<T: AsRef<str>>(&mut self, performer: T) -> &mut Self {
    self.inner.performer = performer.as_ref().to_string();
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

   
  pub fn album_cover_minithumbnail<T: AsRef<Minithumbnail>>(&mut self, album_cover_minithumbnail: T) -> &mut Self {
    self.inner.album_cover_minithumbnail = Some(album_cover_minithumbnail.as_ref().clone());
    self
  }

   
  pub fn album_cover_thumbnail<T: AsRef<PhotoSize>>(&mut self, album_cover_thumbnail: T) -> &mut Self {
    self.inner.album_cover_thumbnail = Some(album_cover_thumbnail.as_ref().clone());
    self
  }

   
  pub fn audio<T: AsRef<File>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = audio.as_ref().clone();
    self
  }

}

impl AsRef<Audio> for Audio {
  fn as_ref(&self) -> &Audio { self }
}

impl AsRef<Audio> for RTDAudioBuilder {
  fn as_ref(&self) -> &Audio { &self.inner }
}



