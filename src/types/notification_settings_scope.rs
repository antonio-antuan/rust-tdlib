use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the types of chats to which notification settings are applied
pub trait TDNotificationSettingsScope: Debug + RObject {}

/// Describes the types of chats to which notification settings are applied
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NotificationSettingsScope {
    #[doc(hidden)]
    _Default(()),
    /// Notification settings applied to all channels when the corresponding chat setting has a default value
    ChannelChats(NotificationSettingsScopeChannelChats),
    /// Notification settings applied to all basic groups and supergroups when the corresponding chat setting has a default value
    GroupChats(NotificationSettingsScopeGroupChats),
    /// Notification settings applied to all private and secret chats when the corresponding chat setting has a default value
    PrivateChats(NotificationSettingsScopePrivateChats),
}

impl Default for NotificationSettingsScope {
    fn default() -> Self {
        NotificationSettingsScope::_Default(())
    }
}

impl<'de> Deserialize<'de> for NotificationSettingsScope {
    fn deserialize<D>(deserializer: D) -> Result<NotificationSettingsScope, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          NotificationSettingsScope,
          (notificationSettingsScopeChannelChats, ChannelChats);
          (notificationSettingsScopeGroupChats, GroupChats);
          (notificationSettingsScopePrivateChats, PrivateChats);

        )(deserializer)
    }
}

impl RObject for NotificationSettingsScope {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            NotificationSettingsScope::ChannelChats(t) => t.td_name(),
            NotificationSettingsScope::GroupChats(t) => t.td_name(),
            NotificationSettingsScope::PrivateChats(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            NotificationSettingsScope::ChannelChats(t) => t.extra(),
            NotificationSettingsScope::GroupChats(t) => t.extra(),
            NotificationSettingsScope::PrivateChats(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl NotificationSettingsScope {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, NotificationSettingsScope::_Default(_))
    }
}

impl AsRef<NotificationSettingsScope> for NotificationSettingsScope {
    fn as_ref(&self) -> &NotificationSettingsScope {
        self
    }
}

/// Notification settings applied to all channels when the corresponding chat setting has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSettingsScopeChannelChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NotificationSettingsScopeChannelChats {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "notificationSettingsScopeChannelChats"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNotificationSettingsScope for NotificationSettingsScopeChannelChats {}

impl NotificationSettingsScopeChannelChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationSettingsScopeChannelChatsBuilder {
        let mut inner = NotificationSettingsScopeChannelChats::default();
        inner.td_name = "notificationSettingsScopeChannelChats".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNotificationSettingsScopeChannelChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNotificationSettingsScopeChannelChatsBuilder {
    inner: NotificationSettingsScopeChannelChats,
}

impl RTDNotificationSettingsScopeChannelChatsBuilder {
    pub fn build(&self) -> NotificationSettingsScopeChannelChats {
        self.inner.clone()
    }
}

impl AsRef<NotificationSettingsScopeChannelChats> for NotificationSettingsScopeChannelChats {
    fn as_ref(&self) -> &NotificationSettingsScopeChannelChats {
        self
    }
}

impl AsRef<NotificationSettingsScopeChannelChats>
    for RTDNotificationSettingsScopeChannelChatsBuilder
{
    fn as_ref(&self) -> &NotificationSettingsScopeChannelChats {
        &self.inner
    }
}

/// Notification settings applied to all basic groups and supergroups when the corresponding chat setting has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSettingsScopeGroupChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NotificationSettingsScopeGroupChats {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "notificationSettingsScopeGroupChats"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNotificationSettingsScope for NotificationSettingsScopeGroupChats {}

impl NotificationSettingsScopeGroupChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationSettingsScopeGroupChatsBuilder {
        let mut inner = NotificationSettingsScopeGroupChats::default();
        inner.td_name = "notificationSettingsScopeGroupChats".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNotificationSettingsScopeGroupChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNotificationSettingsScopeGroupChatsBuilder {
    inner: NotificationSettingsScopeGroupChats,
}

impl RTDNotificationSettingsScopeGroupChatsBuilder {
    pub fn build(&self) -> NotificationSettingsScopeGroupChats {
        self.inner.clone()
    }
}

impl AsRef<NotificationSettingsScopeGroupChats> for NotificationSettingsScopeGroupChats {
    fn as_ref(&self) -> &NotificationSettingsScopeGroupChats {
        self
    }
}

impl AsRef<NotificationSettingsScopeGroupChats> for RTDNotificationSettingsScopeGroupChatsBuilder {
    fn as_ref(&self) -> &NotificationSettingsScopeGroupChats {
        &self.inner
    }
}

/// Notification settings applied to all private and secret chats when the corresponding chat setting has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSettingsScopePrivateChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NotificationSettingsScopePrivateChats {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "notificationSettingsScopePrivateChats"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNotificationSettingsScope for NotificationSettingsScopePrivateChats {}

impl NotificationSettingsScopePrivateChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationSettingsScopePrivateChatsBuilder {
        let mut inner = NotificationSettingsScopePrivateChats::default();
        inner.td_name = "notificationSettingsScopePrivateChats".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNotificationSettingsScopePrivateChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNotificationSettingsScopePrivateChatsBuilder {
    inner: NotificationSettingsScopePrivateChats,
}

impl RTDNotificationSettingsScopePrivateChatsBuilder {
    pub fn build(&self) -> NotificationSettingsScopePrivateChats {
        self.inner.clone()
    }
}

impl AsRef<NotificationSettingsScopePrivateChats> for NotificationSettingsScopePrivateChats {
    fn as_ref(&self) -> &NotificationSettingsScopePrivateChats {
        self
    }
}

impl AsRef<NotificationSettingsScopePrivateChats>
    for RTDNotificationSettingsScopePrivateChatsBuilder
{
    fn as_ref(&self) -> &NotificationSettingsScopePrivateChats {
        &self.inner
    }
}
