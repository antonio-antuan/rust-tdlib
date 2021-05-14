use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a ForceReply reply markup has been used. UpdateChatReplyMarkup will be sent if the reply markup will be changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatReplyMarkup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// The message identifier of the used keyboard
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteChatReplyMarkup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteChatReplyMarkup {}

impl DeleteChatReplyMarkup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteChatReplyMarkupBuilder {
        let mut inner = DeleteChatReplyMarkup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteChatReplyMarkup".to_string();

        RTDDeleteChatReplyMarkupBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct RTDDeleteChatReplyMarkupBuilder {
    inner: DeleteChatReplyMarkup,
}

impl RTDDeleteChatReplyMarkupBuilder {
    pub fn build(&self) -> DeleteChatReplyMarkup {
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

impl AsRef<DeleteChatReplyMarkup> for DeleteChatReplyMarkup {
    fn as_ref(&self) -> &DeleteChatReplyMarkup {
        self
    }
}

impl AsRef<DeleteChatReplyMarkup> for RTDDeleteChatReplyMarkupBuilder {
    fn as_ref(&self) -> &DeleteChatReplyMarkup {
        &self.inner
    }
}
