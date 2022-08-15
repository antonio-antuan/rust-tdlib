use std::fmt::{Debug, Display};
use std::str::FromStr;

use serde::{
    de::{Deserialize, Deserializer, Error as SerdeDeError},
    ser::{Serialize, Serializer},
};

use crate::{errors::*, types::*};

#[allow(dead_code)]
pub fn from_json<'a, T>(json: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    Ok(serde_json::from_str(json)?)
}

/// All tdlib type abstract class defined the same behavior
pub trait RObject: Debug {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str>;
    fn client_id(&self) -> Option<i32>;
}

pub trait RFunction: Debug + RObject + Serialize {
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl<'a, RObj: RObject> RObject for &'a RObj {
    fn extra(&self) -> Option<&str> {
        (*self).extra()
    }
    fn client_id(&self) -> Option<i32> {
        (*self).client_id()
    }
}

impl<'a, RObj: RObject> RObject for &'a mut RObj {
    fn extra(&self) -> Option<&str> {
        (**self).extra()
    }
    fn client_id(&self) -> Option<i32> {
        (**self).client_id()
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

impl<'a, BOTCOMMANDSCOPE: TDBotCommandScope> TDBotCommandScope for &'a BOTCOMMANDSCOPE {}
impl<'a, BOTCOMMANDSCOPE: TDBotCommandScope> TDBotCommandScope for &'a mut BOTCOMMANDSCOPE {}

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

impl<'a, CHECKSTICKERSETNAMERESULT: TDCheckStickerSetNameResult> TDCheckStickerSetNameResult
    for &'a CHECKSTICKERSETNAMERESULT
{
}
impl<'a, CHECKSTICKERSETNAMERESULT: TDCheckStickerSetNameResult> TDCheckStickerSetNameResult
    for &'a mut CHECKSTICKERSETNAMERESULT
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

impl<'a, GROUPCALLVIDEOQUALITY: TDGroupCallVideoQuality> TDGroupCallVideoQuality
    for &'a GROUPCALLVIDEOQUALITY
{
}
impl<'a, GROUPCALLVIDEOQUALITY: TDGroupCallVideoQuality> TDGroupCallVideoQuality
    for &'a mut GROUPCALLVIDEOQUALITY
{
}

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

impl<'a, INTERNALLINKTYPE: TDInternalLinkType> TDInternalLinkType for &'a INTERNALLINKTYPE {}
impl<'a, INTERNALLINKTYPE: TDInternalLinkType> TDInternalLinkType for &'a mut INTERNALLINKTYPE {}

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

impl<'a, MESSAGEFILETYPE: TDMessageFileType> TDMessageFileType for &'a MESSAGEFILETYPE {}
impl<'a, MESSAGEFILETYPE: TDMessageFileType> TDMessageFileType for &'a mut MESSAGEFILETYPE {}

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

impl<'a, RESETPASSWORDRESULT: TDResetPasswordResult> TDResetPasswordResult
    for &'a RESETPASSWORDRESULT
{
}
impl<'a, RESETPASSWORDRESULT: TDResetPasswordResult> TDResetPasswordResult
    for &'a mut RESETPASSWORDRESULT
{
}

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

impl<'a, VECTORPATHCOMMAND: TDVectorPathCommand> TDVectorPathCommand for &'a VECTORPATHCOMMAND {}
impl<'a, VECTORPATHCOMMAND: TDVectorPathCommand> TDVectorPathCommand for &'a mut VECTORPATHCOMMAND {}

pub(super) fn number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(SerdeDeError::custom)
}

pub(super) fn string_to_number<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    serializer.collect_str(value)
}

pub(super) fn vec_of_i64_from_str<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
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

#[cfg(test)]
mod tests {
    use crate::types::{
        ClosedVectorPath, File, Message, MessageContent, Sticker, Thumbnail, Update,
    };
    use serde::de::DeserializeOwned;

    #[test]
    fn test_deserialize_closed_vector_path() {
        assert(
            r#"[{"@type":"closedVectorPath","commands":[{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":196.250000,"y":296.250000},"end_control_point":{"@type":"point","x":175.625000,"y":296.875000},"end_point":{"@type":"point","x":153.750000,"y":295.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":141.250000,"y":294.375000},"end_control_point":{"@type":"point","x":128.125000,"y":291.250000},"end_point":{"@type":"point","x":117.500000,"y":285.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":113.750000,"y":283.750000},"end_control_point":{"@type":"point","x":109.375000,"y":279.375000},"end_point":{"@type":"point","x":105.625000,"y":278.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":96.250000,"y":276.250000},"end_control_point":{"@type":"point","x":84.375000,"y":284.375000},"end_point":{"@type":"point","x":75.000000,"y":283.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":51.875000,"y":280.000000},"end_control_point":{"@type":"point","x":43.125000,"y":251.250000},"end_point":{"@type":"point","x":40.625000,"y":231.875000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":38.125000,"y":211.250000},"end_control_point":{"@type":"point","x":41.250000,"y":178.125000},"end_point":{"@type":"point","x":59.375000,"y":163.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":69.375000,"y":156.250000},"end_control_point":{"@type":"point","x":84.375000,"y":163.125000},"end_point":{"@type":"point","x":96.250000,"y":156.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":105.000000,"y":150.625000},"end_control_point":{"@type":"point","x":108.750000,"y":138.125000},"end_point":{"@type":"point","x":115.625000,"y":130.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":125.625000,"y":119.375000},"end_control_point":{"@type":"point","x":138.125000,"y":110.625000},"end_point":{"@type":"point","x":147.500000,"y":98.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":160.625000,"y":82.500000},"end_control_point":{"@type":"point","x":135.625000,"y":38.125000},"end_point":{"@type":"point","x":168.750000,"y":38.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":173.750000,"y":38.125000},"end_control_point":{"@type":"point","x":178.750000,"y":40.000000},"end_point":{"@type":"point","x":183.125000,"y":43.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":221.875000,"y":73.125000},"end_control_point":{"@type":"point","x":183.750000,"y":112.500000},"end_point":{"@type":"point","x":188.125000,"y":136.875000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":189.375000,"y":143.750000},"end_control_point":{"@type":"point","x":203.125000,"y":143.125000},"end_point":{"@type":"point","x":208.125000,"y":143.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":230.625000,"y":141.875000},"end_control_point":{"@type":"point","x":271.250000,"y":141.250000},"end_point":{"@type":"point","x":280.000000,"y":168.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":284.375000,"y":181.875000},"end_control_point":{"@type":"point","x":277.500000,"y":186.875000},"end_point":{"@type":"point","x":277.500000,"y":196.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":277.500000,"y":198.750000},"end_control_point":{"@type":"point","x":287.500000,"y":209.375000},"end_point":{"@type":"point","x":284.375000,"y":221.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":283.125000,"y":225.000000},"end_control_point":{"@type":"point","x":281.250000,"y":228.125000},"end_point":{"@type":"point","x":279.375000,"y":231.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":279.375000,"y":231.250000},"end_control_point":{"@type":"point","x":273.125000,"y":235.625000},"end_point":{"@type":"point","x":273.125000,"y":236.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":276.250000,"y":246.875000},"end_control_point":{"@type":"point","x":276.875000,"y":257.500000},"end_point":{"@type":"point","x":267.500000,"y":265.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":265.625000,"y":267.500000},"end_control_point":{"@type":"point","x":262.500000,"y":267.500000},"end_point":{"@type":"point","x":261.250000,"y":270.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":260.625000,"y":273.125000},"end_control_point":{"@type":"point","x":261.875000,"y":277.500000},"end_point":{"@type":"point","x":260.000000,"y":280.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":255.000000,"y":298.125000},"end_control_point":{"@type":"point","x":233.750000,"y":301.875000},"end_point":{"@type":"point","x":218.125000,"y":301.875000}}]}]"#,
            |res: &serde_json::Result<Vec<ClosedVectorPath>>| res.is_ok(),
        )
    }

    #[test]
    fn test_deserialize_thumbnail() {
        assert(
            r#"{"@type":"thumbnail","format":{"@type":"thumbnailFormatWebp"},"width":101,"height":128,"file":{"@type":"file","id":573,"size":2672,"expected_size":2672,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AAMCAQADFQABYr34JQdBM97Uqr0NPPmL7BZvomIAAs0BAAJS-jFFL2KLTiLKoDwBAAdtAAMjBA","unique_id":"AQADzQEAAlL6MUVy","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":2672}}}"#,
            |res: &serde_json::Result<Thumbnail>| res.is_ok(),
        )
    }

    #[test]
    fn test_deserialize_file() {
        assert(
            r#"{"@type":"file","id":572,"size":18100,"expected_size":18100,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"CAACAgEAAxUAAWK9-CUHQTPe1Kq9DTz5i-wWb6JiAALNAQACUvoxRS9ii04iyqA8IwQ","unique_id":"AgADzQEAAlL6MUU","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":18100}}"#,
            |res: &serde_json::Result<File>| res.is_ok(),
        )
    }

    #[test]
    fn test_deserialize_sticker() {
        assert(
            r#"{"@type":"sticker","set_id":"1258816259751983","width":300,"height":380,"emoji":"\ud83d\udc4d","is_animated":true,"is_mask":false,"outline":[{"@type":"closedVectorPath","commands":[{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":196.250000,"y":296.250000},"end_control_point":{"@type":"point","x":175.625000,"y":296.875000},"end_point":{"@type":"point","x":153.750000,"y":295.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":141.250000,"y":294.375000},"end_control_point":{"@type":"point","x":128.125000,"y":291.250000},"end_point":{"@type":"point","x":117.500000,"y":285.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":113.750000,"y":283.750000},"end_control_point":{"@type":"point","x":109.375000,"y":279.375000},"end_point":{"@type":"point","x":105.625000,"y":278.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":96.250000,"y":276.250000},"end_control_point":{"@type":"point","x":84.375000,"y":284.375000},"end_point":{"@type":"point","x":75.000000,"y":283.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":51.875000,"y":280.000000},"end_control_point":{"@type":"point","x":43.125000,"y":251.250000},"end_point":{"@type":"point","x":40.625000,"y":231.875000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":38.125000,"y":211.250000},"end_control_point":{"@type":"point","x":41.250000,"y":178.125000},"end_point":{"@type":"point","x":59.375000,"y":163.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":69.375000,"y":156.250000},"end_control_point":{"@type":"point","x":84.375000,"y":163.125000},"end_point":{"@type":"point","x":96.250000,"y":156.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":105.000000,"y":150.625000},"end_control_point":{"@type":"point","x":108.750000,"y":138.125000},"end_point":{"@type":"point","x":115.625000,"y":130.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":125.625000,"y":119.375000},"end_control_point":{"@type":"point","x":138.125000,"y":110.625000},"end_point":{"@type":"point","x":147.500000,"y":98.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":160.625000,"y":82.500000},"end_control_point":{"@type":"point","x":135.625000,"y":38.125000},"end_point":{"@type":"point","x":168.750000,"y":38.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":173.750000,"y":38.125000},"end_control_point":{"@type":"point","x":178.750000,"y":40.000000},"end_point":{"@type":"point","x":183.125000,"y":43.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":221.875000,"y":73.125000},"end_control_point":{"@type":"point","x":183.750000,"y":112.500000},"end_point":{"@type":"point","x":188.125000,"y":136.875000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":189.375000,"y":143.750000},"end_control_point":{"@type":"point","x":203.125000,"y":143.125000},"end_point":{"@type":"point","x":208.125000,"y":143.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":230.625000,"y":141.875000},"end_control_point":{"@type":"point","x":271.250000,"y":141.250000},"end_point":{"@type":"point","x":280.000000,"y":168.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":284.375000,"y":181.875000},"end_control_point":{"@type":"point","x":277.500000,"y":186.875000},"end_point":{"@type":"point","x":277.500000,"y":196.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":277.500000,"y":198.750000},"end_control_point":{"@type":"point","x":287.500000,"y":209.375000},"end_point":{"@type":"point","x":284.375000,"y":221.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":283.125000,"y":225.000000},"end_control_point":{"@type":"point","x":281.250000,"y":228.125000},"end_point":{"@type":"point","x":279.375000,"y":231.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":279.375000,"y":231.250000},"end_control_point":{"@type":"point","x":273.125000,"y":235.625000},"end_point":{"@type":"point","x":273.125000,"y":236.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":276.250000,"y":246.875000},"end_control_point":{"@type":"point","x":276.875000,"y":257.500000},"end_point":{"@type":"point","x":267.500000,"y":265.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":265.625000,"y":267.500000},"end_control_point":{"@type":"point","x":262.500000,"y":267.500000},"end_point":{"@type":"point","x":261.250000,"y":270.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":260.625000,"y":273.125000},"end_control_point":{"@type":"point","x":261.875000,"y":277.500000},"end_point":{"@type":"point","x":260.000000,"y":280.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":255.000000,"y":298.125000},"end_control_point":{"@type":"point","x":233.750000,"y":301.875000},"end_point":{"@type":"point","x":218.125000,"y":301.875000}}]}],"thumbnail":{"@type":"thumbnail","format":{"@type":"thumbnailFormatWebp"},"width":101,"height":128,"file":{"@type":"file","id":573,"size":2672,"expected_size":2672,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AAMCAQADFQABYr34JQdBM97Uqr0NPPmL7BZvomIAAs0BAAJS-jFFL2KLTiLKoDwBAAdtAAMjBA","unique_id":"AQADzQEAAlL6MUVy","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":2672}}},"sticker":{"@type":"file","id":572,"size":18100,"expected_size":18100,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"CAACAgEAAxUAAWK9-CUHQTPe1Kq9DTz5i-wWb6JiAALNAQACUvoxRS9ii04iyqA8IwQ","unique_id":"AgADzQEAAlL6MUU","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":18100}}}"#,
            |res: &serde_json::Result<Sticker>| res.is_ok(),
        )
    }

    #[test]
    fn test_deserialize_message_animated_emoji() {
        assert(
            r#"{"@type":"messageAnimatedEmoji","animated_emoji":{"@type":"animatedEmoji","sticker":{"@type":"sticker","set_id":"1258816259751983","width":300,"height":380,"emoji":"\ud83d\udc4d","is_animated":true,"is_mask":false,"outline":[{"@type":"closedVectorPath","commands":[{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":196.250000,"y":296.250000},"end_control_point":{"@type":"point","x":175.625000,"y":296.875000},"end_point":{"@type":"point","x":153.750000,"y":295.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":141.250000,"y":294.375000},"end_control_point":{"@type":"point","x":128.125000,"y":291.250000},"end_point":{"@type":"point","x":117.500000,"y":285.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":113.750000,"y":283.750000},"end_control_point":{"@type":"point","x":109.375000,"y":279.375000},"end_point":{"@type":"point","x":105.625000,"y":278.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":96.250000,"y":276.250000},"end_control_point":{"@type":"point","x":84.375000,"y":284.375000},"end_point":{"@type":"point","x":75.000000,"y":283.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":51.875000,"y":280.000000},"end_control_point":{"@type":"point","x":43.125000,"y":251.250000},"end_point":{"@type":"point","x":40.625000,"y":231.875000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":38.125000,"y":211.250000},"end_control_point":{"@type":"point","x":41.250000,"y":178.125000},"end_point":{"@type":"point","x":59.375000,"y":163.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":69.375000,"y":156.250000},"end_control_point":{"@type":"point","x":84.375000,"y":163.125000},"end_point":{"@type":"point","x":96.250000,"y":156.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":105.000000,"y":150.625000},"end_control_point":{"@type":"point","x":108.750000,"y":138.125000},"end_point":{"@type":"point","x":115.625000,"y":130.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":125.625000,"y":119.375000},"end_control_point":{"@type":"point","x":138.125000,"y":110.625000},"end_point":{"@type":"point","x":147.500000,"y":98.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":160.625000,"y":82.500000},"end_control_point":{"@type":"point","x":135.625000,"y":38.125000},"end_point":{"@type":"point","x":168.750000,"y":38.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":173.750000,"y":38.125000},"end_control_point":{"@type":"point","x":178.750000,"y":40.000000},"end_point":{"@type":"point","x":183.125000,"y":43.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":221.875000,"y":73.125000},"end_control_point":{"@type":"point","x":183.750000,"y":112.500000},"end_point":{"@type":"point","x":188.125000,"y":136.875000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":189.375000,"y":143.750000},"end_control_point":{"@type":"point","x":203.125000,"y":143.125000},"end_point":{"@type":"point","x":208.125000,"y":143.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":230.625000,"y":141.875000},"end_control_point":{"@type":"point","x":271.250000,"y":141.250000},"end_point":{"@type":"point","x":280.000000,"y":168.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":284.375000,"y":181.875000},"end_control_point":{"@type":"point","x":277.500000,"y":186.875000},"end_point":{"@type":"point","x":277.500000,"y":196.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":277.500000,"y":198.750000},"end_control_point":{"@type":"point","x":287.500000,"y":209.375000},"end_point":{"@type":"point","x":284.375000,"y":221.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":283.125000,"y":225.000000},"end_control_point":{"@type":"point","x":281.250000,"y":228.125000},"end_point":{"@type":"point","x":279.375000,"y":231.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":279.375000,"y":231.250000},"end_control_point":{"@type":"point","x":273.125000,"y":235.625000},"end_point":{"@type":"point","x":273.125000,"y":236.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":276.250000,"y":246.875000},"end_control_point":{"@type":"point","x":276.875000,"y":257.500000},"end_point":{"@type":"point","x":267.500000,"y":265.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":265.625000,"y":267.500000},"end_control_point":{"@type":"point","x":262.500000,"y":267.500000},"end_point":{"@type":"point","x":261.250000,"y":270.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":260.625000,"y":273.125000},"end_control_point":{"@type":"point","x":261.875000,"y":277.500000},"end_point":{"@type":"point","x":260.000000,"y":280.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":255.000000,"y":298.125000},"end_control_point":{"@type":"point","x":233.750000,"y":301.875000},"end_point":{"@type":"point","x":218.125000,"y":301.875000}}]}],"thumbnail":{"@type":"thumbnail","format":{"@type":"thumbnailFormatWebp"},"width":101,"height":128,"file":{"@type":"file","id":573,"size":2672,"expected_size":2672,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AAMCAQADFQABYr34JQdBM97Uqr0NPPmL7BZvomIAAs0BAAJS-jFFL2KLTiLKoDwBAAdtAAMjBA","unique_id":"AQADzQEAAlL6MUVy","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":2672}}},"sticker":{"@type":"file","id":572,"size":18100,"expected_size":18100,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"CAACAgEAAxUAAWK9-CUHQTPe1Kq9DTz5i-wWb6JiAALNAQACUvoxRS9ii04iyqA8IwQ","unique_id":"AgADzQEAAlL6MUU","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":18100}}},"fitzpatrick_type":0},"emoji":"\ud83d\udc4d"}"#,
            |res: &serde_json::Result<MessageContent>| {
                matches!(res, Ok(MessageContent::MessageAnimatedEmoji(_)))
            },
        )
    }

    #[test]
    fn test_deserialize_message() {
        assert(
            r#"{"@type":"message","id":179306496,"sender_id":{"@type":"messageSenderUser","user_id":5049844889},"chat_id":5129286886,"is_outgoing":true,"is_pinned":false,"can_be_edited":false,"can_be_forwarded":true,"can_be_saved":true,"can_be_deleted_only_for_self":true,"can_be_deleted_for_all_users":true,"can_get_statistics":false,"can_get_message_thread":false,"can_get_viewers":false,"can_get_media_timestamp_links":false,"has_timestamped_media":true,"is_channel_post":false,"contains_unread_mention":false,"date":1648402824,"edit_date":0,"reply_in_chat_id":0,"reply_to_message_id":0,"message_thread_id":0,"ttl":0,"ttl_expires_in":0.000000,"via_bot_user_id":0,"author_signature":"","media_album_id":"0","restriction_reason":"","content":{"@type":"messageAnimatedEmoji","animated_emoji":{"@type":"animatedEmoji","sticker":{"@type":"sticker","set_id":"1258816259751983","width":300,"height":380,"emoji":"\ud83d\udc4d","is_animated":true,"is_mask":false,"outline":[{"@type":"closedVectorPath","commands":[{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":196.250000,"y":296.250000},"end_control_point":{"@type":"point","x":175.625000,"y":296.875000},"end_point":{"@type":"point","x":153.750000,"y":295.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":141.250000,"y":294.375000},"end_control_point":{"@type":"point","x":128.125000,"y":291.250000},"end_point":{"@type":"point","x":117.500000,"y":285.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":113.750000,"y":283.750000},"end_control_point":{"@type":"point","x":109.375000,"y":279.375000},"end_point":{"@type":"point","x":105.625000,"y":278.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":96.250000,"y":276.250000},"end_control_point":{"@type":"point","x":84.375000,"y":284.375000},"end_point":{"@type":"point","x":75.000000,"y":283.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":51.875000,"y":280.000000},"end_control_point":{"@type":"point","x":43.125000,"y":251.250000},"end_point":{"@type":"point","x":40.625000,"y":231.875000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":38.125000,"y":211.250000},"end_control_point":{"@type":"point","x":41.250000,"y":178.125000},"end_point":{"@type":"point","x":59.375000,"y":163.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":69.375000,"y":156.250000},"end_control_point":{"@type":"point","x":84.375000,"y":163.125000},"end_point":{"@type":"point","x":96.250000,"y":156.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":105.000000,"y":150.625000},"end_control_point":{"@type":"point","x":108.750000,"y":138.125000},"end_point":{"@type":"point","x":115.625000,"y":130.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":125.625000,"y":119.375000},"end_control_point":{"@type":"point","x":138.125000,"y":110.625000},"end_point":{"@type":"point","x":147.500000,"y":98.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":160.625000,"y":82.500000},"end_control_point":{"@type":"point","x":135.625000,"y":38.125000},"end_point":{"@type":"point","x":168.750000,"y":38.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":173.750000,"y":38.125000},"end_control_point":{"@type":"point","x":178.750000,"y":40.000000},"end_point":{"@type":"point","x":183.125000,"y":43.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":221.875000,"y":73.125000},"end_control_point":{"@type":"point","x":183.750000,"y":112.500000},"end_point":{"@type":"point","x":188.125000,"y":136.875000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":189.375000,"y":143.750000},"end_control_point":{"@type":"point","x":203.125000,"y":143.125000},"end_point":{"@type":"point","x":208.125000,"y":143.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":230.625000,"y":141.875000},"end_control_point":{"@type":"point","x":271.250000,"y":141.250000},"end_point":{"@type":"point","x":280.000000,"y":168.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":284.375000,"y":181.875000},"end_control_point":{"@type":"point","x":277.500000,"y":186.875000},"end_point":{"@type":"point","x":277.500000,"y":196.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":277.500000,"y":198.750000},"end_control_point":{"@type":"point","x":287.500000,"y":209.375000},"end_point":{"@type":"point","x":284.375000,"y":221.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":283.125000,"y":225.000000},"end_control_point":{"@type":"point","x":281.250000,"y":228.125000},"end_point":{"@type":"point","x":279.375000,"y":231.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":279.375000,"y":231.250000},"end_control_point":{"@type":"point","x":273.125000,"y":235.625000},"end_point":{"@type":"point","x":273.125000,"y":236.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":276.250000,"y":246.875000},"end_control_point":{"@type":"point","x":276.875000,"y":257.500000},"end_point":{"@type":"point","x":267.500000,"y":265.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":265.625000,"y":267.500000},"end_control_point":{"@type":"point","x":262.500000,"y":267.500000},"end_point":{"@type":"point","x":261.250000,"y":270.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":260.625000,"y":273.125000},"end_control_point":{"@type":"point","x":261.875000,"y":277.500000},"end_point":{"@type":"point","x":260.000000,"y":280.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":255.000000,"y":298.125000},"end_control_point":{"@type":"point","x":233.750000,"y":301.875000},"end_point":{"@type":"point","x":218.125000,"y":301.875000}}]}],"thumbnail":{"@type":"thumbnail","format":{"@type":"thumbnailFormatWebp"},"width":101,"height":128,"file":{"@type":"file","id":573,"size":2672,"expected_size":2672,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AAMCAQADFQABYr34JQdBM97Uqr0NPPmL7BZvomIAAs0BAAJS-jFFL2KLTiLKoDwBAAdtAAMjBA","unique_id":"AQADzQEAAlL6MUVy","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":2672}}},"sticker":{"@type":"file","id":572,"size":18100,"expected_size":18100,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"CAACAgEAAxUAAWK9-CUHQTPe1Kq9DTz5i-wWb6JiAALNAQACUvoxRS9ii04iyqA8IwQ","unique_id":"AgADzQEAAlL6MUU","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":18100}}},"fitzpatrick_type":0},"emoji":"\ud83d\udc4d"}}"#,
            |res: &serde_json::Result<Message>| res.is_ok(),
        )
    }

    #[test]
    fn test_deserialize_update_chat_last_message() {
        assert(
            r#"{"@type":"updateChatLastMessage","chat_id":5129286886,"last_message":{"@type":"message","id":179306496,"sender_id":{"@type":"messageSenderUser","user_id":5049844889},"chat_id":5129286886,"is_outgoing":true,"is_pinned":false,"can_be_edited":false,"can_be_forwarded":true,"can_be_saved":true,"can_be_deleted_only_for_self":true,"can_be_deleted_for_all_users":true,"can_get_statistics":false,"can_get_message_thread":false,"can_get_viewers":false,"can_get_media_timestamp_links":false,"has_timestamped_media":true,"is_channel_post":false,"contains_unread_mention":false,"date":1648402824,"edit_date":0,"reply_in_chat_id":0,"reply_to_message_id":0,"message_thread_id":0,"ttl":0,"ttl_expires_in":0.000000,"via_bot_user_id":0,"author_signature":"","media_album_id":"0","restriction_reason":"","content":{"@type":"messageAnimatedEmoji","animated_emoji":{"@type":"animatedEmoji","sticker":{"@type":"sticker","set_id":"1258816259751983","width":300,"height":380,"emoji":"\ud83d\udc4d","is_animated":true,"is_mask":false,"outline":[{"@type":"closedVectorPath","commands":[{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":196.250000,"y":296.250000},"end_control_point":{"@type":"point","x":175.625000,"y":296.875000},"end_point":{"@type":"point","x":153.750000,"y":295.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":141.250000,"y":294.375000},"end_control_point":{"@type":"point","x":128.125000,"y":291.250000},"end_point":{"@type":"point","x":117.500000,"y":285.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":113.750000,"y":283.750000},"end_control_point":{"@type":"point","x":109.375000,"y":279.375000},"end_point":{"@type":"point","x":105.625000,"y":278.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":96.250000,"y":276.250000},"end_control_point":{"@type":"point","x":84.375000,"y":284.375000},"end_point":{"@type":"point","x":75.000000,"y":283.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":51.875000,"y":280.000000},"end_control_point":{"@type":"point","x":43.125000,"y":251.250000},"end_point":{"@type":"point","x":40.625000,"y":231.875000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":38.125000,"y":211.250000},"end_control_point":{"@type":"point","x":41.250000,"y":178.125000},"end_point":{"@type":"point","x":59.375000,"y":163.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":69.375000,"y":156.250000},"end_control_point":{"@type":"point","x":84.375000,"y":163.125000},"end_point":{"@type":"point","x":96.250000,"y":156.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":105.000000,"y":150.625000},"end_control_point":{"@type":"point","x":108.750000,"y":138.125000},"end_point":{"@type":"point","x":115.625000,"y":130.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":125.625000,"y":119.375000},"end_control_point":{"@type":"point","x":138.125000,"y":110.625000},"end_point":{"@type":"point","x":147.500000,"y":98.750000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":160.625000,"y":82.500000},"end_control_point":{"@type":"point","x":135.625000,"y":38.125000},"end_point":{"@type":"point","x":168.750000,"y":38.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":173.750000,"y":38.125000},"end_control_point":{"@type":"point","x":178.750000,"y":40.000000},"end_point":{"@type":"point","x":183.125000,"y":43.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":221.875000,"y":73.125000},"end_control_point":{"@type":"point","x":183.750000,"y":112.500000},"end_point":{"@type":"point","x":188.125000,"y":136.875000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":189.375000,"y":143.750000},"end_control_point":{"@type":"point","x":203.125000,"y":143.125000},"end_point":{"@type":"point","x":208.125000,"y":143.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":230.625000,"y":141.875000},"end_control_point":{"@type":"point","x":271.250000,"y":141.250000},"end_point":{"@type":"point","x":280.000000,"y":168.125000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":284.375000,"y":181.875000},"end_control_point":{"@type":"point","x":277.500000,"y":186.875000},"end_point":{"@type":"point","x":277.500000,"y":196.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":277.500000,"y":198.750000},"end_control_point":{"@type":"point","x":287.500000,"y":209.375000},"end_point":{"@type":"point","x":284.375000,"y":221.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":283.125000,"y":225.000000},"end_control_point":{"@type":"point","x":281.250000,"y":228.125000},"end_point":{"@type":"point","x":279.375000,"y":231.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":279.375000,"y":231.250000},"end_control_point":{"@type":"point","x":273.125000,"y":235.625000},"end_point":{"@type":"point","x":273.125000,"y":236.250000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":276.250000,"y":246.875000},"end_control_point":{"@type":"point","x":276.875000,"y":257.500000},"end_point":{"@type":"point","x":267.500000,"y":265.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":265.625000,"y":267.500000},"end_control_point":{"@type":"point","x":262.500000,"y":267.500000},"end_point":{"@type":"point","x":261.250000,"y":270.000000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":260.625000,"y":273.125000},"end_control_point":{"@type":"point","x":261.875000,"y":277.500000},"end_point":{"@type":"point","x":260.000000,"y":280.625000}},{"@type":"vectorPathCommandCubicBezierCurve","start_control_point":{"@type":"point","x":255.000000,"y":298.125000},"end_control_point":{"@type":"point","x":233.750000,"y":301.875000},"end_point":{"@type":"point","x":218.125000,"y":301.875000}}]}],"thumbnail":{"@type":"thumbnail","format":{"@type":"thumbnailFormatWebp"},"width":101,"height":128,"file":{"@type":"file","id":573,"size":2672,"expected_size":2672,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"AAMCAQADFQABYr34JQdBM97Uqr0NPPmL7BZvomIAAs0BAAJS-jFFL2KLTiLKoDwBAAdtAAMjBA","unique_id":"AQADzQEAAlL6MUVy","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":2672}}},"sticker":{"@type":"file","id":572,"size":18100,"expected_size":18100,"local":{"@type":"localFile","path":"","can_be_downloaded":true,"can_be_deleted":false,"is_downloading_active":false,"is_downloading_completed":false,"download_offset":0,"downloaded_prefix_size":0,"downloaded_size":0},"remote":{"@type":"remoteFile","id":"CAACAgEAAxUAAWK9-CUHQTPe1Kq9DTz5i-wWb6JiAALNAQACUvoxRS9ii04iyqA8IwQ","unique_id":"AgADzQEAAlL6MUU","is_uploading_active":false,"is_uploading_completed":true,"uploaded_size":18100}}},"fitzpatrick_type":0},"emoji":"\ud83d\udc4d"}},"positions":[],"@client_id":1}"#,
            |res: &serde_json::Result<Update>| matches!(res, Ok(Update::ChatLastMessage(_))),
        )
    }

    #[test]
    fn test_deserialize_update_auth_state_wait_tdlib_params() {
        assert(
            r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#,
            |res: &serde_json::Result<Update>| matches!(res, Ok(Update::AuthorizationState(_))),
        )
    }

    fn assert<T: std::fmt::Debug + DeserializeOwned>(
        data: &'_ str,
        exp: fn(&serde_json::Result<T>) -> bool,
    ) {
        let result = serde_json::from_str(data);
        if !exp(&result) {
            panic!("does not match, got: {:?}", result);
        }
    }
}
