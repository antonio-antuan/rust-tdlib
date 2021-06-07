use crate::errors::*;
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
    #[serde(rename(serialize = "testUseUpdate", deserialize = "testUseUpdate"))]
    TestUseUpdate(TestUseUpdate),
    /// Contains active notifications that was shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
    #[serde(rename(
        serialize = "updateActiveNotifications",
        deserialize = "updateActiveNotifications"
    ))]
    ActiveNotifications(UpdateActiveNotifications),
    /// The parameters of animation search through GetOption("animation_search_bot_username") bot has changed
    #[serde(rename(
        serialize = "updateAnimationSearchParameters",
        deserialize = "updateAnimationSearchParameters"
    ))]
    AnimationSearchParameters(UpdateAnimationSearchParameters),
    /// The user authorization state has changed
    #[serde(rename(
        serialize = "updateAuthorizationState",
        deserialize = "updateAuthorizationState"
    ))]
    AuthorizationState(UpdateAuthorizationState),
    /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
    #[serde(rename(serialize = "updateBasicGroup", deserialize = "updateBasicGroup"))]
    BasicGroup(UpdateBasicGroup),
    /// Some data from basicGroupFullInfo has been changed
    #[serde(rename(
        serialize = "updateBasicGroupFullInfo",
        deserialize = "updateBasicGroupFullInfo"
    ))]
    BasicGroupFullInfo(UpdateBasicGroupFullInfo),
    /// New call was created or information about a call was updated
    #[serde(rename(serialize = "updateCall", deserialize = "updateCall"))]
    Call(UpdateCall),
    /// The chat action bar was changed
    #[serde(rename(serialize = "updateChatActionBar", deserialize = "updateChatActionBar"))]
    ChatActionBar(UpdateChatActionBar),
    /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
    #[serde(rename(
        serialize = "updateChatDefaultDisableNotification",
        deserialize = "updateChatDefaultDisableNotification"
    ))]
    ChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
    /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied
    #[serde(rename(
        serialize = "updateChatDraftMessage",
        deserialize = "updateChatDraftMessage"
    ))]
    ChatDraftMessage(UpdateChatDraftMessage),
    /// The list of chat filters or a chat filter has changed
    #[serde(rename(serialize = "updateChatFilters", deserialize = "updateChatFilters"))]
    ChatFilters(UpdateChatFilters),
    /// A chat's has_scheduled_messages field has changed
    #[serde(rename(
        serialize = "updateChatHasScheduledMessages",
        deserialize = "updateChatHasScheduledMessages"
    ))]
    ChatHasScheduledMessages(UpdateChatHasScheduledMessages),
    /// A chat was blocked or unblocked
    #[serde(rename(serialize = "updateChatIsBlocked", deserialize = "updateChatIsBlocked"))]
    ChatIsBlocked(UpdateChatIsBlocked),
    /// A chat was marked as unread or was read
    #[serde(rename(
        serialize = "updateChatIsMarkedAsUnread",
        deserialize = "updateChatIsMarkedAsUnread"
    ))]
    ChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
    /// The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
    #[serde(rename(
        serialize = "updateChatLastMessage",
        deserialize = "updateChatLastMessage"
    ))]
    ChatLastMessage(UpdateChatLastMessage),
    /// Notification settings for a chat were changed
    #[serde(rename(
        serialize = "updateChatNotificationSettings",
        deserialize = "updateChatNotificationSettings"
    ))]
    ChatNotificationSettings(UpdateChatNotificationSettings),
    /// The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed
    #[serde(rename(
        serialize = "updateChatOnlineMemberCount",
        deserialize = "updateChatOnlineMemberCount"
    ))]
    ChatOnlineMemberCount(UpdateChatOnlineMemberCount),
    /// Chat permissions was changed
    #[serde(rename(
        serialize = "updateChatPermissions",
        deserialize = "updateChatPermissions"
    ))]
    ChatPermissions(UpdateChatPermissions),
    /// A chat photo was changed
    #[serde(rename(serialize = "updateChatPhoto", deserialize = "updateChatPhoto"))]
    ChatPhoto(UpdateChatPhoto),
    /// The position of a chat in a chat list has changed. Instead of this update updateChatLastMessage or updateChatDraftMessage might be sent
    #[serde(rename(serialize = "updateChatPosition", deserialize = "updateChatPosition"))]
    ChatPosition(UpdateChatPosition),
    /// Incoming messages were read or number of unread messages has been changed
    #[serde(rename(serialize = "updateChatReadInbox", deserialize = "updateChatReadInbox"))]
    ChatReadInbox(UpdateChatReadInbox),
    /// Outgoing messages were read
    #[serde(rename(
        serialize = "updateChatReadOutbox",
        deserialize = "updateChatReadOutbox"
    ))]
    ChatReadOutbox(UpdateChatReadOutbox),
    /// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
    #[serde(rename(
        serialize = "updateChatReplyMarkup",
        deserialize = "updateChatReplyMarkup"
    ))]
    ChatReplyMarkup(UpdateChatReplyMarkup),
    /// The title of a chat was changed
    #[serde(rename(serialize = "updateChatTitle", deserialize = "updateChatTitle"))]
    ChatTitle(UpdateChatTitle),
    /// The chat unread_mention_count has changed
    #[serde(rename(
        serialize = "updateChatUnreadMentionCount",
        deserialize = "updateChatUnreadMentionCount"
    ))]
    ChatUnreadMentionCount(UpdateChatUnreadMentionCount),
    /// The connection state has changed. This update must be used only to show a human-readable description of the connection state
    #[serde(rename(
        serialize = "updateConnectionState",
        deserialize = "updateConnectionState"
    ))]
    ConnectionState(UpdateConnectionState),
    /// Some messages were deleted
    #[serde(rename(
        serialize = "updateDeleteMessages",
        deserialize = "updateDeleteMessages"
    ))]
    DeleteMessages(UpdateDeleteMessages),
    /// The list of supported dice emojis has changed
    #[serde(rename(serialize = "updateDiceEmojis", deserialize = "updateDiceEmojis"))]
    DiceEmojis(UpdateDiceEmojis),
    /// The list of favorite stickers was updated
    #[serde(rename(
        serialize = "updateFavoriteStickers",
        deserialize = "updateFavoriteStickers"
    ))]
    FavoriteStickers(UpdateFavoriteStickers),
    /// Information about a file was updated
    #[serde(rename(serialize = "updateFile", deserialize = "updateFile"))]
    File(UpdateFile),
    /// The file generation process needs to be started by the application
    #[serde(rename(
        serialize = "updateFileGenerationStart",
        deserialize = "updateFileGenerationStart"
    ))]
    FileGenerationStart(UpdateFileGenerationStart),
    /// File generation is no longer needed
    #[serde(rename(
        serialize = "updateFileGenerationStop",
        deserialize = "updateFileGenerationStop"
    ))]
    FileGenerationStop(UpdateFileGenerationStop),
    /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
    #[serde(rename(
        serialize = "updateHavePendingNotifications",
        deserialize = "updateHavePendingNotifications"
    ))]
    HavePendingNotifications(UpdateHavePendingNotifications),
    /// The list of installed sticker sets was updated
    #[serde(rename(
        serialize = "updateInstalledStickerSets",
        deserialize = "updateInstalledStickerSets"
    ))]
    InstalledStickerSets(UpdateInstalledStickerSets),
    /// Some language pack strings have been updated
    #[serde(rename(
        serialize = "updateLanguagePackStrings",
        deserialize = "updateLanguagePackStrings"
    ))]
    LanguagePackStrings(UpdateLanguagePackStrings),
    /// The message content has changed
    #[serde(rename(
        serialize = "updateMessageContent",
        deserialize = "updateMessageContent"
    ))]
    MessageContent(UpdateMessageContent),
    /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
    #[serde(rename(
        serialize = "updateMessageContentOpened",
        deserialize = "updateMessageContentOpened"
    ))]
    MessageContentOpened(UpdateMessageContentOpened),
    /// A message was edited. Changes in the message content will come in a separate updateMessageContent
    #[serde(rename(serialize = "updateMessageEdited", deserialize = "updateMessageEdited"))]
    MessageEdited(UpdateMessageEdited),
    /// The information about interactions with a message has changed
    #[serde(rename(
        serialize = "updateMessageInteractionInfo",
        deserialize = "updateMessageInteractionInfo"
    ))]
    MessageInteractionInfo(UpdateMessageInteractionInfo),
    /// The message pinned state was changed
    #[serde(rename(
        serialize = "updateMessageIsPinned",
        deserialize = "updateMessageIsPinned"
    ))]
    MessageIsPinned(UpdateMessageIsPinned),
    /// A message with a live location was viewed. When the update is received, the application is supposed to update the live location
    #[serde(rename(
        serialize = "updateMessageLiveLocationViewed",
        deserialize = "updateMessageLiveLocationViewed"
    ))]
    MessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
    /// A message with an unread mention was read
    #[serde(rename(
        serialize = "updateMessageMentionRead",
        deserialize = "updateMessageMentionRead"
    ))]
    MessageMentionRead(UpdateMessageMentionRead),
    /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
    #[serde(rename(
        serialize = "updateMessageSendAcknowledged",
        deserialize = "updateMessageSendAcknowledged"
    ))]
    MessageSendAcknowledged(UpdateMessageSendAcknowledged),
    /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
    #[serde(rename(
        serialize = "updateMessageSendFailed",
        deserialize = "updateMessageSendFailed"
    ))]
    MessageSendFailed(UpdateMessageSendFailed),
    /// A message has been successfully sent
    #[serde(rename(
        serialize = "updateMessageSendSucceeded",
        deserialize = "updateMessageSendSucceeded"
    ))]
    MessageSendSucceeded(UpdateMessageSendSucceeded),
    /// New call signaling data arrived
    #[serde(rename(
        serialize = "updateNewCallSignalingData",
        deserialize = "updateNewCallSignalingData"
    ))]
    NewCallSignalingData(UpdateNewCallSignalingData),
    /// A new incoming callback query; for bots only
    #[serde(rename(
        serialize = "updateNewCallbackQuery",
        deserialize = "updateNewCallbackQuery"
    ))]
    NewCallbackQuery(UpdateNewCallbackQuery),
    /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
    #[serde(rename(serialize = "updateNewChat", deserialize = "updateNewChat"))]
    NewChat(UpdateNewChat),
    /// The user has chosen a result of an inline query; for bots only
    #[serde(rename(
        serialize = "updateNewChosenInlineResult",
        deserialize = "updateNewChosenInlineResult"
    ))]
    NewChosenInlineResult(UpdateNewChosenInlineResult),
    /// A new incoming event; for bots only
    #[serde(rename(
        serialize = "updateNewCustomEvent",
        deserialize = "updateNewCustomEvent"
    ))]
    NewCustomEvent(UpdateNewCustomEvent),
    /// A new incoming query; for bots only
    #[serde(rename(
        serialize = "updateNewCustomQuery",
        deserialize = "updateNewCustomQuery"
    ))]
    NewCustomQuery(UpdateNewCustomQuery),
    /// A new incoming callback query from a message sent via a bot; for bots only
    #[serde(rename(
        serialize = "updateNewInlineCallbackQuery",
        deserialize = "updateNewInlineCallbackQuery"
    ))]
    NewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
    /// A new incoming inline query; for bots only
    #[serde(rename(
        serialize = "updateNewInlineQuery",
        deserialize = "updateNewInlineQuery"
    ))]
    NewInlineQuery(UpdateNewInlineQuery),
    /// A new message was received; can also be an outgoing message
    #[serde(rename(serialize = "updateNewMessage", deserialize = "updateNewMessage"))]
    NewMessage(UpdateNewMessage),
    /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
    #[serde(rename(
        serialize = "updateNewPreCheckoutQuery",
        deserialize = "updateNewPreCheckoutQuery"
    ))]
    NewPreCheckoutQuery(UpdateNewPreCheckoutQuery),
    /// A new incoming shipping query; for bots only. Only for invoices with flexible price
    #[serde(rename(
        serialize = "updateNewShippingQuery",
        deserialize = "updateNewShippingQuery"
    ))]
    NewShippingQuery(UpdateNewShippingQuery),
    /// A notification was changed
    #[serde(rename(serialize = "updateNotification", deserialize = "updateNotification"))]
    Notification(UpdateNotification),
    /// A list of active notifications in a notification group has changed
    #[serde(rename(
        serialize = "updateNotificationGroup",
        deserialize = "updateNotificationGroup"
    ))]
    NotificationGroup(UpdateNotificationGroup),
    /// An option changed its value
    #[serde(rename(serialize = "updateOption", deserialize = "updateOption"))]
    Option(UpdateOption),
    /// A poll was updated; for bots only
    #[serde(rename(serialize = "updatePoll", deserialize = "updatePoll"))]
    Poll(UpdatePoll),
    /// A user changed the answer to a poll; for bots only
    #[serde(rename(serialize = "updatePollAnswer", deserialize = "updatePollAnswer"))]
    PollAnswer(UpdatePollAnswer),
    /// The list of recently used stickers was updated
    #[serde(rename(
        serialize = "updateRecentStickers",
        deserialize = "updateRecentStickers"
    ))]
    RecentStickers(UpdateRecentStickers),
    /// The list of saved animations was updated
    #[serde(rename(
        serialize = "updateSavedAnimations",
        deserialize = "updateSavedAnimations"
    ))]
    SavedAnimations(UpdateSavedAnimations),
    /// Notification settings for some type of chats were updated
    #[serde(rename(
        serialize = "updateScopeNotificationSettings",
        deserialize = "updateScopeNotificationSettings"
    ))]
    ScopeNotificationSettings(UpdateScopeNotificationSettings),
    /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
    #[serde(rename(serialize = "updateSecretChat", deserialize = "updateSecretChat"))]
    SecretChat(UpdateSecretChat),
    /// The selected background has changed
    #[serde(rename(
        serialize = "updateSelectedBackground",
        deserialize = "updateSelectedBackground"
    ))]
    SelectedBackground(UpdateSelectedBackground),
    /// Service notification from the server. Upon receiving this the application must show a popup with the content of the notification
    #[serde(rename(
        serialize = "updateServiceNotification",
        deserialize = "updateServiceNotification"
    ))]
    ServiceNotification(UpdateServiceNotification),
    /// A sticker set has changed
    #[serde(rename(serialize = "updateStickerSet", deserialize = "updateStickerSet"))]
    StickerSet(UpdateStickerSet),
    /// The list of suggested to the user actions has changed
    #[serde(rename(
        serialize = "updateSuggestedActions",
        deserialize = "updateSuggestedActions"
    ))]
    SuggestedActions(UpdateSuggestedActions),
    /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
    #[serde(rename(serialize = "updateSupergroup", deserialize = "updateSupergroup"))]
    Supergroup(UpdateSupergroup),
    /// Some data from supergroupFullInfo has been changed
    #[serde(rename(
        serialize = "updateSupergroupFullInfo",
        deserialize = "updateSupergroupFullInfo"
    ))]
    SupergroupFullInfo(UpdateSupergroupFullInfo),
    /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method should be called with the reason "Decline ToS update"
    #[serde(rename(
        serialize = "updateTermsOfService",
        deserialize = "updateTermsOfService"
    ))]
    TermsOfService(UpdateTermsOfService),
    /// The list of trending sticker sets was updated or some of them were viewed
    #[serde(rename(
        serialize = "updateTrendingStickerSets",
        deserialize = "updateTrendingStickerSets"
    ))]
    TrendingStickerSets(UpdateTrendingStickerSets),
    /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
    #[serde(rename(
        serialize = "updateUnreadChatCount",
        deserialize = "updateUnreadChatCount"
    ))]
    UnreadChatCount(UpdateUnreadChatCount),
    /// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
    #[serde(rename(
        serialize = "updateUnreadMessageCount",
        deserialize = "updateUnreadMessageCount"
    ))]
    UnreadMessageCount(UpdateUnreadMessageCount),
    /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
    #[serde(rename(serialize = "updateUser", deserialize = "updateUser"))]
    User(UpdateUser),
    /// User activity in the chat has changed
    #[serde(rename(
        serialize = "updateUserChatAction",
        deserialize = "updateUserChatAction"
    ))]
    UserChatAction(UpdateUserChatAction),
    /// Some data from userFullInfo has been changed
    #[serde(rename(serialize = "updateUserFullInfo", deserialize = "updateUserFullInfo"))]
    UserFullInfo(UpdateUserFullInfo),
    /// Some privacy setting rules have been changed
    #[serde(rename(
        serialize = "updateUserPrivacySettingRules",
        deserialize = "updateUserPrivacySettingRules"
    ))]
    UserPrivacySettingRules(UpdateUserPrivacySettingRules),
    /// The user went online or offline
    #[serde(rename(serialize = "updateUserStatus", deserialize = "updateUserStatus"))]
    UserStatus(UpdateUserStatus),
    /// The list of users nearby has changed. The update is guaranteed to be sent only 60 seconds after a successful searchChatsNearby request
    #[serde(rename(serialize = "updateUsersNearby", deserialize = "updateUsersNearby"))]
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
            Update::AnimationSearchParameters(t) => t.extra(),
            Update::AuthorizationState(t) => t.extra(),
            Update::BasicGroup(t) => t.extra(),
            Update::BasicGroupFullInfo(t) => t.extra(),
            Update::Call(t) => t.extra(),
            Update::ChatActionBar(t) => t.extra(),
            Update::ChatDefaultDisableNotification(t) => t.extra(),
            Update::ChatDraftMessage(t) => t.extra(),
            Update::ChatFilters(t) => t.extra(),
            Update::ChatHasScheduledMessages(t) => t.extra(),
            Update::ChatIsBlocked(t) => t.extra(),
            Update::ChatIsMarkedAsUnread(t) => t.extra(),
            Update::ChatLastMessage(t) => t.extra(),
            Update::ChatNotificationSettings(t) => t.extra(),
            Update::ChatOnlineMemberCount(t) => t.extra(),
            Update::ChatPermissions(t) => t.extra(),
            Update::ChatPhoto(t) => t.extra(),
            Update::ChatPosition(t) => t.extra(),
            Update::ChatReadInbox(t) => t.extra(),
            Update::ChatReadOutbox(t) => t.extra(),
            Update::ChatReplyMarkup(t) => t.extra(),
            Update::ChatTitle(t) => t.extra(),
            Update::ChatUnreadMentionCount(t) => t.extra(),
            Update::ConnectionState(t) => t.extra(),
            Update::DeleteMessages(t) => t.extra(),
            Update::DiceEmojis(t) => t.extra(),
            Update::FavoriteStickers(t) => t.extra(),
            Update::File(t) => t.extra(),
            Update::FileGenerationStart(t) => t.extra(),
            Update::FileGenerationStop(t) => t.extra(),
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
            Update::UserChatAction(t) => t.extra(),
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
            Update::AnimationSearchParameters(t) => t.client_id(),
            Update::AuthorizationState(t) => t.client_id(),
            Update::BasicGroup(t) => t.client_id(),
            Update::BasicGroupFullInfo(t) => t.client_id(),
            Update::Call(t) => t.client_id(),
            Update::ChatActionBar(t) => t.client_id(),
            Update::ChatDefaultDisableNotification(t) => t.client_id(),
            Update::ChatDraftMessage(t) => t.client_id(),
            Update::ChatFilters(t) => t.client_id(),
            Update::ChatHasScheduledMessages(t) => t.client_id(),
            Update::ChatIsBlocked(t) => t.client_id(),
            Update::ChatIsMarkedAsUnread(t) => t.client_id(),
            Update::ChatLastMessage(t) => t.client_id(),
            Update::ChatNotificationSettings(t) => t.client_id(),
            Update::ChatOnlineMemberCount(t) => t.client_id(),
            Update::ChatPermissions(t) => t.client_id(),
            Update::ChatPhoto(t) => t.client_id(),
            Update::ChatPosition(t) => t.client_id(),
            Update::ChatReadInbox(t) => t.client_id(),
            Update::ChatReadOutbox(t) => t.client_id(),
            Update::ChatReplyMarkup(t) => t.client_id(),
            Update::ChatTitle(t) => t.client_id(),
            Update::ChatUnreadMentionCount(t) => t.client_id(),
            Update::ConnectionState(t) => t.client_id(),
            Update::DeleteMessages(t) => t.client_id(),
            Update::DiceEmojis(t) => t.client_id(),
            Update::FavoriteStickers(t) => t.client_id(),
            Update::File(t) => t.client_id(),
            Update::FileGenerationStart(t) => t.client_id(),
            Update::FileGenerationStop(t) => t.client_id(),
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
            Update::UserChatAction(t) => t.client_id(),
            Update::UserFullInfo(t) => t.client_id(),
            Update::UserPrivacySettingRules(t) => t.client_id(),
            Update::UserStatus(t) => t.client_id(),
            Update::UsersNearby(t) => t.client_id(),

            _ => None,
        }
    }
}

impl Update {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateActiveNotificationsBuilder {
        let mut inner = UpdateActiveNotifications::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateActiveNotificationsBuilder { inner }
    }

    pub fn groups(&self) -> &Vec<NotificationGroup> {
        &self.groups
    }
}

#[doc(hidden)]
pub struct RTDUpdateActiveNotificationsBuilder {
    inner: UpdateActiveNotifications,
}

impl RTDUpdateActiveNotificationsBuilder {
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

impl AsRef<UpdateActiveNotifications> for RTDUpdateActiveNotificationsBuilder {
    fn as_ref(&self) -> &UpdateActiveNotifications {
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
    provider: String,
    /// The new list of emojis suggested for searching
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateAnimationSearchParametersBuilder {
        let mut inner = UpdateAnimationSearchParameters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateAnimationSearchParametersBuilder { inner }
    }

    pub fn provider(&self) -> &String {
        &self.provider
    }

    pub fn emojis(&self) -> &Vec<String> {
        &self.emojis
    }
}

#[doc(hidden)]
pub struct RTDUpdateAnimationSearchParametersBuilder {
    inner: UpdateAnimationSearchParameters,
}

impl RTDUpdateAnimationSearchParametersBuilder {
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

impl AsRef<UpdateAnimationSearchParameters> for RTDUpdateAnimationSearchParametersBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateAuthorizationStateBuilder {
        let mut inner = UpdateAuthorizationState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateAuthorizationStateBuilder { inner }
    }

    pub fn authorization_state(&self) -> &AuthorizationState {
        &self.authorization_state
    }
}

#[doc(hidden)]
pub struct RTDUpdateAuthorizationStateBuilder {
    inner: UpdateAuthorizationState,
}

impl RTDUpdateAuthorizationStateBuilder {
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

impl AsRef<UpdateAuthorizationState> for RTDUpdateAuthorizationStateBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateBasicGroupBuilder {
        let mut inner = UpdateBasicGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateBasicGroupBuilder { inner }
    }

    pub fn basic_group(&self) -> &BasicGroup {
        &self.basic_group
    }
}

#[doc(hidden)]
pub struct RTDUpdateBasicGroupBuilder {
    inner: UpdateBasicGroup,
}

impl RTDUpdateBasicGroupBuilder {
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

impl AsRef<UpdateBasicGroup> for RTDUpdateBasicGroupBuilder {
    fn as_ref(&self) -> &UpdateBasicGroup {
        &self.inner
    }
}

/// Some data from basicGroupFullInfo has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBasicGroupFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a basic group
    basic_group_id: i32,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateBasicGroupFullInfoBuilder {
        let mut inner = UpdateBasicGroupFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateBasicGroupFullInfoBuilder { inner }
    }

    pub fn basic_group_id(&self) -> i32 {
        self.basic_group_id
    }

    pub fn basic_group_full_info(&self) -> &BasicGroupFullInfo {
        &self.basic_group_full_info
    }
}

#[doc(hidden)]
pub struct RTDUpdateBasicGroupFullInfoBuilder {
    inner: UpdateBasicGroupFullInfo,
}

impl RTDUpdateBasicGroupFullInfoBuilder {
    pub fn build(&self) -> UpdateBasicGroupFullInfo {
        self.inner.clone()
    }

    pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self {
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

impl AsRef<UpdateBasicGroupFullInfo> for RTDUpdateBasicGroupFullInfoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateCallBuilder {
        let mut inner = UpdateCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateCallBuilder { inner }
    }

    pub fn call(&self) -> &Call {
        &self.call
    }
}

#[doc(hidden)]
pub struct RTDUpdateCallBuilder {
    inner: UpdateCall,
}

impl RTDUpdateCallBuilder {
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

impl AsRef<UpdateCall> for RTDUpdateCallBuilder {
    fn as_ref(&self) -> &UpdateCall {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatActionBarBuilder {
        let mut inner = UpdateChatActionBar::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatActionBarBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn action_bar(&self) -> &Option<ChatActionBar> {
        &self.action_bar
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatActionBarBuilder {
    inner: UpdateChatActionBar,
}

impl RTDUpdateChatActionBarBuilder {
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

impl AsRef<UpdateChatActionBar> for RTDUpdateChatActionBarBuilder {
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
    chat_id: i64,
    /// The new default_disable_notification value
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatDefaultDisableNotificationBuilder {
        let mut inner = UpdateChatDefaultDisableNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatDefaultDisableNotificationBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn default_disable_notification(&self) -> bool {
        self.default_disable_notification
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatDefaultDisableNotificationBuilder {
    inner: UpdateChatDefaultDisableNotification,
}

impl RTDUpdateChatDefaultDisableNotificationBuilder {
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

impl AsRef<UpdateChatDefaultDisableNotification>
    for RTDUpdateChatDefaultDisableNotificationBuilder
{
    fn as_ref(&self) -> &UpdateChatDefaultDisableNotification {
        &self.inner
    }
}

/// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatDraftMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// The new draft message; may be null
    draft_message: Option<DraftMessage>,
    /// The new chat positions in the chat lists
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatDraftMessageBuilder {
        let mut inner = UpdateChatDraftMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatDraftMessageBuilder { inner }
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
pub struct RTDUpdateChatDraftMessageBuilder {
    inner: UpdateChatDraftMessage,
}

impl RTDUpdateChatDraftMessageBuilder {
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

impl AsRef<UpdateChatDraftMessage> for RTDUpdateChatDraftMessageBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatFiltersBuilder {
        let mut inner = UpdateChatFilters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatFiltersBuilder { inner }
    }

    pub fn chat_filters(&self) -> &Vec<ChatFilterInfo> {
        &self.chat_filters
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatFiltersBuilder {
    inner: UpdateChatFilters,
}

impl RTDUpdateChatFiltersBuilder {
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

impl AsRef<UpdateChatFilters> for RTDUpdateChatFiltersBuilder {
    fn as_ref(&self) -> &UpdateChatFilters {
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
    chat_id: i64,
    /// New value of has_scheduled_messages
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatHasScheduledMessagesBuilder {
        let mut inner = UpdateChatHasScheduledMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatHasScheduledMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn has_scheduled_messages(&self) -> bool {
        self.has_scheduled_messages
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatHasScheduledMessagesBuilder {
    inner: UpdateChatHasScheduledMessages,
}

impl RTDUpdateChatHasScheduledMessagesBuilder {
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

impl AsRef<UpdateChatHasScheduledMessages> for RTDUpdateChatHasScheduledMessagesBuilder {
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
    chat_id: i64,
    /// New value of is_blocked
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatIsBlockedBuilder {
        let mut inner = UpdateChatIsBlocked::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatIsBlockedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn is_blocked(&self) -> bool {
        self.is_blocked
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatIsBlockedBuilder {
    inner: UpdateChatIsBlocked,
}

impl RTDUpdateChatIsBlockedBuilder {
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

impl AsRef<UpdateChatIsBlocked> for RTDUpdateChatIsBlockedBuilder {
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
    chat_id: i64,
    /// New value of is_marked_as_unread
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatIsMarkedAsUnreadBuilder {
        let mut inner = UpdateChatIsMarkedAsUnread::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatIsMarkedAsUnreadBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn is_marked_as_unread(&self) -> bool {
        self.is_marked_as_unread
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatIsMarkedAsUnreadBuilder {
    inner: UpdateChatIsMarkedAsUnread,
}

impl RTDUpdateChatIsMarkedAsUnreadBuilder {
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

impl AsRef<UpdateChatIsMarkedAsUnread> for RTDUpdateChatIsMarkedAsUnreadBuilder {
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
    chat_id: i64,
    /// The new last message in the chat; may be null
    last_message: Option<Message>,
    /// The new chat positions in the chat lists
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatLastMessageBuilder {
        let mut inner = UpdateChatLastMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatLastMessageBuilder { inner }
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
pub struct RTDUpdateChatLastMessageBuilder {
    inner: UpdateChatLastMessage,
}

impl RTDUpdateChatLastMessageBuilder {
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

impl AsRef<UpdateChatLastMessage> for RTDUpdateChatLastMessageBuilder {
    fn as_ref(&self) -> &UpdateChatLastMessage {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatNotificationSettingsBuilder {
        let mut inner = UpdateChatNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatNotificationSettingsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn notification_settings(&self) -> &ChatNotificationSettings {
        &self.notification_settings
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatNotificationSettingsBuilder {
    inner: UpdateChatNotificationSettings,
}

impl RTDUpdateChatNotificationSettingsBuilder {
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

impl AsRef<UpdateChatNotificationSettings> for RTDUpdateChatNotificationSettingsBuilder {
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
    chat_id: i64,
    /// New number of online members in the chat, or 0 if unknown
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatOnlineMemberCountBuilder {
        let mut inner = UpdateChatOnlineMemberCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatOnlineMemberCountBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn online_member_count(&self) -> i32 {
        self.online_member_count
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatOnlineMemberCountBuilder {
    inner: UpdateChatOnlineMemberCount,
}

impl RTDUpdateChatOnlineMemberCountBuilder {
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

impl AsRef<UpdateChatOnlineMemberCount> for RTDUpdateChatOnlineMemberCountBuilder {
    fn as_ref(&self) -> &UpdateChatOnlineMemberCount {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatPermissionsBuilder {
        let mut inner = UpdateChatPermissions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatPermissionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn permissions(&self) -> &ChatPermissions {
        &self.permissions
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatPermissionsBuilder {
    inner: UpdateChatPermissions,
}

impl RTDUpdateChatPermissionsBuilder {
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

impl AsRef<UpdateChatPermissions> for RTDUpdateChatPermissionsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatPhotoBuilder {
        let mut inner = UpdateChatPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatPhotoBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn photo(&self) -> &Option<ChatPhotoInfo> {
        &self.photo
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatPhotoBuilder {
    inner: UpdateChatPhoto,
}

impl RTDUpdateChatPhotoBuilder {
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

impl AsRef<UpdateChatPhoto> for RTDUpdateChatPhotoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatPositionBuilder {
        let mut inner = UpdateChatPosition::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatPositionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn position(&self) -> &ChatPosition {
        &self.position
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatPositionBuilder {
    inner: UpdateChatPosition,
}

impl RTDUpdateChatPositionBuilder {
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

impl AsRef<UpdateChatPosition> for RTDUpdateChatPositionBuilder {
    fn as_ref(&self) -> &UpdateChatPosition {
        &self.inner
    }
}

/// Incoming messages were read or number of unread messages has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateChatReadInbox {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Identifier of the last read incoming message
    last_read_inbox_message_id: i64,
    /// The number of unread messages left in the chat
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatReadInboxBuilder {
        let mut inner = UpdateChatReadInbox::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatReadInboxBuilder { inner }
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
pub struct RTDUpdateChatReadInboxBuilder {
    inner: UpdateChatReadInbox,
}

impl RTDUpdateChatReadInboxBuilder {
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

impl AsRef<UpdateChatReadInbox> for RTDUpdateChatReadInboxBuilder {
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
    chat_id: i64,
    /// Identifier of last read outgoing message
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatReadOutboxBuilder {
        let mut inner = UpdateChatReadOutbox::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatReadOutboxBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn last_read_outbox_message_id(&self) -> i64 {
        self.last_read_outbox_message_id
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatReadOutboxBuilder {
    inner: UpdateChatReadOutbox,
}

impl RTDUpdateChatReadOutboxBuilder {
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

impl AsRef<UpdateChatReadOutbox> for RTDUpdateChatReadOutboxBuilder {
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
    chat_id: i64,
    /// Identifier of the message from which reply markup needs to be used; 0 if there is no default custom reply markup in the chat
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatReplyMarkupBuilder {
        let mut inner = UpdateChatReplyMarkup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatReplyMarkupBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn reply_markup_message_id(&self) -> i64 {
        self.reply_markup_message_id
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatReplyMarkupBuilder {
    inner: UpdateChatReplyMarkup,
}

impl RTDUpdateChatReplyMarkupBuilder {
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

impl AsRef<UpdateChatReplyMarkup> for RTDUpdateChatReplyMarkupBuilder {
    fn as_ref(&self) -> &UpdateChatReplyMarkup {
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
    chat_id: i64,
    /// The new chat title
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatTitleBuilder {
        let mut inner = UpdateChatTitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatTitleBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatTitleBuilder {
    inner: UpdateChatTitle,
}

impl RTDUpdateChatTitleBuilder {
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

impl AsRef<UpdateChatTitle> for RTDUpdateChatTitleBuilder {
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
    chat_id: i64,
    /// The number of unread mention messages left in the chat
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateChatUnreadMentionCountBuilder {
        let mut inner = UpdateChatUnreadMentionCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateChatUnreadMentionCountBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn unread_mention_count(&self) -> i32 {
        self.unread_mention_count
    }
}

#[doc(hidden)]
pub struct RTDUpdateChatUnreadMentionCountBuilder {
    inner: UpdateChatUnreadMentionCount,
}

impl RTDUpdateChatUnreadMentionCountBuilder {
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

impl AsRef<UpdateChatUnreadMentionCount> for RTDUpdateChatUnreadMentionCountBuilder {
    fn as_ref(&self) -> &UpdateChatUnreadMentionCount {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateConnectionStateBuilder {
        let mut inner = UpdateConnectionState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateConnectionStateBuilder { inner }
    }

    pub fn state(&self) -> &ConnectionState {
        &self.state
    }
}

#[doc(hidden)]
pub struct RTDUpdateConnectionStateBuilder {
    inner: UpdateConnectionState,
}

impl RTDUpdateConnectionStateBuilder {
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

impl AsRef<UpdateConnectionState> for RTDUpdateConnectionStateBuilder {
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
    chat_id: i64,
    /// Identifiers of the deleted messages
    message_ids: Vec<i64>,
    /// True, if the messages are permanently deleted by a user (as opposed to just becoming inaccessible)
    is_permanent: bool,
    /// True, if the messages are deleted only from the cache and can possibly be retrieved again in the future
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateDeleteMessagesBuilder {
        let mut inner = UpdateDeleteMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateDeleteMessagesBuilder { inner }
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
pub struct RTDUpdateDeleteMessagesBuilder {
    inner: UpdateDeleteMessages,
}

impl RTDUpdateDeleteMessagesBuilder {
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

impl AsRef<UpdateDeleteMessages> for RTDUpdateDeleteMessagesBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateDiceEmojisBuilder {
        let mut inner = UpdateDiceEmojis::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateDiceEmojisBuilder { inner }
    }

    pub fn emojis(&self) -> &Vec<String> {
        &self.emojis
    }
}

#[doc(hidden)]
pub struct RTDUpdateDiceEmojisBuilder {
    inner: UpdateDiceEmojis,
}

impl RTDUpdateDiceEmojisBuilder {
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

impl AsRef<UpdateDiceEmojis> for RTDUpdateDiceEmojisBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateFavoriteStickersBuilder {
        let mut inner = UpdateFavoriteStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateFavoriteStickersBuilder { inner }
    }

    pub fn sticker_ids(&self) -> &Vec<i32> {
        &self.sticker_ids
    }
}

#[doc(hidden)]
pub struct RTDUpdateFavoriteStickersBuilder {
    inner: UpdateFavoriteStickers,
}

impl RTDUpdateFavoriteStickersBuilder {
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

impl AsRef<UpdateFavoriteStickers> for RTDUpdateFavoriteStickersBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateFileBuilder {
        let mut inner = UpdateFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateFileBuilder { inner }
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

#[doc(hidden)]
pub struct RTDUpdateFileBuilder {
    inner: UpdateFile,
}

impl RTDUpdateFileBuilder {
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

impl AsRef<UpdateFile> for RTDUpdateFileBuilder {
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

    #[serde(deserialize_with = "super::_common::number_from_string")]
    generation_id: i64,
    /// The path to a file from which a new file is generated; may be empty
    original_path: String,
    /// The path to a file that should be created and where the new file should be generated
    destination_path: String,
    /// String specifying the conversion applied to the original file. If conversion is "#url#" than original_path contains an HTTP/HTTPS URL of a file, which should be downloaded by the application
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateFileGenerationStartBuilder {
        let mut inner = UpdateFileGenerationStart::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateFileGenerationStartBuilder { inner }
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
pub struct RTDUpdateFileGenerationStartBuilder {
    inner: UpdateFileGenerationStart,
}

impl RTDUpdateFileGenerationStartBuilder {
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

impl AsRef<UpdateFileGenerationStart> for RTDUpdateFileGenerationStartBuilder {
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

    #[serde(deserialize_with = "super::_common::number_from_string")]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateFileGenerationStopBuilder {
        let mut inner = UpdateFileGenerationStop::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateFileGenerationStopBuilder { inner }
    }

    pub fn generation_id(&self) -> i64 {
        self.generation_id
    }
}

#[doc(hidden)]
pub struct RTDUpdateFileGenerationStopBuilder {
    inner: UpdateFileGenerationStop,
}

impl RTDUpdateFileGenerationStopBuilder {
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

impl AsRef<UpdateFileGenerationStop> for RTDUpdateFileGenerationStopBuilder {
    fn as_ref(&self) -> &UpdateFileGenerationStop {
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
    have_delayed_notifications: bool,
    /// True, if there can be some yet unreceived notifications, which are being fetched from the server
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateHavePendingNotificationsBuilder {
        let mut inner = UpdateHavePendingNotifications::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateHavePendingNotificationsBuilder { inner }
    }

    pub fn have_delayed_notifications(&self) -> bool {
        self.have_delayed_notifications
    }

    pub fn have_unreceived_notifications(&self) -> bool {
        self.have_unreceived_notifications
    }
}

#[doc(hidden)]
pub struct RTDUpdateHavePendingNotificationsBuilder {
    inner: UpdateHavePendingNotifications,
}

impl RTDUpdateHavePendingNotificationsBuilder {
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

impl AsRef<UpdateHavePendingNotifications> for RTDUpdateHavePendingNotificationsBuilder {
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
    is_masks: bool,
    /// The new list of installed ordinary sticker sets

    #[serde(deserialize_with = "super::_common::vec_of_i64_from_str")]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateInstalledStickerSetsBuilder {
        let mut inner = UpdateInstalledStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateInstalledStickerSetsBuilder { inner }
    }

    pub fn is_masks(&self) -> bool {
        self.is_masks
    }

    pub fn sticker_set_ids(&self) -> &Vec<i64> {
        &self.sticker_set_ids
    }
}

#[doc(hidden)]
pub struct RTDUpdateInstalledStickerSetsBuilder {
    inner: UpdateInstalledStickerSets,
}

impl RTDUpdateInstalledStickerSetsBuilder {
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

impl AsRef<UpdateInstalledStickerSets> for RTDUpdateInstalledStickerSetsBuilder {
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
    localization_target: String,
    /// Identifier of the updated language pack
    language_pack_id: String,
    /// List of changed language pack strings
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateLanguagePackStringsBuilder {
        let mut inner = UpdateLanguagePackStrings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateLanguagePackStringsBuilder { inner }
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
pub struct RTDUpdateLanguagePackStringsBuilder {
    inner: UpdateLanguagePackStrings,
}

impl RTDUpdateLanguagePackStringsBuilder {
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

impl AsRef<UpdateLanguagePackStrings> for RTDUpdateLanguagePackStringsBuilder {
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
    chat_id: i64,
    /// Message identifier
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateMessageContentBuilder {
        let mut inner = UpdateMessageContent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateMessageContentBuilder { inner }
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
pub struct RTDUpdateMessageContentBuilder {
    inner: UpdateMessageContent,
}

impl RTDUpdateMessageContentBuilder {
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

impl AsRef<UpdateMessageContent> for RTDUpdateMessageContentBuilder {
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
    chat_id: i64,
    /// Message identifier
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateMessageContentOpenedBuilder {
        let mut inner = UpdateMessageContentOpened::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateMessageContentOpenedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct RTDUpdateMessageContentOpenedBuilder {
    inner: UpdateMessageContentOpened,
}

impl RTDUpdateMessageContentOpenedBuilder {
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

impl AsRef<UpdateMessageContentOpened> for RTDUpdateMessageContentOpenedBuilder {
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
    chat_id: i64,
    /// Message identifier
    message_id: i64,
    /// Point in time (Unix timestamp) when the message was edited
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateMessageEditedBuilder {
        let mut inner = UpdateMessageEdited::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateMessageEditedBuilder { inner }
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
pub struct RTDUpdateMessageEditedBuilder {
    inner: UpdateMessageEdited,
}

impl RTDUpdateMessageEditedBuilder {
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

impl AsRef<UpdateMessageEdited> for RTDUpdateMessageEditedBuilder {
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
    chat_id: i64,
    /// Message identifier
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateMessageInteractionInfoBuilder {
        let mut inner = UpdateMessageInteractionInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateMessageInteractionInfoBuilder { inner }
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
pub struct RTDUpdateMessageInteractionInfoBuilder {
    inner: UpdateMessageInteractionInfo,
}

impl RTDUpdateMessageInteractionInfoBuilder {
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

impl AsRef<UpdateMessageInteractionInfo> for RTDUpdateMessageInteractionInfoBuilder {
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
    chat_id: i64,
    /// The message identifier
    message_id: i64,
    /// True, if the message is pinned
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateMessageIsPinnedBuilder {
        let mut inner = UpdateMessageIsPinned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateMessageIsPinnedBuilder { inner }
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
pub struct RTDUpdateMessageIsPinnedBuilder {
    inner: UpdateMessageIsPinned,
}

impl RTDUpdateMessageIsPinnedBuilder {
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

impl AsRef<UpdateMessageIsPinned> for RTDUpdateMessageIsPinnedBuilder {
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
    chat_id: i64,
    /// Identifier of the message with live location
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateMessageLiveLocationViewedBuilder {
        let mut inner = UpdateMessageLiveLocationViewed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateMessageLiveLocationViewedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct RTDUpdateMessageLiveLocationViewedBuilder {
    inner: UpdateMessageLiveLocationViewed,
}

impl RTDUpdateMessageLiveLocationViewedBuilder {
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

impl AsRef<UpdateMessageLiveLocationViewed> for RTDUpdateMessageLiveLocationViewedBuilder {
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
    chat_id: i64,
    /// Message identifier
    message_id: i64,
    /// The new number of unread mention messages left in the chat
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateMessageMentionReadBuilder {
        let mut inner = UpdateMessageMentionRead::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateMessageMentionReadBuilder { inner }
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
pub struct RTDUpdateMessageMentionReadBuilder {
    inner: UpdateMessageMentionRead,
}

impl RTDUpdateMessageMentionReadBuilder {
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

impl AsRef<UpdateMessageMentionRead> for RTDUpdateMessageMentionReadBuilder {
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
    chat_id: i64,
    /// A temporary message identifier
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateMessageSendAcknowledgedBuilder {
        let mut inner = UpdateMessageSendAcknowledged::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateMessageSendAcknowledgedBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct RTDUpdateMessageSendAcknowledgedBuilder {
    inner: UpdateMessageSendAcknowledged,
}

impl RTDUpdateMessageSendAcknowledgedBuilder {
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

impl AsRef<UpdateMessageSendAcknowledged> for RTDUpdateMessageSendAcknowledgedBuilder {
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
    /// Contains information about the message which failed to send
    message: Message,
    /// The previous temporary message identifier
    old_message_id: i64,
    /// An error code
    error_code: i32,
    /// Error message
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateMessageSendFailedBuilder {
        let mut inner = UpdateMessageSendFailed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateMessageSendFailedBuilder { inner }
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
pub struct RTDUpdateMessageSendFailedBuilder {
    inner: UpdateMessageSendFailed,
}

impl RTDUpdateMessageSendFailedBuilder {
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

impl AsRef<UpdateMessageSendFailed> for RTDUpdateMessageSendFailedBuilder {
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
    /// Information about the sent message. Usually only the message identifier, date, and content are changed, but almost all other fields can also change
    message: Message,
    /// The previous temporary message identifier
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateMessageSendSucceededBuilder {
        let mut inner = UpdateMessageSendSucceeded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateMessageSendSucceededBuilder { inner }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }

    pub fn old_message_id(&self) -> i64 {
        self.old_message_id
    }
}

#[doc(hidden)]
pub struct RTDUpdateMessageSendSucceededBuilder {
    inner: UpdateMessageSendSucceeded,
}

impl RTDUpdateMessageSendSucceededBuilder {
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

impl AsRef<UpdateMessageSendSucceeded> for RTDUpdateMessageSendSucceededBuilder {
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
    call_id: i32,
    /// The data
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewCallSignalingDataBuilder {
        let mut inner = UpdateNewCallSignalingData::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewCallSignalingDataBuilder { inner }
    }

    pub fn call_id(&self) -> i32 {
        self.call_id
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDUpdateNewCallSignalingDataBuilder {
    inner: UpdateNewCallSignalingData,
}

impl RTDUpdateNewCallSignalingDataBuilder {
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

impl AsRef<UpdateNewCallSignalingData> for RTDUpdateNewCallSignalingDataBuilder {
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

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
    /// Identifier of the user who sent the query
    sender_user_id: i32,
    /// Identifier of the chat where the query was sent
    chat_id: i64,
    /// Identifier of the message, from which the query originated
    message_id: i64,
    /// Identifier that uniquely corresponds to the chat to which the message was sent

    #[serde(deserialize_with = "super::_common::number_from_string")]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewCallbackQueryBuilder {
        let mut inner = UpdateNewCallbackQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewCallbackQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_user_id(&self) -> i32 {
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
pub struct RTDUpdateNewCallbackQueryBuilder {
    inner: UpdateNewCallbackQuery,
}

impl RTDUpdateNewCallbackQueryBuilder {
    pub fn build(&self) -> UpdateNewCallbackQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self {
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

impl AsRef<UpdateNewCallbackQuery> for RTDUpdateNewCallbackQueryBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewChatBuilder {
        let mut inner = UpdateNewChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewChatBuilder { inner }
    }

    pub fn chat(&self) -> &Chat {
        &self.chat
    }
}

#[doc(hidden)]
pub struct RTDUpdateNewChatBuilder {
    inner: UpdateNewChat,
}

impl RTDUpdateNewChatBuilder {
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

impl AsRef<UpdateNewChat> for RTDUpdateNewChatBuilder {
    fn as_ref(&self) -> &UpdateNewChat {
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
    sender_user_id: i32,
    /// User location; may be null
    user_location: Option<Location>,
    /// Text of the query
    query: String,
    /// Identifier of the chosen result
    result_id: String,
    /// Identifier of the sent inline message, if known
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewChosenInlineResultBuilder {
        let mut inner = UpdateNewChosenInlineResult::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewChosenInlineResultBuilder { inner }
    }

    pub fn sender_user_id(&self) -> i32 {
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
pub struct RTDUpdateNewChosenInlineResultBuilder {
    inner: UpdateNewChosenInlineResult,
}

impl RTDUpdateNewChosenInlineResultBuilder {
    pub fn build(&self) -> UpdateNewChosenInlineResult {
        self.inner.clone()
    }

    pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self {
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

impl AsRef<UpdateNewChosenInlineResult> for RTDUpdateNewChosenInlineResultBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewCustomEventBuilder {
        let mut inner = UpdateNewCustomEvent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewCustomEventBuilder { inner }
    }

    pub fn event(&self) -> &String {
        &self.event
    }
}

#[doc(hidden)]
pub struct RTDUpdateNewCustomEventBuilder {
    inner: UpdateNewCustomEvent,
}

impl RTDUpdateNewCustomEventBuilder {
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

impl AsRef<UpdateNewCustomEvent> for RTDUpdateNewCustomEventBuilder {
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

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
    /// JSON-serialized query data
    data: String,
    /// Query timeout
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewCustomQueryBuilder {
        let mut inner = UpdateNewCustomQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewCustomQueryBuilder { inner }
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
pub struct RTDUpdateNewCustomQueryBuilder {
    inner: UpdateNewCustomQuery,
}

impl RTDUpdateNewCustomQueryBuilder {
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

impl AsRef<UpdateNewCustomQuery> for RTDUpdateNewCustomQueryBuilder {
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

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
    /// Identifier of the user who sent the query
    sender_user_id: i32,
    /// Identifier of the inline message, from which the query originated
    inline_message_id: String,
    /// An identifier uniquely corresponding to the chat a message was sent to

    #[serde(deserialize_with = "super::_common::number_from_string")]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewInlineCallbackQueryBuilder {
        let mut inner = UpdateNewInlineCallbackQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewInlineCallbackQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_user_id(&self) -> i32 {
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
pub struct RTDUpdateNewInlineCallbackQueryBuilder {
    inner: UpdateNewInlineCallbackQuery,
}

impl RTDUpdateNewInlineCallbackQueryBuilder {
    pub fn build(&self) -> UpdateNewInlineCallbackQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self {
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

impl AsRef<UpdateNewInlineCallbackQuery> for RTDUpdateNewInlineCallbackQueryBuilder {
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

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
    /// Identifier of the user who sent the query
    sender_user_id: i32,
    /// User location; may be null
    user_location: Option<Location>,
    /// Text of the query
    query: String,
    /// Offset of the first entry to return
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewInlineQueryBuilder {
        let mut inner = UpdateNewInlineQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewInlineQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_user_id(&self) -> i32 {
        self.sender_user_id
    }

    pub fn user_location(&self) -> &Option<Location> {
        &self.user_location
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn offset(&self) -> &String {
        &self.offset
    }
}

#[doc(hidden)]
pub struct RTDUpdateNewInlineQueryBuilder {
    inner: UpdateNewInlineQuery,
}

impl RTDUpdateNewInlineQueryBuilder {
    pub fn build(&self) -> UpdateNewInlineQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self {
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

impl AsRef<UpdateNewInlineQuery> for RTDUpdateNewInlineQueryBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewMessageBuilder {
        let mut inner = UpdateNewMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewMessageBuilder { inner }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }
}

#[doc(hidden)]
pub struct RTDUpdateNewMessageBuilder {
    inner: UpdateNewMessage,
}

impl RTDUpdateNewMessageBuilder {
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

impl AsRef<UpdateNewMessage> for RTDUpdateNewMessageBuilder {
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

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
    /// Identifier of the user who sent the query
    sender_user_id: i32,
    /// Currency for the product price
    currency: String,
    /// Total price for the product, in the minimal quantity of the currency
    total_amount: i64,
    /// Invoice payload
    invoice_payload: String,
    /// Identifier of a shipping option chosen by the user; may be empty if not applicable
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewPreCheckoutQueryBuilder {
        let mut inner = UpdateNewPreCheckoutQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewPreCheckoutQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_user_id(&self) -> i32 {
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
pub struct RTDUpdateNewPreCheckoutQueryBuilder {
    inner: UpdateNewPreCheckoutQuery,
}

impl RTDUpdateNewPreCheckoutQueryBuilder {
    pub fn build(&self) -> UpdateNewPreCheckoutQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self {
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

impl AsRef<UpdateNewPreCheckoutQuery> for RTDUpdateNewPreCheckoutQueryBuilder {
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

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
    /// Identifier of the user who sent the query
    sender_user_id: i32,
    /// Invoice payload
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNewShippingQueryBuilder {
        let mut inner = UpdateNewShippingQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNewShippingQueryBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn sender_user_id(&self) -> i32 {
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
pub struct RTDUpdateNewShippingQueryBuilder {
    inner: UpdateNewShippingQuery,
}

impl RTDUpdateNewShippingQueryBuilder {
    pub fn build(&self) -> UpdateNewShippingQuery {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self {
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

impl AsRef<UpdateNewShippingQuery> for RTDUpdateNewShippingQueryBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNotificationBuilder {
        let mut inner = UpdateNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNotificationBuilder { inner }
    }

    pub fn notification_group_id(&self) -> i32 {
        self.notification_group_id
    }

    pub fn notification(&self) -> &Notification {
        &self.notification
    }
}

#[doc(hidden)]
pub struct RTDUpdateNotificationBuilder {
    inner: UpdateNotification,
}

impl RTDUpdateNotificationBuilder {
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

impl AsRef<UpdateNotification> for RTDUpdateNotificationBuilder {
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
    notification_group_id: i32,
    /// New type of the notification group

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "NotificationGroupType::_is_default")]
    type_: NotificationGroupType,
    /// Identifier of a chat to which all notifications in the group belong
    chat_id: i64,
    /// Chat identifier, which notification settings must be applied to the added notifications
    notification_settings_chat_id: i64,
    /// True, if the notifications should be shown without sound
    is_silent: bool,
    /// Total number of unread notifications in the group, can be bigger than number of active notifications
    total_count: i32,
    /// List of added group notifications, sorted by notification ID
    added_notifications: Vec<Notification>,
    /// Identifiers of removed group notifications, sorted by notification ID
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateNotificationGroupBuilder {
        let mut inner = UpdateNotificationGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateNotificationGroupBuilder { inner }
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
pub struct RTDUpdateNotificationGroupBuilder {
    inner: UpdateNotificationGroup,
}

impl RTDUpdateNotificationGroupBuilder {
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

impl AsRef<UpdateNotificationGroup> for RTDUpdateNotificationGroupBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateOptionBuilder {
        let mut inner = UpdateOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateOptionBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &OptionValue {
        &self.value
    }
}

#[doc(hidden)]
pub struct RTDUpdateOptionBuilder {
    inner: UpdateOption,
}

impl RTDUpdateOptionBuilder {
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

impl AsRef<UpdateOption> for RTDUpdateOptionBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdatePollBuilder {
        let mut inner = UpdatePoll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdatePollBuilder { inner }
    }

    pub fn poll(&self) -> &Poll {
        &self.poll
    }
}

#[doc(hidden)]
pub struct RTDUpdatePollBuilder {
    inner: UpdatePoll,
}

impl RTDUpdatePollBuilder {
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

impl AsRef<UpdatePoll> for RTDUpdatePollBuilder {
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

    #[serde(deserialize_with = "super::_common::number_from_string")]
    poll_id: i64,
    /// The user, who changed the answer to the poll
    user_id: i32,
    /// 0-based identifiers of answer options, chosen by the user
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdatePollAnswerBuilder {
        let mut inner = UpdatePollAnswer::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdatePollAnswerBuilder { inner }
    }

    pub fn poll_id(&self) -> i64 {
        self.poll_id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn option_ids(&self) -> &Vec<i32> {
        &self.option_ids
    }
}

#[doc(hidden)]
pub struct RTDUpdatePollAnswerBuilder {
    inner: UpdatePollAnswer,
}

impl RTDUpdatePollAnswerBuilder {
    pub fn build(&self) -> UpdatePollAnswer {
        self.inner.clone()
    }

    pub fn poll_id(&mut self, poll_id: i64) -> &mut Self {
        self.inner.poll_id = poll_id;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
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

impl AsRef<UpdatePollAnswer> for RTDUpdatePollAnswerBuilder {
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
    is_attached: bool,
    /// The new list of file identifiers of recently used stickers
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateRecentStickersBuilder {
        let mut inner = UpdateRecentStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateRecentStickersBuilder { inner }
    }

    pub fn is_attached(&self) -> bool {
        self.is_attached
    }

    pub fn sticker_ids(&self) -> &Vec<i32> {
        &self.sticker_ids
    }
}

#[doc(hidden)]
pub struct RTDUpdateRecentStickersBuilder {
    inner: UpdateRecentStickers,
}

impl RTDUpdateRecentStickersBuilder {
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

impl AsRef<UpdateRecentStickers> for RTDUpdateRecentStickersBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateSavedAnimationsBuilder {
        let mut inner = UpdateSavedAnimations::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateSavedAnimationsBuilder { inner }
    }

    pub fn animation_ids(&self) -> &Vec<i32> {
        &self.animation_ids
    }
}

#[doc(hidden)]
pub struct RTDUpdateSavedAnimationsBuilder {
    inner: UpdateSavedAnimations,
}

impl RTDUpdateSavedAnimationsBuilder {
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

impl AsRef<UpdateSavedAnimations> for RTDUpdateSavedAnimationsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateScopeNotificationSettingsBuilder {
        let mut inner = UpdateScopeNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateScopeNotificationSettingsBuilder { inner }
    }

    pub fn scope(&self) -> &NotificationSettingsScope {
        &self.scope
    }

    pub fn notification_settings(&self) -> &ScopeNotificationSettings {
        &self.notification_settings
    }
}

#[doc(hidden)]
pub struct RTDUpdateScopeNotificationSettingsBuilder {
    inner: UpdateScopeNotificationSettings,
}

impl RTDUpdateScopeNotificationSettingsBuilder {
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

impl AsRef<UpdateScopeNotificationSettings> for RTDUpdateScopeNotificationSettingsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateSecretChatBuilder {
        let mut inner = UpdateSecretChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateSecretChatBuilder { inner }
    }

    pub fn secret_chat(&self) -> &SecretChat {
        &self.secret_chat
    }
}

#[doc(hidden)]
pub struct RTDUpdateSecretChatBuilder {
    inner: UpdateSecretChat,
}

impl RTDUpdateSecretChatBuilder {
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

impl AsRef<UpdateSecretChat> for RTDUpdateSecretChatBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateSelectedBackgroundBuilder {
        let mut inner = UpdateSelectedBackground::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateSelectedBackgroundBuilder { inner }
    }

    pub fn for_dark_theme(&self) -> bool {
        self.for_dark_theme
    }

    pub fn background(&self) -> &Option<Background> {
        &self.background
    }
}

#[doc(hidden)]
pub struct RTDUpdateSelectedBackgroundBuilder {
    inner: UpdateSelectedBackground,
}

impl RTDUpdateSelectedBackgroundBuilder {
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

impl AsRef<UpdateSelectedBackground> for RTDUpdateSelectedBackgroundBuilder {
    fn as_ref(&self) -> &UpdateSelectedBackground {
        &self.inner
    }
}

/// Service notification from the server. Upon receiving this the application must show a popup with the content of the notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateServiceNotification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Notification type. If type begins with "AUTH_KEY_DROP_", then two buttons "Cancel" and "Log out" should be shown under notification; if user presses the second, all local data should be destroyed using Destroy method

    #[serde(rename(serialize = "type", deserialize = "type"))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateServiceNotificationBuilder {
        let mut inner = UpdateServiceNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateServiceNotificationBuilder { inner }
    }

    pub fn type_(&self) -> &String {
        &self.type_
    }

    pub fn content(&self) -> &MessageContent {
        &self.content
    }
}

#[doc(hidden)]
pub struct RTDUpdateServiceNotificationBuilder {
    inner: UpdateServiceNotification,
}

impl RTDUpdateServiceNotificationBuilder {
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

impl AsRef<UpdateServiceNotification> for RTDUpdateServiceNotificationBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateStickerSetBuilder {
        let mut inner = UpdateStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateStickerSetBuilder { inner }
    }

    pub fn sticker_set(&self) -> &StickerSet {
        &self.sticker_set
    }
}

#[doc(hidden)]
pub struct RTDUpdateStickerSetBuilder {
    inner: UpdateStickerSet,
}

impl RTDUpdateStickerSetBuilder {
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

impl AsRef<UpdateStickerSet> for RTDUpdateStickerSetBuilder {
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
    added_actions: Vec<SuggestedAction>,
    /// Removed suggested actions
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateSuggestedActionsBuilder {
        let mut inner = UpdateSuggestedActions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateSuggestedActionsBuilder { inner }
    }

    pub fn added_actions(&self) -> &Vec<SuggestedAction> {
        &self.added_actions
    }

    pub fn removed_actions(&self) -> &Vec<SuggestedAction> {
        &self.removed_actions
    }
}

#[doc(hidden)]
pub struct RTDUpdateSuggestedActionsBuilder {
    inner: UpdateSuggestedActions,
}

impl RTDUpdateSuggestedActionsBuilder {
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

impl AsRef<UpdateSuggestedActions> for RTDUpdateSuggestedActionsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateSupergroupBuilder {
        let mut inner = UpdateSupergroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateSupergroupBuilder { inner }
    }

    pub fn supergroup(&self) -> &Supergroup {
        &self.supergroup
    }
}

#[doc(hidden)]
pub struct RTDUpdateSupergroupBuilder {
    inner: UpdateSupergroup,
}

impl RTDUpdateSupergroupBuilder {
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

impl AsRef<UpdateSupergroup> for RTDUpdateSupergroupBuilder {
    fn as_ref(&self) -> &UpdateSupergroup {
        &self.inner
    }
}

/// Some data from supergroupFullInfo has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSupergroupFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup or channel
    supergroup_id: i32,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateSupergroupFullInfoBuilder {
        let mut inner = UpdateSupergroupFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateSupergroupFullInfoBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i32 {
        self.supergroup_id
    }

    pub fn supergroup_full_info(&self) -> &SupergroupFullInfo {
        &self.supergroup_full_info
    }
}

#[doc(hidden)]
pub struct RTDUpdateSupergroupFullInfoBuilder {
    inner: UpdateSupergroupFullInfo,
}

impl RTDUpdateSupergroupFullInfoBuilder {
    pub fn build(&self) -> UpdateSupergroupFullInfo {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
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

impl AsRef<UpdateSupergroupFullInfo> for RTDUpdateSupergroupFullInfoBuilder {
    fn as_ref(&self) -> &UpdateSupergroupFullInfo {
        &self.inner
    }
}

/// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method should be called with the reason "Decline ToS update"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTermsOfService {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the terms of service
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateTermsOfServiceBuilder {
        let mut inner = UpdateTermsOfService::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateTermsOfServiceBuilder { inner }
    }

    pub fn terms_of_service_id(&self) -> &String {
        &self.terms_of_service_id
    }

    pub fn terms_of_service(&self) -> &TermsOfService {
        &self.terms_of_service
    }
}

#[doc(hidden)]
pub struct RTDUpdateTermsOfServiceBuilder {
    inner: UpdateTermsOfService,
}

impl RTDUpdateTermsOfServiceBuilder {
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

impl AsRef<UpdateTermsOfService> for RTDUpdateTermsOfServiceBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateTrendingStickerSetsBuilder {
        let mut inner = UpdateTrendingStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateTrendingStickerSetsBuilder { inner }
    }

    pub fn sticker_sets(&self) -> &StickerSets {
        &self.sticker_sets
    }
}

#[doc(hidden)]
pub struct RTDUpdateTrendingStickerSetsBuilder {
    inner: UpdateTrendingStickerSets,
}

impl RTDUpdateTrendingStickerSetsBuilder {
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

impl AsRef<UpdateTrendingStickerSets> for RTDUpdateTrendingStickerSetsBuilder {
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
    total_count: i32,
    /// Total number of unread chats
    unread_count: i32,
    /// Total number of unread unmuted chats
    unread_unmuted_count: i32,
    /// Total number of chats marked as unread
    marked_as_unread_count: i32,
    /// Total number of unmuted chats marked as unread
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateUnreadChatCountBuilder {
        let mut inner = UpdateUnreadChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateUnreadChatCountBuilder { inner }
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
pub struct RTDUpdateUnreadChatCountBuilder {
    inner: UpdateUnreadChatCount,
}

impl RTDUpdateUnreadChatCountBuilder {
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

impl AsRef<UpdateUnreadChatCount> for RTDUpdateUnreadChatCountBuilder {
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
    unread_count: i32,
    /// Total number of unread messages in unmuted chats
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateUnreadMessageCountBuilder {
        let mut inner = UpdateUnreadMessageCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateUnreadMessageCountBuilder { inner }
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
pub struct RTDUpdateUnreadMessageCountBuilder {
    inner: UpdateUnreadMessageCount,
}

impl RTDUpdateUnreadMessageCountBuilder {
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

impl AsRef<UpdateUnreadMessageCount> for RTDUpdateUnreadMessageCountBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateUserBuilder {
        let mut inner = UpdateUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateUserBuilder { inner }
    }

    pub fn user(&self) -> &User {
        &self.user
    }
}

#[doc(hidden)]
pub struct RTDUpdateUserBuilder {
    inner: UpdateUser,
}

impl RTDUpdateUserBuilder {
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

impl AsRef<UpdateUser> for RTDUpdateUserBuilder {
    fn as_ref(&self) -> &UpdateUser {
        &self.inner
    }
}

/// User activity in the chat has changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserChatAction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// If not 0, a message thread identifier in which the action was performed
    message_thread_id: i64,
    /// Identifier of a user performing an action
    user_id: i32,
    /// The action description

    #[serde(skip_serializing_if = "ChatAction::_is_default")]
    action: ChatAction,
}

impl RObject for UpdateUserChatAction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for UpdateUserChatAction {}

impl UpdateUserChatAction {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateUserChatActionBuilder {
        let mut inner = UpdateUserChatAction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateUserChatActionBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn action(&self) -> &ChatAction {
        &self.action
    }
}

#[doc(hidden)]
pub struct RTDUpdateUserChatActionBuilder {
    inner: UpdateUserChatAction,
}

impl RTDUpdateUserChatActionBuilder {
    pub fn build(&self) -> UpdateUserChatAction {
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

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn action<T: AsRef<ChatAction>>(&mut self, action: T) -> &mut Self {
        self.inner.action = action.as_ref().clone();
        self
    }
}

impl AsRef<UpdateUserChatAction> for UpdateUserChatAction {
    fn as_ref(&self) -> &UpdateUserChatAction {
        self
    }
}

impl AsRef<UpdateUserChatAction> for RTDUpdateUserChatActionBuilder {
    fn as_ref(&self) -> &UpdateUserChatAction {
        &self.inner
    }
}

/// Some data from userFullInfo has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier
    user_id: i32,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateUserFullInfoBuilder {
        let mut inner = UpdateUserFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateUserFullInfoBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn user_full_info(&self) -> &UserFullInfo {
        &self.user_full_info
    }
}

#[doc(hidden)]
pub struct RTDUpdateUserFullInfoBuilder {
    inner: UpdateUserFullInfo,
}

impl RTDUpdateUserFullInfoBuilder {
    pub fn build(&self) -> UpdateUserFullInfo {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
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

impl AsRef<UpdateUserFullInfo> for RTDUpdateUserFullInfoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateUserPrivacySettingRulesBuilder {
        let mut inner = UpdateUserPrivacySettingRules::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateUserPrivacySettingRulesBuilder { inner }
    }

    pub fn setting(&self) -> &UserPrivacySetting {
        &self.setting
    }

    pub fn rules(&self) -> &UserPrivacySettingRules {
        &self.rules
    }
}

#[doc(hidden)]
pub struct RTDUpdateUserPrivacySettingRulesBuilder {
    inner: UpdateUserPrivacySettingRules,
}

impl RTDUpdateUserPrivacySettingRulesBuilder {
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

impl AsRef<UpdateUserPrivacySettingRules> for RTDUpdateUserPrivacySettingRulesBuilder {
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
    user_id: i32,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateUserStatusBuilder {
        let mut inner = UpdateUserStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateUserStatusBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }
}

#[doc(hidden)]
pub struct RTDUpdateUserStatusBuilder {
    inner: UpdateUserStatus,
}

impl RTDUpdateUserStatusBuilder {
    pub fn build(&self) -> UpdateUserStatus {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
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

impl AsRef<UpdateUserStatus> for RTDUpdateUserStatusBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdateUsersNearbyBuilder {
        let mut inner = UpdateUsersNearby::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUpdateUsersNearbyBuilder { inner }
    }

    pub fn users_nearby(&self) -> &Vec<ChatNearby> {
        &self.users_nearby
    }
}

#[doc(hidden)]
pub struct RTDUpdateUsersNearbyBuilder {
    inner: UpdateUsersNearby,
}

impl RTDUpdateUsersNearbyBuilder {
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

impl AsRef<UpdateUsersNearby> for RTDUpdateUsersNearbyBuilder {
    fn as_ref(&self) -> &UpdateUsersNearby {
        &self.inner
    }
}
