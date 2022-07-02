use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Loads more chats from a chat list. The loaded chats and their positions in the chat list will be sent through updates. Chats are sorted by the pair (chat.position.order, chat.id) in descending order. Returns a 404 error if all chats have been loaded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoadChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat list in which to load chats; pass null to load chats from the main chat list

    #[serde(skip_serializing_if = "ChatList::_is_default")]
    chat_list: ChatList,
    /// The maximum number of chats to be loaded. For optimal performance, the number of loaded chats is chosen by TDLib and can be smaller than the specified limit, even if the end of the list is not reached

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for LoadChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for LoadChats {}

impl LoadChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LoadChatsBuilder {
        let mut inner = LoadChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "loadChats".to_string();

        LoadChatsBuilder { inner }
    }

    pub fn chat_list(&self) -> &ChatList {
        &self.chat_list
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct LoadChatsBuilder {
    inner: LoadChats,
}

#[deprecated]
pub type RTDLoadChatsBuilder = LoadChatsBuilder;

impl LoadChatsBuilder {
    pub fn build(&self) -> LoadChats {
        self.inner.clone()
    }

    pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
        self.inner.chat_list = chat_list.as_ref().clone();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<LoadChats> for LoadChats {
    fn as_ref(&self) -> &LoadChats {
        self
    }
}

impl AsRef<LoadChats> for LoadChatsBuilder {
    fn as_ref(&self) -> &LoadChats {
        &self.inner
    }
}
