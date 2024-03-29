use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of chat lists
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatLists {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of chat lists

    #[serde(default)]
    chat_lists: Vec<ChatList>,
}

impl RObject for ChatLists {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatLists {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatListsBuilder {
        let mut inner = ChatLists::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatListsBuilder { inner }
    }

    pub fn chat_lists(&self) -> &Vec<ChatList> {
        &self.chat_lists
    }
}

#[doc(hidden)]
pub struct ChatListsBuilder {
    inner: ChatLists,
}

#[deprecated]
pub type RTDChatListsBuilder = ChatListsBuilder;

impl ChatListsBuilder {
    pub fn build(&self) -> ChatLists {
        self.inner.clone()
    }

    pub fn chat_lists(&mut self, chat_lists: Vec<ChatList>) -> &mut Self {
        self.inner.chat_lists = chat_lists;
        self
    }
}

impl AsRef<ChatLists> for ChatLists {
    fn as_ref(&self) -> &ChatLists {
        self
    }
}

impl AsRef<ChatLists> for ChatListsBuilder {
    fn as_ref(&self) -> &ChatLists {
        &self.inner
    }
}
