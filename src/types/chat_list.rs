use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a list of chats
pub trait TDChatList: Debug + RObject {}

/// Describes a list of chats
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum ChatList {
    #[doc(hidden)]
    #[default]
    _Default,
    /// A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives
    #[serde(rename = "chatListArchive")]
    Archive(ChatListArchive),
    /// A list of chats added to a chat folder
    #[serde(rename = "chatListFolder")]
    Folder(ChatListFolder),
    /// A main list of chats
    #[serde(rename = "chatListMain")]
    Main(ChatListMain),
}

impl RObject for ChatList {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatList::Archive(t) => t.extra(),
            ChatList::Folder(t) => t.extra(),
            ChatList::Main(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatList::Archive(t) => t.client_id(),
            ChatList::Folder(t) => t.client_id(),
            ChatList::Main(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatList {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatList::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatListArchive {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatList for ChatListArchive {}

impl ChatListArchive {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatListArchiveBuilder {
        let mut inner = ChatListArchive::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatListArchiveBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatListArchiveBuilder {
    inner: ChatListArchive,
}

#[deprecated]
pub type RTDChatListArchiveBuilder = ChatListArchiveBuilder;

impl ChatListArchiveBuilder {
    pub fn build(&self) -> ChatListArchive {
        self.inner.clone()
    }
}

impl AsRef<ChatListArchive> for ChatListArchive {
    fn as_ref(&self) -> &ChatListArchive {
        self
    }
}

impl AsRef<ChatListArchive> for ChatListArchiveBuilder {
    fn as_ref(&self) -> &ChatListArchive {
        &self.inner
    }
}

/// A list of chats added to a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListFolder {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat folder identifier

    #[serde(default)]
    chat_folder_id: i32,
}

impl RObject for ChatListFolder {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatList for ChatListFolder {}

impl ChatListFolder {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatListFolderBuilder {
        let mut inner = ChatListFolder::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatListFolderBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }
}

#[doc(hidden)]
pub struct ChatListFolderBuilder {
    inner: ChatListFolder,
}

#[deprecated]
pub type RTDChatListFolderBuilder = ChatListFolderBuilder;

impl ChatListFolderBuilder {
    pub fn build(&self) -> ChatListFolder {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
        self
    }
}

impl AsRef<ChatListFolder> for ChatListFolder {
    fn as_ref(&self) -> &ChatListFolder {
        self
    }
}

impl AsRef<ChatListFolder> for ChatListFolderBuilder {
    fn as_ref(&self) -> &ChatListFolder {
        &self.inner
    }
}

/// A main list of chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListMain {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatListMain {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatList for ChatListMain {}

impl ChatListMain {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatListMainBuilder {
        let mut inner = ChatListMain::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatListMainBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatListMainBuilder {
    inner: ChatListMain,
}

#[deprecated]
pub type RTDChatListMainBuilder = ChatListMainBuilder;

impl ChatListMainBuilder {
    pub fn build(&self) -> ChatListMain {
        self.inner.clone()
    }
}

impl AsRef<ChatListMain> for ChatListMain {
    fn as_ref(&self) -> &ChatListMain {
        self
    }
}

impl AsRef<ChatListMain> for ChatListMainBuilder {
    fn as_ref(&self) -> &ChatListMain {
        &self.inner
    }
}
