use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents the categories of chats for which a list of frequently used chats can be retrieved
pub trait TDTopChatCategory: Debug + RObject {}

/// Represents the categories of chats for which a list of frequently used chats can be retrieved
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TopChatCategory {
    #[doc(hidden)]
    _Default,
    /// A category containing frequently used private chats with bot users
    #[serde(rename = "topChatCategoryBots")]
    Bots(TopChatCategoryBots),
    /// A category containing frequently used chats used for calls
    #[serde(rename = "topChatCategoryCalls")]
    Calls(TopChatCategoryCalls),
    /// A category containing frequently used channels
    #[serde(rename = "topChatCategoryChannels")]
    Channels(TopChatCategoryChannels),
    /// A category containing frequently used chats used to forward messages
    #[serde(rename = "topChatCategoryForwardChats")]
    ForwardChats(TopChatCategoryForwardChats),
    /// A category containing frequently used basic groups and supergroups
    #[serde(rename = "topChatCategoryGroups")]
    Groups(TopChatCategoryGroups),
    /// A category containing frequently used chats with inline bots sorted by their usage in inline mode
    #[serde(rename = "topChatCategoryInlineBots")]
    InlineBots(TopChatCategoryInlineBots),
    /// A category containing frequently used private chats with non-bot users
    #[serde(rename = "topChatCategoryUsers")]
    Users(TopChatCategoryUsers),
}

impl Default for TopChatCategory {
    fn default() -> Self {
        TopChatCategory::_Default
    }
}

impl RObject for TopChatCategory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            TopChatCategory::Bots(t) => t.extra(),
            TopChatCategory::Calls(t) => t.extra(),
            TopChatCategory::Channels(t) => t.extra(),
            TopChatCategory::ForwardChats(t) => t.extra(),
            TopChatCategory::Groups(t) => t.extra(),
            TopChatCategory::InlineBots(t) => t.extra(),
            TopChatCategory::Users(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            TopChatCategory::Bots(t) => t.client_id(),
            TopChatCategory::Calls(t) => t.client_id(),
            TopChatCategory::Channels(t) => t.client_id(),
            TopChatCategory::ForwardChats(t) => t.client_id(),
            TopChatCategory::Groups(t) => t.client_id(),
            TopChatCategory::InlineBots(t) => t.client_id(),
            TopChatCategory::Users(t) => t.client_id(),

            _ => None,
        }
    }
}

impl TopChatCategory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, TopChatCategory::_Default)
    }
}

impl AsRef<TopChatCategory> for TopChatCategory {
    fn as_ref(&self) -> &TopChatCategory {
        self
    }
}

/// A category containing frequently used private chats with bot users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryBots {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TopChatCategoryBots {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTopChatCategory for TopChatCategoryBots {}

impl TopChatCategoryBots {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TopChatCategoryBotsBuilder {
        let mut inner = TopChatCategoryBots::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TopChatCategoryBotsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TopChatCategoryBotsBuilder {
    inner: TopChatCategoryBots,
}

#[deprecated]
pub type RTDTopChatCategoryBotsBuilder = TopChatCategoryBotsBuilder;

impl TopChatCategoryBotsBuilder {
    pub fn build(&self) -> TopChatCategoryBots {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryBots> for TopChatCategoryBots {
    fn as_ref(&self) -> &TopChatCategoryBots {
        self
    }
}

impl AsRef<TopChatCategoryBots> for TopChatCategoryBotsBuilder {
    fn as_ref(&self) -> &TopChatCategoryBots {
        &self.inner
    }
}

/// A category containing frequently used chats used for calls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryCalls {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TopChatCategoryCalls {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTopChatCategory for TopChatCategoryCalls {}

impl TopChatCategoryCalls {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TopChatCategoryCallsBuilder {
        let mut inner = TopChatCategoryCalls::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TopChatCategoryCallsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TopChatCategoryCallsBuilder {
    inner: TopChatCategoryCalls,
}

#[deprecated]
pub type RTDTopChatCategoryCallsBuilder = TopChatCategoryCallsBuilder;

impl TopChatCategoryCallsBuilder {
    pub fn build(&self) -> TopChatCategoryCalls {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryCalls> for TopChatCategoryCalls {
    fn as_ref(&self) -> &TopChatCategoryCalls {
        self
    }
}

impl AsRef<TopChatCategoryCalls> for TopChatCategoryCallsBuilder {
    fn as_ref(&self) -> &TopChatCategoryCalls {
        &self.inner
    }
}

/// A category containing frequently used channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryChannels {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TopChatCategoryChannels {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTopChatCategory for TopChatCategoryChannels {}

impl TopChatCategoryChannels {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TopChatCategoryChannelsBuilder {
        let mut inner = TopChatCategoryChannels::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TopChatCategoryChannelsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TopChatCategoryChannelsBuilder {
    inner: TopChatCategoryChannels,
}

#[deprecated]
pub type RTDTopChatCategoryChannelsBuilder = TopChatCategoryChannelsBuilder;

impl TopChatCategoryChannelsBuilder {
    pub fn build(&self) -> TopChatCategoryChannels {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryChannels> for TopChatCategoryChannels {
    fn as_ref(&self) -> &TopChatCategoryChannels {
        self
    }
}

impl AsRef<TopChatCategoryChannels> for TopChatCategoryChannelsBuilder {
    fn as_ref(&self) -> &TopChatCategoryChannels {
        &self.inner
    }
}

/// A category containing frequently used chats used to forward messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryForwardChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TopChatCategoryForwardChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTopChatCategory for TopChatCategoryForwardChats {}

impl TopChatCategoryForwardChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TopChatCategoryForwardChatsBuilder {
        let mut inner = TopChatCategoryForwardChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TopChatCategoryForwardChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TopChatCategoryForwardChatsBuilder {
    inner: TopChatCategoryForwardChats,
}

#[deprecated]
pub type RTDTopChatCategoryForwardChatsBuilder = TopChatCategoryForwardChatsBuilder;

impl TopChatCategoryForwardChatsBuilder {
    pub fn build(&self) -> TopChatCategoryForwardChats {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryForwardChats> for TopChatCategoryForwardChats {
    fn as_ref(&self) -> &TopChatCategoryForwardChats {
        self
    }
}

impl AsRef<TopChatCategoryForwardChats> for TopChatCategoryForwardChatsBuilder {
    fn as_ref(&self) -> &TopChatCategoryForwardChats {
        &self.inner
    }
}

/// A category containing frequently used basic groups and supergroups
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryGroups {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TopChatCategoryGroups {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTopChatCategory for TopChatCategoryGroups {}

impl TopChatCategoryGroups {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TopChatCategoryGroupsBuilder {
        let mut inner = TopChatCategoryGroups::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TopChatCategoryGroupsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TopChatCategoryGroupsBuilder {
    inner: TopChatCategoryGroups,
}

#[deprecated]
pub type RTDTopChatCategoryGroupsBuilder = TopChatCategoryGroupsBuilder;

impl TopChatCategoryGroupsBuilder {
    pub fn build(&self) -> TopChatCategoryGroups {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryGroups> for TopChatCategoryGroups {
    fn as_ref(&self) -> &TopChatCategoryGroups {
        self
    }
}

impl AsRef<TopChatCategoryGroups> for TopChatCategoryGroupsBuilder {
    fn as_ref(&self) -> &TopChatCategoryGroups {
        &self.inner
    }
}

/// A category containing frequently used chats with inline bots sorted by their usage in inline mode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryInlineBots {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TopChatCategoryInlineBots {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTopChatCategory for TopChatCategoryInlineBots {}

impl TopChatCategoryInlineBots {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TopChatCategoryInlineBotsBuilder {
        let mut inner = TopChatCategoryInlineBots::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TopChatCategoryInlineBotsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TopChatCategoryInlineBotsBuilder {
    inner: TopChatCategoryInlineBots,
}

#[deprecated]
pub type RTDTopChatCategoryInlineBotsBuilder = TopChatCategoryInlineBotsBuilder;

impl TopChatCategoryInlineBotsBuilder {
    pub fn build(&self) -> TopChatCategoryInlineBots {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryInlineBots> for TopChatCategoryInlineBots {
    fn as_ref(&self) -> &TopChatCategoryInlineBots {
        self
    }
}

impl AsRef<TopChatCategoryInlineBots> for TopChatCategoryInlineBotsBuilder {
    fn as_ref(&self) -> &TopChatCategoryInlineBots {
        &self.inner
    }
}

/// A category containing frequently used private chats with non-bot users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryUsers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TopChatCategoryUsers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTopChatCategory for TopChatCategoryUsers {}

impl TopChatCategoryUsers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TopChatCategoryUsersBuilder {
        let mut inner = TopChatCategoryUsers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TopChatCategoryUsersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TopChatCategoryUsersBuilder {
    inner: TopChatCategoryUsers,
}

#[deprecated]
pub type RTDTopChatCategoryUsersBuilder = TopChatCategoryUsersBuilder;

impl TopChatCategoryUsersBuilder {
    pub fn build(&self) -> TopChatCategoryUsers {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryUsers> for TopChatCategoryUsers {
    fn as_ref(&self) -> &TopChatCategoryUsers {
        self
    }
}

impl AsRef<TopChatCategoryUsers> for TopChatCategoryUsersBuilder {
    fn as_ref(&self) -> &TopChatCategoryUsers {
        &self.inner
    }
}
