use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Specifies the kind of chat members to return in searchChatMembers
pub trait TDChatMembersFilter: Debug + RObject {}

/// Specifies the kind of chat members to return in searchChatMembers
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMembersFilter {
    #[doc(hidden)]
    _Default,
    /// Returns the owner and administrators
    #[serde(rename = "chatMembersFilterAdministrators")]
    Administrators(ChatMembersFilterAdministrators),
    /// Returns users banned from the chat; can be used only by administrators in a supergroup or in a channel
    #[serde(rename = "chatMembersFilterBanned")]
    Banned(ChatMembersFilterBanned),
    /// Returns bot members of the chat
    #[serde(rename = "chatMembersFilterBots")]
    Bots(ChatMembersFilterBots),
    /// Returns contacts of the user
    #[serde(rename = "chatMembersFilterContacts")]
    Contacts(ChatMembersFilterContacts),
    /// Returns all chat members, including restricted chat members
    #[serde(rename = "chatMembersFilterMembers")]
    Members(ChatMembersFilterMembers),
    /// Returns users which can be mentioned in the chat
    #[serde(rename = "chatMembersFilterMention")]
    Mention(ChatMembersFilterMention),
    /// Returns users under certain restrictions in the chat; can be used only by administrators in a supergroup
    #[serde(rename = "chatMembersFilterRestricted")]
    Restricted(ChatMembersFilterRestricted),
}

impl Default for ChatMembersFilter {
    fn default() -> Self {
        ChatMembersFilter::_Default
    }
}

impl RObject for ChatMembersFilter {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatMembersFilter::Administrators(t) => t.extra(),
            ChatMembersFilter::Banned(t) => t.extra(),
            ChatMembersFilter::Bots(t) => t.extra(),
            ChatMembersFilter::Contacts(t) => t.extra(),
            ChatMembersFilter::Members(t) => t.extra(),
            ChatMembersFilter::Mention(t) => t.extra(),
            ChatMembersFilter::Restricted(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatMembersFilter::Administrators(t) => t.client_id(),
            ChatMembersFilter::Banned(t) => t.client_id(),
            ChatMembersFilter::Bots(t) => t.client_id(),
            ChatMembersFilter::Contacts(t) => t.client_id(),
            ChatMembersFilter::Members(t) => t.client_id(),
            ChatMembersFilter::Mention(t) => t.client_id(),
            ChatMembersFilter::Restricted(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatMembersFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatMembersFilter::_Default)
    }
}

impl AsRef<ChatMembersFilter> for ChatMembersFilter {
    fn as_ref(&self) -> &ChatMembersFilter {
        self
    }
}

/// Returns the owner and administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterAdministrators {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatMembersFilterAdministrators {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMembersFilter for ChatMembersFilterAdministrators {}

impl ChatMembersFilterAdministrators {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMembersFilterAdministratorsBuilder {
        let mut inner = ChatMembersFilterAdministrators::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMembersFilterAdministratorsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatMembersFilterAdministratorsBuilder {
    inner: ChatMembersFilterAdministrators,
}

#[deprecated]
pub type RTDChatMembersFilterAdministratorsBuilder = ChatMembersFilterAdministratorsBuilder;

impl ChatMembersFilterAdministratorsBuilder {
    pub fn build(&self) -> ChatMembersFilterAdministrators {
        self.inner.clone()
    }
}

impl AsRef<ChatMembersFilterAdministrators> for ChatMembersFilterAdministrators {
    fn as_ref(&self) -> &ChatMembersFilterAdministrators {
        self
    }
}

impl AsRef<ChatMembersFilterAdministrators> for ChatMembersFilterAdministratorsBuilder {
    fn as_ref(&self) -> &ChatMembersFilterAdministrators {
        &self.inner
    }
}

/// Returns users banned from the chat; can be used only by administrators in a supergroup or in a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterBanned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatMembersFilterBanned {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMembersFilter for ChatMembersFilterBanned {}

impl ChatMembersFilterBanned {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMembersFilterBannedBuilder {
        let mut inner = ChatMembersFilterBanned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMembersFilterBannedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatMembersFilterBannedBuilder {
    inner: ChatMembersFilterBanned,
}

#[deprecated]
pub type RTDChatMembersFilterBannedBuilder = ChatMembersFilterBannedBuilder;

impl ChatMembersFilterBannedBuilder {
    pub fn build(&self) -> ChatMembersFilterBanned {
        self.inner.clone()
    }
}

impl AsRef<ChatMembersFilterBanned> for ChatMembersFilterBanned {
    fn as_ref(&self) -> &ChatMembersFilterBanned {
        self
    }
}

impl AsRef<ChatMembersFilterBanned> for ChatMembersFilterBannedBuilder {
    fn as_ref(&self) -> &ChatMembersFilterBanned {
        &self.inner
    }
}

/// Returns bot members of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterBots {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatMembersFilterBots {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMembersFilter for ChatMembersFilterBots {}

impl ChatMembersFilterBots {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMembersFilterBotsBuilder {
        let mut inner = ChatMembersFilterBots::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMembersFilterBotsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatMembersFilterBotsBuilder {
    inner: ChatMembersFilterBots,
}

#[deprecated]
pub type RTDChatMembersFilterBotsBuilder = ChatMembersFilterBotsBuilder;

impl ChatMembersFilterBotsBuilder {
    pub fn build(&self) -> ChatMembersFilterBots {
        self.inner.clone()
    }
}

impl AsRef<ChatMembersFilterBots> for ChatMembersFilterBots {
    fn as_ref(&self) -> &ChatMembersFilterBots {
        self
    }
}

impl AsRef<ChatMembersFilterBots> for ChatMembersFilterBotsBuilder {
    fn as_ref(&self) -> &ChatMembersFilterBots {
        &self.inner
    }
}

/// Returns contacts of the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatMembersFilterContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMembersFilter for ChatMembersFilterContacts {}

impl ChatMembersFilterContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMembersFilterContactsBuilder {
        let mut inner = ChatMembersFilterContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMembersFilterContactsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatMembersFilterContactsBuilder {
    inner: ChatMembersFilterContacts,
}

#[deprecated]
pub type RTDChatMembersFilterContactsBuilder = ChatMembersFilterContactsBuilder;

impl ChatMembersFilterContactsBuilder {
    pub fn build(&self) -> ChatMembersFilterContacts {
        self.inner.clone()
    }
}

impl AsRef<ChatMembersFilterContacts> for ChatMembersFilterContacts {
    fn as_ref(&self) -> &ChatMembersFilterContacts {
        self
    }
}

impl AsRef<ChatMembersFilterContacts> for ChatMembersFilterContactsBuilder {
    fn as_ref(&self) -> &ChatMembersFilterContacts {
        &self.inner
    }
}

/// Returns all chat members, including restricted chat members
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatMembersFilterMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMembersFilter for ChatMembersFilterMembers {}

impl ChatMembersFilterMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMembersFilterMembersBuilder {
        let mut inner = ChatMembersFilterMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMembersFilterMembersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatMembersFilterMembersBuilder {
    inner: ChatMembersFilterMembers,
}

#[deprecated]
pub type RTDChatMembersFilterMembersBuilder = ChatMembersFilterMembersBuilder;

impl ChatMembersFilterMembersBuilder {
    pub fn build(&self) -> ChatMembersFilterMembers {
        self.inner.clone()
    }
}

impl AsRef<ChatMembersFilterMembers> for ChatMembersFilterMembers {
    fn as_ref(&self) -> &ChatMembersFilterMembers {
        self
    }
}

impl AsRef<ChatMembersFilterMembers> for ChatMembersFilterMembersBuilder {
    fn as_ref(&self) -> &ChatMembersFilterMembers {
        &self.inner
    }
}

/// Returns users which can be mentioned in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterMention {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If non-zero, the identifier of the current message thread

    #[serde(default)]
    message_thread_id: i64,
}

impl RObject for ChatMembersFilterMention {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMembersFilter for ChatMembersFilterMention {}

impl ChatMembersFilterMention {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMembersFilterMentionBuilder {
        let mut inner = ChatMembersFilterMention::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMembersFilterMentionBuilder { inner }
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }
}

#[doc(hidden)]
pub struct ChatMembersFilterMentionBuilder {
    inner: ChatMembersFilterMention,
}

#[deprecated]
pub type RTDChatMembersFilterMentionBuilder = ChatMembersFilterMentionBuilder;

impl ChatMembersFilterMentionBuilder {
    pub fn build(&self) -> ChatMembersFilterMention {
        self.inner.clone()
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }
}

impl AsRef<ChatMembersFilterMention> for ChatMembersFilterMention {
    fn as_ref(&self) -> &ChatMembersFilterMention {
        self
    }
}

impl AsRef<ChatMembersFilterMention> for ChatMembersFilterMentionBuilder {
    fn as_ref(&self) -> &ChatMembersFilterMention {
        &self.inner
    }
}

/// Returns users under certain restrictions in the chat; can be used only by administrators in a supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterRestricted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatMembersFilterRestricted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMembersFilter for ChatMembersFilterRestricted {}

impl ChatMembersFilterRestricted {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatMembersFilterRestrictedBuilder {
        let mut inner = ChatMembersFilterRestricted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatMembersFilterRestrictedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatMembersFilterRestrictedBuilder {
    inner: ChatMembersFilterRestricted,
}

#[deprecated]
pub type RTDChatMembersFilterRestrictedBuilder = ChatMembersFilterRestrictedBuilder;

impl ChatMembersFilterRestrictedBuilder {
    pub fn build(&self) -> ChatMembersFilterRestricted {
        self.inner.clone()
    }
}

impl AsRef<ChatMembersFilterRestricted> for ChatMembersFilterRestricted {
    fn as_ref(&self) -> &ChatMembersFilterRestricted {
        self
    }
}

impl AsRef<ChatMembersFilterRestricted> for ChatMembersFilterRestrictedBuilder {
    fn as_ref(&self) -> &ChatMembersFilterRestricted {
        &self.inner
    }
}
