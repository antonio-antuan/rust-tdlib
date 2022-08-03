use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes actions which must be possible to do through a chat action bar
pub trait TDChatActionBar: Debug + RObject {}

/// Describes actions which must be possible to do through a chat action bar
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatActionBar {
    #[doc(hidden)]
    _Default,
    /// The chat is a private or secret chat and the other user can be added to the contact list using the method addContact
    #[serde(rename = "chatActionBarAddContact")]
    AddContact(ChatActionBarAddContact),
    /// The chat is a recently created group chat to which new members can be invited
    #[serde(rename = "chatActionBarInviteMembers")]
    InviteMembers(ChatActionBarInviteMembers),
    /// The chat is a private chat with an administrator of a chat to which the user sent join request
    #[serde(rename = "chatActionBarJoinRequest")]
    JoinRequest(ChatActionBarJoinRequest),
    /// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be blocked using the method toggleMessageSenderIsBlocked, or the other user can be added to the contact list using the method addContact
    #[serde(rename = "chatActionBarReportAddBlock")]
    ReportAddBlock(ChatActionBarReportAddBlock),
    /// The chat can be reported as spam using the method reportChat with the reason chatReportReasonSpam
    #[serde(rename = "chatActionBarReportSpam")]
    ReportSpam(ChatActionBarReportSpam),
    /// The chat is a location-based supergroup, which can be reported as having unrelated location using the method reportChat with the reason chatReportReasonUnrelatedLocation
    #[serde(rename = "chatActionBarReportUnrelatedLocation")]
    ReportUnrelatedLocation(ChatActionBarReportUnrelatedLocation),
    /// The chat is a private or secret chat with a mutual contact and the user's phone number can be shared with the other user using the method sharePhoneNumber
    #[serde(rename = "chatActionBarSharePhoneNumber")]
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
            ChatActionBar::InviteMembers(t) => t.extra(),
            ChatActionBar::JoinRequest(t) => t.extra(),
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
            ChatActionBar::InviteMembers(t) => t.client_id(),
            ChatActionBar::JoinRequest(t) => t.client_id(),
            ChatActionBar::ReportAddBlock(t) => t.client_id(),
            ChatActionBar::ReportSpam(t) => t.client_id(),
            ChatActionBar::ReportUnrelatedLocation(t) => t.client_id(),
            ChatActionBar::SharePhoneNumber(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatActionBar {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionBarAddContactBuilder {
        let mut inner = ChatActionBarAddContact::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionBarAddContactBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionBarAddContactBuilder {
    inner: ChatActionBarAddContact,
}

#[deprecated]
pub type RTDChatActionBarAddContactBuilder = ChatActionBarAddContactBuilder;

impl ChatActionBarAddContactBuilder {
    pub fn build(&self) -> ChatActionBarAddContact {
        self.inner.clone()
    }
}

impl AsRef<ChatActionBarAddContact> for ChatActionBarAddContact {
    fn as_ref(&self) -> &ChatActionBarAddContact {
        self
    }
}

impl AsRef<ChatActionBarAddContact> for ChatActionBarAddContactBuilder {
    fn as_ref(&self) -> &ChatActionBarAddContact {
        &self.inner
    }
}

/// The chat is a recently created group chat to which new members can be invited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarInviteMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionBarInviteMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatActionBar for ChatActionBarInviteMembers {}

impl ChatActionBarInviteMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionBarInviteMembersBuilder {
        let mut inner = ChatActionBarInviteMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionBarInviteMembersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionBarInviteMembersBuilder {
    inner: ChatActionBarInviteMembers,
}

#[deprecated]
pub type RTDChatActionBarInviteMembersBuilder = ChatActionBarInviteMembersBuilder;

impl ChatActionBarInviteMembersBuilder {
    pub fn build(&self) -> ChatActionBarInviteMembers {
        self.inner.clone()
    }
}

impl AsRef<ChatActionBarInviteMembers> for ChatActionBarInviteMembers {
    fn as_ref(&self) -> &ChatActionBarInviteMembers {
        self
    }
}

impl AsRef<ChatActionBarInviteMembers> for ChatActionBarInviteMembersBuilder {
    fn as_ref(&self) -> &ChatActionBarInviteMembers {
        &self.inner
    }
}

/// The chat is a private chat with an administrator of a chat to which the user sent join request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarJoinRequest {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Title of the chat to which the join request was sent

    #[serde(default)]
    title: String,
    /// True, if the join request was sent to a channel chat

    #[serde(default)]
    is_channel: bool,
    /// Point in time (Unix timestamp) when the join request was sent

    #[serde(default)]
    request_date: i32,
}

impl RObject for ChatActionBarJoinRequest {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatActionBar for ChatActionBarJoinRequest {}

impl ChatActionBarJoinRequest {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionBarJoinRequestBuilder {
        let mut inner = ChatActionBarJoinRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionBarJoinRequestBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn is_channel(&self) -> bool {
        self.is_channel
    }

    pub fn request_date(&self) -> i32 {
        self.request_date
    }
}

#[doc(hidden)]
pub struct ChatActionBarJoinRequestBuilder {
    inner: ChatActionBarJoinRequest,
}

#[deprecated]
pub type RTDChatActionBarJoinRequestBuilder = ChatActionBarJoinRequestBuilder;

impl ChatActionBarJoinRequestBuilder {
    pub fn build(&self) -> ChatActionBarJoinRequest {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn is_channel(&mut self, is_channel: bool) -> &mut Self {
        self.inner.is_channel = is_channel;
        self
    }

    pub fn request_date(&mut self, request_date: i32) -> &mut Self {
        self.inner.request_date = request_date;
        self
    }
}

impl AsRef<ChatActionBarJoinRequest> for ChatActionBarJoinRequest {
    fn as_ref(&self) -> &ChatActionBarJoinRequest {
        self
    }
}

impl AsRef<ChatActionBarJoinRequest> for ChatActionBarJoinRequestBuilder {
    fn as_ref(&self) -> &ChatActionBarJoinRequest {
        &self.inner
    }
}

/// The chat is a private or secret chat, which can be reported using the method reportChat, or the other user can be blocked using the method toggleMessageSenderIsBlocked, or the other user can be added to the contact list using the method addContact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionBarReportAddBlock {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If true, the chat was automatically archived and can be moved back to the main chat list using addChatToList simultaneously with setting chat notification settings to default using setChatNotificationSettings

    #[serde(default)]
    can_unarchive: bool,
    /// If non-negative, the current user was found by the peer through searchChatsNearby and this is the distance between the users

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionBarReportAddBlockBuilder {
        let mut inner = ChatActionBarReportAddBlock::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionBarReportAddBlockBuilder { inner }
    }

    pub fn can_unarchive(&self) -> bool {
        self.can_unarchive
    }

    pub fn distance(&self) -> i32 {
        self.distance
    }
}

#[doc(hidden)]
pub struct ChatActionBarReportAddBlockBuilder {
    inner: ChatActionBarReportAddBlock,
}

#[deprecated]
pub type RTDChatActionBarReportAddBlockBuilder = ChatActionBarReportAddBlockBuilder;

impl ChatActionBarReportAddBlockBuilder {
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

impl AsRef<ChatActionBarReportAddBlock> for ChatActionBarReportAddBlockBuilder {
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionBarReportSpamBuilder {
        let mut inner = ChatActionBarReportSpam::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionBarReportSpamBuilder { inner }
    }

    pub fn can_unarchive(&self) -> bool {
        self.can_unarchive
    }
}

#[doc(hidden)]
pub struct ChatActionBarReportSpamBuilder {
    inner: ChatActionBarReportSpam,
}

#[deprecated]
pub type RTDChatActionBarReportSpamBuilder = ChatActionBarReportSpamBuilder;

impl ChatActionBarReportSpamBuilder {
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

impl AsRef<ChatActionBarReportSpam> for ChatActionBarReportSpamBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionBarReportUnrelatedLocationBuilder {
        let mut inner = ChatActionBarReportUnrelatedLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionBarReportUnrelatedLocationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionBarReportUnrelatedLocationBuilder {
    inner: ChatActionBarReportUnrelatedLocation,
}

#[deprecated]
pub type RTDChatActionBarReportUnrelatedLocationBuilder =
    ChatActionBarReportUnrelatedLocationBuilder;

impl ChatActionBarReportUnrelatedLocationBuilder {
    pub fn build(&self) -> ChatActionBarReportUnrelatedLocation {
        self.inner.clone()
    }
}

impl AsRef<ChatActionBarReportUnrelatedLocation> for ChatActionBarReportUnrelatedLocation {
    fn as_ref(&self) -> &ChatActionBarReportUnrelatedLocation {
        self
    }
}

impl AsRef<ChatActionBarReportUnrelatedLocation> for ChatActionBarReportUnrelatedLocationBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionBarSharePhoneNumberBuilder {
        let mut inner = ChatActionBarSharePhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionBarSharePhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionBarSharePhoneNumberBuilder {
    inner: ChatActionBarSharePhoneNumber,
}

#[deprecated]
pub type RTDChatActionBarSharePhoneNumberBuilder = ChatActionBarSharePhoneNumberBuilder;

impl ChatActionBarSharePhoneNumberBuilder {
    pub fn build(&self) -> ChatActionBarSharePhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<ChatActionBarSharePhoneNumber> for ChatActionBarSharePhoneNumber {
    fn as_ref(&self) -> &ChatActionBarSharePhoneNumber {
        self
    }
}

impl AsRef<ChatActionBarSharePhoneNumber> for ChatActionBarSharePhoneNumberBuilder {
    fn as_ref(&self) -> &ChatActionBarSharePhoneNumber {
        &self.inner
    }
}
