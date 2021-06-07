use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes actions which should be possible to do through a chat action bar
pub trait TDChatActionBar: Debug + RObject {}

/// Describes actions which should be possible to do through a chat action bar
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatActionBar {
    #[doc(hidden)]
    _Default,
    /// The chat is a private or secret chat and the other user can be added to the contact list using the method addContact
    #[serde(rename(
        serialize = "chatActionBarAddContact",
        deserialize = "chatActionBarAddContact"
    ))]
    AddContact(ChatActionBarAddContact),
    /// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be blocked using the method blockUser, or the other user can be added to the contact list using the method addContact
    #[serde(rename(
        serialize = "chatActionBarReportAddBlock",
        deserialize = "chatActionBarReportAddBlock"
    ))]
    ReportAddBlock(ChatActionBarReportAddBlock),
    /// The chat can be reported as spam using the method reportChat with the reason chatReportReasonSpam
    #[serde(rename(
        serialize = "chatActionBarReportSpam",
        deserialize = "chatActionBarReportSpam"
    ))]
    ReportSpam(ChatActionBarReportSpam),
    /// The chat is a location-based supergroup, which can be reported as having unrelated location using the method reportChat with the reason chatReportReasonUnrelatedLocation
    #[serde(rename(
        serialize = "chatActionBarReportUnrelatedLocation",
        deserialize = "chatActionBarReportUnrelatedLocation"
    ))]
    ReportUnrelatedLocation(ChatActionBarReportUnrelatedLocation),
    /// The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber
    #[serde(rename(
        serialize = "chatActionBarSharePhoneNumber",
        deserialize = "chatActionBarSharePhoneNumber"
    ))]
    SharePhoneNumber(ChatActionBarSharePhoneNumber),
}

impl Default for ChatActionBar {
    fn default() -> Self {
        ChatActionBar::_Default
    }
}

impl RObject for ChatActionBar {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatActionBar::AddContact(t) => t.extra(),
            ChatActionBar::ReportAddBlock(t) => t.extra(),
            ChatActionBar::ReportSpam(t) => t.extra(),
            ChatActionBar::ReportUnrelatedLocation(t) => t.extra(),
            ChatActionBar::SharePhoneNumber(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatActionBar::AddContact(t) => t.client_id(),
            ChatActionBar::ReportAddBlock(t) => t.client_id(),
            ChatActionBar::ReportSpam(t) => t.client_id(),
            ChatActionBar::ReportUnrelatedLocation(t) => t.client_id(),
            ChatActionBar::SharePhoneNumber(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatActionBar {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatActionBar::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionBarAddContact {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatActionBar for ChatActionBarAddContact {}

impl ChatActionBarAddContact {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionBarAddContactBuilder {
        let mut inner = ChatActionBarAddContact::default();
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If true, the chat was automatically archived and can be moved back to the main chat list using addChatToList simultaneously with setting chat notification settings to default using setChatNotificationSettings
    can_unarchive: bool,
    /// If non-negative, the current user was found by the peer through searchChatsNearby and this is the distance between the users
    distance: i32,
}

impl RObject for ChatActionBarReportAddBlock {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatActionBar for ChatActionBarReportAddBlock {}

impl ChatActionBarReportAddBlock {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionBarReportAddBlockBuilder {
        let mut inner = ChatActionBarReportAddBlock::default();
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If true, the chat was automatically archived and can be moved back to the main chat list using addChatToList simultaneously with setting chat notification settings to default using setChatNotificationSettings
    can_unarchive: bool,
}

impl RObject for ChatActionBarReportSpam {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatActionBar for ChatActionBarReportSpam {}

impl ChatActionBarReportSpam {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionBarReportSpamBuilder {
        let mut inner = ChatActionBarReportSpam::default();
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionBarReportUnrelatedLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatActionBar for ChatActionBarReportUnrelatedLocation {}

impl ChatActionBarReportUnrelatedLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionBarReportUnrelatedLocationBuilder {
        let mut inner = ChatActionBarReportUnrelatedLocation::default();
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionBarSharePhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatActionBar for ChatActionBarSharePhoneNumber {}

impl ChatActionBarSharePhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionBarSharePhoneNumberBuilder {
        let mut inner = ChatActionBarSharePhoneNumber::default();
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
