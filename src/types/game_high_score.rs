use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains one row of the game high score table
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameHighScore {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Position in the high score table

    #[serde(default)]
    position: i32,
    /// User identifier

    #[serde(default)]
    user_id: i64,
    /// User score

    #[serde(default)]
    score: i32,
}

impl RObject for GameHighScore {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl GameHighScore {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GameHighScoreBuilder {
        let mut inner = GameHighScore::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GameHighScoreBuilder { inner }
    }

    pub fn position(&self) -> i32 {
        self.position
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

#[doc(hidden)]
pub struct GameHighScoreBuilder {
    inner: GameHighScore,
}

#[deprecated]
pub type RTDGameHighScoreBuilder = GameHighScoreBuilder;

impl GameHighScoreBuilder {
    pub fn build(&self) -> GameHighScore {
        self.inner.clone()
    }

    pub fn position(&mut self, position: i32) -> &mut Self {
        self.inner.position = position;
        self
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
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

impl AsRef<GameHighScore> for GameHighScoreBuilder {
    fn as_ref(&self) -> &GameHighScore {
        &self.inner
    }
}
