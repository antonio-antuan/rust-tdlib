use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents a chat event
pub trait TDChatEventAction: Debug + RObject {}

/// Represents a chat event
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatEventAction {
    #[doc(hidden)]
    _Default,
    /// The chat description was changed
    #[serde(rename(
        serialize = "chatEventDescriptionChanged",
        deserialize = "chatEventDescriptionChanged"
    ))]
    ChatEventDescriptionChanged(ChatEventDescriptionChanged),
    /// The can_invite_users permission of a supergroup chat was toggled
    #[serde(rename(
        serialize = "chatEventInvitesToggled",
        deserialize = "chatEventInvitesToggled"
    ))]
    ChatEventInvitesToggled(ChatEventInvitesToggled),
    /// The is_all_history_available setting of a supergroup was toggled
    #[serde(rename(
        serialize = "chatEventIsAllHistoryAvailableToggled",
        deserialize = "chatEventIsAllHistoryAvailableToggled"
    ))]
    ChatEventIsAllHistoryAvailableToggled(ChatEventIsAllHistoryAvailableToggled),
    /// The linked chat of a supergroup was changed
    #[serde(rename(
        serialize = "chatEventLinkedChatChanged",
        deserialize = "chatEventLinkedChatChanged"
    ))]
    ChatEventLinkedChatChanged(ChatEventLinkedChatChanged),
    /// The supergroup location was changed
    #[serde(rename(
        serialize = "chatEventLocationChanged",
        deserialize = "chatEventLocationChanged"
    ))]
    ChatEventLocationChanged(ChatEventLocationChanged),
    /// A new chat member was invited
    #[serde(rename(
        serialize = "chatEventMemberInvited",
        deserialize = "chatEventMemberInvited"
    ))]
    ChatEventMemberInvited(ChatEventMemberInvited),
    /// A new member joined the chat
    #[serde(rename(
        serialize = "chatEventMemberJoined",
        deserialize = "chatEventMemberJoined"
    ))]
    ChatEventMemberJoined(ChatEventMemberJoined),
    /// A member left the chat
    #[serde(rename(serialize = "chatEventMemberLeft", deserialize = "chatEventMemberLeft"))]
    ChatEventMemberLeft(ChatEventMemberLeft),
    /// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed
    #[serde(rename(
        serialize = "chatEventMemberPromoted",
        deserialize = "chatEventMemberPromoted"
    ))]
    ChatEventMemberPromoted(ChatEventMemberPromoted),
    /// A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed
    #[serde(rename(
        serialize = "chatEventMemberRestricted",
        deserialize = "chatEventMemberRestricted"
    ))]
    ChatEventMemberRestricted(ChatEventMemberRestricted),
    /// A message was deleted
    #[serde(rename(
        serialize = "chatEventMessageDeleted",
        deserialize = "chatEventMessageDeleted"
    ))]
    ChatEventMessageDeleted(ChatEventMessageDeleted),
    /// A message was edited
    #[serde(rename(
        serialize = "chatEventMessageEdited",
        deserialize = "chatEventMessageEdited"
    ))]
    ChatEventMessageEdited(ChatEventMessageEdited),
    /// A message was pinned
    #[serde(rename(
        serialize = "chatEventMessagePinned",
        deserialize = "chatEventMessagePinned"
    ))]
    ChatEventMessagePinned(ChatEventMessagePinned),
    /// A message was unpinned
    #[serde(rename(
        serialize = "chatEventMessageUnpinned",
        deserialize = "chatEventMessageUnpinned"
    ))]
    ChatEventMessageUnpinned(ChatEventMessageUnpinned),
    /// The chat permissions was changed
    #[serde(rename(
        serialize = "chatEventPermissionsChanged",
        deserialize = "chatEventPermissionsChanged"
    ))]
    ChatEventPermissionsChanged(ChatEventPermissionsChanged),
    /// The chat photo was changed
    #[serde(rename(
        serialize = "chatEventPhotoChanged",
        deserialize = "chatEventPhotoChanged"
    ))]
    ChatEventPhotoChanged(ChatEventPhotoChanged),
    /// A poll in a message was stopped
    #[serde(rename(
        serialize = "chatEventPollStopped",
        deserialize = "chatEventPollStopped"
    ))]
    ChatEventPollStopped(ChatEventPollStopped),
    /// The sign_messages setting of a channel was toggled
    #[serde(rename(
        serialize = "chatEventSignMessagesToggled",
        deserialize = "chatEventSignMessagesToggled"
    ))]
    ChatEventSignMessagesToggled(ChatEventSignMessagesToggled),
    /// The slow_mode_delay setting of a supergroup was changed
    #[serde(rename(
        serialize = "chatEventSlowModeDelayChanged",
        deserialize = "chatEventSlowModeDelayChanged"
    ))]
    ChatEventSlowModeDelayChanged(ChatEventSlowModeDelayChanged),
    /// The supergroup sticker set was changed
    #[serde(rename(
        serialize = "chatEventStickerSetChanged",
        deserialize = "chatEventStickerSetChanged"
    ))]
    ChatEventStickerSetChanged(ChatEventStickerSetChanged),
    /// The chat title was changed
    #[serde(rename(
        serialize = "chatEventTitleChanged",
        deserialize = "chatEventTitleChanged"
    ))]
    ChatEventTitleChanged(ChatEventTitleChanged),
    /// The chat username was changed
    #[serde(rename(
        serialize = "chatEventUsernameChanged",
        deserialize = "chatEventUsernameChanged"
    ))]
    ChatEventUsernameChanged(ChatEventUsernameChanged),
}

impl Default for ChatEventAction {
    fn default() -> Self {
        ChatEventAction::_Default
    }
}

impl RObject for ChatEventAction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatEventAction::ChatEventDescriptionChanged(t) => t.extra(),
            ChatEventAction::ChatEventInvitesToggled(t) => t.extra(),
            ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t) => t.extra(),
            ChatEventAction::ChatEventLinkedChatChanged(t) => t.extra(),
            ChatEventAction::ChatEventLocationChanged(t) => t.extra(),
            ChatEventAction::ChatEventMemberInvited(t) => t.extra(),
            ChatEventAction::ChatEventMemberJoined(t) => t.extra(),
            ChatEventAction::ChatEventMemberLeft(t) => t.extra(),
            ChatEventAction::ChatEventMemberPromoted(t) => t.extra(),
            ChatEventAction::ChatEventMemberRestricted(t) => t.extra(),
            ChatEventAction::ChatEventMessageDeleted(t) => t.extra(),
            ChatEventAction::ChatEventMessageEdited(t) => t.extra(),
            ChatEventAction::ChatEventMessagePinned(t) => t.extra(),
            ChatEventAction::ChatEventMessageUnpinned(t) => t.extra(),
            ChatEventAction::ChatEventPermissionsChanged(t) => t.extra(),
            ChatEventAction::ChatEventPhotoChanged(t) => t.extra(),
            ChatEventAction::ChatEventPollStopped(t) => t.extra(),
            ChatEventAction::ChatEventSignMessagesToggled(t) => t.extra(),
            ChatEventAction::ChatEventSlowModeDelayChanged(t) => t.extra(),
            ChatEventAction::ChatEventStickerSetChanged(t) => t.extra(),
            ChatEventAction::ChatEventTitleChanged(t) => t.extra(),
            ChatEventAction::ChatEventUsernameChanged(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatEventAction::ChatEventDescriptionChanged(t) => t.client_id(),
            ChatEventAction::ChatEventInvitesToggled(t) => t.client_id(),
            ChatEventAction::ChatEventIsAllHistoryAvailableToggled(t) => t.client_id(),
            ChatEventAction::ChatEventLinkedChatChanged(t) => t.client_id(),
            ChatEventAction::ChatEventLocationChanged(t) => t.client_id(),
            ChatEventAction::ChatEventMemberInvited(t) => t.client_id(),
            ChatEventAction::ChatEventMemberJoined(t) => t.client_id(),
            ChatEventAction::ChatEventMemberLeft(t) => t.client_id(),
            ChatEventAction::ChatEventMemberPromoted(t) => t.client_id(),
            ChatEventAction::ChatEventMemberRestricted(t) => t.client_id(),
            ChatEventAction::ChatEventMessageDeleted(t) => t.client_id(),
            ChatEventAction::ChatEventMessageEdited(t) => t.client_id(),
            ChatEventAction::ChatEventMessagePinned(t) => t.client_id(),
            ChatEventAction::ChatEventMessageUnpinned(t) => t.client_id(),
            ChatEventAction::ChatEventPermissionsChanged(t) => t.client_id(),
            ChatEventAction::ChatEventPhotoChanged(t) => t.client_id(),
            ChatEventAction::ChatEventPollStopped(t) => t.client_id(),
            ChatEventAction::ChatEventSignMessagesToggled(t) => t.client_id(),
            ChatEventAction::ChatEventSlowModeDelayChanged(t) => t.client_id(),
            ChatEventAction::ChatEventStickerSetChanged(t) => t.client_id(),
            ChatEventAction::ChatEventTitleChanged(t) => t.client_id(),
            ChatEventAction::ChatEventUsernameChanged(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatEventAction {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatEventAction::_Default)
    }
}

impl AsRef<ChatEventAction> for ChatEventAction {
    fn as_ref(&self) -> &ChatEventAction {
        self
    }
}

/// The chat description was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventDescriptionChanged {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Previous chat description
    old_description: String,
    /// New chat description
    new_description: String,
}

impl RObject for ChatEventDescriptionChanged {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventDescriptionChanged {}

impl ChatEventDescriptionChanged {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventDescriptionChangedBuilder {
        let mut inner = ChatEventDescriptionChanged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventDescriptionChangedBuilder { inner }
    }

    pub fn old_description(&self) -> &String {
        &self.old_description
    }

    pub fn new_description(&self) -> &String {
        &self.new_description
    }
}

#[doc(hidden)]
pub struct RTDChatEventDescriptionChangedBuilder {
    inner: ChatEventDescriptionChanged,
}

impl RTDChatEventDescriptionChangedBuilder {
    pub fn build(&self) -> ChatEventDescriptionChanged {
        self.inner.clone()
    }

    pub fn old_description<T: AsRef<str>>(&mut self, old_description: T) -> &mut Self {
        self.inner.old_description = old_description.as_ref().to_string();
        self
    }

    pub fn new_description<T: AsRef<str>>(&mut self, new_description: T) -> &mut Self {
        self.inner.new_description = new_description.as_ref().to_string();
        self
    }
}

impl AsRef<ChatEventDescriptionChanged> for ChatEventDescriptionChanged {
    fn as_ref(&self) -> &ChatEventDescriptionChanged {
        self
    }
}

impl AsRef<ChatEventDescriptionChanged> for RTDChatEventDescriptionChangedBuilder {
    fn as_ref(&self) -> &ChatEventDescriptionChanged {
        &self.inner
    }
}

/// The can_invite_users permission of a supergroup chat was toggled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventInvitesToggled {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New value of can_invite_users permission
    can_invite_users: bool,
}

impl RObject for ChatEventInvitesToggled {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventInvitesToggled {}

impl ChatEventInvitesToggled {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventInvitesToggledBuilder {
        let mut inner = ChatEventInvitesToggled::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventInvitesToggledBuilder { inner }
    }

    pub fn can_invite_users(&self) -> bool {
        self.can_invite_users
    }
}

#[doc(hidden)]
pub struct RTDChatEventInvitesToggledBuilder {
    inner: ChatEventInvitesToggled,
}

impl RTDChatEventInvitesToggledBuilder {
    pub fn build(&self) -> ChatEventInvitesToggled {
        self.inner.clone()
    }

    pub fn can_invite_users(&mut self, can_invite_users: bool) -> &mut Self {
        self.inner.can_invite_users = can_invite_users;
        self
    }
}

impl AsRef<ChatEventInvitesToggled> for ChatEventInvitesToggled {
    fn as_ref(&self) -> &ChatEventInvitesToggled {
        self
    }
}

impl AsRef<ChatEventInvitesToggled> for RTDChatEventInvitesToggledBuilder {
    fn as_ref(&self) -> &ChatEventInvitesToggled {
        &self.inner
    }
}

/// The is_all_history_available setting of a supergroup was toggled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventIsAllHistoryAvailableToggled {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New value of is_all_history_available
    is_all_history_available: bool,
}

impl RObject for ChatEventIsAllHistoryAvailableToggled {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventIsAllHistoryAvailableToggled {}

impl ChatEventIsAllHistoryAvailableToggled {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventIsAllHistoryAvailableToggledBuilder {
        let mut inner = ChatEventIsAllHistoryAvailableToggled::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventIsAllHistoryAvailableToggledBuilder { inner }
    }

    pub fn is_all_history_available(&self) -> bool {
        self.is_all_history_available
    }
}

#[doc(hidden)]
pub struct RTDChatEventIsAllHistoryAvailableToggledBuilder {
    inner: ChatEventIsAllHistoryAvailableToggled,
}

impl RTDChatEventIsAllHistoryAvailableToggledBuilder {
    pub fn build(&self) -> ChatEventIsAllHistoryAvailableToggled {
        self.inner.clone()
    }

    pub fn is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self {
        self.inner.is_all_history_available = is_all_history_available;
        self
    }
}

impl AsRef<ChatEventIsAllHistoryAvailableToggled> for ChatEventIsAllHistoryAvailableToggled {
    fn as_ref(&self) -> &ChatEventIsAllHistoryAvailableToggled {
        self
    }
}

impl AsRef<ChatEventIsAllHistoryAvailableToggled>
    for RTDChatEventIsAllHistoryAvailableToggledBuilder
{
    fn as_ref(&self) -> &ChatEventIsAllHistoryAvailableToggled {
        &self.inner
    }
}

/// The linked chat of a supergroup was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventLinkedChatChanged {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Previous supergroup linked chat identifier
    old_linked_chat_id: i64,
    /// New supergroup linked chat identifier
    new_linked_chat_id: i64,
}

impl RObject for ChatEventLinkedChatChanged {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventLinkedChatChanged {}

impl ChatEventLinkedChatChanged {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventLinkedChatChangedBuilder {
        let mut inner = ChatEventLinkedChatChanged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventLinkedChatChangedBuilder { inner }
    }

    pub fn old_linked_chat_id(&self) -> i64 {
        self.old_linked_chat_id
    }

    pub fn new_linked_chat_id(&self) -> i64 {
        self.new_linked_chat_id
    }
}

#[doc(hidden)]
pub struct RTDChatEventLinkedChatChangedBuilder {
    inner: ChatEventLinkedChatChanged,
}

impl RTDChatEventLinkedChatChangedBuilder {
    pub fn build(&self) -> ChatEventLinkedChatChanged {
        self.inner.clone()
    }

    pub fn old_linked_chat_id(&mut self, old_linked_chat_id: i64) -> &mut Self {
        self.inner.old_linked_chat_id = old_linked_chat_id;
        self
    }

    pub fn new_linked_chat_id(&mut self, new_linked_chat_id: i64) -> &mut Self {
        self.inner.new_linked_chat_id = new_linked_chat_id;
        self
    }
}

impl AsRef<ChatEventLinkedChatChanged> for ChatEventLinkedChatChanged {
    fn as_ref(&self) -> &ChatEventLinkedChatChanged {
        self
    }
}

impl AsRef<ChatEventLinkedChatChanged> for RTDChatEventLinkedChatChangedBuilder {
    fn as_ref(&self) -> &ChatEventLinkedChatChanged {
        &self.inner
    }
}

/// The supergroup location was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventLocationChanged {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Previous location; may be null
    old_location: Option<ChatLocation>,
    /// New location; may be null
    new_location: Option<ChatLocation>,
}

impl RObject for ChatEventLocationChanged {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventLocationChanged {}

impl ChatEventLocationChanged {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventLocationChangedBuilder {
        let mut inner = ChatEventLocationChanged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventLocationChangedBuilder { inner }
    }

    pub fn old_location(&self) -> &Option<ChatLocation> {
        &self.old_location
    }

    pub fn new_location(&self) -> &Option<ChatLocation> {
        &self.new_location
    }
}

#[doc(hidden)]
pub struct RTDChatEventLocationChangedBuilder {
    inner: ChatEventLocationChanged,
}

impl RTDChatEventLocationChangedBuilder {
    pub fn build(&self) -> ChatEventLocationChanged {
        self.inner.clone()
    }

    pub fn old_location<T: AsRef<ChatLocation>>(&mut self, old_location: T) -> &mut Self {
        self.inner.old_location = Some(old_location.as_ref().clone());
        self
    }

    pub fn new_location<T: AsRef<ChatLocation>>(&mut self, new_location: T) -> &mut Self {
        self.inner.new_location = Some(new_location.as_ref().clone());
        self
    }
}

impl AsRef<ChatEventLocationChanged> for ChatEventLocationChanged {
    fn as_ref(&self) -> &ChatEventLocationChanged {
        self
    }
}

impl AsRef<ChatEventLocationChanged> for RTDChatEventLocationChangedBuilder {
    fn as_ref(&self) -> &ChatEventLocationChanged {
        &self.inner
    }
}

/// A new chat member was invited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberInvited {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New member user identifier
    user_id: i32,
    /// New member status

    #[serde(skip_serializing_if = "ChatMemberStatus::_is_default")]
    status: ChatMemberStatus,
}

impl RObject for ChatEventMemberInvited {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventMemberInvited {}

impl ChatEventMemberInvited {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventMemberInvitedBuilder {
        let mut inner = ChatEventMemberInvited::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventMemberInvitedBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn status(&self) -> &ChatMemberStatus {
        &self.status
    }
}

#[doc(hidden)]
pub struct RTDChatEventMemberInvitedBuilder {
    inner: ChatEventMemberInvited,
}

impl RTDChatEventMemberInvitedBuilder {
    pub fn build(&self) -> ChatEventMemberInvited {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn status<T: AsRef<ChatMemberStatus>>(&mut self, status: T) -> &mut Self {
        self.inner.status = status.as_ref().clone();
        self
    }
}

impl AsRef<ChatEventMemberInvited> for ChatEventMemberInvited {
    fn as_ref(&self) -> &ChatEventMemberInvited {
        self
    }
}

impl AsRef<ChatEventMemberInvited> for RTDChatEventMemberInvitedBuilder {
    fn as_ref(&self) -> &ChatEventMemberInvited {
        &self.inner
    }
}

/// A new member joined the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberJoined {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatEventMemberJoined {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventMemberJoined {}

impl ChatEventMemberJoined {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventMemberJoinedBuilder {
        let mut inner = ChatEventMemberJoined::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventMemberJoinedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatEventMemberJoinedBuilder {
    inner: ChatEventMemberJoined,
}

impl RTDChatEventMemberJoinedBuilder {
    pub fn build(&self) -> ChatEventMemberJoined {
        self.inner.clone()
    }
}

impl AsRef<ChatEventMemberJoined> for ChatEventMemberJoined {
    fn as_ref(&self) -> &ChatEventMemberJoined {
        self
    }
}

impl AsRef<ChatEventMemberJoined> for RTDChatEventMemberJoinedBuilder {
    fn as_ref(&self) -> &ChatEventMemberJoined {
        &self.inner
    }
}

/// A member left the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberLeft {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatEventMemberLeft {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventMemberLeft {}

impl ChatEventMemberLeft {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventMemberLeftBuilder {
        let mut inner = ChatEventMemberLeft::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventMemberLeftBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatEventMemberLeftBuilder {
    inner: ChatEventMemberLeft,
}

impl RTDChatEventMemberLeftBuilder {
    pub fn build(&self) -> ChatEventMemberLeft {
        self.inner.clone()
    }
}

impl AsRef<ChatEventMemberLeft> for ChatEventMemberLeft {
    fn as_ref(&self) -> &ChatEventMemberLeft {
        self
    }
}

impl AsRef<ChatEventMemberLeft> for RTDChatEventMemberLeftBuilder {
    fn as_ref(&self) -> &ChatEventMemberLeft {
        &self.inner
    }
}

/// A chat member has gained/lost administrator status, or the list of their administrator privileges has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberPromoted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat member user identifier
    user_id: i32,
    /// Previous status of the chat member

    #[serde(skip_serializing_if = "ChatMemberStatus::_is_default")]
    old_status: ChatMemberStatus,
    /// New status of the chat member

    #[serde(skip_serializing_if = "ChatMemberStatus::_is_default")]
    new_status: ChatMemberStatus,
}

impl RObject for ChatEventMemberPromoted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventMemberPromoted {}

impl ChatEventMemberPromoted {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventMemberPromotedBuilder {
        let mut inner = ChatEventMemberPromoted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventMemberPromotedBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn old_status(&self) -> &ChatMemberStatus {
        &self.old_status
    }

    pub fn new_status(&self) -> &ChatMemberStatus {
        &self.new_status
    }
}

#[doc(hidden)]
pub struct RTDChatEventMemberPromotedBuilder {
    inner: ChatEventMemberPromoted,
}

impl RTDChatEventMemberPromotedBuilder {
    pub fn build(&self) -> ChatEventMemberPromoted {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn old_status<T: AsRef<ChatMemberStatus>>(&mut self, old_status: T) -> &mut Self {
        self.inner.old_status = old_status.as_ref().clone();
        self
    }

    pub fn new_status<T: AsRef<ChatMemberStatus>>(&mut self, new_status: T) -> &mut Self {
        self.inner.new_status = new_status.as_ref().clone();
        self
    }
}

impl AsRef<ChatEventMemberPromoted> for ChatEventMemberPromoted {
    fn as_ref(&self) -> &ChatEventMemberPromoted {
        self
    }
}

impl AsRef<ChatEventMemberPromoted> for RTDChatEventMemberPromotedBuilder {
    fn as_ref(&self) -> &ChatEventMemberPromoted {
        &self.inner
    }
}

/// A chat member was restricted/unrestricted or banned/unbanned, or the list of their restrictions has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMemberRestricted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat member user identifier
    user_id: i32,
    /// Previous status of the chat member

    #[serde(skip_serializing_if = "ChatMemberStatus::_is_default")]
    old_status: ChatMemberStatus,
    /// New status of the chat member

    #[serde(skip_serializing_if = "ChatMemberStatus::_is_default")]
    new_status: ChatMemberStatus,
}

impl RObject for ChatEventMemberRestricted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventMemberRestricted {}

impl ChatEventMemberRestricted {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventMemberRestrictedBuilder {
        let mut inner = ChatEventMemberRestricted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventMemberRestrictedBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn old_status(&self) -> &ChatMemberStatus {
        &self.old_status
    }

    pub fn new_status(&self) -> &ChatMemberStatus {
        &self.new_status
    }
}

#[doc(hidden)]
pub struct RTDChatEventMemberRestrictedBuilder {
    inner: ChatEventMemberRestricted,
}

impl RTDChatEventMemberRestrictedBuilder {
    pub fn build(&self) -> ChatEventMemberRestricted {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn old_status<T: AsRef<ChatMemberStatus>>(&mut self, old_status: T) -> &mut Self {
        self.inner.old_status = old_status.as_ref().clone();
        self
    }

    pub fn new_status<T: AsRef<ChatMemberStatus>>(&mut self, new_status: T) -> &mut Self {
        self.inner.new_status = new_status.as_ref().clone();
        self
    }
}

impl AsRef<ChatEventMemberRestricted> for ChatEventMemberRestricted {
    fn as_ref(&self) -> &ChatEventMemberRestricted {
        self
    }
}

impl AsRef<ChatEventMemberRestricted> for RTDChatEventMemberRestrictedBuilder {
    fn as_ref(&self) -> &ChatEventMemberRestricted {
        &self.inner
    }
}

/// A message was deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessageDeleted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Deleted message
    message: Message,
}

impl RObject for ChatEventMessageDeleted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventMessageDeleted {}

impl ChatEventMessageDeleted {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventMessageDeletedBuilder {
        let mut inner = ChatEventMessageDeleted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventMessageDeletedBuilder { inner }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}

#[doc(hidden)]
pub struct RTDChatEventMessageDeletedBuilder {
    inner: ChatEventMessageDeleted,
}

impl RTDChatEventMessageDeletedBuilder {
    pub fn build(&self) -> ChatEventMessageDeleted {
        self.inner.clone()
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }
}

impl AsRef<ChatEventMessageDeleted> for ChatEventMessageDeleted {
    fn as_ref(&self) -> &ChatEventMessageDeleted {
        self
    }
}

impl AsRef<ChatEventMessageDeleted> for RTDChatEventMessageDeletedBuilder {
    fn as_ref(&self) -> &ChatEventMessageDeleted {
        &self.inner
    }
}

/// A message was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessageEdited {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The original message before the edit
    old_message: Message,
    /// The message after it was edited
    new_message: Message,
}

impl RObject for ChatEventMessageEdited {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventMessageEdited {}

impl ChatEventMessageEdited {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventMessageEditedBuilder {
        let mut inner = ChatEventMessageEdited::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventMessageEditedBuilder { inner }
    }

    pub fn old_message(&self) -> &Message {
        &self.old_message
    }

    pub fn new_message(&self) -> &Message {
        &self.new_message
    }
}

#[doc(hidden)]
pub struct RTDChatEventMessageEditedBuilder {
    inner: ChatEventMessageEdited,
}

impl RTDChatEventMessageEditedBuilder {
    pub fn build(&self) -> ChatEventMessageEdited {
        self.inner.clone()
    }

    pub fn old_message<T: AsRef<Message>>(&mut self, old_message: T) -> &mut Self {
        self.inner.old_message = old_message.as_ref().clone();
        self
    }

    pub fn new_message<T: AsRef<Message>>(&mut self, new_message: T) -> &mut Self {
        self.inner.new_message = new_message.as_ref().clone();
        self
    }
}

impl AsRef<ChatEventMessageEdited> for ChatEventMessageEdited {
    fn as_ref(&self) -> &ChatEventMessageEdited {
        self
    }
}

impl AsRef<ChatEventMessageEdited> for RTDChatEventMessageEditedBuilder {
    fn as_ref(&self) -> &ChatEventMessageEdited {
        &self.inner
    }
}

/// A message was pinned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessagePinned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pinned message
    message: Message,
}

impl RObject for ChatEventMessagePinned {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventMessagePinned {}

impl ChatEventMessagePinned {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventMessagePinnedBuilder {
        let mut inner = ChatEventMessagePinned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventMessagePinnedBuilder { inner }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}

#[doc(hidden)]
pub struct RTDChatEventMessagePinnedBuilder {
    inner: ChatEventMessagePinned,
}

impl RTDChatEventMessagePinnedBuilder {
    pub fn build(&self) -> ChatEventMessagePinned {
        self.inner.clone()
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }
}

impl AsRef<ChatEventMessagePinned> for ChatEventMessagePinned {
    fn as_ref(&self) -> &ChatEventMessagePinned {
        self
    }
}

impl AsRef<ChatEventMessagePinned> for RTDChatEventMessagePinnedBuilder {
    fn as_ref(&self) -> &ChatEventMessagePinned {
        &self.inner
    }
}

/// A message was unpinned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventMessageUnpinned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unpinned message
    message: Message,
}

impl RObject for ChatEventMessageUnpinned {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventMessageUnpinned {}

impl ChatEventMessageUnpinned {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventMessageUnpinnedBuilder {
        let mut inner = ChatEventMessageUnpinned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventMessageUnpinnedBuilder { inner }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}

#[doc(hidden)]
pub struct RTDChatEventMessageUnpinnedBuilder {
    inner: ChatEventMessageUnpinned,
}

impl RTDChatEventMessageUnpinnedBuilder {
    pub fn build(&self) -> ChatEventMessageUnpinned {
        self.inner.clone()
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }
}

impl AsRef<ChatEventMessageUnpinned> for ChatEventMessageUnpinned {
    fn as_ref(&self) -> &ChatEventMessageUnpinned {
        self
    }
}

impl AsRef<ChatEventMessageUnpinned> for RTDChatEventMessageUnpinnedBuilder {
    fn as_ref(&self) -> &ChatEventMessageUnpinned {
        &self.inner
    }
}

/// The chat permissions was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventPermissionsChanged {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Previous chat permissions
    old_permissions: ChatPermissions,
    /// New chat permissions
    new_permissions: ChatPermissions,
}

impl RObject for ChatEventPermissionsChanged {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventPermissionsChanged {}

impl ChatEventPermissionsChanged {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventPermissionsChangedBuilder {
        let mut inner = ChatEventPermissionsChanged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventPermissionsChangedBuilder { inner }
    }

    pub fn old_permissions(&self) -> &ChatPermissions {
        &self.old_permissions
    }

    pub fn new_permissions(&self) -> &ChatPermissions {
        &self.new_permissions
    }
}

#[doc(hidden)]
pub struct RTDChatEventPermissionsChangedBuilder {
    inner: ChatEventPermissionsChanged,
}

impl RTDChatEventPermissionsChangedBuilder {
    pub fn build(&self) -> ChatEventPermissionsChanged {
        self.inner.clone()
    }

    pub fn old_permissions<T: AsRef<ChatPermissions>>(&mut self, old_permissions: T) -> &mut Self {
        self.inner.old_permissions = old_permissions.as_ref().clone();
        self
    }

    pub fn new_permissions<T: AsRef<ChatPermissions>>(&mut self, new_permissions: T) -> &mut Self {
        self.inner.new_permissions = new_permissions.as_ref().clone();
        self
    }
}

impl AsRef<ChatEventPermissionsChanged> for ChatEventPermissionsChanged {
    fn as_ref(&self) -> &ChatEventPermissionsChanged {
        self
    }
}

impl AsRef<ChatEventPermissionsChanged> for RTDChatEventPermissionsChangedBuilder {
    fn as_ref(&self) -> &ChatEventPermissionsChanged {
        &self.inner
    }
}

/// The chat photo was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventPhotoChanged {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Previous chat photo value; may be null
    old_photo: Option<ChatPhoto>,
    /// New chat photo value; may be null
    new_photo: Option<ChatPhoto>,
}

impl RObject for ChatEventPhotoChanged {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventPhotoChanged {}

impl ChatEventPhotoChanged {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventPhotoChangedBuilder {
        let mut inner = ChatEventPhotoChanged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventPhotoChangedBuilder { inner }
    }

    pub fn old_photo(&self) -> &Option<ChatPhoto> {
        &self.old_photo
    }

    pub fn new_photo(&self) -> &Option<ChatPhoto> {
        &self.new_photo
    }
}

#[doc(hidden)]
pub struct RTDChatEventPhotoChangedBuilder {
    inner: ChatEventPhotoChanged,
}

impl RTDChatEventPhotoChangedBuilder {
    pub fn build(&self) -> ChatEventPhotoChanged {
        self.inner.clone()
    }

    pub fn old_photo<T: AsRef<ChatPhoto>>(&mut self, old_photo: T) -> &mut Self {
        self.inner.old_photo = Some(old_photo.as_ref().clone());
        self
    }

    pub fn new_photo<T: AsRef<ChatPhoto>>(&mut self, new_photo: T) -> &mut Self {
        self.inner.new_photo = Some(new_photo.as_ref().clone());
        self
    }
}

impl AsRef<ChatEventPhotoChanged> for ChatEventPhotoChanged {
    fn as_ref(&self) -> &ChatEventPhotoChanged {
        self
    }
}

impl AsRef<ChatEventPhotoChanged> for RTDChatEventPhotoChangedBuilder {
    fn as_ref(&self) -> &ChatEventPhotoChanged {
        &self.inner
    }
}

/// A poll in a message was stopped
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventPollStopped {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The message with the poll
    message: Message,
}

impl RObject for ChatEventPollStopped {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventPollStopped {}

impl ChatEventPollStopped {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventPollStoppedBuilder {
        let mut inner = ChatEventPollStopped::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventPollStoppedBuilder { inner }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}

#[doc(hidden)]
pub struct RTDChatEventPollStoppedBuilder {
    inner: ChatEventPollStopped,
}

impl RTDChatEventPollStoppedBuilder {
    pub fn build(&self) -> ChatEventPollStopped {
        self.inner.clone()
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }
}

impl AsRef<ChatEventPollStopped> for ChatEventPollStopped {
    fn as_ref(&self) -> &ChatEventPollStopped {
        self
    }
}

impl AsRef<ChatEventPollStopped> for RTDChatEventPollStoppedBuilder {
    fn as_ref(&self) -> &ChatEventPollStopped {
        &self.inner
    }
}

/// The sign_messages setting of a channel was toggled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventSignMessagesToggled {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New value of sign_messages
    sign_messages: bool,
}

impl RObject for ChatEventSignMessagesToggled {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventSignMessagesToggled {}

impl ChatEventSignMessagesToggled {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventSignMessagesToggledBuilder {
        let mut inner = ChatEventSignMessagesToggled::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventSignMessagesToggledBuilder { inner }
    }

    pub fn sign_messages(&self) -> bool {
        self.sign_messages
    }
}

#[doc(hidden)]
pub struct RTDChatEventSignMessagesToggledBuilder {
    inner: ChatEventSignMessagesToggled,
}

impl RTDChatEventSignMessagesToggledBuilder {
    pub fn build(&self) -> ChatEventSignMessagesToggled {
        self.inner.clone()
    }

    pub fn sign_messages(&mut self, sign_messages: bool) -> &mut Self {
        self.inner.sign_messages = sign_messages;
        self
    }
}

impl AsRef<ChatEventSignMessagesToggled> for ChatEventSignMessagesToggled {
    fn as_ref(&self) -> &ChatEventSignMessagesToggled {
        self
    }
}

impl AsRef<ChatEventSignMessagesToggled> for RTDChatEventSignMessagesToggledBuilder {
    fn as_ref(&self) -> &ChatEventSignMessagesToggled {
        &self.inner
    }
}

/// The slow_mode_delay setting of a supergroup was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventSlowModeDelayChanged {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Previous value of slow_mode_delay
    old_slow_mode_delay: i32,
    /// New value of slow_mode_delay
    new_slow_mode_delay: i32,
}

impl RObject for ChatEventSlowModeDelayChanged {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventSlowModeDelayChanged {}

impl ChatEventSlowModeDelayChanged {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventSlowModeDelayChangedBuilder {
        let mut inner = ChatEventSlowModeDelayChanged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventSlowModeDelayChangedBuilder { inner }
    }

    pub fn old_slow_mode_delay(&self) -> i32 {
        self.old_slow_mode_delay
    }

    pub fn new_slow_mode_delay(&self) -> i32 {
        self.new_slow_mode_delay
    }
}

#[doc(hidden)]
pub struct RTDChatEventSlowModeDelayChangedBuilder {
    inner: ChatEventSlowModeDelayChanged,
}

impl RTDChatEventSlowModeDelayChangedBuilder {
    pub fn build(&self) -> ChatEventSlowModeDelayChanged {
        self.inner.clone()
    }

    pub fn old_slow_mode_delay(&mut self, old_slow_mode_delay: i32) -> &mut Self {
        self.inner.old_slow_mode_delay = old_slow_mode_delay;
        self
    }

    pub fn new_slow_mode_delay(&mut self, new_slow_mode_delay: i32) -> &mut Self {
        self.inner.new_slow_mode_delay = new_slow_mode_delay;
        self
    }
}

impl AsRef<ChatEventSlowModeDelayChanged> for ChatEventSlowModeDelayChanged {
    fn as_ref(&self) -> &ChatEventSlowModeDelayChanged {
        self
    }
}

impl AsRef<ChatEventSlowModeDelayChanged> for RTDChatEventSlowModeDelayChangedBuilder {
    fn as_ref(&self) -> &ChatEventSlowModeDelayChanged {
        &self.inner
    }
}

/// The supergroup sticker set was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventStickerSetChanged {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Previous identifier of the chat sticker set; 0 if none

    #[serde(deserialize_with = "super::_common::number_from_string")]
    old_sticker_set_id: i64,
    /// New identifier of the chat sticker set; 0 if none

    #[serde(deserialize_with = "super::_common::number_from_string")]
    new_sticker_set_id: i64,
}

impl RObject for ChatEventStickerSetChanged {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventStickerSetChanged {}

impl ChatEventStickerSetChanged {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventStickerSetChangedBuilder {
        let mut inner = ChatEventStickerSetChanged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventStickerSetChangedBuilder { inner }
    }

    pub fn old_sticker_set_id(&self) -> i64 {
        self.old_sticker_set_id
    }

    pub fn new_sticker_set_id(&self) -> i64 {
        self.new_sticker_set_id
    }
}

#[doc(hidden)]
pub struct RTDChatEventStickerSetChangedBuilder {
    inner: ChatEventStickerSetChanged,
}

impl RTDChatEventStickerSetChangedBuilder {
    pub fn build(&self) -> ChatEventStickerSetChanged {
        self.inner.clone()
    }

    pub fn old_sticker_set_id(&mut self, old_sticker_set_id: i64) -> &mut Self {
        self.inner.old_sticker_set_id = old_sticker_set_id;
        self
    }

    pub fn new_sticker_set_id(&mut self, new_sticker_set_id: i64) -> &mut Self {
        self.inner.new_sticker_set_id = new_sticker_set_id;
        self
    }
}

impl AsRef<ChatEventStickerSetChanged> for ChatEventStickerSetChanged {
    fn as_ref(&self) -> &ChatEventStickerSetChanged {
        self
    }
}

impl AsRef<ChatEventStickerSetChanged> for RTDChatEventStickerSetChangedBuilder {
    fn as_ref(&self) -> &ChatEventStickerSetChanged {
        &self.inner
    }
}

/// The chat title was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventTitleChanged {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Previous chat title
    old_title: String,
    /// New chat title
    new_title: String,
}

impl RObject for ChatEventTitleChanged {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventTitleChanged {}

impl ChatEventTitleChanged {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventTitleChangedBuilder {
        let mut inner = ChatEventTitleChanged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventTitleChangedBuilder { inner }
    }

    pub fn old_title(&self) -> &String {
        &self.old_title
    }

    pub fn new_title(&self) -> &String {
        &self.new_title
    }
}

#[doc(hidden)]
pub struct RTDChatEventTitleChangedBuilder {
    inner: ChatEventTitleChanged,
}

impl RTDChatEventTitleChangedBuilder {
    pub fn build(&self) -> ChatEventTitleChanged {
        self.inner.clone()
    }

    pub fn old_title<T: AsRef<str>>(&mut self, old_title: T) -> &mut Self {
        self.inner.old_title = old_title.as_ref().to_string();
        self
    }

    pub fn new_title<T: AsRef<str>>(&mut self, new_title: T) -> &mut Self {
        self.inner.new_title = new_title.as_ref().to_string();
        self
    }
}

impl AsRef<ChatEventTitleChanged> for ChatEventTitleChanged {
    fn as_ref(&self) -> &ChatEventTitleChanged {
        self
    }
}

impl AsRef<ChatEventTitleChanged> for RTDChatEventTitleChangedBuilder {
    fn as_ref(&self) -> &ChatEventTitleChanged {
        &self.inner
    }
}

/// The chat username was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEventUsernameChanged {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Previous chat username
    old_username: String,
    /// New chat username
    new_username: String,
}

impl RObject for ChatEventUsernameChanged {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatEventAction for ChatEventUsernameChanged {}

impl ChatEventUsernameChanged {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatEventUsernameChangedBuilder {
        let mut inner = ChatEventUsernameChanged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatEventUsernameChangedBuilder { inner }
    }

    pub fn old_username(&self) -> &String {
        &self.old_username
    }

    pub fn new_username(&self) -> &String {
        &self.new_username
    }
}

#[doc(hidden)]
pub struct RTDChatEventUsernameChangedBuilder {
    inner: ChatEventUsernameChanged,
}

impl RTDChatEventUsernameChangedBuilder {
    pub fn build(&self) -> ChatEventUsernameChanged {
        self.inner.clone()
    }

    pub fn old_username<T: AsRef<str>>(&mut self, old_username: T) -> &mut Self {
        self.inner.old_username = old_username.as_ref().to_string();
        self
    }

    pub fn new_username<T: AsRef<str>>(&mut self, new_username: T) -> &mut Self {
        self.inner.new_username = new_username.as_ref().to_string();
        self
    }
}

impl AsRef<ChatEventUsernameChanged> for ChatEventUsernameChanged {
    fn as_ref(&self) -> &ChatEventUsernameChanged {
        self
    }
}

impl AsRef<ChatEventUsernameChanged> for RTDChatEventUsernameChangedBuilder {
    fn as_ref(&self) -> &ChatEventUsernameChanged {
        &self.inner
    }
}
