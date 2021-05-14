use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns an ordered list of chats in a chat list. Chats are sorted by the pair (chat.position.order, chat.id) in descending order. (For example, to get a list of chats from the beginning, the offset_order should be equal to a biggest signed 64-bit number 9223372036854775807 == 2^63  1). For optimal performance the number of returned chats is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat list in which to return chats

    #[serde(skip_serializing_if = "ChatList::_is_default")]
    chat_list: ChatList,
    /// Chat order to return chats from

    #[serde(deserialize_with = "super::_common::number_from_string")]
    offset_order: i64,
    /// Chat identifier to return chats from
    offset_chat_id: i64,
    /// The maximum number of chats to be returned. It is possible that fewer chats than the limit are returned even if the end of the list is not reached
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetChatsBuilder {
        let mut inner = GetChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChats".to_string();

        RTDGetChatsBuilder { inner }
    }

    pub fn chat_list(&self) -> &ChatList {
        &self.chat_list
    }

    pub fn offset_order(&self) -> i64 {
        self.offset_order
    }

    pub fn offset_chat_id(&self) -> i64 {
        self.offset_chat_id
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDGetChatsBuilder {
    inner: GetChats,
}

impl RTDGetChatsBuilder {
    pub fn build(&self) -> GetChats {
        self.inner.clone()
    }

    pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
        self.inner.chat_list = chat_list.as_ref().clone();
        self
    }

    pub fn offset_order(&mut self, offset_order: i64) -> &mut Self {
        self.inner.offset_order = offset_order;
        self
    }

    pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self {
        self.inner.offset_chat_id = offset_chat_id;
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

impl AsRef<GetChats> for RTDGetChatsBuilder {
    fn as_ref(&self) -> &GetChats {
        &self.inner
    }
}
