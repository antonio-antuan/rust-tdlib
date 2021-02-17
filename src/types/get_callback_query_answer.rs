use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCallbackQueryAnswer {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat with the message
    chat_id: i64,
    /// Identifier of the message from which the query originated
    message_id: i64,
    /// Query payload

    #[serde(skip_serializing_if = "CallbackQueryPayload::_is_default")]
    payload: CallbackQueryPayload,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetCallbackQueryAnswer {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetCallbackQueryAnswer {}

impl GetCallbackQueryAnswer {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetCallbackQueryAnswerBuilder {
        let mut inner = GetCallbackQueryAnswer::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getCallbackQueryAnswer".to_string();

        RTDGetCallbackQueryAnswerBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn payload(&self) -> &CallbackQueryPayload {
        &self.payload
    }
}

#[doc(hidden)]
pub struct RTDGetCallbackQueryAnswerBuilder {
    inner: GetCallbackQueryAnswer,
}

impl RTDGetCallbackQueryAnswerBuilder {
    pub fn build(&self) -> GetCallbackQueryAnswer {
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

    pub fn payload<T: AsRef<CallbackQueryPayload>>(&mut self, payload: T) -> &mut Self {
        self.inner.payload = payload.as_ref().clone();
        self
    }
}

impl AsRef<GetCallbackQueryAnswer> for GetCallbackQueryAnswer {
    fn as_ref(&self) -> &GetCallbackQueryAnswer {
        self
    }
}

impl AsRef<GetCallbackQueryAnswer> for RTDGetCallbackQueryAnswerBuilder {
    fn as_ref(&self) -> &GetCallbackQueryAnswer {
        &self.inner
    }
}
