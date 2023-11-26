use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Traverse all chats in a chat list and marks all messages in the chats as read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadChatList {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat list in which to mark all chats as read

    #[serde(skip_serializing_if = "ChatList::_is_default")]
    chat_list: ChatList,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReadChatList {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReadChatList {}

impl ReadChatList {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReadChatListBuilder {
        let mut inner = ReadChatList::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "readChatList".to_string();

        ReadChatListBuilder { inner }
    }

    pub fn chat_list(&self) -> &ChatList {
        &self.chat_list
    }
}

#[doc(hidden)]
pub struct ReadChatListBuilder {
    inner: ReadChatList,
}

#[deprecated]
pub type RTDReadChatListBuilder = ReadChatListBuilder;

impl ReadChatListBuilder {
    pub fn build(&self) -> ReadChatList {
        self.inner.clone()
    }

    pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
        self.inner.chat_list = chat_list.as_ref().clone();
        self
    }
}

impl AsRef<ReadChatList> for ReadChatList {
    fn as_ref(&self) -> &ReadChatList {
        self
    }
}

impl AsRef<ReadChatList> for ReadChatListBuilder {
    fn as_ref(&self) -> &ReadChatList {
        &self.inner
    }
}
