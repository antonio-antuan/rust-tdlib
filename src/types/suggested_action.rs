use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes an action suggested to the current user
pub trait TDSuggestedAction: Debug + RObject {}

/// Describes an action suggested to the current user
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedAction {
    #[doc(hidden)]
    _Default,
    /// Suggests the user to check authorization phone number and change the phone number if it is inaccessible
    #[serde(rename(
        serialize = "suggestedActionCheckPhoneNumber",
        deserialize = "suggestedActionCheckPhoneNumber"
    ))]
    CheckPhoneNumber(SuggestedActionCheckPhoneNumber),
    /// Suggests the user to enable "archive_and_mute_new_chats_from_unknown_users" option
    #[serde(rename(
        serialize = "suggestedActionEnableArchiveAndMuteNewChats",
        deserialize = "suggestedActionEnableArchiveAndMuteNewChats"
    ))]
    EnableArchiveAndMuteNewChats(SuggestedActionEnableArchiveAndMuteNewChats),
}

impl Default for SuggestedAction {
    fn default() -> Self {
        SuggestedAction::_Default
    }
}

impl RObject for SuggestedAction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            SuggestedAction::CheckPhoneNumber(t) => t.extra(),
            SuggestedAction::EnableArchiveAndMuteNewChats(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            SuggestedAction::CheckPhoneNumber(t) => t.client_id(),
            SuggestedAction::EnableArchiveAndMuteNewChats(t) => t.client_id(),

            _ => None,
        }
    }
}

impl SuggestedAction {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, SuggestedAction::_Default)
    }
}

impl AsRef<SuggestedAction> for SuggestedAction {
    fn as_ref(&self) -> &SuggestedAction {
        self
    }
}

/// Suggests the user to check authorization phone number and change the phone number if it is inaccessible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionCheckPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SuggestedActionCheckPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSuggestedAction for SuggestedActionCheckPhoneNumber {}

impl SuggestedActionCheckPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSuggestedActionCheckPhoneNumberBuilder {
        let mut inner = SuggestedActionCheckPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSuggestedActionCheckPhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSuggestedActionCheckPhoneNumberBuilder {
    inner: SuggestedActionCheckPhoneNumber,
}

impl RTDSuggestedActionCheckPhoneNumberBuilder {
    pub fn build(&self) -> SuggestedActionCheckPhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<SuggestedActionCheckPhoneNumber> for SuggestedActionCheckPhoneNumber {
    fn as_ref(&self) -> &SuggestedActionCheckPhoneNumber {
        self
    }
}

impl AsRef<SuggestedActionCheckPhoneNumber> for RTDSuggestedActionCheckPhoneNumberBuilder {
    fn as_ref(&self) -> &SuggestedActionCheckPhoneNumber {
        &self.inner
    }
}

/// Suggests the user to enable "archive_and_mute_new_chats_from_unknown_users" option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionEnableArchiveAndMuteNewChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SuggestedActionEnableArchiveAndMuteNewChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSuggestedAction for SuggestedActionEnableArchiveAndMuteNewChats {}

impl SuggestedActionEnableArchiveAndMuteNewChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder {
        let mut inner = SuggestedActionEnableArchiveAndMuteNewChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder {
    inner: SuggestedActionEnableArchiveAndMuteNewChats,
}

impl RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder {
    pub fn build(&self) -> SuggestedActionEnableArchiveAndMuteNewChats {
        self.inner.clone()
    }
}

impl AsRef<SuggestedActionEnableArchiveAndMuteNewChats>
    for SuggestedActionEnableArchiveAndMuteNewChats
{
    fn as_ref(&self) -> &SuggestedActionEnableArchiveAndMuteNewChats {
        self
    }
}

impl AsRef<SuggestedActionEnableArchiveAndMuteNewChats>
    for RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder
{
    fn as_ref(&self) -> &SuggestedActionEnableArchiveAndMuteNewChats {
        &self.inner
    }
}
