use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the target chat to be opened
pub trait TDTargetChat: Debug + RObject {}

/// Describes the target chat to be opened
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TargetChat {
    #[doc(hidden)]
    _Default,
    /// The chat needs to be chosen by the user among chats of the specified types
    #[serde(rename(deserialize = "targetChatChosen"))]
    Chosen(TargetChatChosen),
    /// The currently opened chat needs to be kept
    #[serde(rename(deserialize = "targetChatCurrent"))]
    Current(TargetChatCurrent),
    /// The chat needs to be open with the provided internal link
    #[serde(rename(deserialize = "targetChatInternalLink"))]
    InternalLink(TargetChatInternalLink),
}

impl Default for TargetChat {
    fn default() -> Self {
        TargetChat::_Default
    }
}

impl RObject for TargetChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            TargetChat::Chosen(t) => t.extra(),
            TargetChat::Current(t) => t.extra(),
            TargetChat::InternalLink(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            TargetChat::Chosen(t) => t.client_id(),
            TargetChat::Current(t) => t.client_id(),
            TargetChat::InternalLink(t) => t.client_id(),

            _ => None,
        }
    }
}

impl TargetChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, TargetChat::_Default)
    }
}

impl AsRef<TargetChat> for TargetChat {
    fn as_ref(&self) -> &TargetChat {
        self
    }
}

/// The chat needs to be chosen by the user among chats of the specified types
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetChatChosen {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if private chats with ordinary users are allowed

    #[serde(default)]
    allow_user_chats: bool,
    /// True, if private chats with other bots are allowed

    #[serde(default)]
    allow_bot_chats: bool,
    /// True, if basic group and supergroup chats are allowed

    #[serde(default)]
    allow_group_chats: bool,
    /// True, if channel chats are allowed

    #[serde(default)]
    allow_channel_chats: bool,
}

impl RObject for TargetChatChosen {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTargetChat for TargetChatChosen {}

impl TargetChatChosen {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTargetChatChosenBuilder {
        let mut inner = TargetChatChosen::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTargetChatChosenBuilder { inner }
    }

    pub fn allow_user_chats(&self) -> bool {
        self.allow_user_chats
    }

    pub fn allow_bot_chats(&self) -> bool {
        self.allow_bot_chats
    }

    pub fn allow_group_chats(&self) -> bool {
        self.allow_group_chats
    }

    pub fn allow_channel_chats(&self) -> bool {
        self.allow_channel_chats
    }
}

#[doc(hidden)]
pub struct RTDTargetChatChosenBuilder {
    inner: TargetChatChosen,
}

impl RTDTargetChatChosenBuilder {
    pub fn build(&self) -> TargetChatChosen {
        self.inner.clone()
    }

    pub fn allow_user_chats(&mut self, allow_user_chats: bool) -> &mut Self {
        self.inner.allow_user_chats = allow_user_chats;
        self
    }

    pub fn allow_bot_chats(&mut self, allow_bot_chats: bool) -> &mut Self {
        self.inner.allow_bot_chats = allow_bot_chats;
        self
    }

    pub fn allow_group_chats(&mut self, allow_group_chats: bool) -> &mut Self {
        self.inner.allow_group_chats = allow_group_chats;
        self
    }

    pub fn allow_channel_chats(&mut self, allow_channel_chats: bool) -> &mut Self {
        self.inner.allow_channel_chats = allow_channel_chats;
        self
    }
}

impl AsRef<TargetChatChosen> for TargetChatChosen {
    fn as_ref(&self) -> &TargetChatChosen {
        self
    }
}

impl AsRef<TargetChatChosen> for RTDTargetChatChosenBuilder {
    fn as_ref(&self) -> &TargetChatChosen {
        &self.inner
    }
}

/// The currently opened chat needs to be kept
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetChatCurrent {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TargetChatCurrent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTargetChat for TargetChatCurrent {}

impl TargetChatCurrent {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTargetChatCurrentBuilder {
        let mut inner = TargetChatCurrent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTargetChatCurrentBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTargetChatCurrentBuilder {
    inner: TargetChatCurrent,
}

impl RTDTargetChatCurrentBuilder {
    pub fn build(&self) -> TargetChatCurrent {
        self.inner.clone()
    }
}

impl AsRef<TargetChatCurrent> for TargetChatCurrent {
    fn as_ref(&self) -> &TargetChatCurrent {
        self
    }
}

impl AsRef<TargetChatCurrent> for RTDTargetChatCurrentBuilder {
    fn as_ref(&self) -> &TargetChatCurrent {
        &self.inner
    }
}

/// The chat needs to be open with the provided internal link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetChatInternalLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// An internal link pointing to the chat

    #[serde(skip_serializing_if = "InternalLinkType::_is_default")]
    link: Box<InternalLinkType>,
}

impl RObject for TargetChatInternalLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTargetChat for TargetChatInternalLink {}

impl TargetChatInternalLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTargetChatInternalLinkBuilder {
        let mut inner = TargetChatInternalLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTargetChatInternalLinkBuilder { inner }
    }

    pub fn link(&self) -> &Box<InternalLinkType> {
        &self.link
    }
}

#[doc(hidden)]
pub struct RTDTargetChatInternalLinkBuilder {
    inner: TargetChatInternalLink,
}

impl RTDTargetChatInternalLinkBuilder {
    pub fn build(&self) -> TargetChatInternalLink {
        self.inner.clone()
    }

    pub fn link<T: AsRef<Box<InternalLinkType>>>(&mut self, link: T) -> &mut Self {
        self.inner.link = link.as_ref().clone();
        self
    }
}

impl AsRef<TargetChatInternalLink> for TargetChatInternalLink {
    fn as_ref(&self) -> &TargetChatInternalLink {
        self
    }
}

impl AsRef<TargetChatInternalLink> for RTDTargetChatInternalLinkBuilder {
    fn as_ref(&self) -> &TargetChatInternalLink {
        &self.inner
    }
}
