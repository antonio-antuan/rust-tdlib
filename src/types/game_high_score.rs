use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains one row of the game high score table
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameHighScore {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Position in the high score table
    position: i32,
    /// User identifier
    user_id: i32,
    /// User score
    score: i32,
}

impl RObject for GameHighScore {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "gameHighScore"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl GameHighScore {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGameHighScoreBuilder {
        let mut inner = GameHighScore::default();
        inner.td_name = "gameHighScore".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDGameHighScoreBuilder { inner }
    }

    pub fn position(&self) -> i32 {
        self.position
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

#[doc(hidden)]
pub struct RTDGameHighScoreBuilder {
    inner: GameHighScore,
}

impl RTDGameHighScoreBuilder {
    pub fn build(&self) -> GameHighScore {
        self.inner.clone()
    }

    pub fn position(&mut self, position: i32) -> &mut Self {
        self.inner.position = position;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn score(&mut self, score: i32) -> &mut Self {
        self.inner.score = score;
        self
    }
}

impl AsRef<GameHighScore> for GameHighScore {
    fn as_ref(&self) -> &GameHighScore {
        self
    }
}

impl AsRef<GameHighScore> for RTDGameHighScoreBuilder {
    fn as_ref(&self) -> &GameHighScore {
        &self.inner
    }
}
