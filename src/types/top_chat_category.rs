use crate::errors::*;
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
    #[serde(rename(serialize = "topChatCategoryBots", deserialize = "topChatCategoryBots"))]
    Bots(TopChatCategoryBots),
    /// A category containing frequently used chats used for calls
    #[serde(rename(
        serialize = "topChatCategoryCalls",
        deserialize = "topChatCategoryCalls"
    ))]
    Calls(TopChatCategoryCalls),
    /// A category containing frequently used channels
    #[serde(rename(
        serialize = "topChatCategoryChannels",
        deserialize = "topChatCategoryChannels"
    ))]
    Channels(TopChatCategoryChannels),
    /// A category containing frequently used chats used to forward messages
    #[serde(rename(
        serialize = "topChatCategoryForwardChats",
        deserialize = "topChatCategoryForwardChats"
    ))]
    ForwardChats(TopChatCategoryForwardChats),
    /// A category containing frequently used basic groups and supergroups
    #[serde(rename(
        serialize = "topChatCategoryGroups",
        deserialize = "topChatCategoryGroups"
    ))]
    Groups(TopChatCategoryGroups),
    /// A category containing frequently used chats with inline bots sorted by their usage in inline mode
    #[serde(rename(
        serialize = "topChatCategoryInlineBots",
        deserialize = "topChatCategoryInlineBots"
    ))]
    InlineBots(TopChatCategoryInlineBots),
    /// A category containing frequently used private chats with non-bot users
    #[serde(rename(
        serialize = "topChatCategoryUsers",
        deserialize = "topChatCategoryUsers"
    ))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTopChatCategoryBotsBuilder {
        let mut inner = TopChatCategoryBots::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTopChatCategoryBotsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTopChatCategoryBotsBuilder {
    inner: TopChatCategoryBots,
}

impl RTDTopChatCategoryBotsBuilder {
    pub fn build(&self) -> TopChatCategoryBots {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryBots> for TopChatCategoryBots {
    fn as_ref(&self) -> &TopChatCategoryBots {
        self
    }
}

impl AsRef<TopChatCategoryBots> for RTDTopChatCategoryBotsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTopChatCategoryCallsBuilder {
        let mut inner = TopChatCategoryCalls::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTopChatCategoryCallsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTopChatCategoryCallsBuilder {
    inner: TopChatCategoryCalls,
}

impl RTDTopChatCategoryCallsBuilder {
    pub fn build(&self) -> TopChatCategoryCalls {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryCalls> for TopChatCategoryCalls {
    fn as_ref(&self) -> &TopChatCategoryCalls {
        self
    }
}

impl AsRef<TopChatCategoryCalls> for RTDTopChatCategoryCallsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTopChatCategoryChannelsBuilder {
        let mut inner = TopChatCategoryChannels::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTopChatCategoryChannelsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTopChatCategoryChannelsBuilder {
    inner: TopChatCategoryChannels,
}

impl RTDTopChatCategoryChannelsBuilder {
    pub fn build(&self) -> TopChatCategoryChannels {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryChannels> for TopChatCategoryChannels {
    fn as_ref(&self) -> &TopChatCategoryChannels {
        self
    }
}

impl AsRef<TopChatCategoryChannels> for RTDTopChatCategoryChannelsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTopChatCategoryForwardChatsBuilder {
        let mut inner = TopChatCategoryForwardChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTopChatCategoryForwardChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTopChatCategoryForwardChatsBuilder {
    inner: TopChatCategoryForwardChats,
}

impl RTDTopChatCategoryForwardChatsBuilder {
    pub fn build(&self) -> TopChatCategoryForwardChats {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryForwardChats> for TopChatCategoryForwardChats {
    fn as_ref(&self) -> &TopChatCategoryForwardChats {
        self
    }
}

impl AsRef<TopChatCategoryForwardChats> for RTDTopChatCategoryForwardChatsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTopChatCategoryGroupsBuilder {
        let mut inner = TopChatCategoryGroups::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTopChatCategoryGroupsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTopChatCategoryGroupsBuilder {
    inner: TopChatCategoryGroups,
}

impl RTDTopChatCategoryGroupsBuilder {
    pub fn build(&self) -> TopChatCategoryGroups {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryGroups> for TopChatCategoryGroups {
    fn as_ref(&self) -> &TopChatCategoryGroups {
        self
    }
}

impl AsRef<TopChatCategoryGroups> for RTDTopChatCategoryGroupsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTopChatCategoryInlineBotsBuilder {
        let mut inner = TopChatCategoryInlineBots::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTopChatCategoryInlineBotsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTopChatCategoryInlineBotsBuilder {
    inner: TopChatCategoryInlineBots,
}

impl RTDTopChatCategoryInlineBotsBuilder {
    pub fn build(&self) -> TopChatCategoryInlineBots {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryInlineBots> for TopChatCategoryInlineBots {
    fn as_ref(&self) -> &TopChatCategoryInlineBots {
        self
    }
}

impl AsRef<TopChatCategoryInlineBots> for RTDTopChatCategoryInlineBotsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTopChatCategoryUsersBuilder {
        let mut inner = TopChatCategoryUsers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTopChatCategoryUsersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTopChatCategoryUsersBuilder {
    inner: TopChatCategoryUsers,
}

impl RTDTopChatCategoryUsersBuilder {
    pub fn build(&self) -> TopChatCategoryUsers {
        self.inner.clone()
    }
}

impl AsRef<TopChatCategoryUsers> for TopChatCategoryUsers {
    fn as_ref(&self) -> &TopChatCategoryUsers {
        self
    }
}

impl AsRef<TopChatCategoryUsers> for RTDTopChatCategoryUsersBuilder {
    fn as_ref(&self) -> &TopChatCategoryUsers {
        &self.inner
    }
}
