use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes actions which should be possible to do through a chat action bar
pub trait TDChatActionBar: Debug + RObject {}

/// Describes actions which should be possible to do through a chat action bar
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatActionBar {
    #[doc(hidden)]
    _Default(()),
    /// The chat is a private or secret chat and the other user can be added to the contact list using the method addContact
    AddContact(ChatActionBarAddContact),
    /// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be blocked using the method blockUser, or the other user can be added to the contact list using the method addContact
    ReportAddBlock(ChatActionBarReportAddBlock),
    /// The chat can be reported as spam using the method reportChat with the reason chatReportReasonSpam
    ReportSpam(ChatActionBarReportSpam),
    /// The chat is a location-based supergroup, which can be reported as having unrelated location using the method reportChat with the reason chatReportReasonUnrelatedLocation
    ReportUnrelatedLocation(ChatActionBarReportUnrelatedLocation),
    /// The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber
    SharePhoneNumber(ChatActionBarSharePhoneNumber),
}

impl Default for ChatActionBar {
    fn default() -> Self {
        ChatActionBar::_Default(())
    }
}

impl<'de> Deserialize<'de> for ChatActionBar {
    fn deserialize<D>(deserializer: D) -> Result<ChatActionBar, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          ChatActionBar,
          (chatActionBarAddContact, AddContact);
          (chatActionBarReportAddBlock, ReportAddBlock);
          (chatActionBarReportSpam, ReportSpam);
          (chatActionBarReportUnrelatedLocation, ReportUnrelatedLocation);
          (chatActionBarSharePhoneNumber, SharePhoneNumber);

        )(deserializer)
    }
}

impl RObject for ChatActionBar {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            ChatActionBar::AddContact(t) => t.td_name(),
            ChatActionBar::ReportAddBlock(t) => t.td_name(),
            ChatActionBar::ReportSpam(t) => t.td_name(),
            ChatActionBar::ReportUnrelatedLocation(t) => t.td_name(),
            ChatActionBar::SharePhoneNumber(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            ChatActionBar::AddContact(t) => t.extra(),
            ChatActionBar::ReportAddBlock(t) => t.extra(),
            ChatActionBar::ReportSpam(t) => t.extra(),
            ChatActionBar::ReportUnrelatedLocation(t) => t.extra(),
            ChatActionBar::SharePhoneNumber(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl ChatActionBar {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatActionBar::_Default(_))
    }
}

impl AsRef<ChatActionBar> for ChatActionBar {
    fn as_ref(&self) -> &ChatActionBar {
        self
    }
}

/// The chat is a private or secret chat and the other user can be added to the contact list using the method addContact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarAddContact {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatActionBarAddContact {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatActionBarAddContact"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatActionBar for ChatActionBarAddContact {}

impl ChatActionBarAddContact {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionBarAddContactBuilder {
        let mut inner = ChatActionBarAddContact::default();
        inner.td_name = "chatActionBarAddContact".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatActionBarAddContactBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionBarAddContactBuilder {
    inner: ChatActionBarAddContact,
}

impl RTDChatActionBarAddContactBuilder {
    pub fn build(&self) -> ChatActionBarAddContact {
        self.inner.clone()
    }
}

impl AsRef<ChatActionBarAddContact> for ChatActionBarAddContact {
    fn as_ref(&self) -> &ChatActionBarAddContact {
        self
    }
}

impl AsRef<ChatActionBarAddContact> for RTDChatActionBarAddContactBuilder {
    fn as_ref(&self) -> &ChatActionBarAddContact {
        &self.inner
    }
}

/// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be blocked using the method blockUser, or the other user can be added to the contact list using the method addContact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarReportAddBlock {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// If true, the chat was automatically archived and can be moved back to the main chat list using addChatToList simultaneously with setting chat notification settings to default using setChatNotificationSettings
    can_unarchive: bool,
    /// If non-negative, the current user was found by the peer through searchChatsNearby and this is the distance between the users
    distance: i32,
}

impl RObject for ChatActionBarReportAddBlock {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatActionBarReportAddBlock"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatActionBar for ChatActionBarReportAddBlock {}

impl ChatActionBarReportAddBlock {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionBarReportAddBlockBuilder {
        let mut inner = ChatActionBarReportAddBlock::default();
        inner.td_name = "chatActionBarReportAddBlock".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatActionBarReportAddBlockBuilder { inner }
    }

    pub fn can_unarchive(&self) -> bool {
        self.can_unarchive
    }

    pub fn distance(&self) -> i32 {
        self.distance
    }
}

#[doc(hidden)]
pub struct RTDChatActionBarReportAddBlockBuilder {
    inner: ChatActionBarReportAddBlock,
}

impl RTDChatActionBarReportAddBlockBuilder {
    pub fn build(&self) -> ChatActionBarReportAddBlock {
        self.inner.clone()
    }

    pub fn can_unarchive(&mut self, can_unarchive: bool) -> &mut Self {
        self.inner.can_unarchive = can_unarchive;
        self
    }

    pub fn distance(&mut self, distance: i32) -> &mut Self {
        self.inner.distance = distance;
        self
    }
}

impl AsRef<ChatActionBarReportAddBlock> for ChatActionBarReportAddBlock {
    fn as_ref(&self) -> &ChatActionBarReportAddBlock {
        self
    }
}

impl AsRef<ChatActionBarReportAddBlock> for RTDChatActionBarReportAddBlockBuilder {
    fn as_ref(&self) -> &ChatActionBarReportAddBlock {
        &self.inner
    }
}

/// The chat can be reported as spam using the method reportChat with the reason chatReportReasonSpam
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarReportSpam {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// If true, the chat was automatically archived and can be moved back to the main chat list using addChatToList simultaneously with setting chat notification settings to default using setChatNotificationSettings
    can_unarchive: bool,
}

impl RObject for ChatActionBarReportSpam {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatActionBarReportSpam"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatActionBar for ChatActionBarReportSpam {}

impl ChatActionBarReportSpam {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionBarReportSpamBuilder {
        let mut inner = ChatActionBarReportSpam::default();
        inner.td_name = "chatActionBarReportSpam".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatActionBarReportSpamBuilder { inner }
    }

    pub fn can_unarchive(&self) -> bool {
        self.can_unarchive
    }
}

#[doc(hidden)]
pub struct RTDChatActionBarReportSpamBuilder {
    inner: ChatActionBarReportSpam,
}

impl RTDChatActionBarReportSpamBuilder {
    pub fn build(&self) -> ChatActionBarReportSpam {
        self.inner.clone()
    }

    pub fn can_unarchive(&mut self, can_unarchive: bool) -> &mut Self {
        self.inner.can_unarchive = can_unarchive;
        self
    }
}

impl AsRef<ChatActionBarReportSpam> for ChatActionBarReportSpam {
    fn as_ref(&self) -> &ChatActionBarReportSpam {
        self
    }
}

impl AsRef<ChatActionBarReportSpam> for RTDChatActionBarReportSpamBuilder {
    fn as_ref(&self) -> &ChatActionBarReportSpam {
        &self.inner
    }
}

/// The chat is a location-based supergroup, which can be reported as having unrelated location using the method reportChat with the reason chatReportReasonUnrelatedLocation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarReportUnrelatedLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatActionBarReportUnrelatedLocation {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatActionBarReportUnrelatedLocation"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatActionBar for ChatActionBarReportUnrelatedLocation {}

impl ChatActionBarReportUnrelatedLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionBarReportUnrelatedLocationBuilder {
        let mut inner = ChatActionBarReportUnrelatedLocation::default();
        inner.td_name = "chatActionBarReportUnrelatedLocation".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatActionBarReportUnrelatedLocationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionBarReportUnrelatedLocationBuilder {
    inner: ChatActionBarReportUnrelatedLocation,
}

impl RTDChatActionBarReportUnrelatedLocationBuilder {
    pub fn build(&self) -> ChatActionBarReportUnrelatedLocation {
        self.inner.clone()
    }
}

impl AsRef<ChatActionBarReportUnrelatedLocation> for ChatActionBarReportUnrelatedLocation {
    fn as_ref(&self) -> &ChatActionBarReportUnrelatedLocation {
        self
    }
}

impl AsRef<ChatActionBarReportUnrelatedLocation>
    for RTDChatActionBarReportUnrelatedLocationBuilder
{
    fn as_ref(&self) -> &ChatActionBarReportUnrelatedLocation {
        &self.inner
    }
}

/// The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarSharePhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for ChatActionBarSharePhoneNumber {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatActionBarSharePhoneNumber"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDChatActionBar for ChatActionBarSharePhoneNumber {}

impl ChatActionBarSharePhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionBarSharePhoneNumberBuilder {
        let mut inner = ChatActionBarSharePhoneNumber::default();
        inner.td_name = "chatActionBarSharePhoneNumber".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDChatActionBarSharePhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionBarSharePhoneNumberBuilder {
    inner: ChatActionBarSharePhoneNumber,
}

impl RTDChatActionBarSharePhoneNumberBuilder {
    pub fn build(&self) -> ChatActionBarSharePhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<ChatActionBarSharePhoneNumber> for ChatActionBarSharePhoneNumber {
    fn as_ref(&self) -> &ChatActionBarSharePhoneNumber {
        self
    }
}

impl AsRef<ChatActionBarSharePhoneNumber> for RTDChatActionBarSharePhoneNumberBuilder {
    fn as_ref(&self) -> &ChatActionBarSharePhoneNumber {
        &self.inner
    }
}
