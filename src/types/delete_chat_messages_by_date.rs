use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes all messages between the specified dates in a chat. Supported only for private chats and basic groups. Messages sent in the last 30 seconds will not be deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatMessagesByDate {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The minimum date of the messages to delete

    #[serde(default)]
    min_date: i32,
    /// The maximum date of the messages to delete

    #[serde(default)]
    max_date: i32,
    /// Pass true to delete chat messages for all users; private chats only

    #[serde(default)]
    revoke: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteChatMessagesByDate {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteChatMessagesByDate {}

impl DeleteChatMessagesByDate {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteChatMessagesByDateBuilder {
        let mut inner = DeleteChatMessagesByDate::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteChatMessagesByDate".to_string();

        DeleteChatMessagesByDateBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn min_date(&self) -> i32 {
        self.min_date
    }

    pub fn max_date(&self) -> i32 {
        self.max_date
    }

    pub fn revoke(&self) -> bool {
        self.revoke
    }
}

#[doc(hidden)]
pub struct DeleteChatMessagesByDateBuilder {
    inner: DeleteChatMessagesByDate,
}

#[deprecated]
pub type RTDDeleteChatMessagesByDateBuilder = DeleteChatMessagesByDateBuilder;

impl DeleteChatMessagesByDateBuilder {
    pub fn build(&self) -> DeleteChatMessagesByDate {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn min_date(&mut self, min_date: i32) -> &mut Self {
        self.inner.min_date = min_date;
        self
    }

    pub fn max_date(&mut self, max_date: i32) -> &mut Self {
        self.inner.max_date = max_date;
        self
    }

    pub fn revoke(&mut self, revoke: bool) -> &mut Self {
        self.inner.revoke = revoke;
        self
    }
}

impl AsRef<DeleteChatMessagesByDate> for DeleteChatMessagesByDate {
    fn as_ref(&self) -> &DeleteChatMessagesByDate {
        self
    }
}

impl AsRef<DeleteChatMessagesByDate> for DeleteChatMessagesByDateBuilder {
    fn as_ref(&self) -> &DeleteChatMessagesByDate {
        &self.inner
    }
}
