use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a type of public chats
pub trait TDPublicChatType: Debug + RObject {}

/// Describes a type of public chats
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PublicChatType {
    #[doc(hidden)]
    _Default,
    /// The chat is public, because it has username
    #[serde(rename(
        serialize = "publicChatTypeHasUsername",
        deserialize = "publicChatTypeHasUsername"
    ))]
    HasUsername(PublicChatTypeHasUsername),
    /// The chat is public, because it is a location-based supergroup
    #[serde(rename(
        serialize = "publicChatTypeIsLocationBased",
        deserialize = "publicChatTypeIsLocationBased"
    ))]
    IsLocationBased(PublicChatTypeIsLocationBased),
}

impl Default for PublicChatType {
    fn default() -> Self {
        PublicChatType::_Default
    }
}

impl RObject for PublicChatType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PublicChatType::HasUsername(t) => t.extra(),
            PublicChatType::IsLocationBased(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PublicChatType::HasUsername(t) => t.client_id(),
            PublicChatType::IsLocationBased(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PublicChatType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PublicChatType::_Default)
    }
}

impl AsRef<PublicChatType> for PublicChatType {
    fn as_ref(&self) -> &PublicChatType {
        self
    }
}

/// The chat is public, because it has username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicChatTypeHasUsername {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PublicChatTypeHasUsername {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPublicChatType for PublicChatTypeHasUsername {}

impl PublicChatTypeHasUsername {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPublicChatTypeHasUsernameBuilder {
        let mut inner = PublicChatTypeHasUsername::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPublicChatTypeHasUsernameBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPublicChatTypeHasUsernameBuilder {
    inner: PublicChatTypeHasUsername,
}

impl RTDPublicChatTypeHasUsernameBuilder {
    pub fn build(&self) -> PublicChatTypeHasUsername {
        self.inner.clone()
    }
}

impl AsRef<PublicChatTypeHasUsername> for PublicChatTypeHasUsername {
    fn as_ref(&self) -> &PublicChatTypeHasUsername {
        self
    }
}

impl AsRef<PublicChatTypeHasUsername> for RTDPublicChatTypeHasUsernameBuilder {
    fn as_ref(&self) -> &PublicChatTypeHasUsername {
        &self.inner
    }
}

/// The chat is public, because it is a location-based supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicChatTypeIsLocationBased {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PublicChatTypeIsLocationBased {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPublicChatType for PublicChatTypeIsLocationBased {}

impl PublicChatTypeIsLocationBased {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPublicChatTypeIsLocationBasedBuilder {
        let mut inner = PublicChatTypeIsLocationBased::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPublicChatTypeIsLocationBasedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPublicChatTypeIsLocationBasedBuilder {
    inner: PublicChatTypeIsLocationBased,
}

impl RTDPublicChatTypeIsLocationBasedBuilder {
    pub fn build(&self) -> PublicChatTypeIsLocationBased {
        self.inner.clone()
    }
}

impl AsRef<PublicChatTypeIsLocationBased> for PublicChatTypeIsLocationBased {
    fn as_ref(&self) -> &PublicChatTypeIsLocationBased {
        self
    }
}

impl AsRef<PublicChatTypeIsLocationBased> for RTDPublicChatTypeIsLocationBasedBuilder {
    fn as_ref(&self) -> &PublicChatTypeIsLocationBased {
        &self.inner
    }
}
