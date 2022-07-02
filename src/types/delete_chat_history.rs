use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes all messages in the chat. Use chat.can_be_deleted_only_for_self and chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteChatHistory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Pass true if the chat needs to be removed from the chat list

    #[serde(default)]
    remove_from_chat_list: bool,
    /// Pass true to delete chat history for all users

    #[serde(default)]
    revoke: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteChatHistory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteChatHistory {}

impl DeleteChatHistory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteChatHistoryBuilder {
        let mut inner = DeleteChatHistory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteChatHistory".to_string();

        DeleteChatHistoryBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn remove_from_chat_list(&self) -> bool {
        self.remove_from_chat_list
    }

    pub fn revoke(&self) -> bool {
        self.revoke
    }
}

#[doc(hidden)]
pub struct DeleteChatHistoryBuilder {
    inner: DeleteChatHistory,
}

#[deprecated]
pub type RTDDeleteChatHistoryBuilder = DeleteChatHistoryBuilder;

impl DeleteChatHistoryBuilder {
    pub fn build(&self) -> DeleteChatHistory {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn remove_from_chat_list(&mut self, remove_from_chat_list: bool) -> &mut Self {
        self.inner.remove_from_chat_list = remove_from_chat_list;
        self
    }

    pub fn revoke(&mut self, revoke: bool) -> &mut Self {
        self.inner.revoke = revoke;
        self
    }
}

impl AsRef<DeleteChatHistory> for DeleteChatHistory {
    fn as_ref(&self) -> &DeleteChatHistory {
        self
    }
}

impl AsRef<DeleteChatHistory> for DeleteChatHistoryBuilder {
    fn as_ref(&self) -> &DeleteChatHistory {
        &self.inner
    }
}
