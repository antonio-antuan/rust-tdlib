use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of game high scores
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameHighScores {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A list of game high scores

    #[serde(default)]
    scores: Vec<GameHighScore>,
}

impl RObject for GameHighScores {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl GameHighScores {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GameHighScoresBuilder {
        let mut inner = GameHighScores::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GameHighScoresBuilder { inner }
    }

    pub fn scores(&self) -> &Vec<GameHighScore> {
        &self.scores
    }
}

#[doc(hidden)]
pub struct GameHighScoresBuilder {
    inner: GameHighScores,
}

#[deprecated]
pub type RTDGameHighScoresBuilder = GameHighScoresBuilder;

impl GameHighScoresBuilder {
    pub fn build(&self) -> GameHighScores {
        self.inner.clone()
    }

    pub fn scores(&mut self, scores: Vec<GameHighScore>) -> &mut Self {
        self.inner.scores = scores;
        self
    }
}

impl AsRef<GameHighScores> for GameHighScores {
    fn as_ref(&self) -> &GameHighScores {
        self
    }
}

impl AsRef<GameHighScores> for GameHighScoresBuilder {
    fn as_ref(&self) -> &GameHighScores {
        &self.inner
    }
}
