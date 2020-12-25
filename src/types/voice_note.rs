
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a voice note. The voice note must be encoded with the Opus codec, and stored inside an OGG container. Voice notes can have only a single audio channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Duration of the voice note, in seconds; as defined by the sender
  duration: i64,
  /// A waveform representation of the voice note in 5-bit format
  waveform: String,
  /// MIME type of the file; as defined by the sender
  mime_type: String,
  /// File containing the voice note
  voice: File,
  
}

impl RObject for VoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "voiceNote" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl VoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDVoiceNoteBuilder {
    let mut inner = VoiceNote::default();
    inner.td_name = "voiceNote".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDVoiceNoteBuilder { inner }
  }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn waveform(&self) -> &String { &self.waveform }

  pub fn mime_type(&self) -> &String { &self.mime_type }

  pub fn voice(&self) -> &File { &self.voice }

}

#[doc(hidden)]
pub struct RTDVoiceNoteBuilder {
  inner: VoiceNote
}

impl RTDVoiceNoteBuilder {
  pub fn build(&self) -> VoiceNote { self.inner.clone() }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

   
  pub fn waveform<T: AsRef<str>>(&mut self, waveform: T) -> &mut Self {
    self.inner.waveform = waveform.as_ref().to_string();
    self
  }

   
  pub fn mime_type<T: AsRef<str>>(&mut self, mime_type: T) -> &mut Self {
    self.inner.mime_type = mime_type.as_ref().to_string();
    self
  }

   
  pub fn voice<T: AsRef<File>>(&mut self, voice: T) -> &mut Self {
    self.inner.voice = voice.as_ref().clone();
    self
  }

}

impl AsRef<VoiceNote> for VoiceNote {
  fn as_ref(&self) -> &VoiceNote { self }
}

impl AsRef<VoiceNote> for RTDVoiceNoteBuilder {
  fn as_ref(&self) -> &VoiceNote { &self.inner }
}



