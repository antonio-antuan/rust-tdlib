use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the type of a chat
pub trait TDChatType: Debug + RObject {}

/// Describes the type of a chat
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatType {
    #[doc(hidden)]
    _Default,
    /// A basic group (i.e., a chat with 0-200 other users)
    #[serde(rename(serialize = "chatTypeBasicGroup", deserialize = "chatTypeBasicGroup"))]
    BasicGroup(ChatTypeBasicGroup),
    /// An ordinary chat with a user
    #[serde(rename(serialize = "chatTypePrivate", deserialize = "chatTypePrivate"))]
    Private(ChatTypePrivate),
    /// A secret chat with a user
    #[serde(rename(serialize = "chatTypeSecret", deserialize = "chatTypeSecret"))]
    Secret(ChatTypeSecret),
    /// A supergroup (i.e. a chat with up to GetOption("supergroup_max_size") other users), or channel (with unlimited members)
    #[serde(rename(serialize = "chatTypeSupergroup", deserialize = "chatTypeSupergroup"))]
    Supergroup(ChatTypeSupergroup),
}

impl Default for ChatType {
    fn default() -> Self {
        ChatType::_Default
    }
}

impl RObject for ChatType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatType::BasicGroup(t) => t.extra(),
            ChatType::Private(t) => t.extra(),
            ChatType::Secret(t) => t.extra(),
            ChatType::Supergroup(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatType::BasicGroup(t) => t.client_id(),
            ChatType::Private(t) => t.client_id(),
            ChatType::Secret(t) => t.client_id(),
            ChatType::Supergroup(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatType::_Default)
    }
}

impl AsRef<ChatType> for ChatType {
    fn as_ref(&self) -> &ChatType {
        self
    }
}

/// A basic group (i.e., a chat with 0-200 other users)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTypeBasicGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Basic group identifier
    basic_group_id: i32,
}

impl RObject for ChatTypeBasicGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatType for ChatTypeBasicGroup {}

impl ChatTypeBasicGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatTypeBasicGroupBuilder {
        let mut inner = ChatTypeBasicGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatTypeBasicGroupBuilder { inner }
    }

    pub fn basic_group_id(&self) -> i32 {
        self.basic_group_id
    }
}

#[doc(hidden)]
pub struct RTDChatTypeBasicGroupBuilder {
    inner: ChatTypeBasicGroup,
}

impl RTDChatTypeBasicGroupBuilder {
    pub fn build(&self) -> ChatTypeBasicGroup {
        self.inner.clone()
    }

    pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self {
        self.inner.basic_group_id = basic_group_id;
        self
    }
}

impl AsRef<ChatTypeBasicGroup> for ChatTypeBasicGroup {
    fn as_ref(&self) -> &ChatTypeBasicGroup {
        self
    }
}

impl AsRef<ChatTypeBasicGroup> for RTDChatTypeBasicGroupBuilder {
    fn as_ref(&self) -> &ChatTypeBasicGroup {
        &self.inner
    }
}

/// An ordinary chat with a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTypePrivate {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier
    user_id: i32,
}

impl RObject for ChatTypePrivate {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatType for ChatTypePrivate {}

impl ChatTypePrivate {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatTypePrivateBuilder {
        let mut inner = ChatTypePrivate::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatTypePrivateBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDChatTypePrivateBuilder {
    inner: ChatTypePrivate,
}

impl RTDChatTypePrivateBuilder {
    pub fn build(&self) -> ChatTypePrivate {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<ChatTypePrivate> for ChatTypePrivate {
    fn as_ref(&self) -> &ChatTypePrivate {
        self
    }
}

impl AsRef<ChatTypePrivate> for RTDChatTypePrivateBuilder {
    fn as_ref(&self) -> &ChatTypePrivate {
        &self.inner
    }
}

/// A secret chat with a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTypeSecret {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Secret chat identifier
    secret_chat_id: i32,
    /// User identifier of the secret chat peer
    user_id: i32,
}

impl RObject for ChatTypeSecret {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatType for ChatTypeSecret {}

impl ChatTypeSecret {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatTypeSecretBuilder {
        let mut inner = ChatTypeSecret::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatTypeSecretBuilder { inner }
    }

    pub fn secret_chat_id(&self) -> i32 {
        self.secret_chat_id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDChatTypeSecretBuilder {
    inner: ChatTypeSecret,
}

impl RTDChatTypeSecretBuilder {
    pub fn build(&self) -> ChatTypeSecret {
        self.inner.clone()
    }

    pub fn secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self {
        self.inner.secret_chat_id = secret_chat_id;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<ChatTypeSecret> for ChatTypeSecret {
    fn as_ref(&self) -> &ChatTypeSecret {
        self
    }
}

impl AsRef<ChatTypeSecret> for RTDChatTypeSecretBuilder {
    fn as_ref(&self) -> &ChatTypeSecret {
        &self.inner
    }
}

/// A supergroup (i.e. a chat with up to GetOption("supergroup_max_size") other users), or channel (with unlimited members)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTypeSupergroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Supergroup or channel identifier
    supergroup_id: i32,
    /// True, if the supergroup is a channel
    is_channel: bool,
}

impl RObject for ChatTypeSupergroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatType for ChatTypeSupergroup {}

impl ChatTypeSupergroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatTypeSupergroupBuilder {
        let mut inner = ChatTypeSupergroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatTypeSupergroupBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i32 {
        self.supergroup_id
    }

    pub fn is_channel(&self) -> bool {
        self.is_channel
    }
}

#[doc(hidden)]
pub struct RTDChatTypeSupergroupBuilder {
    inner: ChatTypeSupergroup,
}

impl RTDChatTypeSupergroupBuilder {
    pub fn build(&self) -> ChatTypeSupergroup {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn is_channel(&mut self, is_channel: bool) -> &mut Self {
        self.inner.is_channel = is_channel;
        self
    }
}

impl AsRef<ChatTypeSupergroup> for ChatTypeSupergroup {
    fn as_ref(&self) -> &ChatTypeSupergroup {
        self
    }
}

impl AsRef<ChatTypeSupergroup> for RTDChatTypeSupergroupBuilder {
    fn as_ref(&self) -> &ChatTypeSupergroup {
        &self.inner
    }
}
