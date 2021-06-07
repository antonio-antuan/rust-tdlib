use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the type of notifications in a notification group
pub trait TDNotificationGroupType: Debug + RObject {}

/// Describes the type of notifications in a notification group
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum NotificationGroupType {
    #[doc(hidden)]
    _Default,
    /// A group containing notifications of type notificationTypeNewCall
    #[serde(rename(
        serialize = "notificationGroupTypeCalls",
        deserialize = "notificationGroupTypeCalls"
    ))]
    Calls(NotificationGroupTypeCalls),
    /// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with unread mentions of the current user, replies to their messages, or a pinned message
    #[serde(rename(
        serialize = "notificationGroupTypeMentions",
        deserialize = "notificationGroupTypeMentions"
    ))]
    Mentions(NotificationGroupTypeMentions),
    /// A group containing notifications of type notificationTypeNewMessage and notificationTypeNewPushMessage with ordinary unread messages
    #[serde(rename(
        serialize = "notificationGroupTypeMessages",
        deserialize = "notificationGroupTypeMessages"
    ))]
    Messages(NotificationGroupTypeMessages),
    /// A group containing a notification of type notificationTypeNewSecretChat
    #[serde(rename(
        serialize = "notificationGroupTypeSecretChat",
        deserialize = "notificationGroupTypeSecretChat"
    ))]
    SecretChat(NotificationGroupTypeSecretChat),
}

impl Default for NotificationGroupType {
    fn default() -> Self {
        NotificationGroupType::_Default
    }
}

impl RObject for NotificationGroupType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            NotificationGroupType::Calls(t) => t.extra(),
            NotificationGroupType::Mentions(t) => t.extra(),
            NotificationGroupType::Messages(t) => t.extra(),
            NotificationGroupType::SecretChat(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            NotificationGroupType::Calls(t) => t.client_id(),
            NotificationGroupType::Mentions(t) => t.client_id(),
            NotificationGroupType::Messages(t) => t.client_id(),
            NotificationGroupType::SecretChat(t) => t.client_id(),

            _ => None,
        }
    }
}

impl NotificationGroupType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, NotificationGroupType::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NotificationGroupTypeCalls {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationGroupType for NotificationGroupTypeCalls {}

impl NotificationGroupTypeCalls {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationGroupTypeCallsBuilder {
        let mut inner = NotificationGroupTypeCalls::default();
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NotificationGroupTypeMentions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationGroupType for NotificationGroupTypeMentions {}

impl NotificationGroupTypeMentions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationGroupTypeMentionsBuilder {
        let mut inner = NotificationGroupTypeMentions::default();
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NotificationGroupTypeMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationGroupType for NotificationGroupTypeMessages {}

impl NotificationGroupTypeMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationGroupTypeMessagesBuilder {
        let mut inner = NotificationGroupTypeMessages::default();
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for NotificationGroupTypeSecretChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDNotificationGroupType for NotificationGroupTypeSecretChat {}

impl NotificationGroupTypeSecretChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDNotificationGroupTypeSecretChatBuilder {
        let mut inner = NotificationGroupTypeSecretChat::default();
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
