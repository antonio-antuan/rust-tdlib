use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Provides information about the status of a member in a chat
pub trait TDChatMemberStatus: Debug + RObject {}

/// Provides information about the status of a member in a chat
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMemberStatus {
    #[doc(hidden)]
    _Default,
    /// The user is a member of a chat and has some additional privileges. In basic groups, administrators can edit and delete messages sent by others, add new members, and ban unprivileged members. In supergroups and channels, there are more detailed options for administrator privileges
    #[serde(rename(
        serialize = "chatMemberStatusAdministrator",
        deserialize = "chatMemberStatusAdministrator"
    ))]
    Administrator(ChatMemberStatusAdministrator),
    /// The user was banned (and hence is not a member of the chat). Implies the user can't return to the chat or view messages
    #[serde(rename(
        serialize = "chatMemberStatusBanned",
        deserialize = "chatMemberStatusBanned"
    ))]
    Banned(ChatMemberStatusBanned),
    /// The user is the owner of a chat and has all the administrator privileges
    #[serde(rename(
        serialize = "chatMemberStatusCreator",
        deserialize = "chatMemberStatusCreator"
    ))]
    Creator(ChatMemberStatusCreator),
    /// The user is not a chat member
    #[serde(rename(
        serialize = "chatMemberStatusLeft",
        deserialize = "chatMemberStatusLeft"
    ))]
    Left(ChatMemberStatusLeft),
    /// The user is a member of a chat, without any additional privileges or restrictions
    #[serde(rename(
        serialize = "chatMemberStatusMember",
        deserialize = "chatMemberStatusMember"
    ))]
    Member(ChatMemberStatusMember),
    /// The user is under certain restrictions in the chat. Not supported in basic groups and channels
    #[serde(rename(
        serialize = "chatMemberStatusRestricted",
        deserialize = "chatMemberStatusRestricted"
    ))]
    Restricted(ChatMemberStatusRestricted),
}

impl Default for ChatMemberStatus {
    fn default() -> Self {
        ChatMemberStatus::_Default
    }
}

impl RObject for ChatMemberStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatMemberStatus::Administrator(t) => t.extra(),
            ChatMemberStatus::Banned(t) => t.extra(),
            ChatMemberStatus::Creator(t) => t.extra(),
            ChatMemberStatus::Left(t) => t.extra(),
            ChatMemberStatus::Member(t) => t.extra(),
            ChatMemberStatus::Restricted(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatMemberStatus::Administrator(t) => t.client_id(),
            ChatMemberStatus::Banned(t) => t.client_id(),
            ChatMemberStatus::Creator(t) => t.client_id(),
            ChatMemberStatus::Left(t) => t.client_id(),
            ChatMemberStatus::Member(t) => t.client_id(),
            ChatMemberStatus::Restricted(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatMemberStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatMemberStatus::_Default)
    }
}

impl AsRef<ChatMemberStatus> for ChatMemberStatus {
    fn as_ref(&self) -> &ChatMemberStatus {
        self
    }
}

/// The user is a member of a chat and has some additional privileges. In basic groups, administrators can edit and delete messages sent by others, add new members, and ban unprivileged members. In supergroups and channels, there are more detailed options for administrator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusAdministrator {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A custom title of the administrator; 0-16 characters without emojis; applicable to supergroups only
    custom_title: String,
    /// True, if the current user can edit the administrator privileges for the called user
    can_be_edited: bool,
    /// True, if the administrator can change the chat title, photo, and other settings
    can_change_info: bool,
    /// True, if the administrator can create channel posts; applicable to channels only
    can_post_messages: bool,
    /// True, if the administrator can edit messages of other users and pin messages; applicable to channels only
    can_edit_messages: bool,
    /// True, if the administrator can delete messages of other users
    can_delete_messages: bool,
    /// True, if the administrator can invite new users to the chat
    can_invite_users: bool,
    /// True, if the administrator can restrict, ban, or unban chat members
    can_restrict_members: bool,
    /// True, if the administrator can pin messages; applicable to groups only
    can_pin_messages: bool,
    /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that were directly or indirectly promoted by them
    can_promote_members: bool,
    /// True, if the administrator isn't shown in the chat member list and sends messages anonymously; applicable to supergroups only
    is_anonymous: bool,
}

impl RObject for ChatMemberStatusAdministrator {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMemberStatus for ChatMemberStatusAdministrator {}

impl ChatMemberStatusAdministrator {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatMemberStatusAdministratorBuilder {
        let mut inner = ChatMemberStatusAdministrator::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatMemberStatusAdministratorBuilder { inner }
    }

    pub fn custom_title(&self) -> &String {
        &self.custom_title
    }

    pub fn can_be_edited(&self) -> bool {
        self.can_be_edited
    }

    pub fn can_change_info(&self) -> bool {
        self.can_change_info
    }

    pub fn can_post_messages(&self) -> bool {
        self.can_post_messages
    }

    pub fn can_edit_messages(&self) -> bool {
        self.can_edit_messages
    }

    pub fn can_delete_messages(&self) -> bool {
        self.can_delete_messages
    }

    pub fn can_invite_users(&self) -> bool {
        self.can_invite_users
    }

    pub fn can_restrict_members(&self) -> bool {
        self.can_restrict_members
    }

    pub fn can_pin_messages(&self) -> bool {
        self.can_pin_messages
    }

    pub fn can_promote_members(&self) -> bool {
        self.can_promote_members
    }

    pub fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusAdministratorBuilder {
    inner: ChatMemberStatusAdministrator,
}

impl RTDChatMemberStatusAdministratorBuilder {
    pub fn build(&self) -> ChatMemberStatusAdministrator {
        self.inner.clone()
    }

    pub fn custom_title<T: AsRef<str>>(&mut self, custom_title: T) -> &mut Self {
        self.inner.custom_title = custom_title.as_ref().to_string();
        self
    }

    pub fn can_be_edited(&mut self, can_be_edited: bool) -> &mut Self {
        self.inner.can_be_edited = can_be_edited;
        self
    }

    pub fn can_change_info(&mut self, can_change_info: bool) -> &mut Self {
        self.inner.can_change_info = can_change_info;
        self
    }

    pub fn can_post_messages(&mut self, can_post_messages: bool) -> &mut Self {
        self.inner.can_post_messages = can_post_messages;
        self
    }

    pub fn can_edit_messages(&mut self, can_edit_messages: bool) -> &mut Self {
        self.inner.can_edit_messages = can_edit_messages;
        self
    }

    pub fn can_delete_messages(&mut self, can_delete_messages: bool) -> &mut Self {
        self.inner.can_delete_messages = can_delete_messages;
        self
    }

    pub fn can_invite_users(&mut self, can_invite_users: bool) -> &mut Self {
        self.inner.can_invite_users = can_invite_users;
        self
    }

    pub fn can_restrict_members(&mut self, can_restrict_members: bool) -> &mut Self {
        self.inner.can_restrict_members = can_restrict_members;
        self
    }

    pub fn can_pin_messages(&mut self, can_pin_messages: bool) -> &mut Self {
        self.inner.can_pin_messages = can_pin_messages;
        self
    }

    pub fn can_promote_members(&mut self, can_promote_members: bool) -> &mut Self {
        self.inner.can_promote_members = can_promote_members;
        self
    }

    pub fn is_anonymous(&mut self, is_anonymous: bool) -> &mut Self {
        self.inner.is_anonymous = is_anonymous;
        self
    }
}

impl AsRef<ChatMemberStatusAdministrator> for ChatMemberStatusAdministrator {
    fn as_ref(&self) -> &ChatMemberStatusAdministrator {
        self
    }
}

impl AsRef<ChatMemberStatusAdministrator> for RTDChatMemberStatusAdministratorBuilder {
    fn as_ref(&self) -> &ChatMemberStatusAdministrator {
        &self.inner
    }
}

/// The user was banned (and hence is not a member of the chat). Implies the user can't return to the chat or view messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusBanned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever
    banned_until_date: i32,
}

impl RObject for ChatMemberStatusBanned {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMemberStatus for ChatMemberStatusBanned {}

impl ChatMemberStatusBanned {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatMemberStatusBannedBuilder {
        let mut inner = ChatMemberStatusBanned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatMemberStatusBannedBuilder { inner }
    }

    pub fn banned_until_date(&self) -> i32 {
        self.banned_until_date
    }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusBannedBuilder {
    inner: ChatMemberStatusBanned,
}

impl RTDChatMemberStatusBannedBuilder {
    pub fn build(&self) -> ChatMemberStatusBanned {
        self.inner.clone()
    }

    pub fn banned_until_date(&mut self, banned_until_date: i32) -> &mut Self {
        self.inner.banned_until_date = banned_until_date;
        self
    }
}

impl AsRef<ChatMemberStatusBanned> for ChatMemberStatusBanned {
    fn as_ref(&self) -> &ChatMemberStatusBanned {
        self
    }
}

impl AsRef<ChatMemberStatusBanned> for RTDChatMemberStatusBannedBuilder {
    fn as_ref(&self) -> &ChatMemberStatusBanned {
        &self.inner
    }
}

/// The user is the owner of a chat and has all the administrator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusCreator {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A custom title of the owner; 0-16 characters without emojis; applicable to supergroups only
    custom_title: String,
    /// True, if the creator isn't shown in the chat member list and sends messages anonymously; applicable to supergroups only
    is_anonymous: bool,
    /// True, if the user is a member of the chat
    is_member: bool,
}

impl RObject for ChatMemberStatusCreator {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMemberStatus for ChatMemberStatusCreator {}

impl ChatMemberStatusCreator {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatMemberStatusCreatorBuilder {
        let mut inner = ChatMemberStatusCreator::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatMemberStatusCreatorBuilder { inner }
    }

    pub fn custom_title(&self) -> &String {
        &self.custom_title
    }

    pub fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }

    pub fn is_member(&self) -> bool {
        self.is_member
    }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusCreatorBuilder {
    inner: ChatMemberStatusCreator,
}

impl RTDChatMemberStatusCreatorBuilder {
    pub fn build(&self) -> ChatMemberStatusCreator {
        self.inner.clone()
    }

    pub fn custom_title<T: AsRef<str>>(&mut self, custom_title: T) -> &mut Self {
        self.inner.custom_title = custom_title.as_ref().to_string();
        self
    }

    pub fn is_anonymous(&mut self, is_anonymous: bool) -> &mut Self {
        self.inner.is_anonymous = is_anonymous;
        self
    }

    pub fn is_member(&mut self, is_member: bool) -> &mut Self {
        self.inner.is_member = is_member;
        self
    }
}

impl AsRef<ChatMemberStatusCreator> for ChatMemberStatusCreator {
    fn as_ref(&self) -> &ChatMemberStatusCreator {
        self
    }
}

impl AsRef<ChatMemberStatusCreator> for RTDChatMemberStatusCreatorBuilder {
    fn as_ref(&self) -> &ChatMemberStatusCreator {
        &self.inner
    }
}

/// The user is not a chat member
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusLeft {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatMemberStatusLeft {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMemberStatus for ChatMemberStatusLeft {}

impl ChatMemberStatusLeft {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatMemberStatusLeftBuilder {
        let mut inner = ChatMemberStatusLeft::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatMemberStatusLeftBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusLeftBuilder {
    inner: ChatMemberStatusLeft,
}

impl RTDChatMemberStatusLeftBuilder {
    pub fn build(&self) -> ChatMemberStatusLeft {
        self.inner.clone()
    }
}

impl AsRef<ChatMemberStatusLeft> for ChatMemberStatusLeft {
    fn as_ref(&self) -> &ChatMemberStatusLeft {
        self
    }
}

impl AsRef<ChatMemberStatusLeft> for RTDChatMemberStatusLeftBuilder {
    fn as_ref(&self) -> &ChatMemberStatusLeft {
        &self.inner
    }
}

/// The user is a member of a chat, without any additional privileges or restrictions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatMemberStatusMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMemberStatus for ChatMemberStatusMember {}

impl ChatMemberStatusMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatMemberStatusMemberBuilder {
        let mut inner = ChatMemberStatusMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatMemberStatusMemberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusMemberBuilder {
    inner: ChatMemberStatusMember,
}

impl RTDChatMemberStatusMemberBuilder {
    pub fn build(&self) -> ChatMemberStatusMember {
        self.inner.clone()
    }
}

impl AsRef<ChatMemberStatusMember> for ChatMemberStatusMember {
    fn as_ref(&self) -> &ChatMemberStatusMember {
        self
    }
}

impl AsRef<ChatMemberStatusMember> for RTDChatMemberStatusMemberBuilder {
    fn as_ref(&self) -> &ChatMemberStatusMember {
        &self.inner
    }
}

/// The user is under certain restrictions in the chat. Not supported in basic groups and channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusRestricted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the user is a member of the chat
    is_member: bool,
    /// Point in time (Unix timestamp) when restrictions will be lifted from the user; 0 if never. If the user is restricted for more than 366 days or for less than 30 seconds from the current time, the user is considered to be restricted forever
    restricted_until_date: i32,
    /// User permissions in the chat
    permissions: ChatPermissions,
}

impl RObject for ChatMemberStatusRestricted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatMemberStatus for ChatMemberStatusRestricted {}

impl ChatMemberStatusRestricted {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatMemberStatusRestrictedBuilder {
        let mut inner = ChatMemberStatusRestricted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatMemberStatusRestrictedBuilder { inner }
    }

    pub fn is_member(&self) -> bool {
        self.is_member
    }

    pub fn restricted_until_date(&self) -> i32 {
        self.restricted_until_date
    }

    pub fn permissions(&self) -> &ChatPermissions {
        &self.permissions
    }
}

#[doc(hidden)]
pub struct RTDChatMemberStatusRestrictedBuilder {
    inner: ChatMemberStatusRestricted,
}

impl RTDChatMemberStatusRestrictedBuilder {
    pub fn build(&self) -> ChatMemberStatusRestricted {
        self.inner.clone()
    }

    pub fn is_member(&mut self, is_member: bool) -> &mut Self {
        self.inner.is_member = is_member;
        self
    }

    pub fn restricted_until_date(&mut self, restricted_until_date: i32) -> &mut Self {
        self.inner.restricted_until_date = restricted_until_date;
        self
    }

    pub fn permissions<T: AsRef<ChatPermissions>>(&mut self, permissions: T) -> &mut Self {
        self.inner.permissions = permissions.as_ref().clone();
        self
    }
}

impl AsRef<ChatMemberStatusRestricted> for ChatMemberStatusRestricted {
    fn as_ref(&self) -> &ChatMemberStatusRestricted {
        self
    }
}

impl AsRef<ChatMemberStatusRestricted> for RTDChatMemberStatusRestrictedBuilder {
    fn as_ref(&self) -> &ChatMemberStatusRestricted {
        &self.inner
    }
}
