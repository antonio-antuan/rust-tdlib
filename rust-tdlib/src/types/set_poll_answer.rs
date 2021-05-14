use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the user answer to a poll. A poll in quiz mode can be answered only once
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPollAnswer {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the poll belongs
    chat_id: i64,
    /// Identifier of the message containing the poll
    message_id: i64,
    /// 0-based identifiers of answer options, chosen by the user. User can choose more than 1 answer option only is the poll allows multiple answers
    option_ids: Vec<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetPollAnswer {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetPollAnswer {}

impl SetPollAnswer {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetPollAnswerBuilder {
        let mut inner = SetPollAnswer::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setPollAnswer".to_string();

        RTDSetPollAnswerBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn option_ids(&self) -> &Vec<i32> {
        &self.option_ids
    }
}

#[doc(hidden)]
pub struct RTDSetPollAnswerBuilder {
    inner: SetPollAnswer,
}

impl RTDSetPollAnswerBuilder {
    pub fn build(&self) -> SetPollAnswer {
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

    pub fn option_ids(&mut self, option_ids: Vec<i32>) -> &mut Self {
        self.inner.option_ids = option_ids;
        self
    }
}

impl AsRef<SetPollAnswer> for SetPollAnswer {
    fn as_ref(&self) -> &SetPollAnswer {
        self
    }
}

impl AsRef<SetPollAnswer> for RTDSetPollAnswerBuilder {
    fn as_ref(&self) -> &SetPollAnswer {
        &self.inner
    }
}
