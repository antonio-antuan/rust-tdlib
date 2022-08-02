use crate::errors::Result;
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
    /// Suggests the user to check whether they still remember their 2-step verification password
    #[serde(rename = "suggestedActionCheckPassword")]
    CheckPassword(SuggestedActionCheckPassword),
    /// Suggests the user to check whether authorization phone number is correct and change the phone number if it is inaccessible
    #[serde(rename = "suggestedActionCheckPhoneNumber")]
    CheckPhoneNumber(SuggestedActionCheckPhoneNumber),
    /// Suggests the user to convert specified supergroup to a broadcast group
    #[serde(rename = "suggestedActionConvertToBroadcastGroup")]
    ConvertToBroadcastGroup(SuggestedActionConvertToBroadcastGroup),
    /// Suggests the user to enable "archive_and_mute_new_chats_from_unknown_users" option
    #[serde(rename = "suggestedActionEnableArchiveAndMuteNewChats")]
    EnableArchiveAndMuteNewChats(SuggestedActionEnableArchiveAndMuteNewChats),
    /// Suggests the user to set a 2-step verification password to be able to log in again
    #[serde(rename = "suggestedActionSetPassword")]
    SetPassword(SuggestedActionSetPassword),
    /// Suggests the user to view a hint about the meaning of one and two check marks on sent messages
    #[serde(rename = "suggestedActionViewChecksHint")]
    ViewChecksHint(SuggestedActionViewChecksHint),
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
            SuggestedAction::CheckPassword(t) => t.extra(),
            SuggestedAction::CheckPhoneNumber(t) => t.extra(),
            SuggestedAction::ConvertToBroadcastGroup(t) => t.extra(),
            SuggestedAction::EnableArchiveAndMuteNewChats(t) => t.extra(),
            SuggestedAction::SetPassword(t) => t.extra(),
            SuggestedAction::ViewChecksHint(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            SuggestedAction::CheckPassword(t) => t.client_id(),
            SuggestedAction::CheckPhoneNumber(t) => t.client_id(),
            SuggestedAction::ConvertToBroadcastGroup(t) => t.client_id(),
            SuggestedAction::EnableArchiveAndMuteNewChats(t) => t.client_id(),
            SuggestedAction::SetPassword(t) => t.client_id(),
            SuggestedAction::ViewChecksHint(t) => t.client_id(),

            _ => None,
        }
    }
}

impl SuggestedAction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
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

/// Suggests the user to check whether they still remember their 2-step verification password
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionCheckPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SuggestedActionCheckPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSuggestedAction for SuggestedActionCheckPassword {}

impl SuggestedActionCheckPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SuggestedActionCheckPasswordBuilder {
        let mut inner = SuggestedActionCheckPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SuggestedActionCheckPasswordBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SuggestedActionCheckPasswordBuilder {
    inner: SuggestedActionCheckPassword,
}

#[deprecated]
pub type RTDSuggestedActionCheckPasswordBuilder = SuggestedActionCheckPasswordBuilder;

impl SuggestedActionCheckPasswordBuilder {
    pub fn build(&self) -> SuggestedActionCheckPassword {
        self.inner.clone()
    }
}

impl AsRef<SuggestedActionCheckPassword> for SuggestedActionCheckPassword {
    fn as_ref(&self) -> &SuggestedActionCheckPassword {
        self
    }
}

impl AsRef<SuggestedActionCheckPassword> for SuggestedActionCheckPasswordBuilder {
    fn as_ref(&self) -> &SuggestedActionCheckPassword {
        &self.inner
    }
}

/// Suggests the user to check whether authorization phone number is correct and change the phone number if it is inaccessible
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SuggestedActionCheckPhoneNumberBuilder {
        let mut inner = SuggestedActionCheckPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SuggestedActionCheckPhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SuggestedActionCheckPhoneNumberBuilder {
    inner: SuggestedActionCheckPhoneNumber,
}

#[deprecated]
pub type RTDSuggestedActionCheckPhoneNumberBuilder = SuggestedActionCheckPhoneNumberBuilder;

impl SuggestedActionCheckPhoneNumberBuilder {
    pub fn build(&self) -> SuggestedActionCheckPhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<SuggestedActionCheckPhoneNumber> for SuggestedActionCheckPhoneNumber {
    fn as_ref(&self) -> &SuggestedActionCheckPhoneNumber {
        self
    }
}

impl AsRef<SuggestedActionCheckPhoneNumber> for SuggestedActionCheckPhoneNumberBuilder {
    fn as_ref(&self) -> &SuggestedActionCheckPhoneNumber {
        &self.inner
    }
}

/// Suggests the user to convert specified supergroup to a broadcast group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionConvertToBroadcastGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Supergroup identifier

    #[serde(default)]
    supergroup_id: i64,
}

impl RObject for SuggestedActionConvertToBroadcastGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSuggestedAction for SuggestedActionConvertToBroadcastGroup {}

impl SuggestedActionConvertToBroadcastGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SuggestedActionConvertToBroadcastGroupBuilder {
        let mut inner = SuggestedActionConvertToBroadcastGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SuggestedActionConvertToBroadcastGroupBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }
}

#[doc(hidden)]
pub struct SuggestedActionConvertToBroadcastGroupBuilder {
    inner: SuggestedActionConvertToBroadcastGroup,
}

#[deprecated]
pub type RTDSuggestedActionConvertToBroadcastGroupBuilder =
    SuggestedActionConvertToBroadcastGroupBuilder;

impl SuggestedActionConvertToBroadcastGroupBuilder {
    pub fn build(&self) -> SuggestedActionConvertToBroadcastGroup {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }
}

impl AsRef<SuggestedActionConvertToBroadcastGroup> for SuggestedActionConvertToBroadcastGroup {
    fn as_ref(&self) -> &SuggestedActionConvertToBroadcastGroup {
        self
    }
}

impl AsRef<SuggestedActionConvertToBroadcastGroup>
    for SuggestedActionConvertToBroadcastGroupBuilder
{
    fn as_ref(&self) -> &SuggestedActionConvertToBroadcastGroup {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SuggestedActionEnableArchiveAndMuteNewChatsBuilder {
        let mut inner = SuggestedActionEnableArchiveAndMuteNewChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SuggestedActionEnableArchiveAndMuteNewChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SuggestedActionEnableArchiveAndMuteNewChatsBuilder {
    inner: SuggestedActionEnableArchiveAndMuteNewChats,
}

#[deprecated]
pub type RTDSuggestedActionEnableArchiveAndMuteNewChatsBuilder =
    SuggestedActionEnableArchiveAndMuteNewChatsBuilder;

impl SuggestedActionEnableArchiveAndMuteNewChatsBuilder {
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
    for SuggestedActionEnableArchiveAndMuteNewChatsBuilder
{
    fn as_ref(&self) -> &SuggestedActionEnableArchiveAndMuteNewChats {
        &self.inner
    }
}

/// Suggests the user to set a 2-step verification password to be able to log in again
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionSetPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The number of days to pass between consecutive authorizations if the user declines to set password

    #[serde(default)]
    authorization_delay: i32,
}

impl RObject for SuggestedActionSetPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSuggestedAction for SuggestedActionSetPassword {}

impl SuggestedActionSetPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SuggestedActionSetPasswordBuilder {
        let mut inner = SuggestedActionSetPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SuggestedActionSetPasswordBuilder { inner }
    }

    pub fn authorization_delay(&self) -> i32 {
        self.authorization_delay
    }
}

#[doc(hidden)]
pub struct SuggestedActionSetPasswordBuilder {
    inner: SuggestedActionSetPassword,
}

#[deprecated]
pub type RTDSuggestedActionSetPasswordBuilder = SuggestedActionSetPasswordBuilder;

impl SuggestedActionSetPasswordBuilder {
    pub fn build(&self) -> SuggestedActionSetPassword {
        self.inner.clone()
    }

    pub fn authorization_delay(&mut self, authorization_delay: i32) -> &mut Self {
        self.inner.authorization_delay = authorization_delay;
        self
    }
}

impl AsRef<SuggestedActionSetPassword> for SuggestedActionSetPassword {
    fn as_ref(&self) -> &SuggestedActionSetPassword {
        self
    }
}

impl AsRef<SuggestedActionSetPassword> for SuggestedActionSetPasswordBuilder {
    fn as_ref(&self) -> &SuggestedActionSetPassword {
        &self.inner
    }
}

/// Suggests the user to view a hint about the meaning of one and two check marks on sent messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedActionViewChecksHint {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SuggestedActionViewChecksHint {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSuggestedAction for SuggestedActionViewChecksHint {}

impl SuggestedActionViewChecksHint {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SuggestedActionViewChecksHintBuilder {
        let mut inner = SuggestedActionViewChecksHint::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SuggestedActionViewChecksHintBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SuggestedActionViewChecksHintBuilder {
    inner: SuggestedActionViewChecksHint,
}

#[deprecated]
pub type RTDSuggestedActionViewChecksHintBuilder = SuggestedActionViewChecksHintBuilder;

impl SuggestedActionViewChecksHintBuilder {
    pub fn build(&self) -> SuggestedActionViewChecksHint {
        self.inner.clone()
    }
}

impl AsRef<SuggestedActionViewChecksHint> for SuggestedActionViewChecksHint {
    fn as_ref(&self) -> &SuggestedActionViewChecksHint {
        self
    }
}

impl AsRef<SuggestedActionViewChecksHint> for SuggestedActionViewChecksHintBuilder {
    fn as_ref(&self) -> &SuggestedActionViewChecksHint {
        &self.inner
    }
}
