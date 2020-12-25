
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of game high scores
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameHighScores {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A list of game high scores
  scores: Vec<GameHighScore>,
  
}

impl RObject for GameHighScores {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "gameHighScores" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GameHighScores {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGameHighScoresBuilder {
    let mut inner = GameHighScores::default();
    inner.td_name = "gameHighScores".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDGameHighScoresBuilder { inner }
  }

  pub fn scores(&self) -> &Vec<GameHighScore> { &self.scores }

}

#[doc(hidden)]
pub struct RTDGameHighScoresBuilder {
  inner: GameHighScores
}

impl RTDGameHighScoresBuilder {
  pub fn build(&self) -> GameHighScores { self.inner.clone() }

   
  pub fn scores(&mut self, scores: Vec<GameHighScore>) -> &mut Self {
    self.inner.scores = scores;
    self
  }

}

impl AsRef<GameHighScores> for GameHighScores {
  fn as_ref(&self) -> &GameHighScores { self }
}

impl AsRef<GameHighScores> for RTDGameHighScoresBuilder {
  fn as_ref(&self) -> &GameHighScores { &self.inner }
}



