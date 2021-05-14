use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Stops a poll. A poll in a message can be stopped when the message has can_be_edited flag set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StopPoll {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the poll belongs
    chat_id: i64,
    /// Identifier of the message containing the poll
    message_id: i64,
    /// The new message reply markup; for bots only

    #[serde(skip_serializing_if = "ReplyMarkup::_is_default")]
    reply_markup: ReplyMarkup,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for StopPoll {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for StopPoll {}

impl StopPoll {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStopPollBuilder {
        let mut inner = StopPoll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "stopPoll".to_string();

        RTDStopPollBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }
}

#[doc(hidden)]
pub struct RTDStopPollBuilder {
    inner: StopPoll,
}

impl RTDStopPollBuilder {
    pub fn build(&self) -> StopPoll {
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

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }
}

impl AsRef<StopPoll> for StopPoll {
    fn as_ref(&self) -> &StopPoll {
        self
    }
}

impl AsRef<StopPoll> for RTDStopPollBuilder {
    fn as_ref(&self) -> &StopPoll {
        &self.inner
    }
}
