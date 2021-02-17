use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a message thread. Can be used only if message.can_get_message_thread == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageThread {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Identifier of the message
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageThread {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageThread {}

impl GetMessageThread {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetMessageThreadBuilder {
        let mut inner = GetMessageThread::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageThread".to_string();

        RTDGetMessageThreadBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct RTDGetMessageThreadBuilder {
    inner: GetMessageThread,
}

impl RTDGetMessageThreadBuilder {
    pub fn build(&self) -> GetMessageThread {
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
}

impl AsRef<GetMessageThread> for GetMessageThread {
    fn as_ref(&self) -> &GetMessageThread {
        self
    }
}

impl AsRef<GetMessageThread> for RTDGetMessageThreadBuilder {
    fn as_ref(&self) -> &GetMessageThread {
        &self.inner
    }
}
