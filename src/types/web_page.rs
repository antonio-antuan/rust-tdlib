
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a web page preview
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebPage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Original URL of the link
  url: String,
  /// URL to display
  display_url: String,
  /// Type of the web page. Can be: article, photo, audio, video, document, profile, app, or something else
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: String,
  /// Short name of the site (e.g., Google Docs, App Store)
  site_name: String,
  /// Title of the content
  title: String,
  /// Describes a web page preview
  description: String,
  /// Image representing the content; may be null
  photo: Option<Photo>,
  /// URL to show in the embedded preview
  embed_url: String,
  /// MIME type of the embedded preview, (e.g., text/html or video/mp4)
  embed_type: String,
  /// Width of the embedded preview
  embed_width: i64,
  /// Height of the embedded preview
  embed_height: i64,
  /// Duration of the content, in seconds
  duration: i64,
  /// Author of the content
  author: String,
  /// Preview of the content as an animation, if available; may be null
  animation: Option<Animation>,
  /// Preview of the content as an audio file, if available; may be null
  audio: Option<Audio>,
  /// Preview of the content as a document, if available (currently only available for small PDF files and ZIP archives); may be null
  document: Option<Document>,
  /// Preview of the content as a sticker for small WEBP files, if available; may be null
  sticker: Option<Sticker>,
  /// Preview of the content as a video, if available; may be null
  video: Option<Video>,
  /// Preview of the content as a video note, if available; may be null
  video_note: Option<VideoNote>,
  /// Preview of the content as a voice note, if available; may be null
  voice_note: Option<VoiceNote>,
  /// Version of instant view, available for the web page (currently can be 1 or 2), 0 if none
  instant_view_version: i64,
  
}

impl RObject for WebPage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "webPage" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl WebPage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDWebPageBuilder {
    let mut inner = WebPage::default();
    inner.td_name = "webPage".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDWebPageBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

  pub fn display_url(&self) -> &String { &self.display_url }

  pub fn type_(&self) -> &String { &self.type_ }

  pub fn site_name(&self) -> &String { &self.site_name }

  pub fn title(&self) -> &String { &self.title }

  pub fn description(&self) -> &String { &self.description }

  pub fn photo(&self) -> &Option<Photo> { &self.photo }

  pub fn embed_url(&self) -> &String { &self.embed_url }

  pub fn embed_type(&self) -> &String { &self.embed_type }

  pub fn embed_width(&self) -> i64 { self.embed_width }

  pub fn embed_height(&self) -> i64 { self.embed_height }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn author(&self) -> &String { &self.author }

  pub fn animation(&self) -> &Option<Animation> { &self.animation }

  pub fn audio(&self) -> &Option<Audio> { &self.audio }

  pub fn document(&self) -> &Option<Document> { &self.document }

  pub fn sticker(&self) -> &Option<Sticker> { &self.sticker }

  pub fn video(&self) -> &Option<Video> { &self.video }

  pub fn video_note(&self) -> &Option<VideoNote> { &self.video_note }

  pub fn voice_note(&self) -> &Option<VoiceNote> { &self.voice_note }

  pub fn instant_view_version(&self) -> i64 { self.instant_view_version }

}

#[doc(hidden)]
pub struct RTDWebPageBuilder {
  inner: WebPage
}

impl RTDWebPageBuilder {
  pub fn build(&self) -> WebPage { self.inner.clone() }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn display_url<T: AsRef<str>>(&mut self, display_url: T) -> &mut Self {
    self.inner.display_url = display_url.as_ref().to_string();
    self
  }

   
  pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().to_string();
    self
  }

   
  pub fn site_name<T: AsRef<str>>(&mut self, site_name: T) -> &mut Self {
    self.inner.site_name = site_name.as_ref().to_string();
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

   
  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

   
  pub fn embed_url<T: AsRef<str>>(&mut self, embed_url: T) -> &mut Self {
    self.inner.embed_url = embed_url.as_ref().to_string();
    self
  }

   
  pub fn embed_type<T: AsRef<str>>(&mut self, embed_type: T) -> &mut Self {
    self.inner.embed_type = embed_type.as_ref().to_string();
    self
  }

   
  pub fn embed_width(&mut self, embed_width: i64) -> &mut Self {
    self.inner.embed_width = embed_width;
    self
  }

   
  pub fn embed_height(&mut self, embed_height: i64) -> &mut Self {
    self.inner.embed_height = embed_height;
    self
  }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

   
  pub fn author<T: AsRef<str>>(&mut self, author: T) -> &mut Self {
    self.inner.author = author.as_ref().to_string();
    self
  }

   
  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = Some(animation.as_ref().clone());
    self
  }

   
  pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = Some(audio.as_ref().clone());
    self
  }

   
  pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
    self.inner.document = Some(document.as_ref().clone());
    self
  }

   
  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = Some(sticker.as_ref().clone());
    self
  }

   
  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = Some(video.as_ref().clone());
    self
  }

   
  pub fn video_note<T: AsRef<VideoNote>>(&mut self, video_note: T) -> &mut Self {
    self.inner.video_note = Some(video_note.as_ref().clone());
    self
  }

   
  pub fn voice_note<T: AsRef<VoiceNote>>(&mut self, voice_note: T) -> &mut Self {
    self.inner.voice_note = Some(voice_note.as_ref().clone());
    self
  }

   
  pub fn instant_view_version(&mut self, instant_view_version: i64) -> &mut Self {
    self.inner.instant_view_version = instant_view_version;
    self
  }

}

impl AsRef<WebPage> for WebPage {
  fn as_ref(&self) -> &WebPage { self }
}

impl AsRef<WebPage> for RTDWebPageBuilder {
  fn as_ref(&self) -> &WebPage { &self.inner }
}



