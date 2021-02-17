use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Updates the game score of the specified user in the game; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetGameScore {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat to which the message with the game belongs
    chat_id: i64,
    /// Identifier of the message
    message_id: i64,
    /// True, if the message should be edited
    edit_message: bool,
    /// User identifier
    user_id: i32,
    /// The new score
    score: i32,
    /// Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table
    force: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetGameScore {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetGameScore {}

impl SetGameScore {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetGameScoreBuilder {
        let mut inner = SetGameScore::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setGameScore".to_string();

        RTDSetGameScoreBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn edit_message(&self) -> bool {
        self.edit_message
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn force(&self) -> bool {
        self.force
    }
}

#[doc(hidden)]
pub struct RTDSetGameScoreBuilder {
    inner: SetGameScore,
}

impl RTDSetGameScoreBuilder {
    pub fn build(&self) -> SetGameScore {
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

    pub fn edit_message(&mut self, edit_message: bool) -> &mut Self {
        self.inner.edit_message = edit_message;
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

    pub fn force(&mut self, force: bool) -> &mut Self {
        self.inner.force = force;
        self
    }
}

impl AsRef<SetGameScore> for SetGameScore {
    fn as_ref(&self) -> &SetGameScore {
        self
    }
}

impl AsRef<SetGameScore> for RTDSetGameScoreBuilder {
    fn as_ref(&self) -> &SetGameScore {
        &self.inner
    }
}
