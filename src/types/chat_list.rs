use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a list of chats
pub trait TDChatList: Debug + RObject {}

/// Describes a list of chats
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatList {
    #[doc(hidden)]
    _Default(()),
    /// A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives
    Archive(ChatListArchive),
    /// A list of chats belonging to a chat filter
    Filter(ChatListFilter),
    /// A main list of chats
    Main(ChatListMain),
}

impl Default for ChatList {
    fn default() -> Self {
        ChatList::_Default(())
    }
}

impl<'de> Deserialize<'de> for ChatList {
    fn deserialize<D>(deserializer: D) -> Result<ChatList, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          ChatList,
          (chatListArchive, Archive);
          (chatListFilter, Filter);
          (chatListMain, Main);

        )(deserializer)
    }
}

impl RObject for ChatList {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            ChatList::Archive(t) => t.td_name(),
            ChatList::Filter(t) => t.td_name(),
            ChatList::Main(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            ChatList::Archive(t) => t.extra(),
            ChatList::Filter(t) => t.extra(),
            ChatList::Main(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl ChatList {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatList::_Default(_))
    }
}

impl AsRef<ChatList> for ChatList {
    fn as_ref(&self) -> &ChatList {
        self
    }
}

/// A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListArchive {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatListArchive {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatListArchive"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatList for ChatListArchive {}

impl ChatListArchive {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatListArchiveBuilder {
        let mut inner = ChatListArchive::default();
        inner.td_name = "chatListArchive".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatListArchiveBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatListArchiveBuilder {
    inner: ChatListArchive,
}

impl RTDChatListArchiveBuilder {
    pub fn build(&self) -> ChatListArchive {
        self.inner.clone()
    }
}

impl AsRef<ChatListArchive> for ChatListArchive {
    fn as_ref(&self) -> &ChatListArchive {
        self
    }
}

impl AsRef<ChatListArchive> for RTDChatListArchiveBuilder {
    fn as_ref(&self) -> &ChatListArchive {
        &self.inner
    }
}

/// A list of chats belonging to a chat filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListFilter {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Chat filter identifier
    chat_filter_id: i32,
}

impl RObject for ChatListFilter {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatListFilter"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatList for ChatListFilter {}

impl ChatListFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatListFilterBuilder {
        let mut inner = ChatListFilter::default();
        inner.td_name = "chatListFilter".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatListFilterBuilder { inner }
    }

    pub fn chat_filter_id(&self) -> i32 {
        self.chat_filter_id
    }
}

#[doc(hidden)]
pub struct RTDChatListFilterBuilder {
    inner: ChatListFilter,
}

impl RTDChatListFilterBuilder {
    pub fn build(&self) -> ChatListFilter {
        self.inner.clone()
    }

    pub fn chat_filter_id(&mut self, chat_filter_id: i32) -> &mut Self {
        self.inner.chat_filter_id = chat_filter_id;
        self
    }
}

impl AsRef<ChatListFilter> for ChatListFilter {
    fn as_ref(&self) -> &ChatListFilter {
        self
    }
}

impl AsRef<ChatListFilter> for RTDChatListFilterBuilder {
    fn as_ref(&self) -> &ChatListFilter {
        &self.inner
    }
}

/// A main list of chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListMain {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatListMain {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatListMain"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatList for ChatListMain {}

impl ChatListMain {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatListMainBuilder {
        let mut inner = ChatListMain::default();
        inner.td_name = "chatListMain".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatListMainBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatListMainBuilder {
    inner: ChatListMain,
}

impl RTDChatListMainBuilder {
    pub fn build(&self) -> ChatListMain {
        self.inner.clone()
    }
}

impl AsRef<ChatListMain> for ChatListMain {
    fn as_ref(&self) -> &ChatListMain {
        self
    }
}

impl AsRef<ChatListMain> for RTDChatListMainBuilder {
    fn as_ref(&self) -> &ChatListMain {
        &self.inner
    }
}
