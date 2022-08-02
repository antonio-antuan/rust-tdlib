use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about a file with messages exported from another app
pub trait TDMessageFileType: Debug + RObject {}

/// Contains information about a file with messages exported from another app
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageFileType {
    #[doc(hidden)]
    _Default,
    /// Returns information about a file with messages exported from another app
    #[serde(rename = "getMessageFileType")]
    GetMessageFileType(GetMessageFileType),
    /// The messages was exported from a group chat
    #[serde(rename = "messageFileTypeGroup")]
    Group(MessageFileTypeGroup),
    /// The messages was exported from a private chat
    #[serde(rename = "messageFileTypePrivate")]
    Private(MessageFileTypePrivate),
    /// The messages was exported from a chat of unknown type
    #[serde(rename = "messageFileTypeUnknown")]
    Unknown(MessageFileTypeUnknown),
}

impl Default for MessageFileType {
    fn default() -> Self {
        MessageFileType::_Default
    }
}

impl RObject for MessageFileType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            MessageFileType::GetMessageFileType(t) => t.extra(),
            MessageFileType::Group(t) => t.extra(),
            MessageFileType::Private(t) => t.extra(),
            MessageFileType::Unknown(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            MessageFileType::GetMessageFileType(t) => t.client_id(),
            MessageFileType::Group(t) => t.client_id(),
            MessageFileType::Private(t) => t.client_id(),
            MessageFileType::Unknown(t) => t.client_id(),

            _ => None,
        }
    }
}

impl MessageFileType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, MessageFileType::_Default)
    }
}

impl AsRef<MessageFileType> for MessageFileType {
    fn as_ref(&self) -> &MessageFileType {
        self
    }
}

/// The messages was exported from a group chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageFileTypeGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Title of the group chat; may be empty if unrecognized

    #[serde(default)]
    title: String,
}

impl RObject for MessageFileTypeGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageFileType for MessageFileTypeGroup {}

impl MessageFileTypeGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageFileTypeGroupBuilder {
        let mut inner = MessageFileTypeGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageFileTypeGroupBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct MessageFileTypeGroupBuilder {
    inner: MessageFileTypeGroup,
}

#[deprecated]
pub type RTDMessageFileTypeGroupBuilder = MessageFileTypeGroupBuilder;

impl MessageFileTypeGroupBuilder {
    pub fn build(&self) -> MessageFileTypeGroup {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }
}

impl AsRef<MessageFileTypeGroup> for MessageFileTypeGroup {
    fn as_ref(&self) -> &MessageFileTypeGroup {
        self
    }
}

impl AsRef<MessageFileTypeGroup> for MessageFileTypeGroupBuilder {
    fn as_ref(&self) -> &MessageFileTypeGroup {
        &self.inner
    }
}

/// The messages was exported from a private chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageFileTypePrivate {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the other party; may be empty if unrecognized

    #[serde(default)]
    name: String,
}

impl RObject for MessageFileTypePrivate {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageFileType for MessageFileTypePrivate {}

impl MessageFileTypePrivate {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageFileTypePrivateBuilder {
        let mut inner = MessageFileTypePrivate::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageFileTypePrivateBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct MessageFileTypePrivateBuilder {
    inner: MessageFileTypePrivate,
}

#[deprecated]
pub type RTDMessageFileTypePrivateBuilder = MessageFileTypePrivateBuilder;

impl MessageFileTypePrivateBuilder {
    pub fn build(&self) -> MessageFileTypePrivate {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<MessageFileTypePrivate> for MessageFileTypePrivate {
    fn as_ref(&self) -> &MessageFileTypePrivate {
        self
    }
}

impl AsRef<MessageFileTypePrivate> for MessageFileTypePrivateBuilder {
    fn as_ref(&self) -> &MessageFileTypePrivate {
        &self.inner
    }
}

/// The messages was exported from a chat of unknown type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageFileTypeUnknown {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for MessageFileTypeUnknown {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageFileType for MessageFileTypeUnknown {}

impl MessageFileTypeUnknown {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageFileTypeUnknownBuilder {
        let mut inner = MessageFileTypeUnknown::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageFileTypeUnknownBuilder { inner }
    }
}

#[doc(hidden)]
pub struct MessageFileTypeUnknownBuilder {
    inner: MessageFileTypeUnknown,
}

#[deprecated]
pub type RTDMessageFileTypeUnknownBuilder = MessageFileTypeUnknownBuilder;

impl MessageFileTypeUnknownBuilder {
    pub fn build(&self) -> MessageFileTypeUnknown {
        self.inner.clone()
    }
}

impl AsRef<MessageFileTypeUnknown> for MessageFileTypeUnknown {
    fn as_ref(&self) -> &MessageFileTypeUnknown {
        self
    }
}

impl AsRef<MessageFileTypeUnknown> for MessageFileTypeUnknownBuilder {
    fn as_ref(&self) -> &MessageFileTypeUnknown {
        &self.inner
    }
}
