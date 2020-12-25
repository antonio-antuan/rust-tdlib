
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Game {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Game ID
  id: isize,
  /// Game short name. To share a game use the URL https://t.me/{bot_username}?game={game_short_name}
  short_name: String,
  /// Game title
  title: String,
  /// Game text, usually containing scoreboards for a game
  text: FormattedText,
  /// Describes a game
  description: String,
  /// Game photo
  photo: Photo,
  /// Game animation; may be null
  animation: Option<Animation>,
  
}

impl RObject for Game {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "game" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Game {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGameBuilder {
    let mut inner = Game::default();
    inner.td_name = "game".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGameBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn short_name(&self) -> &String { &self.short_name }

  pub fn title(&self) -> &String { &self.title }

  pub fn text(&self) -> &FormattedText { &self.text }

  pub fn description(&self) -> &String { &self.description }

  pub fn photo(&self) -> &Photo { &self.photo }

  pub fn animation(&self) -> &Option<Animation> { &self.animation }

}

#[doc(hidden)]
pub struct RTDGameBuilder {
  inner: Game
}

impl RTDGameBuilder {
  pub fn build(&self) -> Game { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn short_name<T: AsRef<str>>(&mut self, short_name: T) -> &mut Self {
    self.inner.short_name = short_name.as_ref().to_string();
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

   
  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

   
  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = Some(animation.as_ref().clone());
    self
  }

}

impl AsRef<Game> for Game {
  fn as_ref(&self) -> &Game { self }
}

impl AsRef<Game> for RTDGameBuilder {
  fn as_ref(&self) -> &Game { &self.inner }
}



