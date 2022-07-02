use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the order of pinned chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPinnedChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat list in which to change the order of pinned chats

    #[serde(skip_serializing_if = "ChatList::_is_default")]
    chat_list: ChatList,
    /// The new list of pinned chats

    #[serde(default)]
    chat_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetPinnedChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetPinnedChats {}

impl SetPinnedChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetPinnedChatsBuilder {
        let mut inner = SetPinnedChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setPinnedChats".to_string();

        SetPinnedChatsBuilder { inner }
    }

    pub fn chat_list(&self) -> &ChatList {
        &self.chat_list
    }

    pub fn chat_ids(&self) -> &Vec<i64> {
        &self.chat_ids
    }
}

#[doc(hidden)]
pub struct SetPinnedChatsBuilder {
    inner: SetPinnedChats,
}

#[deprecated]
pub type RTDSetPinnedChatsBuilder = SetPinnedChatsBuilder;

impl SetPinnedChatsBuilder {
    pub fn build(&self) -> SetPinnedChats {
        self.inner.clone()
    }

    pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
        self.inner.chat_list = chat_list.as_ref().clone();
        self
    }

    pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
        self.inner.chat_ids = chat_ids;
        self
    }
}

impl AsRef<SetPinnedChats> for SetPinnedChats {
    fn as_ref(&self) -> &SetPinnedChats {
        self
    }
}

impl AsRef<SetPinnedChats> for SetPinnedChatsBuilder {
    fn as_ref(&self) -> &SetPinnedChats {
        &self.inner
    }
}
