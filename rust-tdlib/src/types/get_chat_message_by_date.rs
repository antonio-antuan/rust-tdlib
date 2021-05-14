use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the last message sent in a chat no later than the specified date
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatMessageByDate {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Point in time (Unix timestamp) relative to which to search for messages
    date: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatMessageByDate {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatMessageByDate {}

impl GetChatMessageByDate {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetChatMessageByDateBuilder {
        let mut inner = GetChatMessageByDate::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatMessageByDate".to_string();

        RTDGetChatMessageByDateBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn date(&self) -> i32 {
        self.date
    }
}

#[doc(hidden)]
pub struct RTDGetChatMessageByDateBuilder {
    inner: GetChatMessageByDate,
}

impl RTDGetChatMessageByDateBuilder {
    pub fn build(&self) -> GetChatMessageByDate {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }
}

impl AsRef<GetChatMessageByDate> for GetChatMessageByDate {
    fn as_ref(&self) -> &GetChatMessageByDate {
        self
    }
}

impl AsRef<GetChatMessageByDate> for RTDGetChatMessageByDateBuilder {
    fn as_ref(&self) -> &GetChatMessageByDate {
        &self.inner
    }
}
