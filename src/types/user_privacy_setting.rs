use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes available user privacy settings
pub trait TDUserPrivacySetting: Debug + RObject {}

/// Describes available user privacy settings
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserPrivacySetting {
    #[doc(hidden)]
    _Default,
    /// A privacy setting for managing whether the user can be called
    #[serde(rename = "userPrivacySettingAllowCalls")]
    AllowCalls(UserPrivacySettingAllowCalls),
    /// A privacy setting for managing whether the user can be invited to chats
    #[serde(rename = "userPrivacySettingAllowChatInvites")]
    AllowChatInvites(UserPrivacySettingAllowChatInvites),
    /// A privacy setting for managing whether the user can be found by their phone number. Checked only if the phone number is not known to the other user. Can be set only to "Allow contacts" or "Allow all"
    #[serde(rename = "userPrivacySettingAllowFindingByPhoneNumber")]
    AllowFindingByPhoneNumber(UserPrivacySettingAllowFindingByPhoneNumber),
    /// A privacy setting for managing whether peer-to-peer connections can be used for calls
    #[serde(rename = "userPrivacySettingAllowPeerToPeerCalls")]
    AllowPeerToPeerCalls(UserPrivacySettingAllowPeerToPeerCalls),
    /// A privacy setting for managing whether a link to the user's account is included in forwarded messages
    #[serde(rename = "userPrivacySettingShowLinkInForwardedMessages")]
    ShowLinkInForwardedMessages(UserPrivacySettingShowLinkInForwardedMessages),
    /// A privacy setting for managing whether the user's phone number is visible
    #[serde(rename = "userPrivacySettingShowPhoneNumber")]
    ShowPhoneNumber(UserPrivacySettingShowPhoneNumber),
    /// A privacy setting for managing whether the user's profile photo is visible
    #[serde(rename = "userPrivacySettingShowProfilePhoto")]
    ShowProfilePhoto(UserPrivacySettingShowProfilePhoto),
    /// A privacy setting for managing whether the user's online status is visible
    #[serde(rename = "userPrivacySettingShowStatus")]
    ShowStatus(UserPrivacySettingShowStatus),
}

impl Default for UserPrivacySetting {
    fn default() -> Self {
        UserPrivacySetting::_Default
    }
}

impl RObject for UserPrivacySetting {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            UserPrivacySetting::AllowCalls(t) => t.extra(),
            UserPrivacySetting::AllowChatInvites(t) => t.extra(),
            UserPrivacySetting::AllowFindingByPhoneNumber(t) => t.extra(),
            UserPrivacySetting::AllowPeerToPeerCalls(t) => t.extra(),
            UserPrivacySetting::ShowLinkInForwardedMessages(t) => t.extra(),
            UserPrivacySetting::ShowPhoneNumber(t) => t.extra(),
            UserPrivacySetting::ShowProfilePhoto(t) => t.extra(),
            UserPrivacySetting::ShowStatus(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            UserPrivacySetting::AllowCalls(t) => t.client_id(),
            UserPrivacySetting::AllowChatInvites(t) => t.client_id(),
            UserPrivacySetting::AllowFindingByPhoneNumber(t) => t.client_id(),
            UserPrivacySetting::AllowPeerToPeerCalls(t) => t.client_id(),
            UserPrivacySetting::ShowLinkInForwardedMessages(t) => t.client_id(),
            UserPrivacySetting::ShowPhoneNumber(t) => t.client_id(),
            UserPrivacySetting::ShowProfilePhoto(t) => t.client_id(),
            UserPrivacySetting::ShowStatus(t) => t.client_id(),

            _ => None,
        }
    }
}

impl UserPrivacySetting {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, UserPrivacySetting::_Default)
    }
}

impl AsRef<UserPrivacySetting> for UserPrivacySetting {
    fn as_ref(&self) -> &UserPrivacySetting {
        self
    }
}

/// A privacy setting for managing whether the user can be called
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowCalls {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingAllowCalls {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySetting for UserPrivacySettingAllowCalls {}

impl UserPrivacySettingAllowCalls {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserPrivacySettingAllowCallsBuilder {
        let mut inner = UserPrivacySettingAllowCalls::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserPrivacySettingAllowCallsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserPrivacySettingAllowCallsBuilder {
    inner: UserPrivacySettingAllowCalls,
}

#[deprecated]
pub type RTDUserPrivacySettingAllowCallsBuilder = UserPrivacySettingAllowCallsBuilder;

impl UserPrivacySettingAllowCallsBuilder {
    pub fn build(&self) -> UserPrivacySettingAllowCalls {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingAllowCalls> for UserPrivacySettingAllowCalls {
    fn as_ref(&self) -> &UserPrivacySettingAllowCalls {
        self
    }
}

impl AsRef<UserPrivacySettingAllowCalls> for UserPrivacySettingAllowCallsBuilder {
    fn as_ref(&self) -> &UserPrivacySettingAllowCalls {
        &self.inner
    }
}

/// A privacy setting for managing whether the user can be invited to chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowChatInvites {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingAllowChatInvites {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySetting for UserPrivacySettingAllowChatInvites {}

impl UserPrivacySettingAllowChatInvites {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserPrivacySettingAllowChatInvitesBuilder {
        let mut inner = UserPrivacySettingAllowChatInvites::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserPrivacySettingAllowChatInvitesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserPrivacySettingAllowChatInvitesBuilder {
    inner: UserPrivacySettingAllowChatInvites,
}

#[deprecated]
pub type RTDUserPrivacySettingAllowChatInvitesBuilder = UserPrivacySettingAllowChatInvitesBuilder;

impl UserPrivacySettingAllowChatInvitesBuilder {
    pub fn build(&self) -> UserPrivacySettingAllowChatInvites {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingAllowChatInvites> for UserPrivacySettingAllowChatInvites {
    fn as_ref(&self) -> &UserPrivacySettingAllowChatInvites {
        self
    }
}

impl AsRef<UserPrivacySettingAllowChatInvites> for UserPrivacySettingAllowChatInvitesBuilder {
    fn as_ref(&self) -> &UserPrivacySettingAllowChatInvites {
        &self.inner
    }
}

/// A privacy setting for managing whether the user can be found by their phone number. Checked only if the phone number is not known to the other user. Can be set only to "Allow contacts" or "Allow all"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowFindingByPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingAllowFindingByPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySetting for UserPrivacySettingAllowFindingByPhoneNumber {}

impl UserPrivacySettingAllowFindingByPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserPrivacySettingAllowFindingByPhoneNumberBuilder {
        let mut inner = UserPrivacySettingAllowFindingByPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserPrivacySettingAllowFindingByPhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserPrivacySettingAllowFindingByPhoneNumberBuilder {
    inner: UserPrivacySettingAllowFindingByPhoneNumber,
}

#[deprecated]
pub type RTDUserPrivacySettingAllowFindingByPhoneNumberBuilder =
    UserPrivacySettingAllowFindingByPhoneNumberBuilder;

impl UserPrivacySettingAllowFindingByPhoneNumberBuilder {
    pub fn build(&self) -> UserPrivacySettingAllowFindingByPhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingAllowFindingByPhoneNumber>
    for UserPrivacySettingAllowFindingByPhoneNumber
{
    fn as_ref(&self) -> &UserPrivacySettingAllowFindingByPhoneNumber {
        self
    }
}

impl AsRef<UserPrivacySettingAllowFindingByPhoneNumber>
    for UserPrivacySettingAllowFindingByPhoneNumberBuilder
{
    fn as_ref(&self) -> &UserPrivacySettingAllowFindingByPhoneNumber {
        &self.inner
    }
}

/// A privacy setting for managing whether peer-to-peer connections can be used for calls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingAllowPeerToPeerCalls {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingAllowPeerToPeerCalls {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySetting for UserPrivacySettingAllowPeerToPeerCalls {}

impl UserPrivacySettingAllowPeerToPeerCalls {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserPrivacySettingAllowPeerToPeerCallsBuilder {
        let mut inner = UserPrivacySettingAllowPeerToPeerCalls::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserPrivacySettingAllowPeerToPeerCallsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserPrivacySettingAllowPeerToPeerCallsBuilder {
    inner: UserPrivacySettingAllowPeerToPeerCalls,
}

#[deprecated]
pub type RTDUserPrivacySettingAllowPeerToPeerCallsBuilder =
    UserPrivacySettingAllowPeerToPeerCallsBuilder;

impl UserPrivacySettingAllowPeerToPeerCallsBuilder {
    pub fn build(&self) -> UserPrivacySettingAllowPeerToPeerCalls {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingAllowPeerToPeerCalls> for UserPrivacySettingAllowPeerToPeerCalls {
    fn as_ref(&self) -> &UserPrivacySettingAllowPeerToPeerCalls {
        self
    }
}

impl AsRef<UserPrivacySettingAllowPeerToPeerCalls>
    for UserPrivacySettingAllowPeerToPeerCallsBuilder
{
    fn as_ref(&self) -> &UserPrivacySettingAllowPeerToPeerCalls {
        &self.inner
    }
}

/// A privacy setting for managing whether a link to the user's account is included in forwarded messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowLinkInForwardedMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingShowLinkInForwardedMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySetting for UserPrivacySettingShowLinkInForwardedMessages {}

impl UserPrivacySettingShowLinkInForwardedMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserPrivacySettingShowLinkInForwardedMessagesBuilder {
        let mut inner = UserPrivacySettingShowLinkInForwardedMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserPrivacySettingShowLinkInForwardedMessagesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserPrivacySettingShowLinkInForwardedMessagesBuilder {
    inner: UserPrivacySettingShowLinkInForwardedMessages,
}

#[deprecated]
pub type RTDUserPrivacySettingShowLinkInForwardedMessagesBuilder =
    UserPrivacySettingShowLinkInForwardedMessagesBuilder;

impl UserPrivacySettingShowLinkInForwardedMessagesBuilder {
    pub fn build(&self) -> UserPrivacySettingShowLinkInForwardedMessages {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingShowLinkInForwardedMessages>
    for UserPrivacySettingShowLinkInForwardedMessages
{
    fn as_ref(&self) -> &UserPrivacySettingShowLinkInForwardedMessages {
        self
    }
}

impl AsRef<UserPrivacySettingShowLinkInForwardedMessages>
    for UserPrivacySettingShowLinkInForwardedMessagesBuilder
{
    fn as_ref(&self) -> &UserPrivacySettingShowLinkInForwardedMessages {
        &self.inner
    }
}

/// A privacy setting for managing whether the user's phone number is visible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingShowPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySetting for UserPrivacySettingShowPhoneNumber {}

impl UserPrivacySettingShowPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserPrivacySettingShowPhoneNumberBuilder {
        let mut inner = UserPrivacySettingShowPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserPrivacySettingShowPhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserPrivacySettingShowPhoneNumberBuilder {
    inner: UserPrivacySettingShowPhoneNumber,
}

#[deprecated]
pub type RTDUserPrivacySettingShowPhoneNumberBuilder = UserPrivacySettingShowPhoneNumberBuilder;

impl UserPrivacySettingShowPhoneNumberBuilder {
    pub fn build(&self) -> UserPrivacySettingShowPhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingShowPhoneNumber> for UserPrivacySettingShowPhoneNumber {
    fn as_ref(&self) -> &UserPrivacySettingShowPhoneNumber {
        self
    }
}

impl AsRef<UserPrivacySettingShowPhoneNumber> for UserPrivacySettingShowPhoneNumberBuilder {
    fn as_ref(&self) -> &UserPrivacySettingShowPhoneNumber {
        &self.inner
    }
}

/// A privacy setting for managing whether the user's profile photo is visible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowProfilePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingShowProfilePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySetting for UserPrivacySettingShowProfilePhoto {}

impl UserPrivacySettingShowProfilePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserPrivacySettingShowProfilePhotoBuilder {
        let mut inner = UserPrivacySettingShowProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserPrivacySettingShowProfilePhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserPrivacySettingShowProfilePhotoBuilder {
    inner: UserPrivacySettingShowProfilePhoto,
}

#[deprecated]
pub type RTDUserPrivacySettingShowProfilePhotoBuilder = UserPrivacySettingShowProfilePhotoBuilder;

impl UserPrivacySettingShowProfilePhotoBuilder {
    pub fn build(&self) -> UserPrivacySettingShowProfilePhoto {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingShowProfilePhoto> for UserPrivacySettingShowProfilePhoto {
    fn as_ref(&self) -> &UserPrivacySettingShowProfilePhoto {
        self
    }
}

impl AsRef<UserPrivacySettingShowProfilePhoto> for UserPrivacySettingShowProfilePhotoBuilder {
    fn as_ref(&self) -> &UserPrivacySettingShowProfilePhoto {
        &self.inner
    }
}

/// A privacy setting for managing whether the user's online status is visible
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingShowStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingShowStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySetting for UserPrivacySettingShowStatus {}

impl UserPrivacySettingShowStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserPrivacySettingShowStatusBuilder {
        let mut inner = UserPrivacySettingShowStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserPrivacySettingShowStatusBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserPrivacySettingShowStatusBuilder {
    inner: UserPrivacySettingShowStatus,
}

#[deprecated]
pub type RTDUserPrivacySettingShowStatusBuilder = UserPrivacySettingShowStatusBuilder;

impl UserPrivacySettingShowStatusBuilder {
    pub fn build(&self) -> UserPrivacySettingShowStatus {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingShowStatus> for UserPrivacySettingShowStatus {
    fn as_ref(&self) -> &UserPrivacySettingShowStatus {
        self
    }
}

impl AsRef<UserPrivacySettingShowStatus> for UserPrivacySettingShowStatusBuilder {
    fn as_ref(&self) -> &UserPrivacySettingShowStatus {
        &self.inner
    }
}
