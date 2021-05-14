use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about messages. If a message is not found, returns null on the corresponding position of the result
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat the messages belong to
    chat_id: i64,
    /// Identifiers of the messages to get
    message_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessages {}

impl GetMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetMessagesBuilder {
        let mut inner = GetMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessages".to_string();

        RTDGetMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }
}

#[doc(hidden)]
pub struct RTDGetMessagesBuilder {
    inner: GetMessages,
}

impl RTDGetMessagesBuilder {
    pub fn build(&self) -> GetMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
        self.inner.message_ids = message_ids;
        self
    }
}

impl AsRef<GetMessages> for GetMessages {
    fn as_ref(&self) -> &GetMessages {
        self
    }
}

impl AsRef<GetMessages> for RTDGetMessagesBuilder {
    fn as_ref(&self) -> &GetMessages {
        &self.inner
    }
}
