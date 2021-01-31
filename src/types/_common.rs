use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde::de::{Deserialize, Deserializer, Error as SerdeError};

use crate::{errors::*, types::*};
use serde::de;

macro_rules! rtd_enum_deserialize {
  ($type_name:ident, $(($td_name:ident, $enum_item:ident));*;) => {
    // example json
    // {"@type":"authorizationStateWaitEncryptionKey","is_encrypted":false}
    |deserializer: D| -> Result<$type_name, D::Error> {
      let rtd_trait_value: serde_json::Value = Deserialize::deserialize(deserializer)?;
      // the `rtd_trait_value` variable type is &serde_json::Value, tdlib trait will return a object, convert this type to object `&Map<String, Value>`
      let rtd_trait_map = match rtd_trait_value.as_object() {
        Some(map) => map,
        None => return Err(D::Error::unknown_field(stringify!($type_name), &[stringify!("{} is not the correct type", $type_name)])) // &format!("{} is not the correct type", stringify!($field))[..]
      };
      // get `@type` value, detect specific types
      let rtd_trait_type = match rtd_trait_map.get("@type") {
        // the `t` variable type is `serde_json::Value`, convert `t` to str
        Some(t) => match t.as_str() {
          Some(s) => s,
          None => return Err(D::Error::unknown_field(stringify!("{} -> @type", $field), &[stringify!("{} -> @type is not the correct type", $type_name)])) // &format!("{} -> @type is not the correct type", stringify!($field))[..]
        },
        None => return Err(D::Error::missing_field(stringify!("{} -> @type", $field)))
      };

      let obj = match rtd_trait_type {
        $(
          stringify!($td_name) => $type_name::$enum_item(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("{} can't deserialize to {}::{}: {}", stringify!($td_name), stringify!($type_name), stringify!($enum_item), _e)))
          }),
        )*
        _ => return Err(D::Error::missing_field(stringify!($field)))
      };
      Ok(obj)
    }
  }
}

#[allow(dead_code)]
pub fn from_json<'a, T>(json: &'a str) -> RTDResult<T>
where
    T: serde::de::Deserialize<'a>,
{
    Ok(serde_json::from_str(json)?)
}

/// All tdlib type abstract class defined the same behavior
pub trait RObject: Debug {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str;
    #[doc(hidden)]
    fn extra(&self) -> Option<String>;
    /// Return td type to json string
    fn to_json(&self) -> RTDResult<String>;
}

pub trait RFunction: Debug + RObject {}

impl<'a, RObj: RObject> RObject for &'a RObj {
    fn td_name(&self) -> &'static str {
        (*self).td_name()
    }
    fn to_json(&self) -> RTDResult<String> {
        (*self).to_json()
    }
    fn extra(&self) -> Option<String> {
        (*self).extra()
    }
}

impl<'a, RObj: RObject> RObject for &'a mut RObj {
    fn td_name(&self) -> &'static str {
        (**self).td_name()
    }
    fn to_json(&self) -> RTDResult<String> {
        (**self).to_json()
    }
    fn extra(&self) -> Option<String> {
        (**self).extra()
    }
}

impl<'a, Fnc: RFunction> RFunction for &'a Fnc {}
impl<'a, Fnc: RFunction> RFunction for &'a mut Fnc {}

impl<'a, AUTHENTICATIONCODETYPE: TDAuthenticationCodeType> TDAuthenticationCodeType
    for &'a AUTHENTICATIONCODETYPE
{
}
impl<'a, AUTHENTICATIONCODETYPE: TDAuthenticationCodeType> TDAuthenticationCodeType
    for &'a mut AUTHENTICATIONCODETYPE
{
}

impl<'a, AUTHORIZATIONSTATE: TDAuthorizationState> TDAuthorizationState for &'a AUTHORIZATIONSTATE {}
impl<'a, AUTHORIZATIONSTATE: TDAuthorizationState> TDAuthorizationState
    for &'a mut AUTHORIZATIONSTATE
{
}

impl<'a, BACKGROUNDFILL: TDBackgroundFill> TDBackgroundFill for &'a BACKGROUNDFILL {}
impl<'a, BACKGROUNDFILL: TDBackgroundFill> TDBackgroundFill for &'a mut BACKGROUNDFILL {}

impl<'a, BACKGROUNDTYPE: TDBackgroundType> TDBackgroundType for &'a BACKGROUNDTYPE {}
impl<'a, BACKGROUNDTYPE: TDBackgroundType> TDBackgroundType for &'a mut BACKGROUNDTYPE {}

impl<'a, CALLDISCARDREASON: TDCallDiscardReason> TDCallDiscardReason for &'a CALLDISCARDREASON {}
impl<'a, CALLDISCARDREASON: TDCallDiscardReason> TDCallDiscardReason for &'a mut CALLDISCARDREASON {}

impl<'a, CALLPROBLEM: TDCallProblem> TDCallProblem for &'a CALLPROBLEM {}
impl<'a, CALLPROBLEM: TDCallProblem> TDCallProblem for &'a mut CALLPROBLEM {}

impl<'a, CALLSERVERTYPE: TDCallServerType> TDCallServerType for &'a CALLSERVERTYPE {}
impl<'a, CALLSERVERTYPE: TDCallServerType> TDCallServerType for &'a mut CALLSERVERTYPE {}

impl<'a, CALLSTATE: TDCallState> TDCallState for &'a CALLSTATE {}
impl<'a, CALLSTATE: TDCallState> TDCallState for &'a mut CALLSTATE {}

impl<'a, CALLBACKQUERYPAYLOAD: TDCallbackQueryPayload> TDCallbackQueryPayload
    for &'a CALLBACKQUERYPAYLOAD
{
}
impl<'a, CALLBACKQUERYPAYLOAD: TDCallbackQueryPayload> TDCallbackQueryPayload
    for &'a mut CALLBACKQUERYPAYLOAD
{
}

impl<'a, CANTRANSFEROWNERSHIPRESULT: TDCanTransferOwnershipResult> TDCanTransferOwnershipResult
    for &'a CANTRANSFEROWNERSHIPRESULT
{
}
impl<'a, CANTRANSFEROWNERSHIPRESULT: TDCanTransferOwnershipResult> TDCanTransferOwnershipResult
    for &'a mut CANTRANSFEROWNERSHIPRESULT
{
}

impl<'a, CHATACTION: TDChatAction> TDChatAction for &'a CHATACTION {}
impl<'a, CHATACTION: TDChatAction> TDChatAction for &'a mut CHATACTION {}

impl<'a, CHATACTIONBAR: TDChatActionBar> TDChatActionBar for &'a CHATACTIONBAR {}
impl<'a, CHATACTIONBAR: TDChatActionBar> TDChatActionBar for &'a mut CHATACTIONBAR {}

impl<'a, CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &'a CHATEVENTACTION {}
impl<'a, CHATEVENTACTION: TDChatEventAction> TDChatEventAction for &'a mut CHATEVENTACTION {}

impl<'a, CHATLIST: TDChatList> TDChatList for &'a CHATLIST {}
impl<'a, CHATLIST: TDChatList> TDChatList for &'a mut CHATLIST {}

impl<'a, CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &'a CHATMEMBERSTATUS {}
impl<'a, CHATMEMBERSTATUS: TDChatMemberStatus> TDChatMemberStatus for &'a mut CHATMEMBERSTATUS {}

impl<'a, CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &'a CHATMEMBERSFILTER {}
impl<'a, CHATMEMBERSFILTER: TDChatMembersFilter> TDChatMembersFilter for &'a mut CHATMEMBERSFILTER {}

impl<'a, CHATREPORTREASON: TDChatReportReason> TDChatReportReason for &'a CHATREPORTREASON {}
impl<'a, CHATREPORTREASON: TDChatReportReason> TDChatReportReason for &'a mut CHATREPORTREASON {}

impl<'a, CHATSOURCE: TDChatSource> TDChatSource for &'a CHATSOURCE {}
impl<'a, CHATSOURCE: TDChatSource> TDChatSource for &'a mut CHATSOURCE {}

impl<'a, CHATSTATISTICS: TDChatStatistics> TDChatStatistics for &'a CHATSTATISTICS {}
impl<'a, CHATSTATISTICS: TDChatStatistics> TDChatStatistics for &'a mut CHATSTATISTICS {}

impl<'a, CHATTYPE: TDChatType> TDChatType for &'a CHATTYPE {}
impl<'a, CHATTYPE: TDChatType> TDChatType for &'a mut CHATTYPE {}

impl<'a, CHECKCHATUSERNAMERESULT: TDCheckChatUsernameResult> TDCheckChatUsernameResult
    for &'a CHECKCHATUSERNAMERESULT
{
}
impl<'a, CHECKCHATUSERNAMERESULT: TDCheckChatUsernameResult> TDCheckChatUsernameResult
    for &'a mut CHECKCHATUSERNAMERESULT
{
}

impl<'a, CONNECTIONSTATE: TDConnectionState> TDConnectionState for &'a CONNECTIONSTATE {}
impl<'a, CONNECTIONSTATE: TDConnectionState> TDConnectionState for &'a mut CONNECTIONSTATE {}

impl<'a, DEVICETOKEN: TDDeviceToken> TDDeviceToken for &'a DEVICETOKEN {}
impl<'a, DEVICETOKEN: TDDeviceToken> TDDeviceToken for &'a mut DEVICETOKEN {}

impl<'a, DICESTICKERS: TDDiceStickers> TDDiceStickers for &'a DICESTICKERS {}
impl<'a, DICESTICKERS: TDDiceStickers> TDDiceStickers for &'a mut DICESTICKERS {}

impl<'a, FILETYPE: TDFileType> TDFileType for &'a FILETYPE {}
impl<'a, FILETYPE: TDFileType> TDFileType for &'a mut FILETYPE {}

impl<'a, INLINEKEYBOARDBUTTONTYPE: TDInlineKeyboardButtonType> TDInlineKeyboardButtonType
    for &'a INLINEKEYBOARDBUTTONTYPE
{
}
impl<'a, INLINEKEYBOARDBUTTONTYPE: TDInlineKeyboardButtonType> TDInlineKeyboardButtonType
    for &'a mut INLINEKEYBOARDBUTTONTYPE
{
}

impl<'a, INLINEQUERYRESULT: TDInlineQueryResult> TDInlineQueryResult for &'a INLINEQUERYRESULT {}
impl<'a, INLINEQUERYRESULT: TDInlineQueryResult> TDInlineQueryResult for &'a mut INLINEQUERYRESULT {}

impl<'a, INPUTBACKGROUND: TDInputBackground> TDInputBackground for &'a INPUTBACKGROUND {}
impl<'a, INPUTBACKGROUND: TDInputBackground> TDInputBackground for &'a mut INPUTBACKGROUND {}

impl<'a, INPUTCHATPHOTO: TDInputChatPhoto> TDInputChatPhoto for &'a INPUTCHATPHOTO {}
impl<'a, INPUTCHATPHOTO: TDInputChatPhoto> TDInputChatPhoto for &'a mut INPUTCHATPHOTO {}

impl<'a, INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &'a INPUTCREDENTIALS {}
impl<'a, INPUTCREDENTIALS: TDInputCredentials> TDInputCredentials for &'a mut INPUTCREDENTIALS {}

impl<'a, INPUTFILE: TDInputFile> TDInputFile for &'a INPUTFILE {}
impl<'a, INPUTFILE: TDInputFile> TDInputFile for &'a mut INPUTFILE {}

impl<'a, INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult
    for &'a INPUTINLINEQUERYRESULT
{
}
impl<'a, INPUTINLINEQUERYRESULT: TDInputInlineQueryResult> TDInputInlineQueryResult
    for &'a mut INPUTINLINEQUERYRESULT
{
}

impl<'a, INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent
    for &'a INPUTMESSAGECONTENT
{
}
impl<'a, INPUTMESSAGECONTENT: TDInputMessageContent> TDInputMessageContent
    for &'a mut INPUTMESSAGECONTENT
{
}

impl<'a, INPUTPASSPORTELEMENT: TDInputPassportElement> TDInputPassportElement
    for &'a INPUTPASSPORTELEMENT
{
}
impl<'a, INPUTPASSPORTELEMENT: TDInputPassportElement> TDInputPassportElement
    for &'a mut INPUTPASSPORTELEMENT
{
}

impl<'a, INPUTPASSPORTELEMENTERRORSOURCE: TDInputPassportElementErrorSource>
    TDInputPassportElementErrorSource for &'a INPUTPASSPORTELEMENTERRORSOURCE
{
}
impl<'a, INPUTPASSPORTELEMENTERRORSOURCE: TDInputPassportElementErrorSource>
    TDInputPassportElementErrorSource for &'a mut INPUTPASSPORTELEMENTERRORSOURCE
{
}

impl<'a, INPUTSTICKER: TDInputSticker> TDInputSticker for &'a INPUTSTICKER {}
impl<'a, INPUTSTICKER: TDInputSticker> TDInputSticker for &'a mut INPUTSTICKER {}

impl<'a, JSONVALUE: TDJsonValue> TDJsonValue for &'a JSONVALUE {}
impl<'a, JSONVALUE: TDJsonValue> TDJsonValue for &'a mut JSONVALUE {}

impl<'a, KEYBOARDBUTTONTYPE: TDKeyboardButtonType> TDKeyboardButtonType for &'a KEYBOARDBUTTONTYPE {}
impl<'a, KEYBOARDBUTTONTYPE: TDKeyboardButtonType> TDKeyboardButtonType
    for &'a mut KEYBOARDBUTTONTYPE
{
}

impl<'a, LANGUAGEPACKSTRINGVALUE: TDLanguagePackStringValue> TDLanguagePackStringValue
    for &'a LANGUAGEPACKSTRINGVALUE
{
}
impl<'a, LANGUAGEPACKSTRINGVALUE: TDLanguagePackStringValue> TDLanguagePackStringValue
    for &'a mut LANGUAGEPACKSTRINGVALUE
{
}

impl<'a, LOGSTREAM: TDLogStream> TDLogStream for &'a LOGSTREAM {}
impl<'a, LOGSTREAM: TDLogStream> TDLogStream for &'a mut LOGSTREAM {}

impl<'a, LOGINURLINFO: TDLoginUrlInfo> TDLoginUrlInfo for &'a LOGINURLINFO {}
impl<'a, LOGINURLINFO: TDLoginUrlInfo> TDLoginUrlInfo for &'a mut LOGINURLINFO {}

impl<'a, MASKPOINT: TDMaskPoint> TDMaskPoint for &'a MASKPOINT {}
impl<'a, MASKPOINT: TDMaskPoint> TDMaskPoint for &'a mut MASKPOINT {}

impl<'a, MESSAGECONTENT: TDMessageContent> TDMessageContent for &'a MESSAGECONTENT {}
impl<'a, MESSAGECONTENT: TDMessageContent> TDMessageContent for &'a mut MESSAGECONTENT {}

impl<'a, MESSAGEFORWARDORIGIN: TDMessageForwardOrigin> TDMessageForwardOrigin
    for &'a MESSAGEFORWARDORIGIN
{
}
impl<'a, MESSAGEFORWARDORIGIN: TDMessageForwardOrigin> TDMessageForwardOrigin
    for &'a mut MESSAGEFORWARDORIGIN
{
}

impl<'a, MESSAGESCHEDULINGSTATE: TDMessageSchedulingState> TDMessageSchedulingState
    for &'a MESSAGESCHEDULINGSTATE
{
}
impl<'a, MESSAGESCHEDULINGSTATE: TDMessageSchedulingState> TDMessageSchedulingState
    for &'a mut MESSAGESCHEDULINGSTATE
{
}

impl<'a, MESSAGESENDER: TDMessageSender> TDMessageSender for &'a MESSAGESENDER {}
impl<'a, MESSAGESENDER: TDMessageSender> TDMessageSender for &'a mut MESSAGESENDER {}

impl<'a, MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState
    for &'a MESSAGESENDINGSTATE
{
}
impl<'a, MESSAGESENDINGSTATE: TDMessageSendingState> TDMessageSendingState
    for &'a mut MESSAGESENDINGSTATE
{
}

impl<'a, NETWORKSTATISTICSENTRY: TDNetworkStatisticsEntry> TDNetworkStatisticsEntry
    for &'a NETWORKSTATISTICSENTRY
{
}
impl<'a, NETWORKSTATISTICSENTRY: TDNetworkStatisticsEntry> TDNetworkStatisticsEntry
    for &'a mut NETWORKSTATISTICSENTRY
{
}

impl<'a, NETWORKTYPE: TDNetworkType> TDNetworkType for &'a NETWORKTYPE {}
impl<'a, NETWORKTYPE: TDNetworkType> TDNetworkType for &'a mut NETWORKTYPE {}

impl<'a, NOTIFICATIONGROUPTYPE: TDNotificationGroupType> TDNotificationGroupType
    for &'a NOTIFICATIONGROUPTYPE
{
}
impl<'a, NOTIFICATIONGROUPTYPE: TDNotificationGroupType> TDNotificationGroupType
    for &'a mut NOTIFICATIONGROUPTYPE
{
}

impl<'a, NOTIFICATIONSETTINGSSCOPE: TDNotificationSettingsScope> TDNotificationSettingsScope
    for &'a NOTIFICATIONSETTINGSSCOPE
{
}
impl<'a, NOTIFICATIONSETTINGSSCOPE: TDNotificationSettingsScope> TDNotificationSettingsScope
    for &'a mut NOTIFICATIONSETTINGSSCOPE
{
}

impl<'a, NOTIFICATIONTYPE: TDNotificationType> TDNotificationType for &'a NOTIFICATIONTYPE {}
impl<'a, NOTIFICATIONTYPE: TDNotificationType> TDNotificationType for &'a mut NOTIFICATIONTYPE {}

impl<'a, OPTIONVALUE: TDOptionValue> TDOptionValue for &'a OPTIONVALUE {}
impl<'a, OPTIONVALUE: TDOptionValue> TDOptionValue for &'a mut OPTIONVALUE {}

impl<'a, PAGEBLOCK: TDPageBlock> TDPageBlock for &'a PAGEBLOCK {}
impl<'a, PAGEBLOCK: TDPageBlock> TDPageBlock for &'a mut PAGEBLOCK {}

impl<'a, PAGEBLOCKHORIZONTALALIGNMENT: TDPageBlockHorizontalAlignment>
    TDPageBlockHorizontalAlignment for &'a PAGEBLOCKHORIZONTALALIGNMENT
{
}
impl<'a, PAGEBLOCKHORIZONTALALIGNMENT: TDPageBlockHorizontalAlignment>
    TDPageBlockHorizontalAlignment for &'a mut PAGEBLOCKHORIZONTALALIGNMENT
{
}

impl<'a, PAGEBLOCKVERTICALALIGNMENT: TDPageBlockVerticalAlignment> TDPageBlockVerticalAlignment
    for &'a PAGEBLOCKVERTICALALIGNMENT
{
}
impl<'a, PAGEBLOCKVERTICALALIGNMENT: TDPageBlockVerticalAlignment> TDPageBlockVerticalAlignment
    for &'a mut PAGEBLOCKVERTICALALIGNMENT
{
}

impl<'a, PASSPORTELEMENT: TDPassportElement> TDPassportElement for &'a PASSPORTELEMENT {}
impl<'a, PASSPORTELEMENT: TDPassportElement> TDPassportElement for &'a mut PASSPORTELEMENT {}

impl<'a, PASSPORTELEMENTERRORSOURCE: TDPassportElementErrorSource> TDPassportElementErrorSource
    for &'a PASSPORTELEMENTERRORSOURCE
{
}
impl<'a, PASSPORTELEMENTERRORSOURCE: TDPassportElementErrorSource> TDPassportElementErrorSource
    for &'a mut PASSPORTELEMENTERRORSOURCE
{
}

impl<'a, PASSPORTELEMENTTYPE: TDPassportElementType> TDPassportElementType
    for &'a PASSPORTELEMENTTYPE
{
}
impl<'a, PASSPORTELEMENTTYPE: TDPassportElementType> TDPassportElementType
    for &'a mut PASSPORTELEMENTTYPE
{
}

impl<'a, POLLTYPE: TDPollType> TDPollType for &'a POLLTYPE {}
impl<'a, POLLTYPE: TDPollType> TDPollType for &'a mut POLLTYPE {}

impl<'a, PROXYTYPE: TDProxyType> TDProxyType for &'a PROXYTYPE {}
impl<'a, PROXYTYPE: TDProxyType> TDProxyType for &'a mut PROXYTYPE {}

impl<'a, PUBLICCHATTYPE: TDPublicChatType> TDPublicChatType for &'a PUBLICCHATTYPE {}
impl<'a, PUBLICCHATTYPE: TDPublicChatType> TDPublicChatType for &'a mut PUBLICCHATTYPE {}

impl<'a, PUSHMESSAGECONTENT: TDPushMessageContent> TDPushMessageContent for &'a PUSHMESSAGECONTENT {}
impl<'a, PUSHMESSAGECONTENT: TDPushMessageContent> TDPushMessageContent
    for &'a mut PUSHMESSAGECONTENT
{
}

impl<'a, REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &'a REPLYMARKUP {}
impl<'a, REPLYMARKUP: TDReplyMarkup> TDReplyMarkup for &'a mut REPLYMARKUP {}

impl<'a, RICHTEXT: TDRichText> TDRichText for &'a RICHTEXT {}
impl<'a, RICHTEXT: TDRichText> TDRichText for &'a mut RICHTEXT {}

impl<'a, SEARCHMESSAGESFILTER: TDSearchMessagesFilter> TDSearchMessagesFilter
    for &'a SEARCHMESSAGESFILTER
{
}
impl<'a, SEARCHMESSAGESFILTER: TDSearchMessagesFilter> TDSearchMessagesFilter
    for &'a mut SEARCHMESSAGESFILTER
{
}

impl<'a, SECRETCHATSTATE: TDSecretChatState> TDSecretChatState for &'a SECRETCHATSTATE {}
impl<'a, SECRETCHATSTATE: TDSecretChatState> TDSecretChatState for &'a mut SECRETCHATSTATE {}

impl<'a, STATISTICALGRAPH: TDStatisticalGraph> TDStatisticalGraph for &'a STATISTICALGRAPH {}
impl<'a, STATISTICALGRAPH: TDStatisticalGraph> TDStatisticalGraph for &'a mut STATISTICALGRAPH {}

impl<'a, SUGGESTEDACTION: TDSuggestedAction> TDSuggestedAction for &'a SUGGESTEDACTION {}
impl<'a, SUGGESTEDACTION: TDSuggestedAction> TDSuggestedAction for &'a mut SUGGESTEDACTION {}

impl<'a, SUPERGROUPMEMBERSFILTER: TDSupergroupMembersFilter> TDSupergroupMembersFilter
    for &'a SUPERGROUPMEMBERSFILTER
{
}
impl<'a, SUPERGROUPMEMBERSFILTER: TDSupergroupMembersFilter> TDSupergroupMembersFilter
    for &'a mut SUPERGROUPMEMBERSFILTER
{
}

impl<'a, TMEURLTYPE: TDTMeUrlType> TDTMeUrlType for &'a TMEURLTYPE {}
impl<'a, TMEURLTYPE: TDTMeUrlType> TDTMeUrlType for &'a mut TMEURLTYPE {}

impl<'a, TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &'a TEXTENTITYTYPE {}
impl<'a, TEXTENTITYTYPE: TDTextEntityType> TDTextEntityType for &'a mut TEXTENTITYTYPE {}

impl<'a, TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &'a TEXTPARSEMODE {}
impl<'a, TEXTPARSEMODE: TDTextParseMode> TDTextParseMode for &'a mut TEXTPARSEMODE {}

impl<'a, THUMBNAILFORMAT: TDThumbnailFormat> TDThumbnailFormat for &'a THUMBNAILFORMAT {}
impl<'a, THUMBNAILFORMAT: TDThumbnailFormat> TDThumbnailFormat for &'a mut THUMBNAILFORMAT {}

impl<'a, TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &'a TOPCHATCATEGORY {}
impl<'a, TOPCHATCATEGORY: TDTopChatCategory> TDTopChatCategory for &'a mut TOPCHATCATEGORY {}

impl<'a, UPDATE: TDUpdate> TDUpdate for &'a UPDATE {}
impl<'a, UPDATE: TDUpdate> TDUpdate for &'a mut UPDATE {}

impl<'a, USERPRIVACYSETTING: TDUserPrivacySetting> TDUserPrivacySetting for &'a USERPRIVACYSETTING {}
impl<'a, USERPRIVACYSETTING: TDUserPrivacySetting> TDUserPrivacySetting
    for &'a mut USERPRIVACYSETTING
{
}

impl<'a, USERPRIVACYSETTINGRULE: TDUserPrivacySettingRule> TDUserPrivacySettingRule
    for &'a USERPRIVACYSETTINGRULE
{
}
impl<'a, USERPRIVACYSETTINGRULE: TDUserPrivacySettingRule> TDUserPrivacySettingRule
    for &'a mut USERPRIVACYSETTINGRULE
{
}

impl<'a, USERSTATUS: TDUserStatus> TDUserStatus for &'a USERSTATUS {}
impl<'a, USERSTATUS: TDUserStatus> TDUserStatus for &'a mut USERSTATUS {}

impl<'a, USERTYPE: TDUserType> TDUserType for &'a USERTYPE {}
impl<'a, USERTYPE: TDUserType> TDUserType for &'a mut USERTYPE {}

#[derive(Debug, Clone)]
pub enum TdType {
    TestUseUpdate(TestUseUpdate),
    UpdateActiveNotifications(UpdateActiveNotifications),
    UpdateAnimationSearchParameters(UpdateAnimationSearchParameters),
    UpdateAuthorizationState(UpdateAuthorizationState),
    UpdateBasicGroup(UpdateBasicGroup),
    UpdateBasicGroupFullInfo(UpdateBasicGroupFullInfo),
    UpdateCall(UpdateCall),
    UpdateChatActionBar(UpdateChatActionBar),
    UpdateChatDefaultDisableNotification(UpdateChatDefaultDisableNotification),
    UpdateChatDraftMessage(UpdateChatDraftMessage),
    UpdateChatFilters(UpdateChatFilters),
    UpdateChatHasScheduledMessages(UpdateChatHasScheduledMessages),
    UpdateChatIsBlocked(UpdateChatIsBlocked),
    UpdateChatIsMarkedAsUnread(UpdateChatIsMarkedAsUnread),
    UpdateChatLastMessage(UpdateChatLastMessage),
    UpdateChatNotificationSettings(UpdateChatNotificationSettings),
    UpdateChatOnlineMemberCount(UpdateChatOnlineMemberCount),
    UpdateChatPermissions(UpdateChatPermissions),
    UpdateChatPhoto(UpdateChatPhoto),
    UpdateChatPosition(UpdateChatPosition),
    UpdateChatReadInbox(UpdateChatReadInbox),
    UpdateChatReadOutbox(UpdateChatReadOutbox),
    UpdateChatReplyMarkup(UpdateChatReplyMarkup),
    UpdateChatTitle(UpdateChatTitle),
    UpdateChatUnreadMentionCount(UpdateChatUnreadMentionCount),
    UpdateConnectionState(UpdateConnectionState),
    UpdateDeleteMessages(UpdateDeleteMessages),
    UpdateDiceEmojis(UpdateDiceEmojis),
    UpdateFavoriteStickers(UpdateFavoriteStickers),
    UpdateFile(UpdateFile),
    UpdateFileGenerationStart(UpdateFileGenerationStart),
    UpdateFileGenerationStop(UpdateFileGenerationStop),
    UpdateHavePendingNotifications(UpdateHavePendingNotifications),
    UpdateInstalledStickerSets(UpdateInstalledStickerSets),
    UpdateLanguagePackStrings(UpdateLanguagePackStrings),
    UpdateMessageContent(UpdateMessageContent),
    UpdateMessageContentOpened(UpdateMessageContentOpened),
    UpdateMessageEdited(UpdateMessageEdited),
    UpdateMessageInteractionInfo(UpdateMessageInteractionInfo),
    UpdateMessageIsPinned(UpdateMessageIsPinned),
    UpdateMessageLiveLocationViewed(UpdateMessageLiveLocationViewed),
    UpdateMessageMentionRead(UpdateMessageMentionRead),
    UpdateMessageSendAcknowledged(UpdateMessageSendAcknowledged),
    UpdateMessageSendFailed(UpdateMessageSendFailed),
    UpdateMessageSendSucceeded(UpdateMessageSendSucceeded),
    UpdateNewCallSignalingData(UpdateNewCallSignalingData),
    UpdateNewCallbackQuery(UpdateNewCallbackQuery),
    UpdateNewChat(UpdateNewChat),
    UpdateNewChosenInlineResult(UpdateNewChosenInlineResult),
    UpdateNewCustomEvent(UpdateNewCustomEvent),
    UpdateNewCustomQuery(UpdateNewCustomQuery),
    UpdateNewInlineCallbackQuery(UpdateNewInlineCallbackQuery),
    UpdateNewInlineQuery(UpdateNewInlineQuery),
    UpdateNewMessage(UpdateNewMessage),
    UpdateNewPreCheckoutQuery(UpdateNewPreCheckoutQuery),
    UpdateNewShippingQuery(UpdateNewShippingQuery),
    UpdateNotification(UpdateNotification),
    UpdateNotificationGroup(UpdateNotificationGroup),
    UpdateOption(UpdateOption),
    UpdatePoll(UpdatePoll),
    UpdatePollAnswer(UpdatePollAnswer),
    UpdateRecentStickers(UpdateRecentStickers),
    UpdateSavedAnimations(UpdateSavedAnimations),
    UpdateScopeNotificationSettings(UpdateScopeNotificationSettings),
    UpdateSecretChat(UpdateSecretChat),
    UpdateSelectedBackground(UpdateSelectedBackground),
    UpdateServiceNotification(UpdateServiceNotification),
    UpdateStickerSet(UpdateStickerSet),
    UpdateSuggestedActions(UpdateSuggestedActions),
    UpdateSupergroup(UpdateSupergroup),
    UpdateSupergroupFullInfo(UpdateSupergroupFullInfo),
    UpdateTermsOfService(UpdateTermsOfService),
    UpdateTrendingStickerSets(UpdateTrendingStickerSets),
    UpdateUnreadChatCount(UpdateUnreadChatCount),
    UpdateUnreadMessageCount(UpdateUnreadMessageCount),
    UpdateUser(UpdateUser),
    UpdateUserChatAction(UpdateUserChatAction),
    UpdateUserFullInfo(UpdateUserFullInfo),
    UpdateUserPrivacySettingRules(UpdateUserPrivacySettingRules),
    UpdateUserStatus(UpdateUserStatus),
    UpdateUsersNearby(UpdateUsersNearby),

    AuthorizationState(AuthorizationState),
    CanTransferOwnershipResult(CanTransferOwnershipResult),
    ChatStatistics(ChatStatistics),
    CheckChatUsernameResult(CheckChatUsernameResult),
    JsonValue(JsonValue),
    LanguagePackStringValue(LanguagePackStringValue),
    LogStream(LogStream),
    LoginUrlInfo(LoginUrlInfo),
    OptionValue(OptionValue),
    PassportElement(PassportElement),
    StatisticalGraph(StatisticalGraph),
    Update(Update),
    AccountTtl(AccountTtl),
    Animations(Animations),
    AuthenticationCodeInfo(AuthenticationCodeInfo),
    AutoDownloadSettingsPresets(AutoDownloadSettingsPresets),
    Background(Background),
    Backgrounds(Backgrounds),
    BankCardInfo(BankCardInfo),
    BasicGroup(BasicGroup),
    BasicGroupFullInfo(BasicGroupFullInfo),
    CallId(CallId),
    CallbackQueryAnswer(CallbackQueryAnswer),
    Chat(Chat),
    ChatAdministrators(ChatAdministrators),
    ChatEvents(ChatEvents),
    ChatFilter(ChatFilter),
    ChatFilterInfo(ChatFilterInfo),
    ChatInviteLink(ChatInviteLink),
    ChatInviteLinkInfo(ChatInviteLinkInfo),
    ChatLists(ChatLists),
    ChatMember(ChatMember),
    ChatMembers(ChatMembers),
    ChatPhotos(ChatPhotos),
    Chats(Chats),
    ChatsNearby(ChatsNearby),
    ConnectedWebsites(ConnectedWebsites),
    Count(Count),
    Countries(Countries),
    CustomRequestResult(CustomRequestResult),
    DatabaseStatistics(DatabaseStatistics),
    DeepLinkInfo(DeepLinkInfo),
    EmailAddressAuthenticationCodeInfo(EmailAddressAuthenticationCodeInfo),
    Emojis(Emojis),
    Error(Error),
    File(File),
    FilePart(FilePart),
    FormattedText(FormattedText),
    FoundMessages(FoundMessages),
    GameHighScores(GameHighScores),
    Hashtags(Hashtags),
    HttpUrl(HttpUrl),
    ImportedContacts(ImportedContacts),
    InlineQueryResults(InlineQueryResults),
    LanguagePackInfo(LanguagePackInfo),
    LanguagePackStrings(LanguagePackStrings),
    LocalizationTargetInfo(LocalizationTargetInfo),
    LogTags(LogTags),
    LogVerbosityLevel(LogVerbosityLevel),
    Message(Message),
    MessageLink(MessageLink),
    MessageLinkInfo(MessageLinkInfo),
    MessageSenders(MessageSenders),
    MessageStatistics(MessageStatistics),
    MessageThreadInfo(MessageThreadInfo),
    Messages(Messages),
    NetworkStatistics(NetworkStatistics),
    Ok(Ok),
    OrderInfo(OrderInfo),
    PassportAuthorizationForm(PassportAuthorizationForm),
    PassportElements(PassportElements),
    PassportElementsWithErrors(PassportElementsWithErrors),
    PasswordState(PasswordState),
    PaymentForm(PaymentForm),
    PaymentReceipt(PaymentReceipt),
    PaymentResult(PaymentResult),
    PhoneNumberInfo(PhoneNumberInfo),
    Proxies(Proxies),
    Proxy(Proxy),
    PushReceiverId(PushReceiverId),
    RecommendedChatFilters(RecommendedChatFilters),
    RecoveryEmailAddress(RecoveryEmailAddress),
    ScopeNotificationSettings(ScopeNotificationSettings),
    Seconds(Seconds),
    SecretChat(SecretChat),
    Session(Session),
    Sessions(Sessions),
    StickerSet(StickerSet),
    StickerSets(StickerSets),
    Stickers(Stickers),
    StorageStatistics(StorageStatistics),
    StorageStatisticsFast(StorageStatisticsFast),
    Supergroup(Supergroup),
    SupergroupFullInfo(SupergroupFullInfo),
    TMeUrls(TMeUrls),
    TemporaryPasswordState(TemporaryPasswordState),
    TestBytes(TestBytes),
    TestInt(TestInt),
    TestString(TestString),
    TestVectorInt(TestVectorInt),
    TestVectorIntObject(TestVectorIntObject),
    TestVectorString(TestVectorString),
    TestVectorStringObject(TestVectorStringObject),
    Text(Text),
    TextEntities(TextEntities),
    Updates(Updates),
    User(User),
    UserFullInfo(UserFullInfo),
    UserPrivacySettingRules(UserPrivacySettingRules),
    Users(Users),
    ValidatedOrderInfo(ValidatedOrderInfo),
    WebPage(WebPage),
    WebPageInstantView(WebPageInstantView),
}
impl<'de> Deserialize<'de> for TdType {
    fn deserialize<D>(deserializer: D) -> Result<TdType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
             TdType,
         (testUseUpdate, TestUseUpdate);
         (updateActiveNotifications, UpdateActiveNotifications);
         (updateAnimationSearchParameters, UpdateAnimationSearchParameters);
         (updateAuthorizationState, UpdateAuthorizationState);
         (updateBasicGroup, UpdateBasicGroup);
         (updateBasicGroupFullInfo, UpdateBasicGroupFullInfo);
         (updateCall, UpdateCall);
         (updateChatActionBar, UpdateChatActionBar);
         (updateChatDefaultDisableNotification, UpdateChatDefaultDisableNotification);
         (updateChatDraftMessage, UpdateChatDraftMessage);
         (updateChatFilters, UpdateChatFilters);
         (updateChatHasScheduledMessages, UpdateChatHasScheduledMessages);
         (updateChatIsBlocked, UpdateChatIsBlocked);
         (updateChatIsMarkedAsUnread, UpdateChatIsMarkedAsUnread);
         (updateChatLastMessage, UpdateChatLastMessage);
         (updateChatNotificationSettings, UpdateChatNotificationSettings);
         (updateChatOnlineMemberCount, UpdateChatOnlineMemberCount);
         (updateChatPermissions, UpdateChatPermissions);
         (updateChatPhoto, UpdateChatPhoto);
         (updateChatPosition, UpdateChatPosition);
         (updateChatReadInbox, UpdateChatReadInbox);
         (updateChatReadOutbox, UpdateChatReadOutbox);
         (updateChatReplyMarkup, UpdateChatReplyMarkup);
         (updateChatTitle, UpdateChatTitle);
         (updateChatUnreadMentionCount, UpdateChatUnreadMentionCount);
         (updateConnectionState, UpdateConnectionState);
         (updateDeleteMessages, UpdateDeleteMessages);
         (updateDiceEmojis, UpdateDiceEmojis);
         (updateFavoriteStickers, UpdateFavoriteStickers);
         (updateFile, UpdateFile);
         (updateFileGenerationStart, UpdateFileGenerationStart);
         (updateFileGenerationStop, UpdateFileGenerationStop);
         (updateHavePendingNotifications, UpdateHavePendingNotifications);
         (updateInstalledStickerSets, UpdateInstalledStickerSets);
         (updateLanguagePackStrings, UpdateLanguagePackStrings);
         (updateMessageContent, UpdateMessageContent);
         (updateMessageContentOpened, UpdateMessageContentOpened);
         (updateMessageEdited, UpdateMessageEdited);
         (updateMessageInteractionInfo, UpdateMessageInteractionInfo);
         (updateMessageIsPinned, UpdateMessageIsPinned);
         (updateMessageLiveLocationViewed, UpdateMessageLiveLocationViewed);
         (updateMessageMentionRead, UpdateMessageMentionRead);
         (updateMessageSendAcknowledged, UpdateMessageSendAcknowledged);
         (updateMessageSendFailed, UpdateMessageSendFailed);
         (updateMessageSendSucceeded, UpdateMessageSendSucceeded);
         (updateNewCallSignalingData, UpdateNewCallSignalingData);
         (updateNewCallbackQuery, UpdateNewCallbackQuery);
         (updateNewChat, UpdateNewChat);
         (updateNewChosenInlineResult, UpdateNewChosenInlineResult);
         (updateNewCustomEvent, UpdateNewCustomEvent);
         (updateNewCustomQuery, UpdateNewCustomQuery);
         (updateNewInlineCallbackQuery, UpdateNewInlineCallbackQuery);
         (updateNewInlineQuery, UpdateNewInlineQuery);
         (updateNewMessage, UpdateNewMessage);
         (updateNewPreCheckoutQuery, UpdateNewPreCheckoutQuery);
         (updateNewShippingQuery, UpdateNewShippingQuery);
         (updateNotification, UpdateNotification);
         (updateNotificationGroup, UpdateNotificationGroup);
         (updateOption, UpdateOption);
         (updatePoll, UpdatePoll);
         (updatePollAnswer, UpdatePollAnswer);
         (updateRecentStickers, UpdateRecentStickers);
         (updateSavedAnimations, UpdateSavedAnimations);
         (updateScopeNotificationSettings, UpdateScopeNotificationSettings);
         (updateSecretChat, UpdateSecretChat);
         (updateSelectedBackground, UpdateSelectedBackground);
         (updateServiceNotification, UpdateServiceNotification);
         (updateStickerSet, UpdateStickerSet);
         (updateSuggestedActions, UpdateSuggestedActions);
         (updateSupergroup, UpdateSupergroup);
         (updateSupergroupFullInfo, UpdateSupergroupFullInfo);
         (updateTermsOfService, UpdateTermsOfService);
         (updateTrendingStickerSets, UpdateTrendingStickerSets);
         (updateUnreadChatCount, UpdateUnreadChatCount);
         (updateUnreadMessageCount, UpdateUnreadMessageCount);
         (updateUser, UpdateUser);
         (updateUserChatAction, UpdateUserChatAction);
         (updateUserFullInfo, UpdateUserFullInfo);
         (updateUserPrivacySettingRules, UpdateUserPrivacySettingRules);
         (updateUserStatus, UpdateUserStatus);
         (updateUsersNearby, UpdateUsersNearby);

         (AuthorizationState, AuthorizationState);
         (CanTransferOwnershipResult, CanTransferOwnershipResult);
         (ChatStatistics, ChatStatistics);
         (CheckChatUsernameResult, CheckChatUsernameResult);
         (JsonValue, JsonValue);
         (LanguagePackStringValue, LanguagePackStringValue);
         (LogStream, LogStream);
         (LoginUrlInfo, LoginUrlInfo);
         (OptionValue, OptionValue);
         (PassportElement, PassportElement);
         (StatisticalGraph, StatisticalGraph);
         (Update, Update);
         (accountTtl, AccountTtl);
         (animations, Animations);
         (authenticationCodeInfo, AuthenticationCodeInfo);
         (autoDownloadSettingsPresets, AutoDownloadSettingsPresets);
         (background, Background);
         (backgrounds, Backgrounds);
         (bankCardInfo, BankCardInfo);
         (basicGroup, BasicGroup);
         (basicGroupFullInfo, BasicGroupFullInfo);
         (callId, CallId);
         (callbackQueryAnswer, CallbackQueryAnswer);
         (chat, Chat);
         (chatAdministrators, ChatAdministrators);
         (chatEvents, ChatEvents);
         (chatFilter, ChatFilter);
         (chatFilterInfo, ChatFilterInfo);
         (chatInviteLink, ChatInviteLink);
         (chatInviteLinkInfo, ChatInviteLinkInfo);
         (chatLists, ChatLists);
         (chatMember, ChatMember);
         (chatMembers, ChatMembers);
         (chatPhotos, ChatPhotos);
         (chats, Chats);
         (chatsNearby, ChatsNearby);
         (connectedWebsites, ConnectedWebsites);
         (count, Count);
         (countries, Countries);
         (customRequestResult, CustomRequestResult);
         (databaseStatistics, DatabaseStatistics);
         (deepLinkInfo, DeepLinkInfo);
         (emailAddressAuthenticationCodeInfo, EmailAddressAuthenticationCodeInfo);
         (emojis, Emojis);
         (error, Error);
         (file, File);
         (filePart, FilePart);
         (formattedText, FormattedText);
         (foundMessages, FoundMessages);
         (gameHighScores, GameHighScores);
         (hashtags, Hashtags);
         (httpUrl, HttpUrl);
         (importedContacts, ImportedContacts);
         (inlineQueryResults, InlineQueryResults);
         (languagePackInfo, LanguagePackInfo);
         (languagePackStrings, LanguagePackStrings);
         (localizationTargetInfo, LocalizationTargetInfo);
         (logTags, LogTags);
         (logVerbosityLevel, LogVerbosityLevel);
         (message, Message);
         (messageLink, MessageLink);
         (messageLinkInfo, MessageLinkInfo);
         (messageSenders, MessageSenders);
         (messageStatistics, MessageStatistics);
         (messageThreadInfo, MessageThreadInfo);
         (messages, Messages);
         (networkStatistics, NetworkStatistics);
         (ok, Ok);
         (orderInfo, OrderInfo);
         (passportAuthorizationForm, PassportAuthorizationForm);
         (passportElements, PassportElements);
         (passportElementsWithErrors, PassportElementsWithErrors);
         (passwordState, PasswordState);
         (paymentForm, PaymentForm);
         (paymentReceipt, PaymentReceipt);
         (paymentResult, PaymentResult);
         (phoneNumberInfo, PhoneNumberInfo);
         (proxies, Proxies);
         (proxy, Proxy);
         (pushReceiverId, PushReceiverId);
         (recommendedChatFilters, RecommendedChatFilters);
         (recoveryEmailAddress, RecoveryEmailAddress);
         (scopeNotificationSettings, ScopeNotificationSettings);
         (seconds, Seconds);
         (secretChat, SecretChat);
         (session, Session);
         (sessions, Sessions);
         (stickerSet, StickerSet);
         (stickerSets, StickerSets);
         (stickers, Stickers);
         (storageStatistics, StorageStatistics);
         (storageStatisticsFast, StorageStatisticsFast);
         (supergroup, Supergroup);
         (supergroupFullInfo, SupergroupFullInfo);
         (tMeUrls, TMeUrls);
         (temporaryPasswordState, TemporaryPasswordState);
         (testBytes, TestBytes);
         (testInt, TestInt);
         (testString, TestString);
         (testVectorInt, TestVectorInt);
         (testVectorIntObject, TestVectorIntObject);
         (testVectorString, TestVectorString);
         (testVectorStringObject, TestVectorStringObject);
         (text, Text);
         (textEntities, TextEntities);
         (updates, Updates);
         (user, User);
         (userFullInfo, UserFullInfo);
         (userPrivacySettingRules, UserPrivacySettingRules);
         (users, Users);
         (validatedOrderInfo, ValidatedOrderInfo);
         (webPage, WebPage);
         (webPageInstantView, WebPageInstantView);

        )(deserializer)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{from_json, TdType, UpdateAuthorizationState};

    #[test]
    fn test_deserialize_enum() {
        match from_json::<UpdateAuthorizationState>(
            r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#,
        ) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e)
            }
        };

        match from_json::<TdType>(
            r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#,
        ) {
            Ok(t) => match t {
                TdType::UpdateAuthorizationState(_) => {}
                _ => panic!("from_json failed: {:?}", t),
            },
            Err(e) => {
                panic!("{}", e)
            }
        };
    }
}

pub(super) fn number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub fn vec_of_i64_from_str<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Vec::<String>::deserialize(deserializer)?;
    let mut r = Vec::new();
    for v in s {
        match v.parse::<i64>() {
            Ok(v) => r.push(v),
            Err(e) => return Err(D::Error::custom(format!("can't deserialize to i64: {}", e))),
        }
    }
    Ok(r)
}
