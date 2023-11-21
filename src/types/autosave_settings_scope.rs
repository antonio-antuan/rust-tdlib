use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes scope of autosave settings
pub trait TDAutosaveSettingsScope: Debug + RObject {}

/// Describes scope of autosave settings
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum AutosaveSettingsScope {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Autosave settings applied to all channel chats without chat-specific settings
    #[serde(rename = "autosaveSettingsScopeChannelChats")]
    ChannelChats(AutosaveSettingsScopeChannelChats),
    /// Autosave settings applied to a chat
    #[serde(rename = "autosaveSettingsScopeChat")]
    Chat(AutosaveSettingsScopeChat),
    /// Autosave settings applied to all basic group and supergroup chats without chat-specific settings
    #[serde(rename = "autosaveSettingsScopeGroupChats")]
    GroupChats(AutosaveSettingsScopeGroupChats),
    /// Autosave settings applied to all private chats without chat-specific settings
    #[serde(rename = "autosaveSettingsScopePrivateChats")]
    PrivateChats(AutosaveSettingsScopePrivateChats),
}

impl RObject for AutosaveSettingsScope {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            AutosaveSettingsScope::ChannelChats(t) => t.extra(),
            AutosaveSettingsScope::Chat(t) => t.extra(),
            AutosaveSettingsScope::GroupChats(t) => t.extra(),
            AutosaveSettingsScope::PrivateChats(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            AutosaveSettingsScope::ChannelChats(t) => t.client_id(),
            AutosaveSettingsScope::Chat(t) => t.client_id(),
            AutosaveSettingsScope::GroupChats(t) => t.client_id(),
            AutosaveSettingsScope::PrivateChats(t) => t.client_id(),

            _ => None,
        }
    }
}

impl AutosaveSettingsScope {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, AutosaveSettingsScope::_Default)
    }
}

impl AsRef<AutosaveSettingsScope> for AutosaveSettingsScope {
    fn as_ref(&self) -> &AutosaveSettingsScope {
        self
    }
}

/// Autosave settings applied to all channel chats without chat-specific settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettingsScopeChannelChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for AutosaveSettingsScopeChannelChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAutosaveSettingsScope for AutosaveSettingsScopeChannelChats {}

impl AutosaveSettingsScopeChannelChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AutosaveSettingsScopeChannelChatsBuilder {
        let mut inner = AutosaveSettingsScopeChannelChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AutosaveSettingsScopeChannelChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct AutosaveSettingsScopeChannelChatsBuilder {
    inner: AutosaveSettingsScopeChannelChats,
}

#[deprecated]
pub type RTDAutosaveSettingsScopeChannelChatsBuilder = AutosaveSettingsScopeChannelChatsBuilder;

impl AutosaveSettingsScopeChannelChatsBuilder {
    pub fn build(&self) -> AutosaveSettingsScopeChannelChats {
        self.inner.clone()
    }
}

impl AsRef<AutosaveSettingsScopeChannelChats> for AutosaveSettingsScopeChannelChats {
    fn as_ref(&self) -> &AutosaveSettingsScopeChannelChats {
        self
    }
}

impl AsRef<AutosaveSettingsScopeChannelChats> for AutosaveSettingsScopeChannelChatsBuilder {
    fn as_ref(&self) -> &AutosaveSettingsScopeChannelChats {
        &self.inner
    }
}

/// Autosave settings applied to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettingsScopeChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
}

impl RObject for AutosaveSettingsScopeChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAutosaveSettingsScope for AutosaveSettingsScopeChat {}

impl AutosaveSettingsScopeChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AutosaveSettingsScopeChatBuilder {
        let mut inner = AutosaveSettingsScopeChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AutosaveSettingsScopeChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct AutosaveSettingsScopeChatBuilder {
    inner: AutosaveSettingsScopeChat,
}

#[deprecated]
pub type RTDAutosaveSettingsScopeChatBuilder = AutosaveSettingsScopeChatBuilder;

impl AutosaveSettingsScopeChatBuilder {
    pub fn build(&self) -> AutosaveSettingsScopeChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<AutosaveSettingsScopeChat> for AutosaveSettingsScopeChat {
    fn as_ref(&self) -> &AutosaveSettingsScopeChat {
        self
    }
}

impl AsRef<AutosaveSettingsScopeChat> for AutosaveSettingsScopeChatBuilder {
    fn as_ref(&self) -> &AutosaveSettingsScopeChat {
        &self.inner
    }
}

/// Autosave settings applied to all basic group and supergroup chats without chat-specific settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettingsScopeGroupChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for AutosaveSettingsScopeGroupChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAutosaveSettingsScope for AutosaveSettingsScopeGroupChats {}

impl AutosaveSettingsScopeGroupChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AutosaveSettingsScopeGroupChatsBuilder {
        let mut inner = AutosaveSettingsScopeGroupChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AutosaveSettingsScopeGroupChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct AutosaveSettingsScopeGroupChatsBuilder {
    inner: AutosaveSettingsScopeGroupChats,
}

#[deprecated]
pub type RTDAutosaveSettingsScopeGroupChatsBuilder = AutosaveSettingsScopeGroupChatsBuilder;

impl AutosaveSettingsScopeGroupChatsBuilder {
    pub fn build(&self) -> AutosaveSettingsScopeGroupChats {
        self.inner.clone()
    }
}

impl AsRef<AutosaveSettingsScopeGroupChats> for AutosaveSettingsScopeGroupChats {
    fn as_ref(&self) -> &AutosaveSettingsScopeGroupChats {
        self
    }
}

impl AsRef<AutosaveSettingsScopeGroupChats> for AutosaveSettingsScopeGroupChatsBuilder {
    fn as_ref(&self) -> &AutosaveSettingsScopeGroupChats {
        &self.inner
    }
}

/// Autosave settings applied to all private chats without chat-specific settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutosaveSettingsScopePrivateChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for AutosaveSettingsScopePrivateChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAutosaveSettingsScope for AutosaveSettingsScopePrivateChats {}

impl AutosaveSettingsScopePrivateChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AutosaveSettingsScopePrivateChatsBuilder {
        let mut inner = AutosaveSettingsScopePrivateChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AutosaveSettingsScopePrivateChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct AutosaveSettingsScopePrivateChatsBuilder {
    inner: AutosaveSettingsScopePrivateChats,
}

#[deprecated]
pub type RTDAutosaveSettingsScopePrivateChatsBuilder = AutosaveSettingsScopePrivateChatsBuilder;

impl AutosaveSettingsScopePrivateChatsBuilder {
    pub fn build(&self) -> AutosaveSettingsScopePrivateChats {
        self.inner.clone()
    }
}

impl AsRef<AutosaveSettingsScopePrivateChats> for AutosaveSettingsScopePrivateChats {
    fn as_ref(&self) -> &AutosaveSettingsScopePrivateChats {
        self
    }
}

impl AsRef<AutosaveSettingsScopePrivateChats> for AutosaveSettingsScopePrivateChatsBuilder {
    fn as_ref(&self) -> &AutosaveSettingsScopePrivateChats {
        &self.inner
    }
}
