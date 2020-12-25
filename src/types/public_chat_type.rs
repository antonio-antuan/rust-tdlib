use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a type of public chats
pub trait TDPublicChatType: Debug + RObject {}

/// Describes a type of public chats
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PublicChatType {
    #[doc(hidden)]
    _Default(()),
    /// The chat is public, because it has username
    HasUsername(PublicChatTypeHasUsername),
    /// The chat is public, because it is a location-based supergroup
    IsLocationBased(PublicChatTypeIsLocationBased),
}

impl Default for PublicChatType {
    fn default() -> Self {
        PublicChatType::_Default(())
    }
}

impl<'de> Deserialize<'de> for PublicChatType {
    fn deserialize<D>(deserializer: D) -> Result<PublicChatType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          PublicChatType,
          (publicChatTypeHasUsername, HasUsername);
          (publicChatTypeIsLocationBased, IsLocationBased);

        )(deserializer)
    }
}

impl RObject for PublicChatType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            PublicChatType::HasUsername(t) => t.td_name(),
            PublicChatType::IsLocationBased(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            PublicChatType::HasUsername(t) => t.extra(),
            PublicChatType::IsLocationBased(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl PublicChatType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PublicChatType::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PublicChatTypeHasUsername {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "publicChatTypeHasUsername"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPublicChatType for PublicChatTypeHasUsername {}

impl PublicChatTypeHasUsername {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPublicChatTypeHasUsernameBuilder {
        let mut inner = PublicChatTypeHasUsername::default();
        inner.td_name = "publicChatTypeHasUsername".to_string();
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PublicChatTypeIsLocationBased {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "publicChatTypeIsLocationBased"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPublicChatType for PublicChatTypeIsLocationBased {}

impl PublicChatTypeIsLocationBased {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPublicChatTypeIsLocationBasedBuilder {
        let mut inner = PublicChatTypeIsLocationBased::default();
        inner.td_name = "publicChatTypeIsLocationBased".to_string();
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
