use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGameHighScores {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat that contains the message with the game

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message

    #[serde(default)]
    message_id: i64,
    /// User identifier

    #[serde(default)]
    user_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetGameHighScores {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetGameHighScores {}

impl GetGameHighScores {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetGameHighScoresBuilder {
        let mut inner = GetGameHighScores::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getGameHighScores".to_string();

        GetGameHighScoresBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct GetGameHighScoresBuilder {
    inner: GetGameHighScores,
}

#[deprecated]
pub type RTDGetGameHighScoresBuilder = GetGameHighScoresBuilder;

impl GetGameHighScoresBuilder {
    pub fn build(&self) -> GetGameHighScores {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<GetGameHighScores> for GetGameHighScores {
    fn as_ref(&self) -> &GetGameHighScores {
        self
    }
}

impl AsRef<GetGameHighScores> for GetGameHighScoresBuilder {
    fn as_ref(&self) -> &GetGameHighScores {
        &self.inner
    }
}
