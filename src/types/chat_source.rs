use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a reason why an external chat is shown in a chat list
pub trait TDChatSource: Debug + RObject {}

/// Describes a reason why an external chat is shown in a chat list
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatSource {
    #[doc(hidden)]
    _Default,
    /// The chat is sponsored by the user's MTProxy server
    #[serde(rename(
        serialize = "chatSourceMtprotoProxy",
        deserialize = "chatSourceMtprotoProxy"
    ))]
    MtprotoProxy(ChatSourceMtprotoProxy),
    /// The chat contains a public service announcement
    #[serde(rename(
        serialize = "chatSourcePublicServiceAnnouncement",
        deserialize = "chatSourcePublicServiceAnnouncement"
    ))]
    PublicServiceAnnouncement(ChatSourcePublicServiceAnnouncement),
}

impl Default for ChatSource {
    fn default() -> Self {
        ChatSource::_Default
    }
}

impl RObject for ChatSource {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatSource::MtprotoProxy(t) => t.extra(),
            ChatSource::PublicServiceAnnouncement(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatSource::MtprotoProxy(t) => t.client_id(),
            ChatSource::PublicServiceAnnouncement(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatSource {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatSource::_Default)
    }
}

impl AsRef<ChatSource> for ChatSource {
    fn as_ref(&self) -> &ChatSource {
        self
    }
}

/// The chat is sponsored by the user's MTProxy server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatSourceMtprotoProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatSourceMtprotoProxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatSource for ChatSourceMtprotoProxy {}

impl ChatSourceMtprotoProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatSourceMtprotoProxyBuilder {
        let mut inner = ChatSourceMtprotoProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatSourceMtprotoProxyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatSourceMtprotoProxyBuilder {
    inner: ChatSourceMtprotoProxy,
}

impl RTDChatSourceMtprotoProxyBuilder {
    pub fn build(&self) -> ChatSourceMtprotoProxy {
        self.inner.clone()
    }
}

impl AsRef<ChatSourceMtprotoProxy> for ChatSourceMtprotoProxy {
    fn as_ref(&self) -> &ChatSourceMtprotoProxy {
        self
    }
}

impl AsRef<ChatSourceMtprotoProxy> for RTDChatSourceMtprotoProxyBuilder {
    fn as_ref(&self) -> &ChatSourceMtprotoProxy {
        &self.inner
    }
}

/// The chat contains a public service announcement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatSourcePublicServiceAnnouncement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The type of the announcement

    #[serde(rename(serialize = "type", deserialize = "type"))]
    type_: String,
    /// The text of the announcement
    text: String,
}

impl RObject for ChatSourcePublicServiceAnnouncement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatSource for ChatSourcePublicServiceAnnouncement {}

impl ChatSourcePublicServiceAnnouncement {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatSourcePublicServiceAnnouncementBuilder {
        let mut inner = ChatSourcePublicServiceAnnouncement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatSourcePublicServiceAnnouncementBuilder { inner }
    }

    pub fn type_(&self) -> &String {
        &self.type_
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDChatSourcePublicServiceAnnouncementBuilder {
    inner: ChatSourcePublicServiceAnnouncement,
}

impl RTDChatSourcePublicServiceAnnouncementBuilder {
    pub fn build(&self) -> ChatSourcePublicServiceAnnouncement {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().to_string();
        self
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }
}

impl AsRef<ChatSourcePublicServiceAnnouncement> for ChatSourcePublicServiceAnnouncement {
    fn as_ref(&self) -> &ChatSourcePublicServiceAnnouncement {
        self
    }
}

impl AsRef<ChatSourcePublicServiceAnnouncement> for RTDChatSourcePublicServiceAnnouncementBuilder {
    fn as_ref(&self) -> &ChatSourcePublicServiceAnnouncement {
        &self.inner
    }
}
