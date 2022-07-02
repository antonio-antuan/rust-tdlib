use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an ordered list of chats from the beginning of a chat list. For informational purposes only. Use loadChats and updates processing instead to maintain chat lists in a consistent state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat list in which to return chats; pass null to get chats from the main chat list

    #[serde(skip_serializing_if = "ChatList::_is_default")]
    chat_list: ChatList,
    /// The maximum number of chats to be returned

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChats {}

impl GetChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatsBuilder {
        let mut inner = GetChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChats".to_string();

        GetChatsBuilder { inner }
    }

    pub fn chat_list(&self) -> &ChatList {
        &self.chat_list
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetChatsBuilder {
    inner: GetChats,
}

#[deprecated]
pub type RTDGetChatsBuilder = GetChatsBuilder;

impl GetChatsBuilder {
    pub fn build(&self) -> GetChats {
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

impl AsRef<GetChats> for GetChats {
    fn as_ref(&self) -> &GetChats {
        self
    }
}

impl AsRef<GetChats> for GetChatsBuilder {
    fn as_ref(&self) -> &GetChats {
        &self.inner
    }
}
