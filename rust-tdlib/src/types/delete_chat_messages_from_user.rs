use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes all messages sent by the specified user to a chat. Supported only for supergroups; requires can_delete_messages administrator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatMessagesFromUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// User identifier
    user_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteChatMessagesFromUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteChatMessagesFromUser {}

impl DeleteChatMessagesFromUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteChatMessagesFromUserBuilder {
        let mut inner = DeleteChatMessagesFromUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteChatMessagesFromUser".to_string();

        RTDDeleteChatMessagesFromUserBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDDeleteChatMessagesFromUserBuilder {
    inner: DeleteChatMessagesFromUser,
}

impl RTDDeleteChatMessagesFromUserBuilder {
    pub fn build(&self) -> DeleteChatMessagesFromUser {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<DeleteChatMessagesFromUser> for DeleteChatMessagesFromUser {
    fn as_ref(&self) -> &DeleteChatMessagesFromUser {
        self
    }
}

impl AsRef<DeleteChatMessagesFromUser> for RTDDeleteChatMessagesFromUserBuilder {
    fn as_ref(&self) -> &DeleteChatMessagesFromUser {
        &self.inner
    }
}
