use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a message with the callback button that originated a callback query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCallbackQueryMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat the message belongs to
    chat_id: i64,
    /// Message identifier
    message_id: i64,
    /// Identifier of the callback query

    #[serde(deserialize_with = "super::_common::number_from_string")]
    callback_query_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetCallbackQueryMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetCallbackQueryMessage {}

impl GetCallbackQueryMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetCallbackQueryMessageBuilder {
        let mut inner = GetCallbackQueryMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getCallbackQueryMessage".to_string();

        RTDGetCallbackQueryMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn callback_query_id(&self) -> i64 {
        self.callback_query_id
    }
}

#[doc(hidden)]
pub struct RTDGetCallbackQueryMessageBuilder {
    inner: GetCallbackQueryMessage,
}

impl RTDGetCallbackQueryMessageBuilder {
    pub fn build(&self) -> GetCallbackQueryMessage {
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

    pub fn callback_query_id(&mut self, callback_query_id: i64) -> &mut Self {
        self.inner.callback_query_id = callback_query_id;
        self
    }
}

impl AsRef<GetCallbackQueryMessage> for GetCallbackQueryMessage {
    fn as_ref(&self) -> &GetCallbackQueryMessage {
        self
    }
}

impl AsRef<GetCallbackQueryMessage> for RTDGetCallbackQueryMessageBuilder {
    fn as_ref(&self) -> &GetCallbackQueryMessage {
        &self.inner
    }
}
