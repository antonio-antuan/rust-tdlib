use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents the scope to which bot commands are relevant
pub trait TDBotCommandScope: Debug + RObject {}

/// Represents the scope to which bot commands are relevant
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotCommandScope {
    #[doc(hidden)]
    _Default,
    /// A scope covering all group and supergroup chat administrators
    #[serde(rename(deserialize = "botCommandScopeAllChatAdministrators"))]
    AllChatAdministrators(BotCommandScopeAllChatAdministrators),
    /// A scope covering all group and supergroup chats
    #[serde(rename(deserialize = "botCommandScopeAllGroupChats"))]
    AllGroupChats(BotCommandScopeAllGroupChats),
    /// A scope covering all private chats
    #[serde(rename(deserialize = "botCommandScopeAllPrivateChats"))]
    AllPrivateChats(BotCommandScopeAllPrivateChats),
    /// A scope covering all members of a chat
    #[serde(rename(deserialize = "botCommandScopeChat"))]
    Chat(BotCommandScopeChat),
    /// A scope covering all administrators of a chat
    #[serde(rename(deserialize = "botCommandScopeChatAdministrators"))]
    ChatAdministrators(BotCommandScopeChatAdministrators),
    /// A scope covering a member of a chat
    #[serde(rename(deserialize = "botCommandScopeChatMember"))]
    ChatMember(BotCommandScopeChatMember),
    /// A scope covering all users
    #[serde(rename(deserialize = "botCommandScopeDefault"))]
    Default(BotCommandScopeDefault),
}

impl Default for BotCommandScope {
    fn default() -> Self {
        BotCommandScope::_Default
    }
}

impl RObject for BotCommandScope {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            BotCommandScope::AllChatAdministrators(t) => t.extra(),
            BotCommandScope::AllGroupChats(t) => t.extra(),
            BotCommandScope::AllPrivateChats(t) => t.extra(),
            BotCommandScope::Chat(t) => t.extra(),
            BotCommandScope::ChatAdministrators(t) => t.extra(),
            BotCommandScope::ChatMember(t) => t.extra(),
            BotCommandScope::Default(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            BotCommandScope::AllChatAdministrators(t) => t.client_id(),
            BotCommandScope::AllGroupChats(t) => t.client_id(),
            BotCommandScope::AllPrivateChats(t) => t.client_id(),
            BotCommandScope::Chat(t) => t.client_id(),
            BotCommandScope::ChatAdministrators(t) => t.client_id(),
            BotCommandScope::ChatMember(t) => t.client_id(),
            BotCommandScope::Default(t) => t.client_id(),

            _ => None,
        }
    }
}

impl BotCommandScope {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, BotCommandScope::_Default)
    }
}

impl AsRef<BotCommandScope> for BotCommandScope {
    fn as_ref(&self) -> &BotCommandScope {
        self
    }
}

/// A scope covering all group and supergroup chat administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for BotCommandScopeAllChatAdministrators {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotCommandScope for BotCommandScopeAllChatAdministrators {}

impl BotCommandScopeAllChatAdministrators {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBotCommandScopeAllChatAdministratorsBuilder {
        let mut inner = BotCommandScopeAllChatAdministrators::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBotCommandScopeAllChatAdministratorsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDBotCommandScopeAllChatAdministratorsBuilder {
    inner: BotCommandScopeAllChatAdministrators,
}

impl RTDBotCommandScopeAllChatAdministratorsBuilder {
    pub fn build(&self) -> BotCommandScopeAllChatAdministrators {
        self.inner.clone()
    }
}

impl AsRef<BotCommandScopeAllChatAdministrators> for BotCommandScopeAllChatAdministrators {
    fn as_ref(&self) -> &BotCommandScopeAllChatAdministrators {
        self
    }
}

impl AsRef<BotCommandScopeAllChatAdministrators>
    for RTDBotCommandScopeAllChatAdministratorsBuilder
{
    fn as_ref(&self) -> &BotCommandScopeAllChatAdministrators {
        &self.inner
    }
}

/// A scope covering all group and supergroup chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeAllGroupChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for BotCommandScopeAllGroupChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotCommandScope for BotCommandScopeAllGroupChats {}

impl BotCommandScopeAllGroupChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBotCommandScopeAllGroupChatsBuilder {
        let mut inner = BotCommandScopeAllGroupChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBotCommandScopeAllGroupChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDBotCommandScopeAllGroupChatsBuilder {
    inner: BotCommandScopeAllGroupChats,
}

impl RTDBotCommandScopeAllGroupChatsBuilder {
    pub fn build(&self) -> BotCommandScopeAllGroupChats {
        self.inner.clone()
    }
}

impl AsRef<BotCommandScopeAllGroupChats> for BotCommandScopeAllGroupChats {
    fn as_ref(&self) -> &BotCommandScopeAllGroupChats {
        self
    }
}

impl AsRef<BotCommandScopeAllGroupChats> for RTDBotCommandScopeAllGroupChatsBuilder {
    fn as_ref(&self) -> &BotCommandScopeAllGroupChats {
        &self.inner
    }
}

/// A scope covering all private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for BotCommandScopeAllPrivateChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotCommandScope for BotCommandScopeAllPrivateChats {}

impl BotCommandScopeAllPrivateChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBotCommandScopeAllPrivateChatsBuilder {
        let mut inner = BotCommandScopeAllPrivateChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBotCommandScopeAllPrivateChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDBotCommandScopeAllPrivateChatsBuilder {
    inner: BotCommandScopeAllPrivateChats,
}

impl RTDBotCommandScopeAllPrivateChatsBuilder {
    pub fn build(&self) -> BotCommandScopeAllPrivateChats {
        self.inner.clone()
    }
}

impl AsRef<BotCommandScopeAllPrivateChats> for BotCommandScopeAllPrivateChats {
    fn as_ref(&self) -> &BotCommandScopeAllPrivateChats {
        self
    }
}

impl AsRef<BotCommandScopeAllPrivateChats> for RTDBotCommandScopeAllPrivateChatsBuilder {
    fn as_ref(&self) -> &BotCommandScopeAllPrivateChats {
        &self.inner
    }
}

/// A scope covering all members of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
}

impl RObject for BotCommandScopeChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotCommandScope for BotCommandScopeChat {}

impl BotCommandScopeChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBotCommandScopeChatBuilder {
        let mut inner = BotCommandScopeChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBotCommandScopeChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDBotCommandScopeChatBuilder {
    inner: BotCommandScopeChat,
}

impl RTDBotCommandScopeChatBuilder {
    pub fn build(&self) -> BotCommandScopeChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<BotCommandScopeChat> for BotCommandScopeChat {
    fn as_ref(&self) -> &BotCommandScopeChat {
        self
    }
}

impl AsRef<BotCommandScopeChat> for RTDBotCommandScopeChatBuilder {
    fn as_ref(&self) -> &BotCommandScopeChat {
        &self.inner
    }
}

/// A scope covering all administrators of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministrators {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
}

impl RObject for BotCommandScopeChatAdministrators {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotCommandScope for BotCommandScopeChatAdministrators {}

impl BotCommandScopeChatAdministrators {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBotCommandScopeChatAdministratorsBuilder {
        let mut inner = BotCommandScopeChatAdministrators::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBotCommandScopeChatAdministratorsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDBotCommandScopeChatAdministratorsBuilder {
    inner: BotCommandScopeChatAdministrators,
}

impl RTDBotCommandScopeChatAdministratorsBuilder {
    pub fn build(&self) -> BotCommandScopeChatAdministrators {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<BotCommandScopeChatAdministrators> for BotCommandScopeChatAdministrators {
    fn as_ref(&self) -> &BotCommandScopeChatAdministrators {
        self
    }
}

impl AsRef<BotCommandScopeChatAdministrators> for RTDBotCommandScopeChatAdministratorsBuilder {
    fn as_ref(&self) -> &BotCommandScopeChatAdministrators {
        &self.inner
    }
}

/// A scope covering a member of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeChatMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// User identifier

    #[serde(default)]
    user_id: i64,
}

impl RObject for BotCommandScopeChatMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotCommandScope for BotCommandScopeChatMember {}

impl BotCommandScopeChatMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBotCommandScopeChatMemberBuilder {
        let mut inner = BotCommandScopeChatMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBotCommandScopeChatMemberBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDBotCommandScopeChatMemberBuilder {
    inner: BotCommandScopeChatMember,
}

impl RTDBotCommandScopeChatMemberBuilder {
    pub fn build(&self) -> BotCommandScopeChatMember {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<BotCommandScopeChatMember> for BotCommandScopeChatMember {
    fn as_ref(&self) -> &BotCommandScopeChatMember {
        self
    }
}

impl AsRef<BotCommandScopeChatMember> for RTDBotCommandScopeChatMemberBuilder {
    fn as_ref(&self) -> &BotCommandScopeChatMember {
        &self.inner
    }
}

/// A scope covering all users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeDefault {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for BotCommandScopeDefault {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBotCommandScope for BotCommandScopeDefault {}

impl BotCommandScopeDefault {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBotCommandScopeDefaultBuilder {
        let mut inner = BotCommandScopeDefault::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBotCommandScopeDefaultBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDBotCommandScopeDefaultBuilder {
    inner: BotCommandScopeDefault,
}

impl RTDBotCommandScopeDefaultBuilder {
    pub fn build(&self) -> BotCommandScopeDefault {
        self.inner.clone()
    }
}

impl AsRef<BotCommandScopeDefault> for BotCommandScopeDefault {
    fn as_ref(&self) -> &BotCommandScopeDefault {
        self
    }
}

impl AsRef<BotCommandScopeDefault> for RTDBotCommandScopeDefaultBuilder {
    fn as_ref(&self) -> &BotCommandScopeDefault {
        &self.inner
    }
}
