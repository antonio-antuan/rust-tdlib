use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the type of notifications in a notification group
pub trait TDNotificationGroupType: Debug + RObject {}

/// Describes the type of notifications in a notification group
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NotificationGroupType {
    #[doc(hidden)]
    _Default(()),
    /// A group containing notifications of type notificationTypeNewCall
    Calls(NotificationGroupTypeCalls),
    /// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with unread mentions of the current user, replies to their messages, or a pinned message
    Mentions(NotificationGroupTypeMentions),
    /// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with ordinary unread messages
    Messages(NotificationGroupTypeMessages),
    /// A group containing a notification of type notificationTypeNewSecretChat
    SecretChat(NotificationGroupTypeSecretChat),
}

impl Default for NotificationGroupType {
    fn default() -> Self {
        NotificationGroupType::_Default(())
    }
}

impl<'de> Deserialize<'de> for NotificationGroupType {
    fn deserialize<D>(deserializer: D) -> Result<NotificationGroupType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          NotificationGroupType,
          (notificationGroupTypeCalls, Calls);
          (notificationGroupTypeMentions, Mentions);
          (notificationGroupTypeMessages, Messages);
          (notificationGroupTypeSecretChat, SecretChat);

        )(deserializer)
    }
}

impl RObject for NotificationGroupType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            NotificationGroupType::Calls(t) => t.td_name(),
            NotificationGroupType::Mentions(t) => t.td_name(),
            NotificationGroupType::Messages(t) => t.td_name(),
            NotificationGroupType::SecretChat(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            NotificationGroupType::Calls(t) => t.extra(),
            NotificationGroupType::Mentions(t) => t.extra(),
            NotificationGroupType::Messages(t) => t.extra(),
            NotificationGroupType::SecretChat(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl NotificationGroupType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, NotificationGroupType::_Default(_))
    }
}

impl AsRef<NotificationGroupType> for NotificationGroupType {
    fn as_ref(&self) -> &NotificationGroupType {
        self
    }
}

/// A group containing notifications of type notificationTypeNewCall
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationGroupTypeCalls {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NotificationGroupTypeCalls {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "notificationGroupTypeCalls"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNotificationGroupType for NotificationGroupTypeCalls {}

impl NotificationGroupTypeCalls {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationGroupTypeCallsBuilder {
        let mut inner = NotificationGroupTypeCalls::default();
        inner.td_name = "notificationGroupTypeCalls".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNotificationGroupTypeCallsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNotificationGroupTypeCallsBuilder {
    inner: NotificationGroupTypeCalls,
}

impl RTDNotificationGroupTypeCallsBuilder {
    pub fn build(&self) -> NotificationGroupTypeCalls {
        self.inner.clone()
    }
}

impl AsRef<NotificationGroupTypeCalls> for NotificationGroupTypeCalls {
    fn as_ref(&self) -> &NotificationGroupTypeCalls {
        self
    }
}

impl AsRef<NotificationGroupTypeCalls> for RTDNotificationGroupTypeCallsBuilder {
    fn as_ref(&self) -> &NotificationGroupTypeCalls {
        &self.inner
    }
}

/// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with unread mentions of the current user, replies to their messages, or a pinned message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationGroupTypeMentions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NotificationGroupTypeMentions {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "notificationGroupTypeMentions"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNotificationGroupType for NotificationGroupTypeMentions {}

impl NotificationGroupTypeMentions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationGroupTypeMentionsBuilder {
        let mut inner = NotificationGroupTypeMentions::default();
        inner.td_name = "notificationGroupTypeMentions".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNotificationGroupTypeMentionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNotificationGroupTypeMentionsBuilder {
    inner: NotificationGroupTypeMentions,
}

impl RTDNotificationGroupTypeMentionsBuilder {
    pub fn build(&self) -> NotificationGroupTypeMentions {
        self.inner.clone()
    }
}

impl AsRef<NotificationGroupTypeMentions> for NotificationGroupTypeMentions {
    fn as_ref(&self) -> &NotificationGroupTypeMentions {
        self
    }
}

impl AsRef<NotificationGroupTypeMentions> for RTDNotificationGroupTypeMentionsBuilder {
    fn as_ref(&self) -> &NotificationGroupTypeMentions {
        &self.inner
    }
}

/// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with ordinary unread messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationGroupTypeMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NotificationGroupTypeMessages {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "notificationGroupTypeMessages"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNotificationGroupType for NotificationGroupTypeMessages {}

impl NotificationGroupTypeMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationGroupTypeMessagesBuilder {
        let mut inner = NotificationGroupTypeMessages::default();
        inner.td_name = "notificationGroupTypeMessages".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNotificationGroupTypeMessagesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNotificationGroupTypeMessagesBuilder {
    inner: NotificationGroupTypeMessages,
}

impl RTDNotificationGroupTypeMessagesBuilder {
    pub fn build(&self) -> NotificationGroupTypeMessages {
        self.inner.clone()
    }
}

impl AsRef<NotificationGroupTypeMessages> for NotificationGroupTypeMessages {
    fn as_ref(&self) -> &NotificationGroupTypeMessages {
        self
    }
}

impl AsRef<NotificationGroupTypeMessages> for RTDNotificationGroupTypeMessagesBuilder {
    fn as_ref(&self) -> &NotificationGroupTypeMessages {
        &self.inner
    }
}

/// A group containing a notification of type notificationTypeNewSecretChat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationGroupTypeSecretChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for NotificationGroupTypeSecretChat {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "notificationGroupTypeSecretChat"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDNotificationGroupType for NotificationGroupTypeSecretChat {}

impl NotificationGroupTypeSecretChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationGroupTypeSecretChatBuilder {
        let mut inner = NotificationGroupTypeSecretChat::default();
        inner.td_name = "notificationGroupTypeSecretChat".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDNotificationGroupTypeSecretChatBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDNotificationGroupTypeSecretChatBuilder {
    inner: NotificationGroupTypeSecretChat,
}

impl RTDNotificationGroupTypeSecretChatBuilder {
    pub fn build(&self) -> NotificationGroupTypeSecretChat {
        self.inner.clone()
    }
}

impl AsRef<NotificationGroupTypeSecretChat> for NotificationGroupTypeSecretChat {
    fn as_ref(&self) -> &NotificationGroupTypeSecretChat {
        self
    }
}

impl AsRef<NotificationGroupTypeSecretChat> for RTDNotificationGroupTypeSecretChatBuilder {
    fn as_ref(&self) -> &NotificationGroupTypeSecretChat {
        &self.inner
    }
}
