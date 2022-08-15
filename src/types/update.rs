use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains notifications about data changes
pub trait TDUpdate: Debug + RObject {}

/// Contains notifications about data changes
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Update {
    #[doc(hidden)]
    _Default,
    /// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
    #[serde(rename = "testUseUpdate")]
    TestUseUpdate(TestUseUpdate),
    /// Contains active notifications that was shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
    #[serde(rename = "updateActiveNotifications")]
    ActiveNotifications(UpdateActiveNotifications),
    /// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
    #[serde(rename = "updateAnimatedEmojiMessageClicked")]
    AnimatedEmojiMessageClicked(UpdateAnimatedEmojiMessageClicked),
    /// The parameters of animation search through GetOption("animation_search_bot_username") bot has changed
    #[serde(rename = "updateAnimationSearchParameters")]
    AnimationSearchParameters(UpdateAnimationSearchParameters),
    /// The user authorization state has changed
    #[serde(rename = "updateAuthorizationState")]
    AuthorizationState(UpdateAuthorizationState),
    /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
    #[serde(rename = "updateBasicGroup")]
    BasicGroup(UpdateBasicGroup),
    /// Some data in basicGroupFullInfo has been changed
    #[serde(rename = "updateBasicGroupFullInfo")]
    BasicGroupFullInfo(UpdateBasicGroupFullInfo),
    /// New call was created or information about a call was updated
    #[serde(rename = "updateCall")]
    Call(UpdateCall),
    /// A message sender activity in the chat has changed
    #[serde(rename = "updateChatAction")]
    ChatAction(UpdateChatAction),
    /// The chat action bar was changed
    #[serde(rename = "updateChatActionBar")]
    ChatActionBar(UpdateChatActionBar),
    /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
    #[serde(rename = "updateChatDefaultDisableNotification")]
    ChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
    /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
    #[serde(rename = "updateChatDraftMessage")]
    ChatDraftMessage(UpdateChatDraftMessage),
    /// The list of chat filters or a chat filter has changed
    #[serde(rename = "updateChatFilters")]
    ChatFilters(UpdateChatFilters),
    /// A chat content was allowed or restricted for saving
    #[serde(rename = "updateChatHasProtectedContent")]
    ChatHasProtectedContent(UpdateChatHasProtectedContent),
    /// A chat's has_scheduled_messages field has changed
    #[serde(rename = "updateChatHasScheduledMessages")]
    ChatHasScheduledMessages(UpdateChatHasScheduledMessages),
    /// A chat was blocked or unblocked
    #[serde(rename = "updateChatIsBlocked")]
    ChatIsBlocked(UpdateChatIsBlocked),
    /// A chat was marked as unread or was read
    #[serde(rename = "updateChatIsMarkedAsUnread")]
    ChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
    /// The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
    #[serde(rename = "updateChatLastMessage")]
    ChatLastMessage(UpdateChatLastMessage),
    /// User rights changed in a chat; for bots only
    #[serde(rename = "updateChatMember")]
    ChatMember(UpdateChatMember),
    /// The message sender that is selected to send messages in a chat has changed
    #[serde(rename = "updateChatMessageSender")]
    ChatMessageSender(UpdateChatMessageSender),
    /// The message Time To Live setting for a chat was changed
    #[serde(rename = "updateChatMessageTtl")]
    ChatMessageTtl(UpdateChatMessageTtl),
    /// Notification settings for a chat were changed
    #[serde(rename = "updateChatNotificationSettings")]
    ChatNotificationSettings(UpdateChatNotificationSettings),
    /// The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed
    #[serde(rename = "updateChatOnlineMemberCount")]
    ChatOnlineMemberCount(UpdateChatOnlineMemberCount),
    /// The chat pending join requests were changed
    #[serde(rename = "updateChatPendingJoinRequests")]
    ChatPendingJoinRequests(UpdateChatPendingJoinRequests),
    /// Chat permissions was changed
    #[serde(rename = "updateChatPermissions")]
    ChatPermissions(UpdateChatPermissions),
    /// A chat photo was changed
    #[serde(rename = "updateChatPhoto")]
    ChatPhoto(UpdateChatPhoto),
    /// The position of a chat in a chat list has changed. Instead of this update updateChatLastMessage or updateChatDraftMessage might be sent
    #[serde(rename = "updateChatPosition")]
    ChatPosition(UpdateChatPosition),
    /// Incoming messages were read or the number of unread messages has been changed
    #[serde(rename = "updateChatReadInbox")]
    ChatReadInbox(UpdateChatReadInbox),
    /// Outgoing messages were read
    #[serde(rename = "updateChatReadOutbox")]
    ChatReadOutbox(UpdateChatReadOutbox),
    /// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
    #[serde(rename = "updateChatReplyMarkup")]
    ChatReplyMarkup(UpdateChatReplyMarkup),
    /// The chat theme was changed
    #[serde(rename = "updateChatTheme")]
    ChatTheme(UpdateChatTheme),
    /// The list of available chat themes has changed
    #[serde(rename = "updateChatThemes")]
    ChatThemes(UpdateChatThemes),
    /// The title of a chat was changed
    #[serde(rename = "updateChatTitle")]
    ChatTitle(UpdateChatTitle),
    /// The chat unread_mention_count has changed
    #[serde(rename = "updateChatUnreadMentionCount")]
    ChatUnreadMentionCount(UpdateChatUnreadMentionCount),
    /// A chat video chat state has changed
    #[serde(rename = "updateChatVideoChat")]
    ChatVideoChat(UpdateChatVideoChat),
    /// The connection state has changed. This update must be used only to show a human-readable description of the connection state
    #[serde(rename = "updateConnectionState")]
    ConnectionState(UpdateConnectionState),
    /// Some messages were deleted
    #[serde(rename = "updateDeleteMessages")]
    DeleteMessages(UpdateDeleteMessages),
    /// The list of supported dice emojis has changed
    #[serde(rename = "updateDiceEmojis")]
    DiceEmojis(UpdateDiceEmojis),
    /// The list of favorite stickers was updated
    #[serde(rename = "updateFavoriteStickers")]
    FavoriteStickers(UpdateFavoriteStickers),
    /// Information about a file was updated
    #[serde(rename = "updateFile")]
    File(UpdateFile),
    /// The file generation process needs to be started by the application
    #[serde(rename = "updateFileGenerationStart")]
    FileGenerationStart(UpdateFileGenerationStart),
    /// File generation is no longer needed
    #[serde(rename = "updateFileGenerationStop")]
    FileGenerationStop(UpdateFileGenerationStop),
    /// Information about a group call was updated
    #[serde(rename = "updateGroupCall")]
    GroupCall(UpdateGroupCall),
    /// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
    #[serde(rename = "updateGroupCallParticipant")]
    GroupCallParticipant(UpdateGroupCallParticipant),
    /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
    #[serde(rename = "updateHavePendingNotifications")]
    HavePendingNotifications(UpdateHavePendingNotifications),
    /// The list of installed sticker sets was updated
    #[serde(rename = "updateInstalledStickerSets")]
    InstalledStickerSets(UpdateInstalledStickerSets),
    /// Some language pack strings have been updated
    #[serde(rename = "updateLanguagePackStrings")]
    LanguagePackStrings(UpdateLanguagePackStrings),
    /// The message content has changed
    #[serde(rename = "updateMessageContent")]
    MessageContent(UpdateMessageContent),
    /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
    #[serde(rename = "updateMessageContentOpened")]
    MessageContentOpened(UpdateMessageContentOpened),
    /// A message was edited. Changes in the message content will come in a separate updateMessageContent
    #[serde(rename = "updateMessageEdited")]
    MessageEdited(UpdateMessageEdited),
    /// The information about interactions with a message has changed
    #[serde(rename = "updateMessageInteractionInfo")]
    MessageInteractionInfo(UpdateMessageInteractionInfo),
    /// The message pinned state was changed
    #[serde(rename = "updateMessageIsPinned")]
    MessageIsPinned(UpdateMessageIsPinned),
    /// A message with a live location was viewed. When the update is received, the application is supposed to update the live location
    #[serde(rename = "updateMessageLiveLocationViewed")]
    MessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
    /// A message with an unread mention was read
    #[serde(rename = "updateMessageMentionRead")]
    MessageMentionRead(UpdateMessageMentionRead),
    /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
    #[serde(rename = "updateMessageSendAcknowledged")]
    MessageSendAcknowledged(UpdateMessageSendAcknowledged),
    /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
    #[serde(rename = "updateMessageSendFailed")]
    MessageSendFailed(UpdateMessageSendFailed),
    /// A message has been successfully sent
    #[serde(rename = "updateMessageSendSucceeded")]
    MessageSendSucceeded(UpdateMessageSendSucceeded),
    /// New call signaling data arrived
    #[serde(rename = "updateNewCallSignalingData")]
    NewCallSignalingData(UpdateNewCallSignalingData),
    /// A new incoming callback query; for bots only
    #[serde(rename = "updateNewCallbackQuery")]
    NewCallbackQuery(UpdateNewCallbackQuery),
    /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
    #[serde(rename = "updateNewChat")]
    NewChat(Box<UpdateNewChat>),
    /// A user sent a join request to a chat; for bots only
    #[serde(rename = "updateNewChatJoinRequest")]
    NewChatJoinRequest(UpdateNewChatJoinRequest),
    /// The user has chosen a result of an inline query; for bots only
    #[serde(rename = "updateNewChosenInlineResult")]
    NewChosenInlineResult(UpdateNewChosenInlineResult),
    /// A new incoming event; for bots only
    #[serde(rename = "updateNewCustomEvent")]
    NewCustomEvent(UpdateNewCustomEvent),
    /// A new incoming query; for bots only
    #[serde(rename = "updateNewCustomQuery")]
    NewCustomQuery(UpdateNewCustomQuery),
    /// A new incoming callback query from a message sent via a bot; for bots only
    #[serde(rename = "updateNewInlineCallbackQuery")]
    NewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
    /// A new incoming inline query; for bots only
    #[serde(rename = "updateNewInlineQuery")]
    NewInlineQuery(UpdateNewInlineQuery),
    /// A new message was received; can also be an outgoing message
    #[serde(rename = "updateNewMessage")]
    NewMessage(UpdateNewMessage),
    /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
    #[serde(rename = "updateNewPreCheckoutQuery")]
    NewPreCheckoutQuery(UpdateNewPreCheckoutQuery),
    /// A new incoming shipping query; for bots only. Only for invoices with flexible price
    #[serde(rename = "updateNewShippingQuery")]
    NewShippingQuery(UpdateNewShippingQuery),
    /// A notification was changed
    #[serde(rename = "updateNotification")]
    Notification(UpdateNotification),
    /// A list of active notifications in a notification group has changed
    #[serde(rename = "updateNotificationGroup")]
    NotificationGroup(UpdateNotificationGroup),
    /// An option changed its value
    #[serde(rename = "updateOption")]
    Option(UpdateOption),
    /// A poll was updated; for bots only
    #[serde(rename = "updatePoll")]
    Poll(UpdatePoll),
    /// A user changed the answer to a poll; for bots only
    #[serde(rename = "updatePollAnswer")]
    PollAnswer(UpdatePollAnswer),
    /// The list of recently used stickers was updated
    #[serde(rename = "updateRecentStickers")]
    RecentStickers(UpdateRecentStickers),
    /// The list of saved animations was updated
    #[serde(rename = "updateSavedAnimations")]
    SavedAnimations(UpdateSavedAnimations),
    /// Notification settings for some type of chats were updated
    #[serde(rename = "updateScopeNotificationSettings")]
    ScopeNotificationSettings(UpdateScopeNotificationSettings),
    /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
    #[serde(rename = "updateSecretChat")]
    SecretChat(UpdateSecretChat),
    /// The selected background has changed
    #[serde(rename = "updateSelectedBackground")]
    SelectedBackground(UpdateSelectedBackground),
    /// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
    #[serde(rename = "updateServiceNotification")]
    ServiceNotification(UpdateServiceNotification),
    /// A sticker set has changed
    #[serde(rename = "updateStickerSet")]
    StickerSet(UpdateStickerSet),
    /// The list of suggested to the user actions has changed
    #[serde(rename = "updateSuggestedActions")]
    SuggestedActions(UpdateSuggestedActions),
    /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
    #[serde(rename = "updateSupergroup")]
    Supergroup(UpdateSupergroup),
    /// Some data in supergroupFullInfo has been changed
    #[serde(rename = "updateSupergroupFullInfo")]
    SupergroupFullInfo(UpdateSupergroupFullInfo),
    /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
    #[serde(rename = "updateTermsOfService")]
    TermsOfService(UpdateTermsOfService),
    /// The list of trending sticker sets was updated or some of them were viewed
    #[serde(rename = "updateTrendingStickerSets")]
    TrendingStickerSets(UpdateTrendingStickerSets),
    /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
    #[serde(rename = "updateUnreadChatCount")]
    UnreadChatCount(UpdateUnreadChatCount),
    /// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
    #[serde(rename = "updateUnreadMessageCount")]
    UnreadMessageCount(UpdateUnreadMessageCount),
    /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
    #[serde(rename = "updateUser")]
    User(UpdateUser),
    /// Some data in userFullInfo has been changed
    #[serde(rename = "updateUserFullInfo")]
    UserFullInfo(UpdateUserFullInfo),
    /// Some privacy setting rules have been changed
    #[serde(rename = "updateUserPrivacySettingRules")]
    UserPrivacySettingRules(UpdateUserPrivacySettingRules),
    /// The user went online or offline
    #[serde(rename = "updateUserStatus")]
    UserStatus(UpdateUserStatus),
    /// The list of users nearby has changed. The update is guaranteed to be sent only 60 seconds after a successful searchChatsNearby request
    #[serde(rename = "updateUsersNearby")]
    UsersNearby(UpdateUsersNearby),
}

impl Default for Update {
    fn default() -> Self {
        Update::_Default
    }
}

impl RObject for Update {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            Update::TestUseUpdate(t) => t.extra(),
            Update::ActiveNotifications(t) => t.extra(),
            Update::AnimatedEmojiMessageClicked(t) => t.extra(),
            Update::AnimationSearchParameters(t) => t.extra(),
            Update::AuthorizationState(t) => t.extra(),
            Update::BasicGroup(t) => t.extra(),
            Update::BasicGroupFullInfo(t) => t.extra(),
            Update::Call(t) => t.extra(),
            Update::ChatAction(t) => t.extra(),
            Update::ChatActionBar(t) => t.extra(),
            Update::ChatDefaultDisableNotification(t) => t.extra(),
            Update::ChatDraftMessage(t) => t.extra(),
            Update::ChatFilters(t) => t.extra(),
            Update::ChatHasProtectedContent(t) => t.extra(),
            Update::ChatHasScheduledMessages(t) => t.extra(),
            Update::ChatIsBlocked(t) => t.extra(),
            Update::ChatIsMarkedAsUnread(t) => t.extra(),
            Update::ChatLastMessage(t) => t.extra(),
            Update::ChatMember(t) => t.extra(),
            Update::ChatMessageSender(t) => t.extra(),
            Update::ChatMessageTtl(t) => t.extra(),
            Update::ChatNotificationSettings(t) => t.extra(),
            Update::ChatOnlineMemberCount(t) => t.extra(),
            Update::ChatPendingJoinRequests(t) => t.extra(),
            Update::ChatPermissions(t) => t.extra(),
            Update::ChatPhoto(t) => t.extra(),
            Update::ChatPosition(t) => t.extra(),
            Update::ChatReadInbox(t) => t.extra(),
            Update::ChatReadOutbox(t) => t.extra(),
            Update::ChatReplyMarkup(t) => t.extra(),
            Update::ChatTheme(t) => t.extra(),
            Update::ChatThemes(t) => t.extra(),
            Update::ChatTitle(t) => t.extra(),
            Update::ChatUnreadMentionCount(t) => t.extra(),
            Update::ChatVideoChat(t) => t.extra(),
            Update::ConnectionState(t) => t.extra(),
            Update::DeleteMessages(t) => t.extra(),
            Update::DiceEmojis(t) => t.extra(),
            Update::FavoriteStickers(t) => t.extra(),
            Update::File(t) => t.extra(),
            Update::FileGenerationStart(t) => t.extra(),
            Update::FileGenerationStop(t) => t.extra(),
            Update::GroupCall(t) => t.extra(),
            Update::GroupCallParticipant(t) => t.extra(),
            Update::HavePendingNotifications(t) => t.extra(),
            Update::InstalledStickerSets(t) => t.extra(),
            Update::LanguagePackStrings(t) => t.extra(),
            Update::MessageContent(t) => t.extra(),
            Update::MessageContentOpened(t) => t.extra(),
            Update::MessageEdited(t) => t.extra(),
            Update::MessageInteractionInfo(t) => t.extra(),
            Update::MessageIsPinned(t) => t.extra(),
            Update::MessageLiveLocationViewed(t) => t.extra(),
            Update::MessageMentionRead(t) => t.extra(),
            Update::MessageSendAcknowledged(t) => t.extra(),
            Update::MessageSendFailed(t) => t.extra(),
            Update::MessageSendSucceeded(t) => t.extra(),
            Update::NewCallSignalingData(t) => t.extra(),
            Update::NewCallbackQuery(t) => t.extra(),
            Update::NewChat(t) => t.extra(),
            Update::NewChatJoinRequest(t) => t.extra(),
            Update::NewChosenInlineResult(t) => t.extra(),
            Update::NewCustomEvent(t) => t.extra(),
            Update::NewCustomQuery(t) => t.extra(),
            Update::NewInlineCallbackQuery(t) => t.extra(),
            Update::NewInlineQuery(t) => t.extra(),
            Update::NewMessage(t) => t.extra(),
            Update::NewPreCheckoutQuery(t) => t.extra(),
            Update::NewShippingQuery(t) => t.extra(),
            Update::Notification(t) => t.extra(),
            Update::NotificationGroup(t) => t.extra(),
            Update::Option(t) => t.extra(),
            Update::Poll(t) => t.extra(),
            Update::PollAnswer(t) => t.extra(),
            Update::RecentStickers(t) => t.extra(),
            Update::SavedAnimations(t) => t.extra(),
            Update::ScopeNotificationSettings(t) => t.extra(),
            Update::SecretChat(t) => t.extra(),
            Update::SelectedBackground(t) => t.extra(),
            Update::ServiceNotification(t) => t.extra(),
            Update::StickerSet(t) => t.extra(),
            Update::SuggestedActions(t) => t.extra(),
            Update::Supergroup(t) => t.extra(),
            Update::SupergroupFullInfo(t) => t.extra(),
            Update::TermsOfService(t) => t.extra(),
            Update::TrendingStickerSets(t) => t.extra(),
            Update::UnreadChatCount(t) => t.extra(),
            Update::UnreadMessageCount(t) => t.extra(),
            Update::User(t) => t.extra(),
            Update::UserFullInfo(t) => t.extra(),
            Update::UserPrivacySettingRules(t) => t.extra(),
            Update::UserStatus(t) => t.extra(),
            Update::UsersNearby(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            Update::TestUseUpdate(t) => t.client_id(),
            Update::ActiveNotifications(t) => t.client_id(),
            Update::AnimatedEmojiMessageClicked(t) => t.client_id(),
            Update::AnimationSearchParameters(t) => t.client_id(),
            Update::AuthorizationState(t) => t.client_id(),
            Update::BasicGroup(t) => t.client_id(),
            Update::BasicGroupFullInfo(t) => t.client_id(),
            Update::Call(t) => t.client_id(),
            Update::ChatAction(t) => t.client_id(),
            Update::ChatActionBar(t) => t.client_id(),
            Update::ChatDefaultDisableNotification(t) => t.client_id(),
            Update::ChatDraftMessage(t) => t.client_id(),
            Update::ChatFilters(t) => t.client_id(),
            Update::ChatHasProtectedContent(t) => t.client_id(),
            Update::ChatHasScheduledMessages(t) => t.client_id(),
            Update::ChatIsBlocked(t) => t.client_id(),
            Update::ChatIsMarkedAsUnread(t) => t.client_id(),
            Update::ChatLastMessage(t) => t.client_id(),
            Update::ChatMember(t) => t.client_id(),
            Update::ChatMessageSender(t) => t.client_id(),
            Update::ChatMessageTtl(t) => t.client_id(),
            Update::ChatNotificationSettings(t) => t.client_id(),
            Update::ChatOnlineMemberCount(t) => t.client_id(),
            Update::ChatPendingJoinRequests(t) => t.client_id(),
            Update::ChatPermissions(t) => t.client_id(),
            Update::ChatPhoto(t) => t.client_id(),
            Update::ChatPosition(t) => t.client_id(),
            Update::ChatReadInbox(t) => t.client_id(),
            Update::ChatReadOutbox(t) => t.client_id(),
            Update::ChatReplyMarkup(t) => t.client_id(),
            Update::ChatTheme(t) => t.client_id(),
            Update::ChatThemes(t) => t.client_id(),
            Update::ChatTitle(t) => t.client_id(),
            Update::ChatUnreadMentionCount(t) => t.client_id(),
            Update::ChatVideoChat(t) => t.client_id(),
            Update::ConnectionState(t) => t.client_id(),
            Update::DeleteMessages(t) => t.client_id(),
            Update::DiceEmojis(t) => t.client_id(),
            Update::FavoriteStickers(t) => t.client_id(),
            Update::File(t) => t.client_id(),
            Update::FileGenerationStart(t) => t.client_id(),
            Update::FileGenerationStop(t) => t.client_id(),
            Update::GroupCall(t) => t.client_id(),
            Update::GroupCallParticipant(t) => t.client_id(),
            Update::HavePendingNotifications(t) => t.client_id(),
            Update::InstalledStickerSets(t) => t.client_id(),
            Update::LanguagePackStrings(t) => t.client_id(),
            Update::MessageContent(t) => t.client_id(),
            Update::MessageContentOpened(t) => t.client_id(),
            Update::MessageEdited(t) => t.client_id(),
            Update::MessageInteractionInfo(t) => t.client_id(),
            Update::MessageIsPinned(t) => t.client_id(),
            Update::MessageLiveLocationViewed(t) => t.client_id(),
            Update::MessageMentionRead(t) => t.client_id(),
            Update::MessageSendAcknowledged(t) => t.client_id(),
            Update::MessageSendFailed(t) => t.client_id(),
            Update::MessageSendSucceeded(t) => t.client_id(),
            Update::NewCallSignalingData(t) => t.client_id(),
            Update::NewCallbackQuery(t) => t.client_id(),
            Update::NewChat(t) => t.client_id(),
            Update::NewChatJoinRequest(t) => t.client_id(),
            Update::NewChosenInlineResult(t) => t.client_id(),
            Update::NewCustomEvent(t) => t.client_id(),
            Update::NewCustomQuery(t) => t.client_id(),
            Update::NewInlineCallbackQuery(t) => t.client_id(),
            Update::NewInlineQuery(t) => t.client_id(),
            Update::NewMessage(t) => t.client_id(),
            Update::NewPreCheckoutQuery(t) => t.client_id(),
            Update::NewShippingQuery(t) => t.client_id(),
            Update::Notification(t) => t.client_id(),
            Update::NotificationGroup(t) => t.client_id(),
            Update::Option(t) => t.client_id(),
            Update::Poll(t) => t.client_id(),
            Update::PollAnswer(t) => t.client_id(),
            Update::RecentStickers(t) => t.client_id(),
            Update::SavedAnimations(t) => t.client_id(),
            Update::ScopeNotificationSettings(t) => t.client_id(),
            Update::SecretChat(t) => t.client_id(),
            Update::SelectedBackground(t) => t.client_id(),
            Update::ServiceNotification(t) => t.client_id(),
            Update::StickerSet(t) => t.client_id(),
            Update::SuggestedActions(t) => t.client_id(),
            Update::Supergroup(t) => t.client_id(),
            Update::SupergroupFullInfo(t) => t.client_id(),
            Update::TermsOfService(t) => t.client_id(),
            Update::TrendingStickerSets(t) => t.client_id(),
            Update::UnreadChatCount(t) => t.client_id(),
            Update::UnreadMessageCount(t) => t.client_id(),
            Update::User(t) => t.client_id(),
            Update::UserFullInfo(t) => t.client_id(),
            Update::UserPrivacySettingRules(t) => t.client_id(),
            Update::UserStatus(t) => t.client_id(),
            Update::UsersNearby(t) => t.client_id(),

            _ => None,
        }
    }
}

impl Update {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, Update::_Default)
    }
}

impl AsRef<Update> for Update {
    fn as_ref(&self) -> &Update {
        self
    }
}

/// Contains active notifications that was shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateActiveNotifications {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Lists of active notification groups

    #[serde(default)]
    groups: Vec<NotificationGroup>,
}

impl RObject for UpdateActiveNotifications {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateActiveNotifications {}

impl UpdateActiveNotifications {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateActiveNotificationsBuilder {
        let mut inner = UpdateActiveNotifications::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateActiveNotificationsBuilder { inner }
    }

    pub fn groups(&self) -> &Vec<NotificationGroup> {
        &self.groups
    }
}

#[doc(hidden)]
pub struct UpdateActiveNotificationsBuilder {
    inner: UpdateActiveNotifications,
}

#[deprecated]
pub type RTDUpdateActiveNotificationsBuilder = UpdateActiveNotificationsBuilder;

impl UpdateActiveNotificationsBuilder {
    pub fn build(&self) -> UpdateActiveNotifications {
        self.inner.clone()
    }

    pub fn groups(&mut self, groups: Vec<NotificationGroup>) -> &mut Self {
        self.inner.groups = groups;
        self
    }
}

impl AsRef<UpdateActiveNotifications> for UpdateActiveNotifications {
    fn as_ref(&self) -> &UpdateActiveNotifications {
        self
    }
}

impl AsRef<UpdateActiveNotifications> for UpdateActiveNotificationsBuilder {
    fn as_ref(&self) -> &UpdateActiveNotifications {
        &self.inner
    }
}

/// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAnimatedEmojiMessageClicked {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
    /// The animated sticker to be played
    sticker: Sticker,
}

impl RObject for UpdateAnimatedEmojiMessageClicked {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateAnimatedEmojiMessageClicked {}

impl UpdateAnimatedEmojiMessageClicked {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateAnimatedEmojiMessageClickedBuilder {
        let mut inner = UpdateAnimatedEmojiMessageClicked::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateAnimatedEmojiMessageClickedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn sticker(&self) -> &Sticker {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct UpdateAnimatedEmojiMessageClickedBuilder {
    inner: UpdateAnimatedEmojiMessageClicked,
}

#[deprecated]
pub type RTDUpdateAnimatedEmojiMessageClickedBuilder = UpdateAnimatedEmojiMessageClickedBuilder;

impl UpdateAnimatedEmojiMessageClickedBuilder {
    pub fn build(&self) -> UpdateAnimatedEmojiMessageClicked {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<UpdateAnimatedEmojiMessageClicked> for UpdateAnimatedEmojiMessageClicked {
    fn as_ref(&self) -> &UpdateAnimatedEmojiMessageClicked {
        self
    }
}

impl AsRef<UpdateAnimatedEmojiMessageClicked> for UpdateAnimatedEmojiMessageClickedBuilder {
    fn as_ref(&self) -> &UpdateAnimatedEmojiMessageClicked {
        &self.inner
    }
}

/// The parameters of animation search through GetOption("animation_search_bot_username") bot has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAnimationSearchParameters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the animation search provider

    #[serde(default)]
    provider: String,
    /// The new list of emojis suggested for searching

    #[serde(default)]
    emojis: Vec<String>,
}

impl RObject for UpdateAnimationSearchParameters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateAnimationSearchParameters {}

impl UpdateAnimationSearchParameters {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateAnimationSearchParametersBuilder {
        let mut inner = UpdateAnimationSearchParameters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateAnimationSearchParametersBuilder { inner }
    }

    pub fn provider(&self) -> &String {
        &self.provider
    }

    pub fn emojis(&self) -> &Vec<String> {
        &self.emojis
    }
}

#[doc(hidden)]
pub struct UpdateAnimationSearchParametersBuilder {
    inner: UpdateAnimationSearchParameters,
}

#[deprecated]
pub type RTDUpdateAnimationSearchParametersBuilder = UpdateAnimationSearchParametersBuilder;

impl UpdateAnimationSearchParametersBuilder {
    pub fn build(&self) -> UpdateAnimationSearchParameters {
        self.inner.clone()
    }

    pub fn provider<T: AsRef<str>>(&mut self, provider: T) -> &mut Self {
        self.inner.provider = provider.as_ref().to_string();
        self
    }

    pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
        self.inner.emojis = emojis;
        self
    }
}

impl AsRef<UpdateAnimationSearchParameters> for UpdateAnimationSearchParameters {
    fn as_ref(&self) -> &UpdateAnimationSearchParameters {
        self
    }
}

impl AsRef<UpdateAnimationSearchParameters> for UpdateAnimationSearchParametersBuilder {
    fn as_ref(&self) -> &UpdateAnimationSearchParameters {
        &self.inner
    }
}

/// The user authorization state has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAuthorizationState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New authorization state

    #[serde(skip_serializing_if = "AuthorizationState::_is_default")]
    authorization_state: AuthorizationState,
}

impl RObject for UpdateAuthorizationState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateAuthorizationState {}

impl UpdateAuthorizationState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateAuthorizationStateBuilder {
        let mut inner = UpdateAuthorizationState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateAuthorizationStateBuilder { inner }
    }

    pub fn authorization_state(&self) -> &AuthorizationState {
        &self.authorization_state
    }
}

#[doc(hidden)]
pub struct UpdateAuthorizationStateBuilder {
    inner: UpdateAuthorizationState,
}

#[deprecated]
pub type RTDUpdateAuthorizationStateBuilder = UpdateAuthorizationStateBuilder;

impl UpdateAuthorizationStateBuilder {
    pub fn build(&self) -> UpdateAuthorizationState {
        self.inner.clone()
    }

    pub fn authorization_state<T: AsRef<AuthorizationState>>(
        &mut self,
        authorization_state: T,
    ) -> &mut Self {
        self.inner.authorization_state = authorization_state.as_ref().clone();
        self
    }
}

impl AsRef<UpdateAuthorizationState> for UpdateAuthorizationState {
    fn as_ref(&self) -> &UpdateAuthorizationState {
        self
    }
}

impl AsRef<UpdateAuthorizationState> for UpdateAuthorizationStateBuilder {
    fn as_ref(&self) -> &UpdateAuthorizationState {
        &self.inner
    }
}

/// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBasicGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New data about the group
    basic_group: BasicGroup,
}

impl RObject for UpdateBasicGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateBasicGroup {}

impl UpdateBasicGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateBasicGroupBuilder {
        let mut inner = UpdateBasicGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateBasicGroupBuilder { inner }
    }

    pub fn basic_group(&self) -> &BasicGroup {
        &self.basic_group
    }
}

#[doc(hidden)]
pub struct UpdateBasicGroupBuilder {
    inner: UpdateBasicGroup,
}

#[deprecated]
pub type RTDUpdateBasicGroupBuilder = UpdateBasicGroupBuilder;

impl UpdateBasicGroupBuilder {
    pub fn build(&self) -> UpdateBasicGroup {
        self.inner.clone()
    }

    pub fn basic_group<T: AsRef<BasicGroup>>(&mut self, basic_group: T) -> &mut Self {
        self.inner.basic_group = basic_group.as_ref().clone();
        self
    }
}

impl AsRef<UpdateBasicGroup> for UpdateBasicGroup {
    fn as_ref(&self) -> &UpdateBasicGroup {
        self
    }
}

impl AsRef<UpdateBasicGroup> for UpdateBasicGroupBuilder {
    fn as_ref(&self) -> &UpdateBasicGroup {
        &self.inner
    }
}

/// Some data in basicGroupFullInfo has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBasicGroupFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a basic group

    #[serde(default)]
    basic_group_id: i64,
    /// New full information about the group
    basic_group_full_info: BasicGroupFullInfo,
}

impl RObject for UpdateBasicGroupFullInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateBasicGroupFullInfo {}

impl UpdateBasicGroupFullInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateBasicGroupFullInfoBuilder {
        let mut inner = UpdateBasicGroupFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateBasicGroupFullInfoBuilder { inner }
    }

    pub fn basic_group_id(&self) -> i64 {
        self.basic_group_id
    }

    pub fn basic_group_full_info(&self) -> &BasicGroupFullInfo {
        &self.basic_group_full_info
    }
}

#[doc(hidden)]
pub struct UpdateBasicGroupFullInfoBuilder {
    inner: UpdateBasicGroupFullInfo,
}

#[deprecated]
pub type RTDUpdateBasicGroupFullInfoBuilder = UpdateBasicGroupFullInfoBuilder;

impl UpdateBasicGroupFullInfoBuilder {
    pub fn build(&self) -> UpdateBasicGroupFullInfo {
        self.inner.clone()
    }

    pub fn basic_group_id(&mut self, basic_group_id: i64) -> &mut Self {
        self.inner.basic_group_id = basic_group_id;
        self
    }

    pub fn basic_group_full_info<T: AsRef<BasicGroupFullInfo>>(
        &mut self,
        basic_group_full_info: T,
    ) -> &mut Self {
        self.inner.basic_group_full_info = basic_group_full_info.as_ref().clone();
        self
    }
}

impl AsRef<UpdateBasicGroupFullInfo> for UpdateBasicGroupFullInfo {
    fn as_ref(&self) -> &UpdateBasicGroupFullInfo {
        self
    }
}

impl AsRef<UpdateBasicGroupFullInfo> for UpdateBasicGroupFullInfoBuilder {
    fn as_ref(&self) -> &UpdateBasicGroupFullInfo {
        &self.inner
    }
}

/// New call was created or information about a call was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New data about a call
    call: Call,
}

impl RObject for UpdateCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateCall {}

impl UpdateCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateCallBuilder {
        let mut inner = UpdateCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateCallBuilder { inner }
    }

    pub fn call(&self) -> &Call {
        &self.call
    }
}

#[doc(hidden)]
pub struct UpdateCallBuilder {
    inner: UpdateCall,
}

#[deprecated]
pub type RTDUpdateCallBuilder = UpdateCallBuilder;

impl UpdateCallBuilder {
    pub fn build(&self) -> UpdateCall {
        self.inner.clone()
    }

    pub fn call<T: AsRef<Call>>(&mut self, call: T) -> &mut Self {
        self.inner.call = call.as_ref().clone();
        self
    }
}

impl AsRef<UpdateCall> for UpdateCall {
    fn as_ref(&self) -> &UpdateCall {
        self
    }
}

impl AsRef<UpdateCall> for UpdateCallBuilder {
    fn as_ref(&self) -> &UpdateCall {
        &self.inner
    }
}

/// A message sender activity in the chat has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatAction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// If not 0, a message thread identifier in which the action was performed

    #[serde(default)]
    message_thread_id: i64,
    /// Identifier of a message sender performing the action

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,
    /// The action

    #[serde(skip_serializing_if = "ChatAction::_is_default")]
    action: ChatAction,
}

impl RObject for UpdateChatAction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatAction {}

impl UpdateChatAction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatActionBuilder {
        let mut inner = UpdateChatAction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatActionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }

    pub fn action(&self) -> &ChatAction {
        &self.action
    }
}

#[doc(hidden)]
pub struct UpdateChatActionBuilder {
    inner: UpdateChatAction,
}

#[deprecated]
pub type RTDUpdateChatActionBuilder = UpdateChatActionBuilder;

impl UpdateChatActionBuilder {
    pub fn build(&self) -> UpdateChatAction {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }

    pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
        self.inner.sender_id = sender_id.as_ref().clone();
        self
    }

    pub fn action<T: AsRef<ChatAction>>(&mut self, action: T) -> &mut Self {
        self.inner.action = action.as_ref().clone();
        self
    }
}

impl AsRef<UpdateChatAction> for UpdateChatAction {
    fn as_ref(&self) -> &UpdateChatAction {
        self
    }
}

impl AsRef<UpdateChatAction> for UpdateChatActionBuilder {
    fn as_ref(&self) -> &UpdateChatAction {
        &self.inner
    }
}

/// The chat action bar was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatActionBar {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new value of the action bar; may be null
    action_bar: Option<ChatActionBar>,
}

impl RObject for UpdateChatActionBar {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatActionBar {}

impl UpdateChatActionBar {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatActionBarBuilder {
        let mut inner = UpdateChatActionBar::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatActionBarBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn action_bar(&self) -> &Option<ChatActionBar> {
        &self.action_bar
    }
}

#[doc(hidden)]
pub struct UpdateChatActionBarBuilder {
    inner: UpdateChatActionBar,
}

#[deprecated]
pub type RTDUpdateChatActionBarBuilder = UpdateChatActionBarBuilder;

impl UpdateChatActionBarBuilder {
    pub fn build(&self) -> UpdateChatActionBar {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn action_bar<T: AsRef<ChatActionBar>>(&mut self, action_bar: T) -> &mut Self {
        self.inner.action_bar = Some(action_bar.as_ref().clone());
        self
    }
}

impl AsRef<UpdateChatActionBar> for UpdateChatActionBar {
    fn as_ref(&self) -> &UpdateChatActionBar {
        self
    }
}

impl AsRef<UpdateChatActionBar> for UpdateChatActionBarBuilder {
    fn as_ref(&self) -> &UpdateChatActionBar {
        &self.inner
    }
}

/// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatDefaultDisableNotification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new default_disable_notification value

    #[serde(default)]
    default_disable_notification: bool,
}

impl RObject for UpdateChatDefaultDisableNotification {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatDefaultDisableNotification {}

impl UpdateChatDefaultDisableNotification {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatDefaultDisableNotificationBuilder {
        let mut inner = UpdateChatDefaultDisableNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatDefaultDisableNotificationBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn default_disable_notification(&self) -> bool {
        self.default_disable_notification
    }
}

#[doc(hidden)]
pub struct UpdateChatDefaultDisableNotificationBuilder {
    inner: UpdateChatDefaultDisableNotification,
}

#[deprecated]
pub type RTDUpdateChatDefaultDisableNotificationBuilder =
    UpdateChatDefaultDisableNotificationBuilder;

impl UpdateChatDefaultDisableNotificationBuilder {
    pub fn build(&self) -> UpdateChatDefaultDisableNotification {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn default_disable_notification(
        &mut self,
        default_disable_notification: bool,
    ) -> &mut Self {
        self.inner.default_disable_notification = default_disable_notification;
        self
    }
}

impl AsRef<UpdateChatDefaultDisableNotification> for UpdateChatDefaultDisableNotification {
    fn as_ref(&self) -> &UpdateChatDefaultDisableNotification {
        self
    }
}

impl AsRef<UpdateChatDefaultDisableNotification> for UpdateChatDefaultDisableNotificationBuilder {
    fn as_ref(&self) -> &UpdateChatDefaultDisableNotification {
        &self.inner
    }
}

/// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatDraftMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new draft message; may be null
    draft_message: Option<DraftMessage>,
    /// The new chat positions in the chat lists

    #[serde(default)]
    positions: Vec<ChatPosition>,
}

impl RObject for UpdateChatDraftMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatDraftMessage {}

impl UpdateChatDraftMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatDraftMessageBuilder {
        let mut inner = UpdateChatDraftMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatDraftMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn draft_message(&self) -> &Option<DraftMessage> {
        &self.draft_message
    }

    pub fn positions(&self) -> &Vec<ChatPosition> {
        &self.positions
    }
}

#[doc(hidden)]
pub struct UpdateChatDraftMessageBuilder {
    inner: UpdateChatDraftMessage,
}

#[deprecated]
pub type RTDUpdateChatDraftMessageBuilder = UpdateChatDraftMessageBuilder;

impl UpdateChatDraftMessageBuilder {
    pub fn build(&self) -> UpdateChatDraftMessage {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn draft_message<T: AsRef<DraftMessage>>(&mut self, draft_message: T) -> &mut Self {
        self.inner.draft_message = Some(draft_message.as_ref().clone());
        self
    }

    pub fn positions(&mut self, positions: Vec<ChatPosition>) -> &mut Self {
        self.inner.positions = positions;
        self
    }
}

impl AsRef<UpdateChatDraftMessage> for UpdateChatDraftMessage {
    fn as_ref(&self) -> &UpdateChatDraftMessage {
        self
    }
}

impl AsRef<UpdateChatDraftMessage> for UpdateChatDraftMessageBuilder {
    fn as_ref(&self) -> &UpdateChatDraftMessage {
        &self.inner
    }
}

/// The list of chat filters or a chat filter has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatFilters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new list of chat filters

    #[serde(default)]
    chat_filters: Vec<ChatFilterInfo>,
}

impl RObject for UpdateChatFilters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatFilters {}

impl UpdateChatFilters {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatFiltersBuilder {
        let mut inner = UpdateChatFilters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatFiltersBuilder { inner }
    }

    pub fn chat_filters(&self) -> &Vec<ChatFilterInfo> {
        &self.chat_filters
    }
}

#[doc(hidden)]
pub struct UpdateChatFiltersBuilder {
    inner: UpdateChatFilters,
}

#[deprecated]
pub type RTDUpdateChatFiltersBuilder = UpdateChatFiltersBuilder;

impl UpdateChatFiltersBuilder {
    pub fn build(&self) -> UpdateChatFilters {
        self.inner.clone()
    }

    pub fn chat_filters(&mut self, chat_filters: Vec<ChatFilterInfo>) -> &mut Self {
        self.inner.chat_filters = chat_filters;
        self
    }
}

impl AsRef<UpdateChatFilters> for UpdateChatFilters {
    fn as_ref(&self) -> &UpdateChatFilters {
        self
    }
}

impl AsRef<UpdateChatFilters> for UpdateChatFiltersBuilder {
    fn as_ref(&self) -> &UpdateChatFilters {
        &self.inner
    }
}

/// A chat content was allowed or restricted for saving
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatHasProtectedContent {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New value of has_protected_content

    #[serde(default)]
    has_protected_content: bool,
}

impl RObject for UpdateChatHasProtectedContent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatHasProtectedContent {}

impl UpdateChatHasProtectedContent {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatHasProtectedContentBuilder {
        let mut inner = UpdateChatHasProtectedContent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatHasProtectedContentBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn has_protected_content(&self) -> bool {
        self.has_protected_content
    }
}

#[doc(hidden)]
pub struct UpdateChatHasProtectedContentBuilder {
    inner: UpdateChatHasProtectedContent,
}

#[deprecated]
pub type RTDUpdateChatHasProtectedContentBuilder = UpdateChatHasProtectedContentBuilder;

impl UpdateChatHasProtectedContentBuilder {
    pub fn build(&self) -> UpdateChatHasProtectedContent {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn has_protected_content(&mut self, has_protected_content: bool) -> &mut Self {
        self.inner.has_protected_content = has_protected_content;
        self
    }
}

impl AsRef<UpdateChatHasProtectedContent> for UpdateChatHasProtectedContent {
    fn as_ref(&self) -> &UpdateChatHasProtectedContent {
        self
    }
}

impl AsRef<UpdateChatHasProtectedContent> for UpdateChatHasProtectedContentBuilder {
    fn as_ref(&self) -> &UpdateChatHasProtectedContent {
        &self.inner
    }
}

/// A chat's has_scheduled_messages field has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatHasScheduledMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New value of has_scheduled_messages

    #[serde(default)]
    has_scheduled_messages: bool,
}

impl RObject for UpdateChatHasScheduledMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatHasScheduledMessages {}

impl UpdateChatHasScheduledMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatHasScheduledMessagesBuilder {
        let mut inner = UpdateChatHasScheduledMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatHasScheduledMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn has_scheduled_messages(&self) -> bool {
        self.has_scheduled_messages
    }
}

#[doc(hidden)]
pub struct UpdateChatHasScheduledMessagesBuilder {
    inner: UpdateChatHasScheduledMessages,
}

#[deprecated]
pub type RTDUpdateChatHasScheduledMessagesBuilder = UpdateChatHasScheduledMessagesBuilder;

impl UpdateChatHasScheduledMessagesBuilder {
    pub fn build(&self) -> UpdateChatHasScheduledMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn has_scheduled_messages(&mut self, has_scheduled_messages: bool) -> &mut Self {
        self.inner.has_scheduled_messages = has_scheduled_messages;
        self
    }
}

impl AsRef<UpdateChatHasScheduledMessages> for UpdateChatHasScheduledMessages {
    fn as_ref(&self) -> &UpdateChatHasScheduledMessages {
        self
    }
}

impl AsRef<UpdateChatHasScheduledMessages> for UpdateChatHasScheduledMessagesBuilder {
    fn as_ref(&self) -> &UpdateChatHasScheduledMessages {
        &self.inner
    }
}

/// A chat was blocked or unblocked
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatIsBlocked {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New value of is_blocked

    #[serde(default)]
    is_blocked: bool,
}

impl RObject for UpdateChatIsBlocked {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatIsBlocked {}

impl UpdateChatIsBlocked {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatIsBlockedBuilder {
        let mut inner = UpdateChatIsBlocked::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatIsBlockedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn is_blocked(&self) -> bool {
        self.is_blocked
    }
}

#[doc(hidden)]
pub struct UpdateChatIsBlockedBuilder {
    inner: UpdateChatIsBlocked,
}

#[deprecated]
pub type RTDUpdateChatIsBlockedBuilder = UpdateChatIsBlockedBuilder;

impl UpdateChatIsBlockedBuilder {
    pub fn build(&self) -> UpdateChatIsBlocked {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn is_blocked(&mut self, is_blocked: bool) -> &mut Self {
        self.inner.is_blocked = is_blocked;
        self
    }
}

impl AsRef<UpdateChatIsBlocked> for UpdateChatIsBlocked {
    fn as_ref(&self) -> &UpdateChatIsBlocked {
        self
    }
}

impl AsRef<UpdateChatIsBlocked> for UpdateChatIsBlockedBuilder {
    fn as_ref(&self) -> &UpdateChatIsBlocked {
        &self.inner
    }
}

/// A chat was marked as unread or was read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatIsMarkedAsUnread {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New value of is_marked_as_unread

    #[serde(default)]
    is_marked_as_unread: bool,
}

impl RObject for UpdateChatIsMarkedAsUnread {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatIsMarkedAsUnread {}

impl UpdateChatIsMarkedAsUnread {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatIsMarkedAsUnreadBuilder {
        let mut inner = UpdateChatIsMarkedAsUnread::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatIsMarkedAsUnreadBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn is_marked_as_unread(&self) -> bool {
        self.is_marked_as_unread
    }
}

#[doc(hidden)]
pub struct UpdateChatIsMarkedAsUnreadBuilder {
    inner: UpdateChatIsMarkedAsUnread,
}

#[deprecated]
pub type RTDUpdateChatIsMarkedAsUnreadBuilder = UpdateChatIsMarkedAsUnreadBuilder;

impl UpdateChatIsMarkedAsUnreadBuilder {
    pub fn build(&self) -> UpdateChatIsMarkedAsUnread {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self {
        self.inner.is_marked_as_unread = is_marked_as_unread;
        self
    }
}

impl AsRef<UpdateChatIsMarkedAsUnread> for UpdateChatIsMarkedAsUnread {
    fn as_ref(&self) -> &UpdateChatIsMarkedAsUnread {
        self
    }
}

impl AsRef<UpdateChatIsMarkedAsUnread> for UpdateChatIsMarkedAsUnreadBuilder {
    fn as_ref(&self) -> &UpdateChatIsMarkedAsUnread {
        &self.inner
    }
}

/// The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatLastMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new last message in the chat; may be null
    last_message: Option<Message>,
    /// The new chat positions in the chat lists

    #[serde(default)]
    positions: Option<Vec<ChatPosition>>,
}

impl RObject for UpdateChatLastMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatLastMessage {}

impl UpdateChatLastMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatLastMessageBuilder {
        let mut inner = UpdateChatLastMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatLastMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn last_message(&self) -> &Option<Message> {
        &self.last_message
    }

    pub fn positions(&self) -> &Option<Vec<ChatPosition>> {
        &self.positions
    }
}

#[doc(hidden)]
pub struct UpdateChatLastMessageBuilder {
    inner: UpdateChatLastMessage,
}

#[deprecated]
pub type RTDUpdateChatLastMessageBuilder = UpdateChatLastMessageBuilder;

impl UpdateChatLastMessageBuilder {
    pub fn build(&self) -> UpdateChatLastMessage {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn last_message<T: AsRef<Message>>(&mut self, last_message: T) -> &mut Self {
        self.inner.last_message = Some(last_message.as_ref().clone());
        self
    }

    pub fn positions(&mut self, positions: Vec<ChatPosition>) -> &mut Self {
        self.inner.positions = Some(positions);
        self
    }
}

impl AsRef<UpdateChatLastMessage> for UpdateChatLastMessage {
    fn as_ref(&self) -> &UpdateChatLastMessage {
        self
    }
}

impl AsRef<UpdateChatLastMessage> for UpdateChatLastMessageBuilder {
    fn as_ref(&self) -> &UpdateChatLastMessage {
        &self.inner
    }
}

/// User rights changed in a chat; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the user, changing the rights

    #[serde(default)]
    actor_user_id: i64,
    /// Point in time (Unix timestamp) when the user rights was changed

    #[serde(default)]
    date: i32,
    /// If user has joined the chat using an invite link, the invite link; may be null
    invite_link: Option<ChatInviteLink>,
    /// Previous chat member
    old_chat_member: ChatMember,
    /// New chat member
    new_chat_member: ChatMember,
}

impl RObject for UpdateChatMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatMember {}

impl UpdateChatMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatMemberBuilder {
        let mut inner = UpdateChatMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatMemberBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn actor_user_id(&self) -> i64 {
        self.actor_user_id
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn invite_link(&self) -> &Option<ChatInviteLink> {
        &self.invite_link
    }

    pub fn old_chat_member(&self) -> &ChatMember {
        &self.old_chat_member
    }

    pub fn new_chat_member(&self) -> &ChatMember {
        &self.new_chat_member
    }
}

#[doc(hidden)]
pub struct UpdateChatMemberBuilder {
    inner: UpdateChatMember,
}

#[deprecated]
pub type RTDUpdateChatMemberBuilder = UpdateChatMemberBuilder;

impl UpdateChatMemberBuilder {
    pub fn build(&self) -> UpdateChatMember {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn actor_user_id(&mut self, actor_user_id: i64) -> &mut Self {
        self.inner.actor_user_id = actor_user_id;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = Some(invite_link.as_ref().clone());
        self
    }

    pub fn old_chat_member<T: AsRef<ChatMember>>(&mut self, old_chat_member: T) -> &mut Self {
        self.inner.old_chat_member = old_chat_member.as_ref().clone();
        self
    }

    pub fn new_chat_member<T: AsRef<ChatMember>>(&mut self, new_chat_member: T) -> &mut Self {
        self.inner.new_chat_member = new_chat_member.as_ref().clone();
        self
    }
}

impl AsRef<UpdateChatMember> for UpdateChatMember {
    fn as_ref(&self) -> &UpdateChatMember {
        self
    }
}

impl AsRef<UpdateChatMember> for UpdateChatMemberBuilder {
    fn as_ref(&self) -> &UpdateChatMember {
        &self.inner
    }
}

/// The message sender that is selected to send messages in a chat has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatMessageSender {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New value of message_sender_id; may be null if the user can't change message sender
    message_sender_id: Option<MessageSender>,
}

impl RObject for UpdateChatMessageSender {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatMessageSender {}

impl UpdateChatMessageSender {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatMessageSenderBuilder {
        let mut inner = UpdateChatMessageSender::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatMessageSenderBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_sender_id(&self) -> &Option<MessageSender> {
        &self.message_sender_id
    }
}

#[doc(hidden)]
pub struct UpdateChatMessageSenderBuilder {
    inner: UpdateChatMessageSender,
}

#[deprecated]
pub type RTDUpdateChatMessageSenderBuilder = UpdateChatMessageSenderBuilder;

impl UpdateChatMessageSenderBuilder {
    pub fn build(&self) -> UpdateChatMessageSender {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_sender_id<T: AsRef<MessageSender>>(
        &mut self,
        message_sender_id: T,
    ) -> &mut Self {
        self.inner.message_sender_id = Some(message_sender_id.as_ref().clone());
        self
    }
}

impl AsRef<UpdateChatMessageSender> for UpdateChatMessageSender {
    fn as_ref(&self) -> &UpdateChatMessageSender {
        self
    }
}

impl AsRef<UpdateChatMessageSender> for UpdateChatMessageSenderBuilder {
    fn as_ref(&self) -> &UpdateChatMessageSender {
        &self.inner
    }
}

/// The message Time To Live setting for a chat was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatMessageTtl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New value of message_ttl

    #[serde(default)]
    message_ttl: i32,
}

impl RObject for UpdateChatMessageTtl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatMessageTtl {}

impl UpdateChatMessageTtl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatMessageTtlBuilder {
        let mut inner = UpdateChatMessageTtl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatMessageTtlBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_ttl(&self) -> i32 {
        self.message_ttl
    }
}

#[doc(hidden)]
pub struct UpdateChatMessageTtlBuilder {
    inner: UpdateChatMessageTtl,
}

#[deprecated]
pub type RTDUpdateChatMessageTtlBuilder = UpdateChatMessageTtlBuilder;

impl UpdateChatMessageTtlBuilder {
    pub fn build(&self) -> UpdateChatMessageTtl {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_ttl(&mut self, message_ttl: i32) -> &mut Self {
        self.inner.message_ttl = message_ttl;
        self
    }
}

impl AsRef<UpdateChatMessageTtl> for UpdateChatMessageTtl {
    fn as_ref(&self) -> &UpdateChatMessageTtl {
        self
    }
}

impl AsRef<UpdateChatMessageTtl> for UpdateChatMessageTtlBuilder {
    fn as_ref(&self) -> &UpdateChatMessageTtl {
        &self.inner
    }
}

/// Notification settings for a chat were changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new notification settings
    notification_settings: ChatNotificationSettings,
}

impl RObject for UpdateChatNotificationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatNotificationSettings {}

impl UpdateChatNotificationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatNotificationSettingsBuilder {
        let mut inner = UpdateChatNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatNotificationSettingsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn notification_settings(&self) -> &ChatNotificationSettings {
        &self.notification_settings
    }
}

#[doc(hidden)]
pub struct UpdateChatNotificationSettingsBuilder {
    inner: UpdateChatNotificationSettings,
}

#[deprecated]
pub type RTDUpdateChatNotificationSettingsBuilder = UpdateChatNotificationSettingsBuilder;

impl UpdateChatNotificationSettingsBuilder {
    pub fn build(&self) -> UpdateChatNotificationSettings {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn notification_settings<T: AsRef<ChatNotificationSettings>>(
        &mut self,
        notification_settings: T,
    ) -> &mut Self {
        self.inner.notification_settings = notification_settings.as_ref().clone();
        self
    }
}

impl AsRef<UpdateChatNotificationSettings> for UpdateChatNotificationSettings {
    fn as_ref(&self) -> &UpdateChatNotificationSettings {
        self
    }
}

impl AsRef<UpdateChatNotificationSettings> for UpdateChatNotificationSettingsBuilder {
    fn as_ref(&self) -> &UpdateChatNotificationSettings {
        &self.inner
    }
}

/// The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatOnlineMemberCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// New number of online members in the chat, or 0 if unknown

    #[serde(default)]
    online_member_count: i32,
}

impl RObject for UpdateChatOnlineMemberCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatOnlineMemberCount {}

impl UpdateChatOnlineMemberCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatOnlineMemberCountBuilder {
        let mut inner = UpdateChatOnlineMemberCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatOnlineMemberCountBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn online_member_count(&self) -> i32 {
        self.online_member_count
    }
}

#[doc(hidden)]
pub struct UpdateChatOnlineMemberCountBuilder {
    inner: UpdateChatOnlineMemberCount,
}

#[deprecated]
pub type RTDUpdateChatOnlineMemberCountBuilder = UpdateChatOnlineMemberCountBuilder;

impl UpdateChatOnlineMemberCountBuilder {
    pub fn build(&self) -> UpdateChatOnlineMemberCount {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn online_member_count(&mut self, online_member_count: i32) -> &mut Self {
        self.inner.online_member_count = online_member_count;
        self
    }
}

impl AsRef<UpdateChatOnlineMemberCount> for UpdateChatOnlineMemberCount {
    fn as_ref(&self) -> &UpdateChatOnlineMemberCount {
        self
    }
}

impl AsRef<UpdateChatOnlineMemberCount> for UpdateChatOnlineMemberCountBuilder {
    fn as_ref(&self) -> &UpdateChatOnlineMemberCount {
        &self.inner
    }
}

/// The chat pending join requests were changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatPendingJoinRequests {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new data about pending join requests; may be null
    pending_join_requests: Option<ChatJoinRequestsInfo>,
}

impl RObject for UpdateChatPendingJoinRequests {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatPendingJoinRequests {}

impl UpdateChatPendingJoinRequests {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatPendingJoinRequestsBuilder {
        let mut inner = UpdateChatPendingJoinRequests::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatPendingJoinRequestsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn pending_join_requests(&self) -> &Option<ChatJoinRequestsInfo> {
        &self.pending_join_requests
    }
}

#[doc(hidden)]
pub struct UpdateChatPendingJoinRequestsBuilder {
    inner: UpdateChatPendingJoinRequests,
}

#[deprecated]
pub type RTDUpdateChatPendingJoinRequestsBuilder = UpdateChatPendingJoinRequestsBuilder;

impl UpdateChatPendingJoinRequestsBuilder {
    pub fn build(&self) -> UpdateChatPendingJoinRequests {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn pending_join_requests<T: AsRef<ChatJoinRequestsInfo>>(
        &mut self,
        pending_join_requests: T,
    ) -> &mut Self {
        self.inner.pending_join_requests = Some(pending_join_requests.as_ref().clone());
        self
    }
}

impl AsRef<UpdateChatPendingJoinRequests> for UpdateChatPendingJoinRequests {
    fn as_ref(&self) -> &UpdateChatPendingJoinRequests {
        self
    }
}

impl AsRef<UpdateChatPendingJoinRequests> for UpdateChatPendingJoinRequestsBuilder {
    fn as_ref(&self) -> &UpdateChatPendingJoinRequests {
        &self.inner
    }
}

/// Chat permissions was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatPermissions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new chat permissions
    permissions: ChatPermissions,
}

impl RObject for UpdateChatPermissions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatPermissions {}

impl UpdateChatPermissions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatPermissionsBuilder {
        let mut inner = UpdateChatPermissions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatPermissionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn permissions(&self) -> &ChatPermissions {
        &self.permissions
    }
}

#[doc(hidden)]
pub struct UpdateChatPermissionsBuilder {
    inner: UpdateChatPermissions,
}

#[deprecated]
pub type RTDUpdateChatPermissionsBuilder = UpdateChatPermissionsBuilder;

impl UpdateChatPermissionsBuilder {
    pub fn build(&self) -> UpdateChatPermissions {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn permissions<T: AsRef<ChatPermissions>>(&mut self, permissions: T) -> &mut Self {
        self.inner.permissions = permissions.as_ref().clone();
        self
    }
}

impl AsRef<UpdateChatPermissions> for UpdateChatPermissions {
    fn as_ref(&self) -> &UpdateChatPermissions {
        self
    }
}

impl AsRef<UpdateChatPermissions> for UpdateChatPermissionsBuilder {
    fn as_ref(&self) -> &UpdateChatPermissions {
        &self.inner
    }
}

/// A chat photo was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new chat photo; may be null
    photo: Option<ChatPhotoInfo>,
}

impl RObject for UpdateChatPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatPhoto {}

impl UpdateChatPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatPhotoBuilder {
        let mut inner = UpdateChatPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatPhotoBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn photo(&self) -> &Option<ChatPhotoInfo> {
        &self.photo
    }
}

#[doc(hidden)]
pub struct UpdateChatPhotoBuilder {
    inner: UpdateChatPhoto,
}

#[deprecated]
pub type RTDUpdateChatPhotoBuilder = UpdateChatPhotoBuilder;

impl UpdateChatPhotoBuilder {
    pub fn build(&self) -> UpdateChatPhoto {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn photo<T: AsRef<ChatPhotoInfo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }
}

impl AsRef<UpdateChatPhoto> for UpdateChatPhoto {
    fn as_ref(&self) -> &UpdateChatPhoto {
        self
    }
}

impl AsRef<UpdateChatPhoto> for UpdateChatPhotoBuilder {
    fn as_ref(&self) -> &UpdateChatPhoto {
        &self.inner
    }
}

/// The position of a chat in a chat list has changed. Instead of this update updateChatLastMessage or updateChatDraftMessage might be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatPosition {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New chat position. If new order is 0, then the chat needs to be removed from the list
    position: ChatPosition,
}

impl RObject for UpdateChatPosition {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatPosition {}

impl UpdateChatPosition {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatPositionBuilder {
        let mut inner = UpdateChatPosition::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatPositionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn position(&self) -> &ChatPosition {
        &self.position
    }
}

#[doc(hidden)]
pub struct UpdateChatPositionBuilder {
    inner: UpdateChatPosition,
}

#[deprecated]
pub type RTDUpdateChatPositionBuilder = UpdateChatPositionBuilder;

impl UpdateChatPositionBuilder {
    pub fn build(&self) -> UpdateChatPosition {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn position<T: AsRef<ChatPosition>>(&mut self, position: T) -> &mut Self {
        self.inner.position = position.as_ref().clone();
        self
    }
}

impl AsRef<UpdateChatPosition> for UpdateChatPosition {
    fn as_ref(&self) -> &UpdateChatPosition {
        self
    }
}

impl AsRef<UpdateChatPosition> for UpdateChatPositionBuilder {
    fn as_ref(&self) -> &UpdateChatPosition {
        &self.inner
    }
}

/// Incoming messages were read or the number of unread messages has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatReadInbox {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the last read incoming message

    #[serde(default)]
    last_read_inbox_message_id: i64,
    /// The number of unread messages left in the chat

    #[serde(default)]
    unread_count: i32,
}

impl RObject for UpdateChatReadInbox {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatReadInbox {}

impl UpdateChatReadInbox {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatReadInboxBuilder {
        let mut inner = UpdateChatReadInbox::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatReadInboxBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn last_read_inbox_message_id(&self) -> i64 {
        self.last_read_inbox_message_id
    }

    pub fn unread_count(&self) -> i32 {
        self.unread_count
    }
}

#[doc(hidden)]
pub struct UpdateChatReadInboxBuilder {
    inner: UpdateChatReadInbox,
}

#[deprecated]
pub type RTDUpdateChatReadInboxBuilder = UpdateChatReadInboxBuilder;

impl UpdateChatReadInboxBuilder {
    pub fn build(&self) -> UpdateChatReadInbox {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn last_read_inbox_message_id(&mut self, last_read_inbox_message_id: i64) -> &mut Self {
        self.inner.last_read_inbox_message_id = last_read_inbox_message_id;
        self
    }

    pub fn unread_count(&mut self, unread_count: i32) -> &mut Self {
        self.inner.unread_count = unread_count;
        self
    }
}

impl AsRef<UpdateChatReadInbox> for UpdateChatReadInbox {
    fn as_ref(&self) -> &UpdateChatReadInbox {
        self
    }
}

impl AsRef<UpdateChatReadInbox> for UpdateChatReadInboxBuilder {
    fn as_ref(&self) -> &UpdateChatReadInbox {
        &self.inner
    }
}

/// Outgoing messages were read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatReadOutbox {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of last read outgoing message

    #[serde(default)]
    last_read_outbox_message_id: i64,
}

impl RObject for UpdateChatReadOutbox {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatReadOutbox {}

impl UpdateChatReadOutbox {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatReadOutboxBuilder {
        let mut inner = UpdateChatReadOutbox::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatReadOutboxBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn last_read_outbox_message_id(&self) -> i64 {
        self.last_read_outbox_message_id
    }
}

#[doc(hidden)]
pub struct UpdateChatReadOutboxBuilder {
    inner: UpdateChatReadOutbox,
}

#[deprecated]
pub type RTDUpdateChatReadOutboxBuilder = UpdateChatReadOutboxBuilder;

impl UpdateChatReadOutboxBuilder {
    pub fn build(&self) -> UpdateChatReadOutbox {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn last_read_outbox_message_id(&mut self, last_read_outbox_message_id: i64) -> &mut Self {
        self.inner.last_read_outbox_message_id = last_read_outbox_message_id;
        self
    }
}

impl AsRef<UpdateChatReadOutbox> for UpdateChatReadOutbox {
    fn as_ref(&self) -> &UpdateChatReadOutbox {
        self
    }
}

impl AsRef<UpdateChatReadOutbox> for UpdateChatReadOutboxBuilder {
    fn as_ref(&self) -> &UpdateChatReadOutbox {
        &self.inner
    }
}

/// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatReplyMarkup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat

    #[serde(default)]
    reply_markup_message_id: i64,
}

impl RObject for UpdateChatReplyMarkup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatReplyMarkup {}

impl UpdateChatReplyMarkup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatReplyMarkupBuilder {
        let mut inner = UpdateChatReplyMarkup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatReplyMarkupBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn reply_markup_message_id(&self) -> i64 {
        self.reply_markup_message_id
    }
}

#[doc(hidden)]
pub struct UpdateChatReplyMarkupBuilder {
    inner: UpdateChatReplyMarkup,
}

#[deprecated]
pub type RTDUpdateChatReplyMarkupBuilder = UpdateChatReplyMarkupBuilder;

impl UpdateChatReplyMarkupBuilder {
    pub fn build(&self) -> UpdateChatReplyMarkup {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn reply_markup_message_id(&mut self, reply_markup_message_id: i64) -> &mut Self {
        self.inner.reply_markup_message_id = reply_markup_message_id;
        self
    }
}

impl AsRef<UpdateChatReplyMarkup> for UpdateChatReplyMarkup {
    fn as_ref(&self) -> &UpdateChatReplyMarkup {
        self
    }
}

impl AsRef<UpdateChatReplyMarkup> for UpdateChatReplyMarkupBuilder {
    fn as_ref(&self) -> &UpdateChatReplyMarkup {
        &self.inner
    }
}

/// The chat theme was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatTheme {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new name of the chat theme; may be empty if theme was reset to default

    #[serde(default)]
    theme_name: String,
}

impl RObject for UpdateChatTheme {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatTheme {}

impl UpdateChatTheme {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatThemeBuilder {
        let mut inner = UpdateChatTheme::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatThemeBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn theme_name(&self) -> &String {
        &self.theme_name
    }
}

#[doc(hidden)]
pub struct UpdateChatThemeBuilder {
    inner: UpdateChatTheme,
}

#[deprecated]
pub type RTDUpdateChatThemeBuilder = UpdateChatThemeBuilder;

impl UpdateChatThemeBuilder {
    pub fn build(&self) -> UpdateChatTheme {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn theme_name<T: AsRef<str>>(&mut self, theme_name: T) -> &mut Self {
        self.inner.theme_name = theme_name.as_ref().to_string();
        self
    }
}

impl AsRef<UpdateChatTheme> for UpdateChatTheme {
    fn as_ref(&self) -> &UpdateChatTheme {
        self
    }
}

impl AsRef<UpdateChatTheme> for UpdateChatThemeBuilder {
    fn as_ref(&self) -> &UpdateChatTheme {
        &self.inner
    }
}

/// The list of available chat themes has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatThemes {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new list of chat themes

    #[serde(default)]
    chat_themes: Vec<ChatTheme>,
}

impl RObject for UpdateChatThemes {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatThemes {}

impl UpdateChatThemes {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatThemesBuilder {
        let mut inner = UpdateChatThemes::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatThemesBuilder { inner }
    }

    pub fn chat_themes(&self) -> &Vec<ChatTheme> {
        &self.chat_themes
    }
}

#[doc(hidden)]
pub struct UpdateChatThemesBuilder {
    inner: UpdateChatThemes,
}

#[deprecated]
pub type RTDUpdateChatThemesBuilder = UpdateChatThemesBuilder;

impl UpdateChatThemesBuilder {
    pub fn build(&self) -> UpdateChatThemes {
        self.inner.clone()
    }

    pub fn chat_themes(&mut self, chat_themes: Vec<ChatTheme>) -> &mut Self {
        self.inner.chat_themes = chat_themes;
        self
    }
}

impl AsRef<UpdateChatThemes> for UpdateChatThemes {
    fn as_ref(&self) -> &UpdateChatThemes {
        self
    }
}

impl AsRef<UpdateChatThemes> for UpdateChatThemesBuilder {
    fn as_ref(&self) -> &UpdateChatThemes {
        &self.inner
    }
}

/// The title of a chat was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatTitle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new chat title

    #[serde(default)]
    title: String,
}

impl RObject for UpdateChatTitle {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatTitle {}

impl UpdateChatTitle {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatTitleBuilder {
        let mut inner = UpdateChatTitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatTitleBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct UpdateChatTitleBuilder {
    inner: UpdateChatTitle,
}

#[deprecated]
pub type RTDUpdateChatTitleBuilder = UpdateChatTitleBuilder;

impl UpdateChatTitleBuilder {
    pub fn build(&self) -> UpdateChatTitle {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }
}

impl AsRef<UpdateChatTitle> for UpdateChatTitle {
    fn as_ref(&self) -> &UpdateChatTitle {
        self
    }
}

impl AsRef<UpdateChatTitle> for UpdateChatTitleBuilder {
    fn as_ref(&self) -> &UpdateChatTitle {
        &self.inner
    }
}

/// The chat unread_mention_count has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatUnreadMentionCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The number of unread mention messages left in the chat

    #[serde(default)]
    unread_mention_count: i32,
}

impl RObject for UpdateChatUnreadMentionCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatUnreadMentionCount {}

impl UpdateChatUnreadMentionCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatUnreadMentionCountBuilder {
        let mut inner = UpdateChatUnreadMentionCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatUnreadMentionCountBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn unread_mention_count(&self) -> i32 {
        self.unread_mention_count
    }
}

#[doc(hidden)]
pub struct UpdateChatUnreadMentionCountBuilder {
    inner: UpdateChatUnreadMentionCount,
}

#[deprecated]
pub type RTDUpdateChatUnreadMentionCountBuilder = UpdateChatUnreadMentionCountBuilder;

impl UpdateChatUnreadMentionCountBuilder {
    pub fn build(&self) -> UpdateChatUnreadMentionCount {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn unread_mention_count(&mut self, unread_mention_count: i32) -> &mut Self {
        self.inner.unread_mention_count = unread_mention_count;
        self
    }
}

impl AsRef<UpdateChatUnreadMentionCount> for UpdateChatUnreadMentionCount {
    fn as_ref(&self) -> &UpdateChatUnreadMentionCount {
        self
    }
}

impl AsRef<UpdateChatUnreadMentionCount> for UpdateChatUnreadMentionCountBuilder {
    fn as_ref(&self) -> &UpdateChatUnreadMentionCount {
        &self.inner
    }
}

/// A chat video chat state has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatVideoChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New value of video_chat
    video_chat: VideoChat,
}

impl RObject for UpdateChatVideoChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateChatVideoChat {}

impl UpdateChatVideoChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateChatVideoChatBuilder {
        let mut inner = UpdateChatVideoChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateChatVideoChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn video_chat(&self) -> &VideoChat {
        &self.video_chat
    }
}

#[doc(hidden)]
pub struct UpdateChatVideoChatBuilder {
    inner: UpdateChatVideoChat,
}

#[deprecated]
pub type RTDUpdateChatVideoChatBuilder = UpdateChatVideoChatBuilder;

impl UpdateChatVideoChatBuilder {
    pub fn build(&self) -> UpdateChatVideoChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn video_chat<T: AsRef<VideoChat>>(&mut self, video_chat: T) -> &mut Self {
        self.inner.video_chat = video_chat.as_ref().clone();
        self
    }
}

impl AsRef<UpdateChatVideoChat> for UpdateChatVideoChat {
    fn as_ref(&self) -> &UpdateChatVideoChat {
        self
    }
}

impl AsRef<UpdateChatVideoChat> for UpdateChatVideoChatBuilder {
    fn as_ref(&self) -> &UpdateChatVideoChat {
        &self.inner
    }
}

/// The connection state has changed. This update must be used only to show a human-readable description of the connection state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateConnectionState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new connection state

    #[serde(skip_serializing_if = "ConnectionState::_is_default")]
    state: ConnectionState,
}

impl RObject for UpdateConnectionState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateConnectionState {}

impl UpdateConnectionState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateConnectionStateBuilder {
        let mut inner = UpdateConnectionState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateConnectionStateBuilder { inner }
    }

    pub fn state(&self) -> &ConnectionState {
        &self.state
    }
}

#[doc(hidden)]
pub struct UpdateConnectionStateBuilder {
    inner: UpdateConnectionState,
}

#[deprecated]
pub type RTDUpdateConnectionStateBuilder = UpdateConnectionStateBuilder;

impl UpdateConnectionStateBuilder {
    pub fn build(&self) -> UpdateConnectionState {
        self.inner.clone()
    }

    pub fn state<T: AsRef<ConnectionState>>(&mut self, state: T) -> &mut Self {
        self.inner.state = state.as_ref().clone();
        self
    }
}

impl AsRef<UpdateConnectionState> for UpdateConnectionState {
    fn as_ref(&self) -> &UpdateConnectionState {
        self
    }
}

impl AsRef<UpdateConnectionState> for UpdateConnectionStateBuilder {
    fn as_ref(&self) -> &UpdateConnectionState {
        &self.inner
    }
}

/// Some messages were deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDeleteMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifiers of the deleted messages

    #[serde(default)]
    message_ids: Vec<i64>,
    /// True, if the messages are permanently deleted by a user (as opposed to just becoming inaccessible)

    #[serde(default)]
    is_permanent: bool,
    /// True, if the messages are deleted only from the cache and can possibly be retrieved again in the future

    #[serde(default)]
    from_cache: bool,
}

impl RObject for UpdateDeleteMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateDeleteMessages {}

impl UpdateDeleteMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateDeleteMessagesBuilder {
        let mut inner = UpdateDeleteMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateDeleteMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }

    pub fn is_permanent(&self) -> bool {
        self.is_permanent
    }

    pub fn from_cache(&self) -> bool {
        self.from_cache
    }
}

#[doc(hidden)]
pub struct UpdateDeleteMessagesBuilder {
    inner: UpdateDeleteMessages,
}

#[deprecated]
pub type RTDUpdateDeleteMessagesBuilder = UpdateDeleteMessagesBuilder;

impl UpdateDeleteMessagesBuilder {
    pub fn build(&self) -> UpdateDeleteMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
        self.inner.message_ids = message_ids;
        self
    }

    pub fn is_permanent(&mut self, is_permanent: bool) -> &mut Self {
        self.inner.is_permanent = is_permanent;
        self
    }

    pub fn from_cache(&mut self, from_cache: bool) -> &mut Self {
        self.inner.from_cache = from_cache;
        self
    }
}

impl AsRef<UpdateDeleteMessages> for UpdateDeleteMessages {
    fn as_ref(&self) -> &UpdateDeleteMessages {
        self
    }
}

impl AsRef<UpdateDeleteMessages> for UpdateDeleteMessagesBuilder {
    fn as_ref(&self) -> &UpdateDeleteMessages {
        &self.inner
    }
}

/// The list of supported dice emojis has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDiceEmojis {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new list of supported dice emojis

    #[serde(default)]
    emojis: Vec<String>,
}

impl RObject for UpdateDiceEmojis {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateDiceEmojis {}

impl UpdateDiceEmojis {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateDiceEmojisBuilder {
        let mut inner = UpdateDiceEmojis::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateDiceEmojisBuilder { inner }
    }

    pub fn emojis(&self) -> &Vec<String> {
        &self.emojis
    }
}

#[doc(hidden)]
pub struct UpdateDiceEmojisBuilder {
    inner: UpdateDiceEmojis,
}

#[deprecated]
pub type RTDUpdateDiceEmojisBuilder = UpdateDiceEmojisBuilder;

impl UpdateDiceEmojisBuilder {
    pub fn build(&self) -> UpdateDiceEmojis {
        self.inner.clone()
    }

    pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
        self.inner.emojis = emojis;
        self
    }
}

impl AsRef<UpdateDiceEmojis> for UpdateDiceEmojis {
    fn as_ref(&self) -> &UpdateDiceEmojis {
        self
    }
}

impl AsRef<UpdateDiceEmojis> for UpdateDiceEmojisBuilder {
    fn as_ref(&self) -> &UpdateDiceEmojis {
        &self.inner
    }
}

/// The list of favorite stickers was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFavoriteStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new list of file identifiers of favorite stickers

    #[serde(default)]
    sticker_ids: Vec<i32>,
}

impl RObject for UpdateFavoriteStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateFavoriteStickers {}

impl UpdateFavoriteStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateFavoriteStickersBuilder {
        let mut inner = UpdateFavoriteStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateFavoriteStickersBuilder { inner }
    }

    pub fn sticker_ids(&self) -> &Vec<i32> {
        &self.sticker_ids
    }
}

#[doc(hidden)]
pub struct UpdateFavoriteStickersBuilder {
    inner: UpdateFavoriteStickers,
}

#[deprecated]
pub type RTDUpdateFavoriteStickersBuilder = UpdateFavoriteStickersBuilder;

impl UpdateFavoriteStickersBuilder {
    pub fn build(&self) -> UpdateFavoriteStickers {
        self.inner.clone()
    }

    pub fn sticker_ids(&mut self, sticker_ids: Vec<i32>) -> &mut Self {
        self.inner.sticker_ids = sticker_ids;
        self
    }
}

impl AsRef<UpdateFavoriteStickers> for UpdateFavoriteStickers {
    fn as_ref(&self) -> &UpdateFavoriteStickers {
        self
    }
}

impl AsRef<UpdateFavoriteStickers> for UpdateFavoriteStickersBuilder {
    fn as_ref(&self) -> &UpdateFavoriteStickers {
        &self.inner
    }
}

/// Information about a file was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New data about the file
    file: File,
}

impl RObject for UpdateFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateFile {}

impl UpdateFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateFileBuilder {
        let mut inner = UpdateFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateFileBuilder { inner }
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

#[doc(hidden)]
pub struct UpdateFileBuilder {
    inner: UpdateFile,
}

#[deprecated]
pub type RTDUpdateFileBuilder = UpdateFileBuilder;

impl UpdateFileBuilder {
    pub fn build(&self) -> UpdateFile {
        self.inner.clone()
    }

    pub fn file<T: AsRef<File>>(&mut self, file: T) -> &mut Self {
        self.inner.file = file.as_ref().clone();
        self
    }
}

impl AsRef<UpdateFile> for UpdateFile {
    fn as_ref(&self) -> &UpdateFile {
        self
    }
}

impl AsRef<UpdateFile> for UpdateFileBuilder {
    fn as_ref(&self) -> &UpdateFile {
        &self.inner
    }
}

/// The file generation process needs to be started by the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFileGenerationStart {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier for the generation process

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    generation_id: i64,
    /// The path to a file from which a new file is generated; may be empty

    #[serde(default)]
    original_path: String,
    /// The path to a file that must be created and where the new file is generated

    #[serde(default)]
    destination_path: String,
    /// String specifying the conversion applied to the original file. If conversion is "#url#" than original_path contains an HTTP/HTTPS URL of a file, which must be downloaded by the application

    #[serde(default)]
    conversion: String,
}

impl RObject for UpdateFileGenerationStart {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateFileGenerationStart {}

impl UpdateFileGenerationStart {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateFileGenerationStartBuilder {
        let mut inner = UpdateFileGenerationStart::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateFileGenerationStartBuilder { inner }
    }

    pub fn generation_id(&self) -> i64 {
        self.generation_id
    }

    pub fn original_path(&self) -> &String {
        &self.original_path
    }

    pub fn destination_path(&self) -> &String {
        &self.destination_path
    }

    pub fn conversion(&self) -> &String {
        &self.conversion
    }
}

#[doc(hidden)]
pub struct UpdateFileGenerationStartBuilder {
    inner: UpdateFileGenerationStart,
}

#[deprecated]
pub type RTDUpdateFileGenerationStartBuilder = UpdateFileGenerationStartBuilder;

impl UpdateFileGenerationStartBuilder {
    pub fn build(&self) -> UpdateFileGenerationStart {
        self.inner.clone()
    }

    pub fn generation_id(&mut self, generation_id: i64) -> &mut Self {
        self.inner.generation_id = generation_id;
        self
    }

    pub fn original_path<T: AsRef<str>>(&mut self, original_path: T) -> &mut Self {
        self.inner.original_path = original_path.as_ref().to_string();
        self
    }

    pub fn destination_path<T: AsRef<str>>(&mut self, destination_path: T) -> &mut Self {
        self.inner.destination_path = destination_path.as_ref().to_string();
        self
    }

    pub fn conversion<T: AsRef<str>>(&mut self, conversion: T) -> &mut Self {
        self.inner.conversion = conversion.as_ref().to_string();
        self
    }
}

impl AsRef<UpdateFileGenerationStart> for UpdateFileGenerationStart {
    fn as_ref(&self) -> &UpdateFileGenerationStart {
        self
    }
}

impl AsRef<UpdateFileGenerationStart> for UpdateFileGenerationStartBuilder {
    fn as_ref(&self) -> &UpdateFileGenerationStart {
        &self.inner
    }
}

/// File generation is no longer needed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateFileGenerationStop {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier for the generation process

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    generation_id: i64,
}

impl RObject for UpdateFileGenerationStop {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateFileGenerationStop {}

impl UpdateFileGenerationStop {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateFileGenerationStopBuilder {
        let mut inner = UpdateFileGenerationStop::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateFileGenerationStopBuilder { inner }
    }

    pub fn generation_id(&self) -> i64 {
        self.generation_id
    }
}

#[doc(hidden)]
pub struct UpdateFileGenerationStopBuilder {
    inner: UpdateFileGenerationStop,
}

#[deprecated]
pub type RTDUpdateFileGenerationStopBuilder = UpdateFileGenerationStopBuilder;

impl UpdateFileGenerationStopBuilder {
    pub fn build(&self) -> UpdateFileGenerationStop {
        self.inner.clone()
    }

    pub fn generation_id(&mut self, generation_id: i64) -> &mut Self {
        self.inner.generation_id = generation_id;
        self
    }
}

impl AsRef<UpdateFileGenerationStop> for UpdateFileGenerationStop {
    fn as_ref(&self) -> &UpdateFileGenerationStop {
        self
    }
}

impl AsRef<UpdateFileGenerationStop> for UpdateFileGenerationStopBuilder {
    fn as_ref(&self) -> &UpdateFileGenerationStop {
        &self.inner
    }
}

/// Information about a group call was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New data about a group call
    group_call: GroupCall,
}

impl RObject for UpdateGroupCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateGroupCall {}

impl UpdateGroupCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateGroupCallBuilder {
        let mut inner = UpdateGroupCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateGroupCallBuilder { inner }
    }

    pub fn group_call(&self) -> &GroupCall {
        &self.group_call
    }
}

#[doc(hidden)]
pub struct UpdateGroupCallBuilder {
    inner: UpdateGroupCall,
}

#[deprecated]
pub type RTDUpdateGroupCallBuilder = UpdateGroupCallBuilder;

impl UpdateGroupCallBuilder {
    pub fn build(&self) -> UpdateGroupCall {
        self.inner.clone()
    }

    pub fn group_call<T: AsRef<GroupCall>>(&mut self, group_call: T) -> &mut Self {
        self.inner.group_call = group_call.as_ref().clone();
        self
    }
}

impl AsRef<UpdateGroupCall> for UpdateGroupCall {
    fn as_ref(&self) -> &UpdateGroupCall {
        self
    }
}

impl AsRef<UpdateGroupCall> for UpdateGroupCallBuilder {
    fn as_ref(&self) -> &UpdateGroupCall {
        &self.inner
    }
}

/// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateGroupCallParticipant {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of group call

    #[serde(default)]
    group_call_id: i32,
    /// New data about a participant
    participant: GroupCallParticipant,
}

impl RObject for UpdateGroupCallParticipant {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateGroupCallParticipant {}

impl UpdateGroupCallParticipant {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateGroupCallParticipantBuilder {
        let mut inner = UpdateGroupCallParticipant::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateGroupCallParticipantBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn participant(&self) -> &GroupCallParticipant {
        &self.participant
    }
}

#[doc(hidden)]
pub struct UpdateGroupCallParticipantBuilder {
    inner: UpdateGroupCallParticipant,
}

#[deprecated]
pub type RTDUpdateGroupCallParticipantBuilder = UpdateGroupCallParticipantBuilder;

impl UpdateGroupCallParticipantBuilder {
    pub fn build(&self) -> UpdateGroupCallParticipant {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn participant<T: AsRef<GroupCallParticipant>>(&mut self, participant: T) -> &mut Self {
        self.inner.participant = participant.as_ref().clone();
        self
    }
}

impl AsRef<UpdateGroupCallParticipant> for UpdateGroupCallParticipant {
    fn as_ref(&self) -> &UpdateGroupCallParticipant {
        self
    }
}

impl AsRef<UpdateGroupCallParticipant> for UpdateGroupCallParticipantBuilder {
    fn as_ref(&self) -> &UpdateGroupCallParticipant {
        &self.inner
    }
}

/// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateHavePendingNotifications {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if there are some delayed notification updates, which will be sent soon

    #[serde(default)]
    have_delayed_notifications: bool,
    /// True, if there can be some yet unreceived notifications, which are being fetched from the server

    #[serde(default)]
    have_unreceived_notifications: bool,
}

impl RObject for UpdateHavePendingNotifications {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateHavePendingNotifications {}

impl UpdateHavePendingNotifications {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateHavePendingNotificationsBuilder {
        let mut inner = UpdateHavePendingNotifications::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateHavePendingNotificationsBuilder { inner }
    }

    pub fn have_delayed_notifications(&self) -> bool {
        self.have_delayed_notifications
    }

    pub fn have_unreceived_notifications(&self) -> bool {
        self.have_unreceived_notifications
    }
}

#[doc(hidden)]
pub struct UpdateHavePendingNotificationsBuilder {
    inner: UpdateHavePendingNotifications,
}

#[deprecated]
pub type RTDUpdateHavePendingNotificationsBuilder = UpdateHavePendingNotificationsBuilder;

impl UpdateHavePendingNotificationsBuilder {
    pub fn build(&self) -> UpdateHavePendingNotifications {
        self.inner.clone()
    }

    pub fn have_delayed_notifications(&mut self, have_delayed_notifications: bool) -> &mut Self {
        self.inner.have_delayed_notifications = have_delayed_notifications;
        self
    }

    pub fn have_unreceived_notifications(
        &mut self,
        have_unreceived_notifications: bool,
    ) -> &mut Self {
        self.inner.have_unreceived_notifications = have_unreceived_notifications;
        self
    }
}

impl AsRef<UpdateHavePendingNotifications> for UpdateHavePendingNotifications {
    fn as_ref(&self) -> &UpdateHavePendingNotifications {
        self
    }
}

impl AsRef<UpdateHavePendingNotifications> for UpdateHavePendingNotificationsBuilder {
    fn as_ref(&self) -> &UpdateHavePendingNotifications {
        &self.inner
    }
}

/// The list of installed sticker sets was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateInstalledStickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the list of installed mask sticker sets was updated

    #[serde(default)]
    is_masks: bool,
    /// The new list of installed ordinary sticker sets

    #[serde(deserialize_with = "super::_common::vec_of_i64_from_str")]
    #[serde(default)]
    sticker_set_ids: Vec<i64>,
}

impl RObject for UpdateInstalledStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateInstalledStickerSets {}

impl UpdateInstalledStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateInstalledStickerSetsBuilder {
        let mut inner = UpdateInstalledStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateInstalledStickerSetsBuilder { inner }
    }

    pub fn is_masks(&self) -> bool {
        self.is_masks
    }

    pub fn sticker_set_ids(&self) -> &Vec<i64> {
        &self.sticker_set_ids
    }
}

#[doc(hidden)]
pub struct UpdateInstalledStickerSetsBuilder {
    inner: UpdateInstalledStickerSets,
}

#[deprecated]
pub type RTDUpdateInstalledStickerSetsBuilder = UpdateInstalledStickerSetsBuilder;

impl UpdateInstalledStickerSetsBuilder {
    pub fn build(&self) -> UpdateInstalledStickerSets {
        self.inner.clone()
    }

    pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
        self.inner.is_masks = is_masks;
        self
    }

    pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<i64>) -> &mut Self {
        self.inner.sticker_set_ids = sticker_set_ids;
        self
    }
}

impl AsRef<UpdateInstalledStickerSets> for UpdateInstalledStickerSets {
    fn as_ref(&self) -> &UpdateInstalledStickerSets {
        self
    }
}

impl AsRef<UpdateInstalledStickerSets> for UpdateInstalledStickerSetsBuilder {
    fn as_ref(&self) -> &UpdateInstalledStickerSets {
        &self.inner
    }
}

/// Some language pack strings have been updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLanguagePackStrings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Localization target to which the language pack belongs

    #[serde(default)]
    localization_target: String,
    /// Identifier of the updated language pack

    #[serde(default)]
    language_pack_id: String,
    /// List of changed language pack strings

    #[serde(default)]
    strings: Vec<LanguagePackString>,
}

impl RObject for UpdateLanguagePackStrings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateLanguagePackStrings {}

impl UpdateLanguagePackStrings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateLanguagePackStringsBuilder {
        let mut inner = UpdateLanguagePackStrings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateLanguagePackStringsBuilder { inner }
    }

    pub fn localization_target(&self) -> &String {
        &self.localization_target
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }

    pub fn strings(&self) -> &Vec<LanguagePackString> {
        &self.strings
    }
}

#[doc(hidden)]
pub struct UpdateLanguagePackStringsBuilder {
    inner: UpdateLanguagePackStrings,
}

#[deprecated]
pub type RTDUpdateLanguagePackStringsBuilder = UpdateLanguagePackStringsBuilder;

impl UpdateLanguagePackStringsBuilder {
    pub fn build(&self) -> UpdateLanguagePackStrings {
        self.inner.clone()
    }

    pub fn localization_target<T: AsRef<str>>(&mut self, localization_target: T) -> &mut Self {
        self.inner.localization_target = localization_target.as_ref().to_string();
        self
    }

    pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
        self.inner.language_pack_id = language_pack_id.as_ref().to_string();
        self
    }

    pub fn strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self {
        self.inner.strings = strings;
        self
    }
}

impl AsRef<UpdateLanguagePackStrings> for UpdateLanguagePackStrings {
    fn as_ref(&self) -> &UpdateLanguagePackStrings {
        self
    }
}

impl AsRef<UpdateLanguagePackStrings> for UpdateLanguagePackStringsBuilder {
    fn as_ref(&self) -> &UpdateLanguagePackStrings {
        &self.inner
    }
}

/// The message content has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageContent {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
    /// New message content

    #[serde(skip_serializing_if = "MessageContent::_is_default")]
    new_content: MessageContent,
}

impl RObject for UpdateMessageContent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateMessageContent {}

impl UpdateMessageContent {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateMessageContentBuilder {
        let mut inner = UpdateMessageContent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateMessageContentBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn new_content(&self) -> &MessageContent {
        &self.new_content
    }
}

#[doc(hidden)]
pub struct UpdateMessageContentBuilder {
    inner: UpdateMessageContent,
}

#[deprecated]
pub type RTDUpdateMessageContentBuilder = UpdateMessageContentBuilder;

impl UpdateMessageContentBuilder {
    pub fn build(&self) -> UpdateMessageContent {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn new_content<T: AsRef<MessageContent>>(&mut self, new_content: T) -> &mut Self {
        self.inner.new_content = new_content.as_ref().clone();
        self
    }
}

impl AsRef<UpdateMessageContent> for UpdateMessageContent {
    fn as_ref(&self) -> &UpdateMessageContent {
        self
    }
}

impl AsRef<UpdateMessageContent> for UpdateMessageContentBuilder {
    fn as_ref(&self) -> &UpdateMessageContent {
        &self.inner
    }
}

/// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageContentOpened {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
}

impl RObject for UpdateMessageContentOpened {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateMessageContentOpened {}

impl UpdateMessageContentOpened {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateMessageContentOpenedBuilder {
        let mut inner = UpdateMessageContentOpened::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateMessageContentOpenedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct UpdateMessageContentOpenedBuilder {
    inner: UpdateMessageContentOpened,
}

#[deprecated]
pub type RTDUpdateMessageContentOpenedBuilder = UpdateMessageContentOpenedBuilder;

impl UpdateMessageContentOpenedBuilder {
    pub fn build(&self) -> UpdateMessageContentOpened {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }
}

impl AsRef<UpdateMessageContentOpened> for UpdateMessageContentOpened {
    fn as_ref(&self) -> &UpdateMessageContentOpened {
        self
    }
}

impl AsRef<UpdateMessageContentOpened> for UpdateMessageContentOpenedBuilder {
    fn as_ref(&self) -> &UpdateMessageContentOpened {
        &self.inner
    }
}

/// A message was edited. Changes in the message content will come in a separate updateMessageContent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageEdited {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
    /// Point in time (Unix timestamp) when the message was edited

    #[serde(default)]
    edit_date: i32,
    /// New message reply markup; may be null
    reply_markup: Option<ReplyMarkup>,
}

impl RObject for UpdateMessageEdited {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateMessageEdited {}

impl UpdateMessageEdited {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateMessageEditedBuilder {
        let mut inner = UpdateMessageEdited::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateMessageEditedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn edit_date(&self) -> i32 {
        self.edit_date
    }

    pub fn reply_markup(&self) -> &Option<ReplyMarkup> {
        &self.reply_markup
    }
}

#[doc(hidden)]
pub struct UpdateMessageEditedBuilder {
    inner: UpdateMessageEdited,
}

#[deprecated]
pub type RTDUpdateMessageEditedBuilder = UpdateMessageEditedBuilder;

impl UpdateMessageEditedBuilder {
    pub fn build(&self) -> UpdateMessageEdited {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn edit_date(&mut self, edit_date: i32) -> &mut Self {
        self.inner.edit_date = edit_date;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = Some(reply_markup.as_ref().clone());
        self
    }
}

impl AsRef<UpdateMessageEdited> for UpdateMessageEdited {
    fn as_ref(&self) -> &UpdateMessageEdited {
        self
    }
}

impl AsRef<UpdateMessageEdited> for UpdateMessageEditedBuilder {
    fn as_ref(&self) -> &UpdateMessageEdited {
        &self.inner
    }
}

/// The information about interactions with a message has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageInteractionInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
    /// New information about interactions with the message; may be null
    interaction_info: Option<MessageInteractionInfo>,
}

impl RObject for UpdateMessageInteractionInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateMessageInteractionInfo {}

impl UpdateMessageInteractionInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateMessageInteractionInfoBuilder {
        let mut inner = UpdateMessageInteractionInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateMessageInteractionInfoBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn interaction_info(&self) -> &Option<MessageInteractionInfo> {
        &self.interaction_info
    }
}

#[doc(hidden)]
pub struct UpdateMessageInteractionInfoBuilder {
    inner: UpdateMessageInteractionInfo,
}

#[deprecated]
pub type RTDUpdateMessageInteractionInfoBuilder = UpdateMessageInteractionInfoBuilder;

impl UpdateMessageInteractionInfoBuilder {
    pub fn build(&self) -> UpdateMessageInteractionInfo {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn interaction_info<T: AsRef<MessageInteractionInfo>>(
        &mut self,
        interaction_info: T,
    ) -> &mut Self {
        self.inner.interaction_info = Some(interaction_info.as_ref().clone());
        self
    }
}

impl AsRef<UpdateMessageInteractionInfo> for UpdateMessageInteractionInfo {
    fn as_ref(&self) -> &UpdateMessageInteractionInfo {
        self
    }
}

impl AsRef<UpdateMessageInteractionInfo> for UpdateMessageInteractionInfoBuilder {
    fn as_ref(&self) -> &UpdateMessageInteractionInfo {
        &self.inner
    }
}

/// The message pinned state was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageIsPinned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The message identifier

    #[serde(default)]
    message_id: i64,
    /// True, if the message is pinned

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for UpdateMessageIsPinned {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateMessageIsPinned {}

impl UpdateMessageIsPinned {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateMessageIsPinnedBuilder {
        let mut inner = UpdateMessageIsPinned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateMessageIsPinnedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct UpdateMessageIsPinnedBuilder {
    inner: UpdateMessageIsPinned,
}

#[deprecated]
pub type RTDUpdateMessageIsPinnedBuilder = UpdateMessageIsPinnedBuilder;

impl UpdateMessageIsPinnedBuilder {
    pub fn build(&self) -> UpdateMessageIsPinned {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<UpdateMessageIsPinned> for UpdateMessageIsPinned {
    fn as_ref(&self) -> &UpdateMessageIsPinned {
        self
    }
}

impl AsRef<UpdateMessageIsPinned> for UpdateMessageIsPinnedBuilder {
    fn as_ref(&self) -> &UpdateMessageIsPinned {
        &self.inner
    }
}

/// A message with a live location was viewed. When the update is received, the application is supposed to update the live location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageLiveLocationViewed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat with the live location message

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message with live location

    #[serde(default)]
    message_id: i64,
}

impl RObject for UpdateMessageLiveLocationViewed {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateMessageLiveLocationViewed {}

impl UpdateMessageLiveLocationViewed {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateMessageLiveLocationViewedBuilder {
        let mut inner = UpdateMessageLiveLocationViewed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateMessageLiveLocationViewedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct UpdateMessageLiveLocationViewedBuilder {
    inner: UpdateMessageLiveLocationViewed,
}

#[deprecated]
pub type RTDUpdateMessageLiveLocationViewedBuilder = UpdateMessageLiveLocationViewedBuilder;

impl UpdateMessageLiveLocationViewedBuilder {
    pub fn build(&self) -> UpdateMessageLiveLocationViewed {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }
}

impl AsRef<UpdateMessageLiveLocationViewed> for UpdateMessageLiveLocationViewed {
    fn as_ref(&self) -> &UpdateMessageLiveLocationViewed {
        self
    }
}

impl AsRef<UpdateMessageLiveLocationViewed> for UpdateMessageLiveLocationViewedBuilder {
    fn as_ref(&self) -> &UpdateMessageLiveLocationViewed {
        &self.inner
    }
}

/// A message with an unread mention was read
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageMentionRead {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
    /// The new number of unread mention messages left in the chat

    #[serde(default)]
    unread_mention_count: i32,
}

impl RObject for UpdateMessageMentionRead {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateMessageMentionRead {}

impl UpdateMessageMentionRead {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateMessageMentionReadBuilder {
        let mut inner = UpdateMessageMentionRead::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateMessageMentionReadBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn unread_mention_count(&self) -> i32 {
        self.unread_mention_count
    }
}

#[doc(hidden)]
pub struct UpdateMessageMentionReadBuilder {
    inner: UpdateMessageMentionRead,
}

#[deprecated]
pub type RTDUpdateMessageMentionReadBuilder = UpdateMessageMentionReadBuilder;

impl UpdateMessageMentionReadBuilder {
    pub fn build(&self) -> UpdateMessageMentionRead {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn unread_mention_count(&mut self, unread_mention_count: i32) -> &mut Self {
        self.inner.unread_mention_count = unread_mention_count;
        self
    }
}

impl AsRef<UpdateMessageMentionRead> for UpdateMessageMentionRead {
    fn as_ref(&self) -> &UpdateMessageMentionRead {
        self
    }
}

impl AsRef<UpdateMessageMentionRead> for UpdateMessageMentionReadBuilder {
    fn as_ref(&self) -> &UpdateMessageMentionRead {
        &self.inner
    }
}

/// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageSendAcknowledged {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat identifier of the sent message

    #[serde(default)]
    chat_id: i64,
    /// A temporary message identifier

    #[serde(default)]
    message_id: i64,
}

impl RObject for UpdateMessageSendAcknowledged {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateMessageSendAcknowledged {}

impl UpdateMessageSendAcknowledged {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateMessageSendAcknowledgedBuilder {
        let mut inner = UpdateMessageSendAcknowledged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateMessageSendAcknowledgedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct UpdateMessageSendAcknowledgedBuilder {
    inner: UpdateMessageSendAcknowledged,
}

#[deprecated]
pub type RTDUpdateMessageSendAcknowledgedBuilder = UpdateMessageSendAcknowledgedBuilder;

impl UpdateMessageSendAcknowledgedBuilder {
    pub fn build(&self) -> UpdateMessageSendAcknowledged {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }
}

impl AsRef<UpdateMessageSendAcknowledged> for UpdateMessageSendAcknowledged {
    fn as_ref(&self) -> &UpdateMessageSendAcknowledged {
        self
    }
}

impl AsRef<UpdateMessageSendAcknowledged> for UpdateMessageSendAcknowledgedBuilder {
    fn as_ref(&self) -> &UpdateMessageSendAcknowledged {
        &self.inner
    }
}

/// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageSendFailed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The failed to send message
    message: Message,
    /// The previous temporary message identifier

    #[serde(default)]
    old_message_id: i64,
    /// An error code

    #[serde(default)]
    error_code: i32,
    /// Error message

    #[serde(default)]
    error_message: String,
}

impl RObject for UpdateMessageSendFailed {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateMessageSendFailed {}

impl UpdateMessageSendFailed {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateMessageSendFailedBuilder {
        let mut inner = UpdateMessageSendFailed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateMessageSendFailedBuilder { inner }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }

    pub fn old_message_id(&self) -> i64 {
        self.old_message_id
    }

    pub fn error_code(&self) -> i32 {
        self.error_code
    }

    pub fn error_message(&self) -> &String {
        &self.error_message
    }
}

#[doc(hidden)]
pub struct UpdateMessageSendFailedBuilder {
    inner: UpdateMessageSendFailed,
}

#[deprecated]
pub type RTDUpdateMessageSendFailedBuilder = UpdateMessageSendFailedBuilder;

impl UpdateMessageSendFailedBuilder {
    pub fn build(&self) -> UpdateMessageSendFailed {
        self.inner.clone()
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }

    pub fn old_message_id(&mut self, old_message_id: i64) -> &mut Self {
        self.inner.old_message_id = old_message_id;
        self
    }

    pub fn error_code(&mut self, error_code: i32) -> &mut Self {
        self.inner.error_code = error_code;
        self
    }

    pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
        self.inner.error_message = error_message.as_ref().to_string();
        self
    }
}

impl AsRef<UpdateMessageSendFailed> for UpdateMessageSendFailed {
    fn as_ref(&self) -> &UpdateMessageSendFailed {
        self
    }
}

impl AsRef<UpdateMessageSendFailed> for UpdateMessageSendFailedBuilder {
    fn as_ref(&self) -> &UpdateMessageSendFailed {
        &self.inner
    }
}

/// A message has been successfully sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMessageSendSucceeded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The sent message. Usually only the message identifier, date, and content are changed, but almost all other fields can also change
    message: Message,
    /// The previous temporary message identifier

    #[serde(default)]
    old_message_id: i64,
}

impl RObject for UpdateMessageSendSucceeded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateMessageSendSucceeded {}

impl UpdateMessageSendSucceeded {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateMessageSendSucceededBuilder {
        let mut inner = UpdateMessageSendSucceeded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateMessageSendSucceededBuilder { inner }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }

    pub fn old_message_id(&self) -> i64 {
        self.old_message_id
    }
}

#[doc(hidden)]
pub struct UpdateMessageSendSucceededBuilder {
    inner: UpdateMessageSendSucceeded,
}

#[deprecated]
pub type RTDUpdateMessageSendSucceededBuilder = UpdateMessageSendSucceededBuilder;

impl UpdateMessageSendSucceededBuilder {
    pub fn build(&self) -> UpdateMessageSendSucceeded {
        self.inner.clone()
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }

    pub fn old_message_id(&mut self, old_message_id: i64) -> &mut Self {
        self.inner.old_message_id = old_message_id;
        self
    }
}

impl AsRef<UpdateMessageSendSucceeded> for UpdateMessageSendSucceeded {
    fn as_ref(&self) -> &UpdateMessageSendSucceeded {
        self
    }
}

impl AsRef<UpdateMessageSendSucceeded> for UpdateMessageSendSucceededBuilder {
    fn as_ref(&self) -> &UpdateMessageSendSucceeded {
        &self.inner
    }
}

/// New call signaling data arrived
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewCallSignalingData {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The call identifier

    #[serde(default)]
    call_id: i32,
    /// The data

    #[serde(default)]
    data: String,
}

impl RObject for UpdateNewCallSignalingData {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewCallSignalingData {}

impl UpdateNewCallSignalingData {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewCallSignalingDataBuilder {
        let mut inner = UpdateNewCallSignalingData::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewCallSignalingDataBuilder { inner }
    }

    pub fn call_id(&self) -> i32 {
        self.call_id
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct UpdateNewCallSignalingDataBuilder {
    inner: UpdateNewCallSignalingData,
}

#[deprecated]
pub type RTDUpdateNewCallSignalingDataBuilder = UpdateNewCallSignalingDataBuilder;

impl UpdateNewCallSignalingDataBuilder {
    pub fn build(&self) -> UpdateNewCallSignalingData {
        self.inner.clone()
    }

    pub fn call_id(&mut self, call_id: i32) -> &mut Self {
        self.inner.call_id = call_id;
        self
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<UpdateNewCallSignalingData> for UpdateNewCallSignalingData {
    fn as_ref(&self) -> &UpdateNewCallSignalingData {
        self
    }
}

impl AsRef<UpdateNewCallSignalingData> for UpdateNewCallSignalingDataBuilder {
    fn as_ref(&self) -> &UpdateNewCallSignalingData {
        &self.inner
    }
}

/// A new incoming callback query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewCallbackQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique query identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Identifier of the user who sent the query

    #[serde(default)]
    sender_user_id: i64,
    /// Identifier of the chat where the query was sent

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message, from which the query originated

    #[serde(default)]
    message_id: i64,
    /// Identifier that uniquely corresponds to the chat to which the message was sent

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    chat_instance: i64,
    /// Query payload

    #[serde(skip_serializing_if = "CallbackQueryPayload::_is_default")]
    payload: CallbackQueryPayload,
}

impl RObject for UpdateNewCallbackQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewCallbackQuery {}

impl UpdateNewCallbackQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewCallbackQueryBuilder {
        let mut inner = UpdateNewCallbackQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewCallbackQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_user_id(&self) -> i64 {
        self.sender_user_id
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn chat_instance(&self) -> i64 {
        self.chat_instance
    }

    pub fn payload(&self) -> &CallbackQueryPayload {
        &self.payload
    }
}

#[doc(hidden)]
pub struct UpdateNewCallbackQueryBuilder {
    inner: UpdateNewCallbackQuery,
}

#[deprecated]
pub type RTDUpdateNewCallbackQueryBuilder = UpdateNewCallbackQueryBuilder;

impl UpdateNewCallbackQueryBuilder {
    pub fn build(&self) -> UpdateNewCallbackQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
        self.inner.sender_user_id = sender_user_id;
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn chat_instance(&mut self, chat_instance: i64) -> &mut Self {
        self.inner.chat_instance = chat_instance;
        self
    }

    pub fn payload<T: AsRef<CallbackQueryPayload>>(&mut self, payload: T) -> &mut Self {
        self.inner.payload = payload.as_ref().clone();
        self
    }
}

impl AsRef<UpdateNewCallbackQuery> for UpdateNewCallbackQuery {
    fn as_ref(&self) -> &UpdateNewCallbackQuery {
        self
    }
}

impl AsRef<UpdateNewCallbackQuery> for UpdateNewCallbackQueryBuilder {
    fn as_ref(&self) -> &UpdateNewCallbackQuery {
        &self.inner
    }
}

/// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat
    chat: Chat,
}

impl RObject for UpdateNewChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewChat {}

impl UpdateNewChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewChatBuilder {
        let mut inner = UpdateNewChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewChatBuilder { inner }
    }

    pub fn chat(&self) -> &Chat {
        &self.chat
    }
}

#[doc(hidden)]
pub struct UpdateNewChatBuilder {
    inner: UpdateNewChat,
}

#[deprecated]
pub type RTDUpdateNewChatBuilder = UpdateNewChatBuilder;

impl UpdateNewChatBuilder {
    pub fn build(&self) -> UpdateNewChat {
        self.inner.clone()
    }

    pub fn chat<T: AsRef<Chat>>(&mut self, chat: T) -> &mut Self {
        self.inner.chat = chat.as_ref().clone();
        self
    }
}

impl AsRef<UpdateNewChat> for UpdateNewChat {
    fn as_ref(&self) -> &UpdateNewChat {
        self
    }
}

impl AsRef<UpdateNewChat> for UpdateNewChatBuilder {
    fn as_ref(&self) -> &UpdateNewChat {
        &self.inner
    }
}

/// A user sent a join request to a chat; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewChatJoinRequest {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Join request
    request: ChatJoinRequest,
    /// The invite link, which was used to send join request; may be null
    invite_link: Option<ChatInviteLink>,
}

impl RObject for UpdateNewChatJoinRequest {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewChatJoinRequest {}

impl UpdateNewChatJoinRequest {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewChatJoinRequestBuilder {
        let mut inner = UpdateNewChatJoinRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewChatJoinRequestBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn request(&self) -> &ChatJoinRequest {
        &self.request
    }

    pub fn invite_link(&self) -> &Option<ChatInviteLink> {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct UpdateNewChatJoinRequestBuilder {
    inner: UpdateNewChatJoinRequest,
}

#[deprecated]
pub type RTDUpdateNewChatJoinRequestBuilder = UpdateNewChatJoinRequestBuilder;

impl UpdateNewChatJoinRequestBuilder {
    pub fn build(&self) -> UpdateNewChatJoinRequest {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn request<T: AsRef<ChatJoinRequest>>(&mut self, request: T) -> &mut Self {
        self.inner.request = request.as_ref().clone();
        self
    }

    pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = Some(invite_link.as_ref().clone());
        self
    }
}

impl AsRef<UpdateNewChatJoinRequest> for UpdateNewChatJoinRequest {
    fn as_ref(&self) -> &UpdateNewChatJoinRequest {
        self
    }
}

impl AsRef<UpdateNewChatJoinRequest> for UpdateNewChatJoinRequestBuilder {
    fn as_ref(&self) -> &UpdateNewChatJoinRequest {
        &self.inner
    }
}

/// The user has chosen a result of an inline query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewChosenInlineResult {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user who sent the query

    #[serde(default)]
    sender_user_id: i64,
    /// User location; may be null
    user_location: Option<Location>,
    /// Text of the query

    #[serde(default)]
    query: String,
    /// Identifier of the chosen result

    #[serde(default)]
    result_id: String,
    /// Identifier of the sent inline message, if known

    #[serde(default)]
    inline_message_id: String,
}

impl RObject for UpdateNewChosenInlineResult {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewChosenInlineResult {}

impl UpdateNewChosenInlineResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewChosenInlineResultBuilder {
        let mut inner = UpdateNewChosenInlineResult::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewChosenInlineResultBuilder { inner }
    }

    pub fn sender_user_id(&self) -> i64 {
        self.sender_user_id
    }

    pub fn user_location(&self) -> &Option<Location> {
        &self.user_location
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn result_id(&self) -> &String {
        &self.result_id
    }

    pub fn inline_message_id(&self) -> &String {
        &self.inline_message_id
    }
}

#[doc(hidden)]
pub struct UpdateNewChosenInlineResultBuilder {
    inner: UpdateNewChosenInlineResult,
}

#[deprecated]
pub type RTDUpdateNewChosenInlineResultBuilder = UpdateNewChosenInlineResultBuilder;

impl UpdateNewChosenInlineResultBuilder {
    pub fn build(&self) -> UpdateNewChosenInlineResult {
        self.inner.clone()
    }

    pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
        self.inner.sender_user_id = sender_user_id;
        self
    }

    pub fn user_location<T: AsRef<Location>>(&mut self, user_location: T) -> &mut Self {
        self.inner.user_location = Some(user_location.as_ref().clone());
        self
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn result_id<T: AsRef<str>>(&mut self, result_id: T) -> &mut Self {
        self.inner.result_id = result_id.as_ref().to_string();
        self
    }

    pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
        self.inner.inline_message_id = inline_message_id.as_ref().to_string();
        self
    }
}

impl AsRef<UpdateNewChosenInlineResult> for UpdateNewChosenInlineResult {
    fn as_ref(&self) -> &UpdateNewChosenInlineResult {
        self
    }
}

impl AsRef<UpdateNewChosenInlineResult> for UpdateNewChosenInlineResultBuilder {
    fn as_ref(&self) -> &UpdateNewChosenInlineResult {
        &self.inner
    }
}

/// A new incoming event; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewCustomEvent {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A JSON-serialized event

    #[serde(default)]
    event: String,
}

impl RObject for UpdateNewCustomEvent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewCustomEvent {}

impl UpdateNewCustomEvent {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewCustomEventBuilder {
        let mut inner = UpdateNewCustomEvent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewCustomEventBuilder { inner }
    }

    pub fn event(&self) -> &String {
        &self.event
    }
}

#[doc(hidden)]
pub struct UpdateNewCustomEventBuilder {
    inner: UpdateNewCustomEvent,
}

#[deprecated]
pub type RTDUpdateNewCustomEventBuilder = UpdateNewCustomEventBuilder;

impl UpdateNewCustomEventBuilder {
    pub fn build(&self) -> UpdateNewCustomEvent {
        self.inner.clone()
    }

    pub fn event<T: AsRef<str>>(&mut self, event: T) -> &mut Self {
        self.inner.event = event.as_ref().to_string();
        self
    }
}

impl AsRef<UpdateNewCustomEvent> for UpdateNewCustomEvent {
    fn as_ref(&self) -> &UpdateNewCustomEvent {
        self
    }
}

impl AsRef<UpdateNewCustomEvent> for UpdateNewCustomEventBuilder {
    fn as_ref(&self) -> &UpdateNewCustomEvent {
        &self.inner
    }
}

/// A new incoming query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewCustomQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The query identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// JSON-serialized query data

    #[serde(default)]
    data: String,
    /// Query timeout

    #[serde(default)]
    timeout: i32,
}

impl RObject for UpdateNewCustomQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewCustomQuery {}

impl UpdateNewCustomQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewCustomQueryBuilder {
        let mut inner = UpdateNewCustomQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewCustomQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn timeout(&self) -> i32 {
        self.timeout
    }
}

#[doc(hidden)]
pub struct UpdateNewCustomQueryBuilder {
    inner: UpdateNewCustomQuery,
}

#[deprecated]
pub type RTDUpdateNewCustomQueryBuilder = UpdateNewCustomQueryBuilder;

impl UpdateNewCustomQueryBuilder {
    pub fn build(&self) -> UpdateNewCustomQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }

    pub fn timeout(&mut self, timeout: i32) -> &mut Self {
        self.inner.timeout = timeout;
        self
    }
}

impl AsRef<UpdateNewCustomQuery> for UpdateNewCustomQuery {
    fn as_ref(&self) -> &UpdateNewCustomQuery {
        self
    }
}

impl AsRef<UpdateNewCustomQuery> for UpdateNewCustomQueryBuilder {
    fn as_ref(&self) -> &UpdateNewCustomQuery {
        &self.inner
    }
}

/// A new incoming callback query from a message sent via a bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewInlineCallbackQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique query identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Identifier of the user who sent the query

    #[serde(default)]
    sender_user_id: i64,
    /// Identifier of the inline message, from which the query originated

    #[serde(default)]
    inline_message_id: String,
    /// An identifier uniquely corresponding to the chat a message was sent to

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    chat_instance: i64,
    /// Query payload

    #[serde(skip_serializing_if = "CallbackQueryPayload::_is_default")]
    payload: CallbackQueryPayload,
}

impl RObject for UpdateNewInlineCallbackQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewInlineCallbackQuery {}

impl UpdateNewInlineCallbackQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewInlineCallbackQueryBuilder {
        let mut inner = UpdateNewInlineCallbackQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewInlineCallbackQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_user_id(&self) -> i64 {
        self.sender_user_id
    }

    pub fn inline_message_id(&self) -> &String {
        &self.inline_message_id
    }

    pub fn chat_instance(&self) -> i64 {
        self.chat_instance
    }

    pub fn payload(&self) -> &CallbackQueryPayload {
        &self.payload
    }
}

#[doc(hidden)]
pub struct UpdateNewInlineCallbackQueryBuilder {
    inner: UpdateNewInlineCallbackQuery,
}

#[deprecated]
pub type RTDUpdateNewInlineCallbackQueryBuilder = UpdateNewInlineCallbackQueryBuilder;

impl UpdateNewInlineCallbackQueryBuilder {
    pub fn build(&self) -> UpdateNewInlineCallbackQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
        self.inner.sender_user_id = sender_user_id;
        self
    }

    pub fn inline_message_id<T: AsRef<str>>(&mut self, inline_message_id: T) -> &mut Self {
        self.inner.inline_message_id = inline_message_id.as_ref().to_string();
        self
    }

    pub fn chat_instance(&mut self, chat_instance: i64) -> &mut Self {
        self.inner.chat_instance = chat_instance;
        self
    }

    pub fn payload<T: AsRef<CallbackQueryPayload>>(&mut self, payload: T) -> &mut Self {
        self.inner.payload = payload.as_ref().clone();
        self
    }
}

impl AsRef<UpdateNewInlineCallbackQuery> for UpdateNewInlineCallbackQuery {
    fn as_ref(&self) -> &UpdateNewInlineCallbackQuery {
        self
    }
}

impl AsRef<UpdateNewInlineCallbackQuery> for UpdateNewInlineCallbackQueryBuilder {
    fn as_ref(&self) -> &UpdateNewInlineCallbackQuery {
        &self.inner
    }
}

/// A new incoming inline query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewInlineQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique query identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Identifier of the user who sent the query

    #[serde(default)]
    sender_user_id: i64,
    /// User location; may be null
    user_location: Option<Location>,
    /// The type of the chat, from which the query originated; may be null if unknown
    chat_type: Option<ChatType>,
    /// Text of the query

    #[serde(default)]
    query: String,
    /// Offset of the first entry to return

    #[serde(default)]
    offset: String,
}

impl RObject for UpdateNewInlineQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewInlineQuery {}

impl UpdateNewInlineQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewInlineQueryBuilder {
        let mut inner = UpdateNewInlineQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewInlineQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_user_id(&self) -> i64 {
        self.sender_user_id
    }

    pub fn user_location(&self) -> &Option<Location> {
        &self.user_location
    }

    pub fn chat_type(&self) -> &Option<ChatType> {
        &self.chat_type
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn offset(&self) -> &String {
        &self.offset
    }
}

#[doc(hidden)]
pub struct UpdateNewInlineQueryBuilder {
    inner: UpdateNewInlineQuery,
}

#[deprecated]
pub type RTDUpdateNewInlineQueryBuilder = UpdateNewInlineQueryBuilder;

impl UpdateNewInlineQueryBuilder {
    pub fn build(&self) -> UpdateNewInlineQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
        self.inner.sender_user_id = sender_user_id;
        self
    }

    pub fn user_location<T: AsRef<Location>>(&mut self, user_location: T) -> &mut Self {
        self.inner.user_location = Some(user_location.as_ref().clone());
        self
    }

    pub fn chat_type<T: AsRef<ChatType>>(&mut self, chat_type: T) -> &mut Self {
        self.inner.chat_type = Some(chat_type.as_ref().clone());
        self
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn offset<T: AsRef<str>>(&mut self, offset: T) -> &mut Self {
        self.inner.offset = offset.as_ref().to_string();
        self
    }
}

impl AsRef<UpdateNewInlineQuery> for UpdateNewInlineQuery {
    fn as_ref(&self) -> &UpdateNewInlineQuery {
        self
    }
}

impl AsRef<UpdateNewInlineQuery> for UpdateNewInlineQueryBuilder {
    fn as_ref(&self) -> &UpdateNewInlineQuery {
        &self.inner
    }
}

/// A new message was received; can also be an outgoing message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new message
    message: Message,
}

impl RObject for UpdateNewMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewMessage {}

impl UpdateNewMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewMessageBuilder {
        let mut inner = UpdateNewMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewMessageBuilder { inner }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}

#[doc(hidden)]
pub struct UpdateNewMessageBuilder {
    inner: UpdateNewMessage,
}

#[deprecated]
pub type RTDUpdateNewMessageBuilder = UpdateNewMessageBuilder;

impl UpdateNewMessageBuilder {
    pub fn build(&self) -> UpdateNewMessage {
        self.inner.clone()
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }
}

impl AsRef<UpdateNewMessage> for UpdateNewMessage {
    fn as_ref(&self) -> &UpdateNewMessage {
        self
    }
}

impl AsRef<UpdateNewMessage> for UpdateNewMessageBuilder {
    fn as_ref(&self) -> &UpdateNewMessage {
        &self.inner
    }
}

/// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewPreCheckoutQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique query identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Identifier of the user who sent the query

    #[serde(default)]
    sender_user_id: i64,
    /// Currency for the product price

    #[serde(default)]
    currency: String,
    /// Total price for the product, in the smallest units of the currency

    #[serde(default)]
    total_amount: i64,
    /// Invoice payload

    #[serde(default)]
    invoice_payload: String,
    /// Identifier of a shipping option chosen by the user; may be empty if not applicable

    #[serde(default)]
    shipping_option_id: String,
    /// Information about the order; may be null
    order_info: Option<OrderInfo>,
}

impl RObject for UpdateNewPreCheckoutQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewPreCheckoutQuery {}

impl UpdateNewPreCheckoutQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewPreCheckoutQueryBuilder {
        let mut inner = UpdateNewPreCheckoutQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewPreCheckoutQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_user_id(&self) -> i64 {
        self.sender_user_id
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn total_amount(&self) -> i64 {
        self.total_amount
    }

    pub fn invoice_payload(&self) -> &String {
        &self.invoice_payload
    }

    pub fn shipping_option_id(&self) -> &String {
        &self.shipping_option_id
    }

    pub fn order_info(&self) -> &Option<OrderInfo> {
        &self.order_info
    }
}

#[doc(hidden)]
pub struct UpdateNewPreCheckoutQueryBuilder {
    inner: UpdateNewPreCheckoutQuery,
}

#[deprecated]
pub type RTDUpdateNewPreCheckoutQueryBuilder = UpdateNewPreCheckoutQueryBuilder;

impl UpdateNewPreCheckoutQueryBuilder {
    pub fn build(&self) -> UpdateNewPreCheckoutQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
        self.inner.sender_user_id = sender_user_id;
        self
    }

    pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
        self.inner.currency = currency.as_ref().to_string();
        self
    }

    pub fn total_amount(&mut self, total_amount: i64) -> &mut Self {
        self.inner.total_amount = total_amount;
        self
    }

    pub fn invoice_payload<T: AsRef<str>>(&mut self, invoice_payload: T) -> &mut Self {
        self.inner.invoice_payload = invoice_payload.as_ref().to_string();
        self
    }

    pub fn shipping_option_id<T: AsRef<str>>(&mut self, shipping_option_id: T) -> &mut Self {
        self.inner.shipping_option_id = shipping_option_id.as_ref().to_string();
        self
    }

    pub fn order_info<T: AsRef<OrderInfo>>(&mut self, order_info: T) -> &mut Self {
        self.inner.order_info = Some(order_info.as_ref().clone());
        self
    }
}

impl AsRef<UpdateNewPreCheckoutQuery> for UpdateNewPreCheckoutQuery {
    fn as_ref(&self) -> &UpdateNewPreCheckoutQuery {
        self
    }
}

impl AsRef<UpdateNewPreCheckoutQuery> for UpdateNewPreCheckoutQueryBuilder {
    fn as_ref(&self) -> &UpdateNewPreCheckoutQuery {
        &self.inner
    }
}

/// A new incoming shipping query; for bots only. Only for invoices with flexible price
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNewShippingQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique query identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Identifier of the user who sent the query

    #[serde(default)]
    sender_user_id: i64,
    /// Invoice payload

    #[serde(default)]
    invoice_payload: String,
    /// User shipping address
    shipping_address: Address,
}

impl RObject for UpdateNewShippingQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNewShippingQuery {}

impl UpdateNewShippingQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNewShippingQueryBuilder {
        let mut inner = UpdateNewShippingQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNewShippingQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_user_id(&self) -> i64 {
        self.sender_user_id
    }

    pub fn invoice_payload(&self) -> &String {
        &self.invoice_payload
    }

    pub fn shipping_address(&self) -> &Address {
        &self.shipping_address
    }
}

#[doc(hidden)]
pub struct UpdateNewShippingQueryBuilder {
    inner: UpdateNewShippingQuery,
}

#[deprecated]
pub type RTDUpdateNewShippingQueryBuilder = UpdateNewShippingQueryBuilder;

impl UpdateNewShippingQueryBuilder {
    pub fn build(&self) -> UpdateNewShippingQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
        self.inner.sender_user_id = sender_user_id;
        self
    }

    pub fn invoice_payload<T: AsRef<str>>(&mut self, invoice_payload: T) -> &mut Self {
        self.inner.invoice_payload = invoice_payload.as_ref().to_string();
        self
    }

    pub fn shipping_address<T: AsRef<Address>>(&mut self, shipping_address: T) -> &mut Self {
        self.inner.shipping_address = shipping_address.as_ref().clone();
        self
    }
}

impl AsRef<UpdateNewShippingQuery> for UpdateNewShippingQuery {
    fn as_ref(&self) -> &UpdateNewShippingQuery {
        self
    }
}

impl AsRef<UpdateNewShippingQuery> for UpdateNewShippingQueryBuilder {
    fn as_ref(&self) -> &UpdateNewShippingQuery {
        &self.inner
    }
}

/// A notification was changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNotification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique notification group identifier

    #[serde(default)]
    notification_group_id: i32,
    /// Changed notification
    notification: Notification,
}

impl RObject for UpdateNotification {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNotification {}

impl UpdateNotification {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNotificationBuilder {
        let mut inner = UpdateNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNotificationBuilder { inner }
    }

    pub fn notification_group_id(&self) -> i32 {
        self.notification_group_id
    }

    pub fn notification(&self) -> &Notification {
        &self.notification
    }
}

#[doc(hidden)]
pub struct UpdateNotificationBuilder {
    inner: UpdateNotification,
}

#[deprecated]
pub type RTDUpdateNotificationBuilder = UpdateNotificationBuilder;

impl UpdateNotificationBuilder {
    pub fn build(&self) -> UpdateNotification {
        self.inner.clone()
    }

    pub fn notification_group_id(&mut self, notification_group_id: i32) -> &mut Self {
        self.inner.notification_group_id = notification_group_id;
        self
    }

    pub fn notification<T: AsRef<Notification>>(&mut self, notification: T) -> &mut Self {
        self.inner.notification = notification.as_ref().clone();
        self
    }
}

impl AsRef<UpdateNotification> for UpdateNotification {
    fn as_ref(&self) -> &UpdateNotification {
        self
    }
}

impl AsRef<UpdateNotification> for UpdateNotificationBuilder {
    fn as_ref(&self) -> &UpdateNotification {
        &self.inner
    }
}

/// A list of active notifications in a notification group has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNotificationGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique notification group identifier

    #[serde(default)]
    notification_group_id: i32,
    /// New type of the notification group

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "NotificationGroupType::_is_default")]
    type_: NotificationGroupType,
    /// Identifier of a chat to which all notifications in the group belong

    #[serde(default)]
    chat_id: i64,
    /// Chat identifier, which notification settings must be applied to the added notifications

    #[serde(default)]
    notification_settings_chat_id: i64,
    /// True, if the notifications must be shown without sound

    #[serde(default)]
    is_silent: bool,
    /// Total number of unread notifications in the group, can be bigger than number of active notifications

    #[serde(default)]
    total_count: i32,
    /// List of added group notifications, sorted by notification ID

    #[serde(default)]
    added_notifications: Vec<Notification>,
    /// Identifiers of removed group notifications, sorted by notification ID

    #[serde(default)]
    removed_notification_ids: Vec<i32>,
}

impl RObject for UpdateNotificationGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateNotificationGroup {}

impl UpdateNotificationGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateNotificationGroupBuilder {
        let mut inner = UpdateNotificationGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateNotificationGroupBuilder { inner }
    }

    pub fn notification_group_id(&self) -> i32 {
        self.notification_group_id
    }

    pub fn type_(&self) -> &NotificationGroupType {
        &self.type_
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn notification_settings_chat_id(&self) -> i64 {
        self.notification_settings_chat_id
    }

    pub fn is_silent(&self) -> bool {
        self.is_silent
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn added_notifications(&self) -> &Vec<Notification> {
        &self.added_notifications
    }

    pub fn removed_notification_ids(&self) -> &Vec<i32> {
        &self.removed_notification_ids
    }
}

#[doc(hidden)]
pub struct UpdateNotificationGroupBuilder {
    inner: UpdateNotificationGroup,
}

#[deprecated]
pub type RTDUpdateNotificationGroupBuilder = UpdateNotificationGroupBuilder;

impl UpdateNotificationGroupBuilder {
    pub fn build(&self) -> UpdateNotificationGroup {
        self.inner.clone()
    }

    pub fn notification_group_id(&mut self, notification_group_id: i32) -> &mut Self {
        self.inner.notification_group_id = notification_group_id;
        self
    }

    pub fn type_<T: AsRef<NotificationGroupType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn notification_settings_chat_id(
        &mut self,
        notification_settings_chat_id: i64,
    ) -> &mut Self {
        self.inner.notification_settings_chat_id = notification_settings_chat_id;
        self
    }

    pub fn is_silent(&mut self, is_silent: bool) -> &mut Self {
        self.inner.is_silent = is_silent;
        self
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn added_notifications(&mut self, added_notifications: Vec<Notification>) -> &mut Self {
        self.inner.added_notifications = added_notifications;
        self
    }

    pub fn removed_notification_ids(&mut self, removed_notification_ids: Vec<i32>) -> &mut Self {
        self.inner.removed_notification_ids = removed_notification_ids;
        self
    }
}

impl AsRef<UpdateNotificationGroup> for UpdateNotificationGroup {
    fn as_ref(&self) -> &UpdateNotificationGroup {
        self
    }
}

impl AsRef<UpdateNotificationGroup> for UpdateNotificationGroupBuilder {
    fn as_ref(&self) -> &UpdateNotificationGroup {
        &self.inner
    }
}

/// An option changed its value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateOption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The option name

    #[serde(default)]
    name: String,
    /// The new option value

    #[serde(skip_serializing_if = "OptionValue::_is_default")]
    value: OptionValue,
}

impl RObject for UpdateOption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateOption {}

impl UpdateOption {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateOptionBuilder {
        let mut inner = UpdateOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateOptionBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &OptionValue {
        &self.value
    }
}

#[doc(hidden)]
pub struct UpdateOptionBuilder {
    inner: UpdateOption,
}

#[deprecated]
pub type RTDUpdateOptionBuilder = UpdateOptionBuilder;

impl UpdateOptionBuilder {
    pub fn build(&self) -> UpdateOption {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn value<T: AsRef<OptionValue>>(&mut self, value: T) -> &mut Self {
        self.inner.value = value.as_ref().clone();
        self
    }
}

impl AsRef<UpdateOption> for UpdateOption {
    fn as_ref(&self) -> &UpdateOption {
        self
    }
}

impl AsRef<UpdateOption> for UpdateOptionBuilder {
    fn as_ref(&self) -> &UpdateOption {
        &self.inner
    }
}

/// A poll was updated; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePoll {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New data about the poll
    poll: Poll,
}

impl RObject for UpdatePoll {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdatePoll {}

impl UpdatePoll {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdatePollBuilder {
        let mut inner = UpdatePoll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdatePollBuilder { inner }
    }

    pub fn poll(&self) -> &Poll {
        &self.poll
    }
}

#[doc(hidden)]
pub struct UpdatePollBuilder {
    inner: UpdatePoll,
}

#[deprecated]
pub type RTDUpdatePollBuilder = UpdatePollBuilder;

impl UpdatePollBuilder {
    pub fn build(&self) -> UpdatePoll {
        self.inner.clone()
    }

    pub fn poll<T: AsRef<Poll>>(&mut self, poll: T) -> &mut Self {
        self.inner.poll = poll.as_ref().clone();
        self
    }
}

impl AsRef<UpdatePoll> for UpdatePoll {
    fn as_ref(&self) -> &UpdatePoll {
        self
    }
}

impl AsRef<UpdatePoll> for UpdatePollBuilder {
    fn as_ref(&self) -> &UpdatePoll {
        &self.inner
    }
}

/// A user changed the answer to a poll; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePollAnswer {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique poll identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    poll_id: i64,
    /// The user, who changed the answer to the poll

    #[serde(default)]
    user_id: i64,
    /// 0-based identifiers of answer options, chosen by the user

    #[serde(default)]
    option_ids: Vec<i32>,
}

impl RObject for UpdatePollAnswer {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdatePollAnswer {}

impl UpdatePollAnswer {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdatePollAnswerBuilder {
        let mut inner = UpdatePollAnswer::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdatePollAnswerBuilder { inner }
    }

    pub fn poll_id(&self) -> i64 {
        self.poll_id
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn option_ids(&self) -> &Vec<i32> {
        &self.option_ids
    }
}

#[doc(hidden)]
pub struct UpdatePollAnswerBuilder {
    inner: UpdatePollAnswer,
}

#[deprecated]
pub type RTDUpdatePollAnswerBuilder = UpdatePollAnswerBuilder;

impl UpdatePollAnswerBuilder {
    pub fn build(&self) -> UpdatePollAnswer {
        self.inner.clone()
    }

    pub fn poll_id(&mut self, poll_id: i64) -> &mut Self {
        self.inner.poll_id = poll_id;
        self
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn option_ids(&mut self, option_ids: Vec<i32>) -> &mut Self {
        self.inner.option_ids = option_ids;
        self
    }
}

impl AsRef<UpdatePollAnswer> for UpdatePollAnswer {
    fn as_ref(&self) -> &UpdatePollAnswer {
        self
    }
}

impl AsRef<UpdatePollAnswer> for UpdatePollAnswerBuilder {
    fn as_ref(&self) -> &UpdatePollAnswer {
        &self.inner
    }
}

/// The list of recently used stickers was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateRecentStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the list of stickers attached to photo or video files was updated, otherwise the list of sent stickers is updated

    #[serde(default)]
    is_attached: bool,
    /// The new list of file identifiers of recently used stickers

    #[serde(default)]
    sticker_ids: Vec<i32>,
}

impl RObject for UpdateRecentStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateRecentStickers {}

impl UpdateRecentStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateRecentStickersBuilder {
        let mut inner = UpdateRecentStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateRecentStickersBuilder { inner }
    }

    pub fn is_attached(&self) -> bool {
        self.is_attached
    }

    pub fn sticker_ids(&self) -> &Vec<i32> {
        &self.sticker_ids
    }
}

#[doc(hidden)]
pub struct UpdateRecentStickersBuilder {
    inner: UpdateRecentStickers,
}

#[deprecated]
pub type RTDUpdateRecentStickersBuilder = UpdateRecentStickersBuilder;

impl UpdateRecentStickersBuilder {
    pub fn build(&self) -> UpdateRecentStickers {
        self.inner.clone()
    }

    pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
        self.inner.is_attached = is_attached;
        self
    }

    pub fn sticker_ids(&mut self, sticker_ids: Vec<i32>) -> &mut Self {
        self.inner.sticker_ids = sticker_ids;
        self
    }
}

impl AsRef<UpdateRecentStickers> for UpdateRecentStickers {
    fn as_ref(&self) -> &UpdateRecentStickers {
        self
    }
}

impl AsRef<UpdateRecentStickers> for UpdateRecentStickersBuilder {
    fn as_ref(&self) -> &UpdateRecentStickers {
        &self.inner
    }
}

/// The list of saved animations was updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSavedAnimations {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new list of file identifiers of saved animations

    #[serde(default)]
    animation_ids: Vec<i32>,
}

impl RObject for UpdateSavedAnimations {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateSavedAnimations {}

impl UpdateSavedAnimations {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateSavedAnimationsBuilder {
        let mut inner = UpdateSavedAnimations::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateSavedAnimationsBuilder { inner }
    }

    pub fn animation_ids(&self) -> &Vec<i32> {
        &self.animation_ids
    }
}

#[doc(hidden)]
pub struct UpdateSavedAnimationsBuilder {
    inner: UpdateSavedAnimations,
}

#[deprecated]
pub type RTDUpdateSavedAnimationsBuilder = UpdateSavedAnimationsBuilder;

impl UpdateSavedAnimationsBuilder {
    pub fn build(&self) -> UpdateSavedAnimations {
        self.inner.clone()
    }

    pub fn animation_ids(&mut self, animation_ids: Vec<i32>) -> &mut Self {
        self.inner.animation_ids = animation_ids;
        self
    }
}

impl AsRef<UpdateSavedAnimations> for UpdateSavedAnimations {
    fn as_ref(&self) -> &UpdateSavedAnimations {
        self
    }
}

impl AsRef<UpdateSavedAnimations> for UpdateSavedAnimationsBuilder {
    fn as_ref(&self) -> &UpdateSavedAnimations {
        &self.inner
    }
}

/// Notification settings for some type of chats were updated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateScopeNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Types of chats for which notification settings were updated

    #[serde(skip_serializing_if = "NotificationSettingsScope::_is_default")]
    scope: NotificationSettingsScope,
    /// The new notification settings
    notification_settings: ScopeNotificationSettings,
}

impl RObject for UpdateScopeNotificationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateScopeNotificationSettings {}

impl UpdateScopeNotificationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateScopeNotificationSettingsBuilder {
        let mut inner = UpdateScopeNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateScopeNotificationSettingsBuilder { inner }
    }

    pub fn scope(&self) -> &NotificationSettingsScope {
        &self.scope
    }

    pub fn notification_settings(&self) -> &ScopeNotificationSettings {
        &self.notification_settings
    }
}

#[doc(hidden)]
pub struct UpdateScopeNotificationSettingsBuilder {
    inner: UpdateScopeNotificationSettings,
}

#[deprecated]
pub type RTDUpdateScopeNotificationSettingsBuilder = UpdateScopeNotificationSettingsBuilder;

impl UpdateScopeNotificationSettingsBuilder {
    pub fn build(&self) -> UpdateScopeNotificationSettings {
        self.inner.clone()
    }

    pub fn scope<T: AsRef<NotificationSettingsScope>>(&mut self, scope: T) -> &mut Self {
        self.inner.scope = scope.as_ref().clone();
        self
    }

    pub fn notification_settings<T: AsRef<ScopeNotificationSettings>>(
        &mut self,
        notification_settings: T,
    ) -> &mut Self {
        self.inner.notification_settings = notification_settings.as_ref().clone();
        self
    }
}

impl AsRef<UpdateScopeNotificationSettings> for UpdateScopeNotificationSettings {
    fn as_ref(&self) -> &UpdateScopeNotificationSettings {
        self
    }
}

impl AsRef<UpdateScopeNotificationSettings> for UpdateScopeNotificationSettingsBuilder {
    fn as_ref(&self) -> &UpdateScopeNotificationSettings {
        &self.inner
    }
}

/// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSecretChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New data about the secret chat
    secret_chat: SecretChat,
}

impl RObject for UpdateSecretChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateSecretChat {}

impl UpdateSecretChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateSecretChatBuilder {
        let mut inner = UpdateSecretChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateSecretChatBuilder { inner }
    }

    pub fn secret_chat(&self) -> &SecretChat {
        &self.secret_chat
    }
}

#[doc(hidden)]
pub struct UpdateSecretChatBuilder {
    inner: UpdateSecretChat,
}

#[deprecated]
pub type RTDUpdateSecretChatBuilder = UpdateSecretChatBuilder;

impl UpdateSecretChatBuilder {
    pub fn build(&self) -> UpdateSecretChat {
        self.inner.clone()
    }

    pub fn secret_chat<T: AsRef<SecretChat>>(&mut self, secret_chat: T) -> &mut Self {
        self.inner.secret_chat = secret_chat.as_ref().clone();
        self
    }
}

impl AsRef<UpdateSecretChat> for UpdateSecretChat {
    fn as_ref(&self) -> &UpdateSecretChat {
        self
    }
}

impl AsRef<UpdateSecretChat> for UpdateSecretChatBuilder {
    fn as_ref(&self) -> &UpdateSecretChat {
        &self.inner
    }
}

/// The selected background has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSelectedBackground {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if background for dark theme has changed

    #[serde(default)]
    for_dark_theme: bool,
    /// The new selected background; may be null
    background: Option<Background>,
}

impl RObject for UpdateSelectedBackground {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateSelectedBackground {}

impl UpdateSelectedBackground {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateSelectedBackgroundBuilder {
        let mut inner = UpdateSelectedBackground::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateSelectedBackgroundBuilder { inner }
    }

    pub fn for_dark_theme(&self) -> bool {
        self.for_dark_theme
    }

    pub fn background(&self) -> &Option<Background> {
        &self.background
    }
}

#[doc(hidden)]
pub struct UpdateSelectedBackgroundBuilder {
    inner: UpdateSelectedBackground,
}

#[deprecated]
pub type RTDUpdateSelectedBackgroundBuilder = UpdateSelectedBackgroundBuilder;

impl UpdateSelectedBackgroundBuilder {
    pub fn build(&self) -> UpdateSelectedBackground {
        self.inner.clone()
    }

    pub fn for_dark_theme(&mut self, for_dark_theme: bool) -> &mut Self {
        self.inner.for_dark_theme = for_dark_theme;
        self
    }

    pub fn background<T: AsRef<Background>>(&mut self, background: T) -> &mut Self {
        self.inner.background = Some(background.as_ref().clone());
        self
    }
}

impl AsRef<UpdateSelectedBackground> for UpdateSelectedBackground {
    fn as_ref(&self) -> &UpdateSelectedBackground {
        self
    }
}

impl AsRef<UpdateSelectedBackground> for UpdateSelectedBackgroundBuilder {
    fn as_ref(&self) -> &UpdateSelectedBackground {
        &self.inner
    }
}

/// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateServiceNotification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Notification type. If type begins with "AUTH_KEY_DROP_", then two buttons "Cancel" and "Log out" must be shown under notification; if user presses the second, all local data must be destroyed using Destroy method

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(default)]
    type_: String,
    /// Notification content

    #[serde(skip_serializing_if = "MessageContent::_is_default")]
    content: MessageContent,
}

impl RObject for UpdateServiceNotification {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateServiceNotification {}

impl UpdateServiceNotification {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateServiceNotificationBuilder {
        let mut inner = UpdateServiceNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateServiceNotificationBuilder { inner }
    }

    pub fn type_(&self) -> &String {
        &self.type_
    }

    pub fn content(&self) -> &MessageContent {
        &self.content
    }
}

#[doc(hidden)]
pub struct UpdateServiceNotificationBuilder {
    inner: UpdateServiceNotification,
}

#[deprecated]
pub type RTDUpdateServiceNotificationBuilder = UpdateServiceNotificationBuilder;

impl UpdateServiceNotificationBuilder {
    pub fn build(&self) -> UpdateServiceNotification {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().to_string();
        self
    }

    pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
        self.inner.content = content.as_ref().clone();
        self
    }
}

impl AsRef<UpdateServiceNotification> for UpdateServiceNotification {
    fn as_ref(&self) -> &UpdateServiceNotification {
        self
    }
}

impl AsRef<UpdateServiceNotification> for UpdateServiceNotificationBuilder {
    fn as_ref(&self) -> &UpdateServiceNotification {
        &self.inner
    }
}

/// A sticker set has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStickerSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The sticker set
    sticker_set: StickerSet,
}

impl RObject for UpdateStickerSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateStickerSet {}

impl UpdateStickerSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateStickerSetBuilder {
        let mut inner = UpdateStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateStickerSetBuilder { inner }
    }

    pub fn sticker_set(&self) -> &StickerSet {
        &self.sticker_set
    }
}

#[doc(hidden)]
pub struct UpdateStickerSetBuilder {
    inner: UpdateStickerSet,
}

#[deprecated]
pub type RTDUpdateStickerSetBuilder = UpdateStickerSetBuilder;

impl UpdateStickerSetBuilder {
    pub fn build(&self) -> UpdateStickerSet {
        self.inner.clone()
    }

    pub fn sticker_set<T: AsRef<StickerSet>>(&mut self, sticker_set: T) -> &mut Self {
        self.inner.sticker_set = sticker_set.as_ref().clone();
        self
    }
}

impl AsRef<UpdateStickerSet> for UpdateStickerSet {
    fn as_ref(&self) -> &UpdateStickerSet {
        self
    }
}

impl AsRef<UpdateStickerSet> for UpdateStickerSetBuilder {
    fn as_ref(&self) -> &UpdateStickerSet {
        &self.inner
    }
}

/// The list of suggested to the user actions has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSuggestedActions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Added suggested actions

    #[serde(default)]
    added_actions: Vec<SuggestedAction>,
    /// Removed suggested actions

    #[serde(default)]
    removed_actions: Vec<SuggestedAction>,
}

impl RObject for UpdateSuggestedActions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateSuggestedActions {}

impl UpdateSuggestedActions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateSuggestedActionsBuilder {
        let mut inner = UpdateSuggestedActions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateSuggestedActionsBuilder { inner }
    }

    pub fn added_actions(&self) -> &Vec<SuggestedAction> {
        &self.added_actions
    }

    pub fn removed_actions(&self) -> &Vec<SuggestedAction> {
        &self.removed_actions
    }
}

#[doc(hidden)]
pub struct UpdateSuggestedActionsBuilder {
    inner: UpdateSuggestedActions,
}

#[deprecated]
pub type RTDUpdateSuggestedActionsBuilder = UpdateSuggestedActionsBuilder;

impl UpdateSuggestedActionsBuilder {
    pub fn build(&self) -> UpdateSuggestedActions {
        self.inner.clone()
    }

    pub fn added_actions(&mut self, added_actions: Vec<SuggestedAction>) -> &mut Self {
        self.inner.added_actions = added_actions;
        self
    }

    pub fn removed_actions(&mut self, removed_actions: Vec<SuggestedAction>) -> &mut Self {
        self.inner.removed_actions = removed_actions;
        self
    }
}

impl AsRef<UpdateSuggestedActions> for UpdateSuggestedActions {
    fn as_ref(&self) -> &UpdateSuggestedActions {
        self
    }
}

impl AsRef<UpdateSuggestedActions> for UpdateSuggestedActionsBuilder {
    fn as_ref(&self) -> &UpdateSuggestedActions {
        &self.inner
    }
}

/// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSupergroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New data about the supergroup
    supergroup: Supergroup,
}

impl RObject for UpdateSupergroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateSupergroup {}

impl UpdateSupergroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateSupergroupBuilder {
        let mut inner = UpdateSupergroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateSupergroupBuilder { inner }
    }

    pub fn supergroup(&self) -> &Supergroup {
        &self.supergroup
    }
}

#[doc(hidden)]
pub struct UpdateSupergroupBuilder {
    inner: UpdateSupergroup,
}

#[deprecated]
pub type RTDUpdateSupergroupBuilder = UpdateSupergroupBuilder;

impl UpdateSupergroupBuilder {
    pub fn build(&self) -> UpdateSupergroup {
        self.inner.clone()
    }

    pub fn supergroup<T: AsRef<Supergroup>>(&mut self, supergroup: T) -> &mut Self {
        self.inner.supergroup = supergroup.as_ref().clone();
        self
    }
}

impl AsRef<UpdateSupergroup> for UpdateSupergroup {
    fn as_ref(&self) -> &UpdateSupergroup {
        self
    }
}

impl AsRef<UpdateSupergroup> for UpdateSupergroupBuilder {
    fn as_ref(&self) -> &UpdateSupergroup {
        &self.inner
    }
}

/// Some data in supergroupFullInfo has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSupergroupFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup or channel

    #[serde(default)]
    supergroup_id: i64,
    /// New full information about the supergroup
    supergroup_full_info: SupergroupFullInfo,
}

impl RObject for UpdateSupergroupFullInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateSupergroupFullInfo {}

impl UpdateSupergroupFullInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateSupergroupFullInfoBuilder {
        let mut inner = UpdateSupergroupFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateSupergroupFullInfoBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn supergroup_full_info(&self) -> &SupergroupFullInfo {
        &self.supergroup_full_info
    }
}

#[doc(hidden)]
pub struct UpdateSupergroupFullInfoBuilder {
    inner: UpdateSupergroupFullInfo,
}

#[deprecated]
pub type RTDUpdateSupergroupFullInfoBuilder = UpdateSupergroupFullInfoBuilder;

impl UpdateSupergroupFullInfoBuilder {
    pub fn build(&self) -> UpdateSupergroupFullInfo {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn supergroup_full_info<T: AsRef<SupergroupFullInfo>>(
        &mut self,
        supergroup_full_info: T,
    ) -> &mut Self {
        self.inner.supergroup_full_info = supergroup_full_info.as_ref().clone();
        self
    }
}

impl AsRef<UpdateSupergroupFullInfo> for UpdateSupergroupFullInfo {
    fn as_ref(&self) -> &UpdateSupergroupFullInfo {
        self
    }
}

impl AsRef<UpdateSupergroupFullInfo> for UpdateSupergroupFullInfoBuilder {
    fn as_ref(&self) -> &UpdateSupergroupFullInfo {
        &self.inner
    }
}

/// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTermsOfService {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the terms of service

    #[serde(default)]
    terms_of_service_id: String,
    /// The new terms of service
    terms_of_service: TermsOfService,
}

impl RObject for UpdateTermsOfService {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateTermsOfService {}

impl UpdateTermsOfService {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateTermsOfServiceBuilder {
        let mut inner = UpdateTermsOfService::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateTermsOfServiceBuilder { inner }
    }

    pub fn terms_of_service_id(&self) -> &String {
        &self.terms_of_service_id
    }

    pub fn terms_of_service(&self) -> &TermsOfService {
        &self.terms_of_service
    }
}

#[doc(hidden)]
pub struct UpdateTermsOfServiceBuilder {
    inner: UpdateTermsOfService,
}

#[deprecated]
pub type RTDUpdateTermsOfServiceBuilder = UpdateTermsOfServiceBuilder;

impl UpdateTermsOfServiceBuilder {
    pub fn build(&self) -> UpdateTermsOfService {
        self.inner.clone()
    }

    pub fn terms_of_service_id<T: AsRef<str>>(&mut self, terms_of_service_id: T) -> &mut Self {
        self.inner.terms_of_service_id = terms_of_service_id.as_ref().to_string();
        self
    }

    pub fn terms_of_service<T: AsRef<TermsOfService>>(&mut self, terms_of_service: T) -> &mut Self {
        self.inner.terms_of_service = terms_of_service.as_ref().clone();
        self
    }
}

impl AsRef<UpdateTermsOfService> for UpdateTermsOfService {
    fn as_ref(&self) -> &UpdateTermsOfService {
        self
    }
}

impl AsRef<UpdateTermsOfService> for UpdateTermsOfServiceBuilder {
    fn as_ref(&self) -> &UpdateTermsOfService {
        &self.inner
    }
}

/// The list of trending sticker sets was updated or some of them were viewed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTrendingStickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The prefix of the list of trending sticker sets with the newest trending sticker sets
    sticker_sets: StickerSets,
}

impl RObject for UpdateTrendingStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateTrendingStickerSets {}

impl UpdateTrendingStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateTrendingStickerSetsBuilder {
        let mut inner = UpdateTrendingStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateTrendingStickerSetsBuilder { inner }
    }

    pub fn sticker_sets(&self) -> &StickerSets {
        &self.sticker_sets
    }
}

#[doc(hidden)]
pub struct UpdateTrendingStickerSetsBuilder {
    inner: UpdateTrendingStickerSets,
}

#[deprecated]
pub type RTDUpdateTrendingStickerSetsBuilder = UpdateTrendingStickerSetsBuilder;

impl UpdateTrendingStickerSetsBuilder {
    pub fn build(&self) -> UpdateTrendingStickerSets {
        self.inner.clone()
    }

    pub fn sticker_sets<T: AsRef<StickerSets>>(&mut self, sticker_sets: T) -> &mut Self {
        self.inner.sticker_sets = sticker_sets.as_ref().clone();
        self
    }
}

impl AsRef<UpdateTrendingStickerSets> for UpdateTrendingStickerSets {
    fn as_ref(&self) -> &UpdateTrendingStickerSets {
        self
    }
}

impl AsRef<UpdateTrendingStickerSets> for UpdateTrendingStickerSetsBuilder {
    fn as_ref(&self) -> &UpdateTrendingStickerSets {
        &self.inner
    }
}

/// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUnreadChatCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat list with changed number of unread messages

    #[serde(skip_serializing_if = "ChatList::_is_default")]
    chat_list: ChatList,
    /// Approximate total number of chats in the chat list

    #[serde(default)]
    total_count: i32,
    /// Total number of unread chats

    #[serde(default)]
    unread_count: i32,
    /// Total number of unread unmuted chats

    #[serde(default)]
    unread_unmuted_count: i32,
    /// Total number of chats marked as unread

    #[serde(default)]
    marked_as_unread_count: i32,
    /// Total number of unmuted chats marked as unread

    #[serde(default)]
    marked_as_unread_unmuted_count: i32,
}

impl RObject for UpdateUnreadChatCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateUnreadChatCount {}

impl UpdateUnreadChatCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateUnreadChatCountBuilder {
        let mut inner = UpdateUnreadChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateUnreadChatCountBuilder { inner }
    }

    pub fn chat_list(&self) -> &ChatList {
        &self.chat_list
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn unread_count(&self) -> i32 {
        self.unread_count
    }

    pub fn unread_unmuted_count(&self) -> i32 {
        self.unread_unmuted_count
    }

    pub fn marked_as_unread_count(&self) -> i32 {
        self.marked_as_unread_count
    }

    pub fn marked_as_unread_unmuted_count(&self) -> i32 {
        self.marked_as_unread_unmuted_count
    }
}

#[doc(hidden)]
pub struct UpdateUnreadChatCountBuilder {
    inner: UpdateUnreadChatCount,
}

#[deprecated]
pub type RTDUpdateUnreadChatCountBuilder = UpdateUnreadChatCountBuilder;

impl UpdateUnreadChatCountBuilder {
    pub fn build(&self) -> UpdateUnreadChatCount {
        self.inner.clone()
    }

    pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
        self.inner.chat_list = chat_list.as_ref().clone();
        self
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn unread_count(&mut self, unread_count: i32) -> &mut Self {
        self.inner.unread_count = unread_count;
        self
    }

    pub fn unread_unmuted_count(&mut self, unread_unmuted_count: i32) -> &mut Self {
        self.inner.unread_unmuted_count = unread_unmuted_count;
        self
    }

    pub fn marked_as_unread_count(&mut self, marked_as_unread_count: i32) -> &mut Self {
        self.inner.marked_as_unread_count = marked_as_unread_count;
        self
    }

    pub fn marked_as_unread_unmuted_count(
        &mut self,
        marked_as_unread_unmuted_count: i32,
    ) -> &mut Self {
        self.inner.marked_as_unread_unmuted_count = marked_as_unread_unmuted_count;
        self
    }
}

impl AsRef<UpdateUnreadChatCount> for UpdateUnreadChatCount {
    fn as_ref(&self) -> &UpdateUnreadChatCount {
        self
    }
}

impl AsRef<UpdateUnreadChatCount> for UpdateUnreadChatCountBuilder {
    fn as_ref(&self) -> &UpdateUnreadChatCount {
        &self.inner
    }
}

/// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUnreadMessageCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat list with changed number of unread messages

    #[serde(skip_serializing_if = "ChatList::_is_default")]
    chat_list: ChatList,
    /// Total number of unread messages

    #[serde(default)]
    unread_count: i32,
    /// Total number of unread messages in unmuted chats

    #[serde(default)]
    unread_unmuted_count: i32,
}

impl RObject for UpdateUnreadMessageCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateUnreadMessageCount {}

impl UpdateUnreadMessageCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateUnreadMessageCountBuilder {
        let mut inner = UpdateUnreadMessageCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateUnreadMessageCountBuilder { inner }
    }

    pub fn chat_list(&self) -> &ChatList {
        &self.chat_list
    }

    pub fn unread_count(&self) -> i32 {
        self.unread_count
    }

    pub fn unread_unmuted_count(&self) -> i32 {
        self.unread_unmuted_count
    }
}

#[doc(hidden)]
pub struct UpdateUnreadMessageCountBuilder {
    inner: UpdateUnreadMessageCount,
}

#[deprecated]
pub type RTDUpdateUnreadMessageCountBuilder = UpdateUnreadMessageCountBuilder;

impl UpdateUnreadMessageCountBuilder {
    pub fn build(&self) -> UpdateUnreadMessageCount {
        self.inner.clone()
    }

    pub fn chat_list<T: AsRef<ChatList>>(&mut self, chat_list: T) -> &mut Self {
        self.inner.chat_list = chat_list.as_ref().clone();
        self
    }

    pub fn unread_count(&mut self, unread_count: i32) -> &mut Self {
        self.inner.unread_count = unread_count;
        self
    }

    pub fn unread_unmuted_count(&mut self, unread_unmuted_count: i32) -> &mut Self {
        self.inner.unread_unmuted_count = unread_unmuted_count;
        self
    }
}

impl AsRef<UpdateUnreadMessageCount> for UpdateUnreadMessageCount {
    fn as_ref(&self) -> &UpdateUnreadMessageCount {
        self
    }
}

impl AsRef<UpdateUnreadMessageCount> for UpdateUnreadMessageCountBuilder {
    fn as_ref(&self) -> &UpdateUnreadMessageCount {
        &self.inner
    }
}

/// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New data about the user
    user: User,
}

impl RObject for UpdateUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateUser {}

impl UpdateUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateUserBuilder {
        let mut inner = UpdateUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateUserBuilder { inner }
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}

#[doc(hidden)]
pub struct UpdateUserBuilder {
    inner: UpdateUser,
}

#[deprecated]
pub type RTDUpdateUserBuilder = UpdateUserBuilder;

impl UpdateUserBuilder {
    pub fn build(&self) -> UpdateUser {
        self.inner.clone()
    }

    pub fn user<T: AsRef<User>>(&mut self, user: T) -> &mut Self {
        self.inner.user = user.as_ref().clone();
        self
    }
}

impl AsRef<UpdateUser> for UpdateUser {
    fn as_ref(&self) -> &UpdateUser {
        self
    }
}

impl AsRef<UpdateUser> for UpdateUserBuilder {
    fn as_ref(&self) -> &UpdateUser {
        &self.inner
    }
}

/// Some data in userFullInfo has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,
    /// New full information about the user
    user_full_info: UserFullInfo,
}

impl RObject for UpdateUserFullInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateUserFullInfo {}

impl UpdateUserFullInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateUserFullInfoBuilder {
        let mut inner = UpdateUserFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateUserFullInfoBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn user_full_info(&self) -> &UserFullInfo {
        &self.user_full_info
    }
}

#[doc(hidden)]
pub struct UpdateUserFullInfoBuilder {
    inner: UpdateUserFullInfo,
}

#[deprecated]
pub type RTDUpdateUserFullInfoBuilder = UpdateUserFullInfoBuilder;

impl UpdateUserFullInfoBuilder {
    pub fn build(&self) -> UpdateUserFullInfo {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn user_full_info<T: AsRef<UserFullInfo>>(&mut self, user_full_info: T) -> &mut Self {
        self.inner.user_full_info = user_full_info.as_ref().clone();
        self
    }
}

impl AsRef<UpdateUserFullInfo> for UpdateUserFullInfo {
    fn as_ref(&self) -> &UpdateUserFullInfo {
        self
    }
}

impl AsRef<UpdateUserFullInfo> for UpdateUserFullInfoBuilder {
    fn as_ref(&self) -> &UpdateUserFullInfo {
        &self.inner
    }
}

/// Some privacy setting rules have been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserPrivacySettingRules {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The privacy setting

    #[serde(skip_serializing_if = "UserPrivacySetting::_is_default")]
    setting: UserPrivacySetting,
    /// New privacy rules
    rules: UserPrivacySettingRules,
}

impl RObject for UpdateUserPrivacySettingRules {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateUserPrivacySettingRules {}

impl UpdateUserPrivacySettingRules {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateUserPrivacySettingRulesBuilder {
        let mut inner = UpdateUserPrivacySettingRules::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateUserPrivacySettingRulesBuilder { inner }
    }

    pub fn setting(&self) -> &UserPrivacySetting {
        &self.setting
    }

    pub fn rules(&self) -> &UserPrivacySettingRules {
        &self.rules
    }
}

#[doc(hidden)]
pub struct UpdateUserPrivacySettingRulesBuilder {
    inner: UpdateUserPrivacySettingRules,
}

#[deprecated]
pub type RTDUpdateUserPrivacySettingRulesBuilder = UpdateUserPrivacySettingRulesBuilder;

impl UpdateUserPrivacySettingRulesBuilder {
    pub fn build(&self) -> UpdateUserPrivacySettingRules {
        self.inner.clone()
    }

    pub fn setting<T: AsRef<UserPrivacySetting>>(&mut self, setting: T) -> &mut Self {
        self.inner.setting = setting.as_ref().clone();
        self
    }

    pub fn rules<T: AsRef<UserPrivacySettingRules>>(&mut self, rules: T) -> &mut Self {
        self.inner.rules = rules.as_ref().clone();
        self
    }
}

impl AsRef<UpdateUserPrivacySettingRules> for UpdateUserPrivacySettingRules {
    fn as_ref(&self) -> &UpdateUserPrivacySettingRules {
        self
    }
}

impl AsRef<UpdateUserPrivacySettingRules> for UpdateUserPrivacySettingRulesBuilder {
    fn as_ref(&self) -> &UpdateUserPrivacySettingRules {
        &self.inner
    }
}

/// The user went online or offline
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,
    /// New status of the user

    #[serde(skip_serializing_if = "UserStatus::_is_default")]
    status: UserStatus,
}

impl RObject for UpdateUserStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateUserStatus {}

impl UpdateUserStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateUserStatusBuilder {
        let mut inner = UpdateUserStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateUserStatusBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }
}

#[doc(hidden)]
pub struct UpdateUserStatusBuilder {
    inner: UpdateUserStatus,
}

#[deprecated]
pub type RTDUpdateUserStatusBuilder = UpdateUserStatusBuilder;

impl UpdateUserStatusBuilder {
    pub fn build(&self) -> UpdateUserStatus {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn status<T: AsRef<UserStatus>>(&mut self, status: T) -> &mut Self {
        self.inner.status = status.as_ref().clone();
        self
    }
}

impl AsRef<UpdateUserStatus> for UpdateUserStatus {
    fn as_ref(&self) -> &UpdateUserStatus {
        self
    }
}

impl AsRef<UpdateUserStatus> for UpdateUserStatusBuilder {
    fn as_ref(&self) -> &UpdateUserStatus {
        &self.inner
    }
}

/// The list of users nearby has changed. The update is guaranteed to be sent only 60 seconds after a successful searchChatsNearby request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUsersNearby {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new list of users nearby

    #[serde(default)]
    users_nearby: Vec<ChatNearby>,
}

impl RObject for UpdateUsersNearby {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateUsersNearby {}

impl UpdateUsersNearby {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UpdateUsersNearbyBuilder {
        let mut inner = UpdateUsersNearby::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UpdateUsersNearbyBuilder { inner }
    }

    pub fn users_nearby(&self) -> &Vec<ChatNearby> {
        &self.users_nearby
    }
}

#[doc(hidden)]
pub struct UpdateUsersNearbyBuilder {
    inner: UpdateUsersNearby,
}

#[deprecated]
pub type RTDUpdateUsersNearbyBuilder = UpdateUsersNearbyBuilder;

impl UpdateUsersNearbyBuilder {
    pub fn build(&self) -> UpdateUsersNearby {
        self.inner.clone()
    }

    pub fn users_nearby(&mut self, users_nearby: Vec<ChatNearby>) -> &mut Self {
        self.inner.users_nearby = users_nearby;
        self
    }
}

impl AsRef<UpdateUsersNearby> for UpdateUsersNearby {
    fn as_ref(&self) -> &UpdateUsersNearby {
        self
    }
}

impl AsRef<UpdateUsersNearby> for UpdateUsersNearbyBuilder {
    fn as_ref(&self) -> &UpdateUsersNearby {
        &self.inner
    }
}
