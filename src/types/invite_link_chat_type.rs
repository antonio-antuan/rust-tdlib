use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the type of a chat to which points an invite link
pub trait TDInviteLinkChatType: Debug + RObject {}

/// Describes the type of a chat to which points an invite link
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum InviteLinkChatType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The link is an invite link for a basic group
    #[serde(rename = "inviteLinkChatTypeBasicGroup")]
    BasicGroup(InviteLinkChatTypeBasicGroup),
    /// The link is an invite link for a channel
    #[serde(rename = "inviteLinkChatTypeChannel")]
    Channel(InviteLinkChatTypeChannel),
    /// The link is an invite link for a supergroup
    #[serde(rename = "inviteLinkChatTypeSupergroup")]
    Supergroup(InviteLinkChatTypeSupergroup),
}

impl RObject for InviteLinkChatType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InviteLinkChatType::BasicGroup(t) => t.extra(),
            InviteLinkChatType::Channel(t) => t.extra(),
            InviteLinkChatType::Supergroup(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InviteLinkChatType::BasicGroup(t) => t.client_id(),
            InviteLinkChatType::Channel(t) => t.client_id(),
            InviteLinkChatType::Supergroup(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InviteLinkChatType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InviteLinkChatType::_Default)
    }
}

impl AsRef<InviteLinkChatType> for InviteLinkChatType {
    fn as_ref(&self) -> &InviteLinkChatType {
        self
    }
}

/// The link is an invite link for a basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteLinkChatTypeBasicGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InviteLinkChatTypeBasicGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInviteLinkChatType for InviteLinkChatTypeBasicGroup {}

impl InviteLinkChatTypeBasicGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InviteLinkChatTypeBasicGroupBuilder {
        let mut inner = InviteLinkChatTypeBasicGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InviteLinkChatTypeBasicGroupBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InviteLinkChatTypeBasicGroupBuilder {
    inner: InviteLinkChatTypeBasicGroup,
}

#[deprecated]
pub type RTDInviteLinkChatTypeBasicGroupBuilder = InviteLinkChatTypeBasicGroupBuilder;

impl InviteLinkChatTypeBasicGroupBuilder {
    pub fn build(&self) -> InviteLinkChatTypeBasicGroup {
        self.inner.clone()
    }
}

impl AsRef<InviteLinkChatTypeBasicGroup> for InviteLinkChatTypeBasicGroup {
    fn as_ref(&self) -> &InviteLinkChatTypeBasicGroup {
        self
    }
}

impl AsRef<InviteLinkChatTypeBasicGroup> for InviteLinkChatTypeBasicGroupBuilder {
    fn as_ref(&self) -> &InviteLinkChatTypeBasicGroup {
        &self.inner
    }
}

/// The link is an invite link for a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteLinkChatTypeChannel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InviteLinkChatTypeChannel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInviteLinkChatType for InviteLinkChatTypeChannel {}

impl InviteLinkChatTypeChannel {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InviteLinkChatTypeChannelBuilder {
        let mut inner = InviteLinkChatTypeChannel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InviteLinkChatTypeChannelBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InviteLinkChatTypeChannelBuilder {
    inner: InviteLinkChatTypeChannel,
}

#[deprecated]
pub type RTDInviteLinkChatTypeChannelBuilder = InviteLinkChatTypeChannelBuilder;

impl InviteLinkChatTypeChannelBuilder {
    pub fn build(&self) -> InviteLinkChatTypeChannel {
        self.inner.clone()
    }
}

impl AsRef<InviteLinkChatTypeChannel> for InviteLinkChatTypeChannel {
    fn as_ref(&self) -> &InviteLinkChatTypeChannel {
        self
    }
}

impl AsRef<InviteLinkChatTypeChannel> for InviteLinkChatTypeChannelBuilder {
    fn as_ref(&self) -> &InviteLinkChatTypeChannel {
        &self.inner
    }
}

/// The link is an invite link for a supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InviteLinkChatTypeSupergroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InviteLinkChatTypeSupergroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInviteLinkChatType for InviteLinkChatTypeSupergroup {}

impl InviteLinkChatTypeSupergroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InviteLinkChatTypeSupergroupBuilder {
        let mut inner = InviteLinkChatTypeSupergroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InviteLinkChatTypeSupergroupBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InviteLinkChatTypeSupergroupBuilder {
    inner: InviteLinkChatTypeSupergroup,
}

#[deprecated]
pub type RTDInviteLinkChatTypeSupergroupBuilder = InviteLinkChatTypeSupergroupBuilder;

impl InviteLinkChatTypeSupergroupBuilder {
    pub fn build(&self) -> InviteLinkChatTypeSupergroup {
        self.inner.clone()
    }
}

impl AsRef<InviteLinkChatTypeSupergroup> for InviteLinkChatTypeSupergroup {
    fn as_ref(&self) -> &InviteLinkChatTypeSupergroup {
        self
    }
}

impl AsRef<InviteLinkChatTypeSupergroup> for InviteLinkChatTypeSupergroupBuilder {
    fn as_ref(&self) -> &InviteLinkChatTypeSupergroup {
        &self.inner
    }
}
