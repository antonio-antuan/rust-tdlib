use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use serde::de::{Deserialize, Deserializer, Error as SerdeError};

use crate::{errors::*, types::*};
use serde::{de, Serialize};

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
    fn extra(&self) -> Option<&str>;
    fn client_id(&self) -> Option<i32>;
}

pub trait RFunction: Debug + RObject + Serialize {
    fn to_json(&self) -> RTDResult<String> {
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

impl<'a, INPUTINVOICE: TDInputInvoice> TDInputInvoice for &'a INPUTINVOICE {}
impl<'a, INPUTINVOICE: TDInputInvoice> TDInputInvoice for &'a mut INPUTINVOICE {}

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

impl<'a, PAYMENTPROVIDER: TDPaymentProvider> TDPaymentProvider for &'a PAYMENTPROVIDER {}
impl<'a, PAYMENTPROVIDER: TDPaymentProvider> TDPaymentProvider for &'a mut PAYMENTPROVIDER {}

impl<'a, POLLTYPE: TDPollType> TDPollType for &'a POLLTYPE {}
impl<'a, POLLTYPE: TDPollType> TDPollType for &'a mut POLLTYPE {}

impl<'a, PREMIUMFEATURE: TDPremiumFeature> TDPremiumFeature for &'a PREMIUMFEATURE {}
impl<'a, PREMIUMFEATURE: TDPremiumFeature> TDPremiumFeature for &'a mut PREMIUMFEATURE {}

impl<'a, PREMIUMLIMITTYPE: TDPremiumLimitType> TDPremiumLimitType for &'a PREMIUMLIMITTYPE {}
impl<'a, PREMIUMLIMITTYPE: TDPremiumLimitType> TDPremiumLimitType for &'a mut PREMIUMLIMITTYPE {}

impl<'a, PREMIUMSOURCE: TDPremiumSource> TDPremiumSource for &'a PREMIUMSOURCE {}
impl<'a, PREMIUMSOURCE: TDPremiumSource> TDPremiumSource for &'a mut PREMIUMSOURCE {}

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

impl<'a, SESSIONTYPE: TDSessionType> TDSessionType for &'a SESSIONTYPE {}
impl<'a, SESSIONTYPE: TDSessionType> TDSessionType for &'a mut SESSIONTYPE {}

impl<'a, STATISTICALGRAPH: TDStatisticalGraph> TDStatisticalGraph for &'a STATISTICALGRAPH {}
impl<'a, STATISTICALGRAPH: TDStatisticalGraph> TDStatisticalGraph for &'a mut STATISTICALGRAPH {}

impl<'a, STICKERTYPE: TDStickerType> TDStickerType for &'a STICKERTYPE {}
impl<'a, STICKERTYPE: TDStickerType> TDStickerType for &'a mut STICKERTYPE {}

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

impl<'a, TARGETCHAT: TDTargetChat> TDTargetChat for &'a TARGETCHAT {}
impl<'a, TARGETCHAT: TDTargetChat> TDTargetChat for &'a mut TARGETCHAT {}

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

#[derive(Debug, Clone)]
pub(crate) enum TdType {
    AuthorizationState(AuthorizationState),
    CanTransferOwnershipResult(CanTransferOwnershipResult),
    ChatStatistics(ChatStatistics),
    CheckChatUsernameResult(CheckChatUsernameResult),
    CheckStickerSetNameResult(CheckStickerSetNameResult),
    InternalLinkType(InternalLinkType),
    JsonValue(JsonValue),
    LanguagePackStringValue(LanguagePackStringValue),
    LogStream(LogStream),
    LoginUrlInfo(LoginUrlInfo),
    MessageFileType(MessageFileType),
    OptionValue(OptionValue),
    PassportElement(PassportElement),
    ResetPasswordResult(ResetPasswordResult),
    StatisticalGraph(StatisticalGraph),
    Update(Update),
    AccountTtl(AccountTtl),
    AddedReactions(AddedReactions),
    AnimatedEmoji(AnimatedEmoji),
    Animations(Animations),
    AttachmentMenuBot(AttachmentMenuBot),
    AuthenticationCodeInfo(AuthenticationCodeInfo),
    AutoDownloadSettingsPresets(AutoDownloadSettingsPresets),
    AvailableReactions(AvailableReactions),
    Background(Background),
    Backgrounds(Backgrounds),
    BankCardInfo(BankCardInfo),
    BasicGroup(BasicGroup),
    BasicGroupFullInfo(BasicGroupFullInfo),
    BotCommands(BotCommands),
    BotMenuButton(BotMenuButton),
    CallId(CallId),
    CallbackQueryAnswer(CallbackQueryAnswer),
    Chat(Chat),
    ChatAdministrators(ChatAdministrators),
    ChatEvents(ChatEvents),
    ChatFilter(ChatFilter),
    ChatFilterInfo(ChatFilterInfo),
    ChatInviteLink(ChatInviteLink),
    ChatInviteLinkCounts(ChatInviteLinkCounts),
    ChatInviteLinkInfo(ChatInviteLinkInfo),
    ChatInviteLinkMembers(ChatInviteLinkMembers),
    ChatInviteLinks(ChatInviteLinks),
    ChatJoinRequests(ChatJoinRequests),
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
    FileDownloadedPrefixSize(FileDownloadedPrefixSize),
    FilePart(FilePart),
    FormattedText(FormattedText),
    FoundFileDownloads(FoundFileDownloads),
    FoundMessages(FoundMessages),
    GameHighScores(GameHighScores),
    GroupCall(GroupCall),
    GroupCallId(GroupCallId),
    GroupCallStreams(GroupCallStreams),
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
    MessageCalendar(MessageCalendar),
    MessageLink(MessageLink),
    MessageLinkInfo(MessageLinkInfo),
    MessagePositions(MessagePositions),
    MessageSenders(MessageSenders),
    MessageStatistics(MessageStatistics),
    MessageThreadInfo(MessageThreadInfo),
    Messages(Messages),
    NetworkStatistics(NetworkStatistics),
    NotificationSound(NotificationSound),
    NotificationSounds(NotificationSounds),
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
    PremiumFeatures(PremiumFeatures),
    PremiumLimit(PremiumLimit),
    PremiumState(PremiumState),
    Proxies(Proxies),
    Proxy(Proxy),
    PushReceiverId(PushReceiverId),
    RecommendedChatFilters(RecommendedChatFilters),
    RecoveryEmailAddress(RecoveryEmailAddress),
    RtmpUrl(RtmpUrl),
    ScopeNotificationSettings(ScopeNotificationSettings),
    Seconds(Seconds),
    SecretChat(SecretChat),
    SentWebAppMessage(SentWebAppMessage),
    Session(Session),
    Sessions(Sessions),
    SponsoredMessage(SponsoredMessage),
    Sticker(Sticker),
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
    TrendingStickerSets(TrendingStickerSets),
    Updates(Updates),
    User(User),
    UserFullInfo(UserFullInfo),
    UserPrivacySettingRules(UserPrivacySettingRules),
    Users(Users),
    ValidatedOrderInfo(ValidatedOrderInfo),
    WebAppInfo(WebAppInfo),
    WebPage(WebPage),
    WebPageInstantView(WebPageInstantView),
}
impl<'de> Deserialize<'de> for TdType {
    fn deserialize<D>(deserializer: D) -> Result<TdType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let rtd_trait_value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        let rtd_trait_map = match rtd_trait_value.as_object() {
            Some(map) => map,
            None => {
                return Err(D::Error::unknown_field(
                    stringify!(TdType),
                    &[stringify!("{} is not the correct type", TdType)],
                ))
            }
        };

        let rtd_trait_type = match rtd_trait_map.get("@type") {
            Some(t) => match t.as_str() {
                Some(s) => s,
                None => {
                    return Err(D::Error::unknown_field(
                        stringify!( "{} -> @type" , $field ),
                        &[stringify!("{} -> @type is not the correct type", TdType)],
                    ))
                }
            },
            None => return Err(D::Error::custom("@type is empty")),
        };
        if let Some(t) =
            deserialize_traits(rtd_trait_type, rtd_trait_value.clone()).map_err(|err| {
                D::Error::custom(format!(
                    "can't deserialize for {} with error: {}",
                    rtd_trait_type, err
                ))
            })?
        {
            return Ok(t);
        };
        if let Some(t) =
            deserialize_direct_types(rtd_trait_type, rtd_trait_value.clone()).map_err(|err| {
                D::Error::custom(format!(
                    "can't deserialize for {} with error: {}",
                    rtd_trait_type, err
                ))
            })?
        {
            return Ok(t);
        }
        Err(D::Error::custom(format!(
            "got {} @type with unavailable variant",
            rtd_trait_type
        )))
    }
}

fn deserialize_traits(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    if let Some(td_type) = deserialize_update(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_authorization_state(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_json_value(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_option_value(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) =
        deserialize_can_transfer_ownership_result(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_chat_statistics(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) =
        deserialize_check_chat_username_result(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) =
        deserialize_check_sticker_set_name_result(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_internal_link_type(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) =
        deserialize_language_pack_string_value(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_log_stream(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_login_url_info(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_message_file_type(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_passport_element(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    if let Some(td_type) =
        deserialize_reset_password_result(rtd_trait_type, rtd_trait_value.clone())?
    {
        return Ok(Some(td_type));
    };

    if let Some(td_type) = deserialize_statistical_graph(rtd_trait_type, rtd_trait_value.clone())? {
        return Ok(Some(td_type));
    };

    Ok(None)
}

fn deserialize_direct_types(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match rtd_trait_type {
        "accountTtl" => Some(TdType::AccountTtl(serde_json::from_value(rtd_trait_value)?)),
        "addedReactions" => Some(TdType::AddedReactions(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "animatedEmoji" => Some(TdType::AnimatedEmoji(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "animations" => Some(TdType::Animations(serde_json::from_value(rtd_trait_value)?)),
        "attachmentMenuBot" => Some(TdType::AttachmentMenuBot(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "authenticationCodeInfo" => Some(TdType::AuthenticationCodeInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "autoDownloadSettingsPresets" => Some(TdType::AutoDownloadSettingsPresets(
            serde_json::from_value(rtd_trait_value)?,
        )),
        "availableReactions" => Some(TdType::AvailableReactions(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "background" => Some(TdType::Background(serde_json::from_value(rtd_trait_value)?)),
        "backgrounds" => Some(TdType::Backgrounds(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "bankCardInfo" => Some(TdType::BankCardInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "basicGroup" => Some(TdType::BasicGroup(serde_json::from_value(rtd_trait_value)?)),
        "basicGroupFullInfo" => Some(TdType::BasicGroupFullInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "botCommands" => Some(TdType::BotCommands(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "botMenuButton" => Some(TdType::BotMenuButton(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "callId" => Some(TdType::CallId(serde_json::from_value(rtd_trait_value)?)),
        "callbackQueryAnswer" => Some(TdType::CallbackQueryAnswer(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "chat" => Some(TdType::Chat(serde_json::from_value(rtd_trait_value)?)),
        "chatAdministrators" => Some(TdType::ChatAdministrators(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "chatEvents" => Some(TdType::ChatEvents(serde_json::from_value(rtd_trait_value)?)),
        "chatFilter" => Some(TdType::ChatFilter(serde_json::from_value(rtd_trait_value)?)),
        "chatFilterInfo" => Some(TdType::ChatFilterInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "chatInviteLink" => Some(TdType::ChatInviteLink(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "chatInviteLinkCounts" => Some(TdType::ChatInviteLinkCounts(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "chatInviteLinkInfo" => Some(TdType::ChatInviteLinkInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "chatInviteLinkMembers" => Some(TdType::ChatInviteLinkMembers(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "chatInviteLinks" => Some(TdType::ChatInviteLinks(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "chatJoinRequests" => Some(TdType::ChatJoinRequests(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "chatLists" => Some(TdType::ChatLists(serde_json::from_value(rtd_trait_value)?)),
        "chatMember" => Some(TdType::ChatMember(serde_json::from_value(rtd_trait_value)?)),
        "chatMembers" => Some(TdType::ChatMembers(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "chatPhotos" => Some(TdType::ChatPhotos(serde_json::from_value(rtd_trait_value)?)),
        "chats" => Some(TdType::Chats(serde_json::from_value(rtd_trait_value)?)),
        "chatsNearby" => Some(TdType::ChatsNearby(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "connectedWebsites" => Some(TdType::ConnectedWebsites(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "count" => Some(TdType::Count(serde_json::from_value(rtd_trait_value)?)),
        "countries" => Some(TdType::Countries(serde_json::from_value(rtd_trait_value)?)),
        "customRequestResult" => Some(TdType::CustomRequestResult(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "databaseStatistics" => Some(TdType::DatabaseStatistics(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "deepLinkInfo" => Some(TdType::DeepLinkInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "emailAddressAuthenticationCodeInfo" => Some(TdType::EmailAddressAuthenticationCodeInfo(
            serde_json::from_value(rtd_trait_value)?,
        )),
        "emojis" => Some(TdType::Emojis(serde_json::from_value(rtd_trait_value)?)),
        "error" => Some(TdType::Error(serde_json::from_value(rtd_trait_value)?)),
        "file" => Some(TdType::File(serde_json::from_value(rtd_trait_value)?)),
        "fileDownloadedPrefixSize" => Some(TdType::FileDownloadedPrefixSize(
            serde_json::from_value(rtd_trait_value)?,
        )),
        "filePart" => Some(TdType::FilePart(serde_json::from_value(rtd_trait_value)?)),
        "formattedText" => Some(TdType::FormattedText(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "foundFileDownloads" => Some(TdType::FoundFileDownloads(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "foundMessages" => Some(TdType::FoundMessages(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "gameHighScores" => Some(TdType::GameHighScores(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "groupCall" => Some(TdType::GroupCall(serde_json::from_value(rtd_trait_value)?)),
        "groupCallId" => Some(TdType::GroupCallId(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "groupCallStreams" => Some(TdType::GroupCallStreams(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "hashtags" => Some(TdType::Hashtags(serde_json::from_value(rtd_trait_value)?)),
        "httpUrl" => Some(TdType::HttpUrl(serde_json::from_value(rtd_trait_value)?)),
        "importedContacts" => Some(TdType::ImportedContacts(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "inlineQueryResults" => Some(TdType::InlineQueryResults(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "languagePackInfo" => Some(TdType::LanguagePackInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "languagePackStrings" => Some(TdType::LanguagePackStrings(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "localizationTargetInfo" => Some(TdType::LocalizationTargetInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "logTags" => Some(TdType::LogTags(serde_json::from_value(rtd_trait_value)?)),
        "logVerbosityLevel" => Some(TdType::LogVerbosityLevel(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "message" => Some(TdType::Message(serde_json::from_value(rtd_trait_value)?)),
        "messageCalendar" => Some(TdType::MessageCalendar(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "messageLink" => Some(TdType::MessageLink(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "messageLinkInfo" => Some(TdType::MessageLinkInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "messagePositions" => Some(TdType::MessagePositions(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "messageSenders" => Some(TdType::MessageSenders(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "messageStatistics" => Some(TdType::MessageStatistics(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "messageThreadInfo" => Some(TdType::MessageThreadInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "messages" => Some(TdType::Messages(serde_json::from_value(rtd_trait_value)?)),
        "networkStatistics" => Some(TdType::NetworkStatistics(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "notificationSound" => Some(TdType::NotificationSound(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "notificationSounds" => Some(TdType::NotificationSounds(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "ok" => Some(TdType::Ok(serde_json::from_value(rtd_trait_value)?)),
        "orderInfo" => Some(TdType::OrderInfo(serde_json::from_value(rtd_trait_value)?)),
        "passportAuthorizationForm" => Some(TdType::PassportAuthorizationForm(
            serde_json::from_value(rtd_trait_value)?,
        )),
        "passportElements" => Some(TdType::PassportElements(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "passportElementsWithErrors" => Some(TdType::PassportElementsWithErrors(
            serde_json::from_value(rtd_trait_value)?,
        )),
        "passwordState" => Some(TdType::PasswordState(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "paymentForm" => Some(TdType::PaymentForm(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "paymentReceipt" => Some(TdType::PaymentReceipt(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "paymentResult" => Some(TdType::PaymentResult(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "phoneNumberInfo" => Some(TdType::PhoneNumberInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "premiumFeatures" => Some(TdType::PremiumFeatures(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "premiumLimit" => Some(TdType::PremiumLimit(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "premiumState" => Some(TdType::PremiumState(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "proxies" => Some(TdType::Proxies(serde_json::from_value(rtd_trait_value)?)),
        "proxy" => Some(TdType::Proxy(serde_json::from_value(rtd_trait_value)?)),
        "pushReceiverId" => Some(TdType::PushReceiverId(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "recommendedChatFilters" => Some(TdType::RecommendedChatFilters(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "recoveryEmailAddress" => Some(TdType::RecoveryEmailAddress(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "rtmpUrl" => Some(TdType::RtmpUrl(serde_json::from_value(rtd_trait_value)?)),
        "scopeNotificationSettings" => Some(TdType::ScopeNotificationSettings(
            serde_json::from_value(rtd_trait_value)?,
        )),
        "seconds" => Some(TdType::Seconds(serde_json::from_value(rtd_trait_value)?)),
        "secretChat" => Some(TdType::SecretChat(serde_json::from_value(rtd_trait_value)?)),
        "sentWebAppMessage" => Some(TdType::SentWebAppMessage(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "session" => Some(TdType::Session(serde_json::from_value(rtd_trait_value)?)),
        "sessions" => Some(TdType::Sessions(serde_json::from_value(rtd_trait_value)?)),
        "sponsoredMessage" => Some(TdType::SponsoredMessage(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "sticker" => Some(TdType::Sticker(serde_json::from_value(rtd_trait_value)?)),
        "stickerSet" => Some(TdType::StickerSet(serde_json::from_value(rtd_trait_value)?)),
        "stickerSets" => Some(TdType::StickerSets(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "stickers" => Some(TdType::Stickers(serde_json::from_value(rtd_trait_value)?)),
        "storageStatistics" => Some(TdType::StorageStatistics(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "storageStatisticsFast" => Some(TdType::StorageStatisticsFast(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "supergroup" => Some(TdType::Supergroup(serde_json::from_value(rtd_trait_value)?)),
        "supergroupFullInfo" => Some(TdType::SupergroupFullInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "tMeUrls" => Some(TdType::TMeUrls(serde_json::from_value(rtd_trait_value)?)),
        "temporaryPasswordState" => Some(TdType::TemporaryPasswordState(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "testBytes" => Some(TdType::TestBytes(serde_json::from_value(rtd_trait_value)?)),
        "testInt" => Some(TdType::TestInt(serde_json::from_value(rtd_trait_value)?)),
        "testString" => Some(TdType::TestString(serde_json::from_value(rtd_trait_value)?)),
        "testVectorInt" => Some(TdType::TestVectorInt(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "testVectorIntObject" => Some(TdType::TestVectorIntObject(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "testVectorString" => Some(TdType::TestVectorString(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "testVectorStringObject" => Some(TdType::TestVectorStringObject(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "text" => Some(TdType::Text(serde_json::from_value(rtd_trait_value)?)),
        "textEntities" => Some(TdType::TextEntities(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "trendingStickerSets" => Some(TdType::TrendingStickerSets(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "updates" => Some(TdType::Updates(serde_json::from_value(rtd_trait_value)?)),
        "user" => Some(TdType::User(serde_json::from_value(rtd_trait_value)?)),
        "userFullInfo" => Some(TdType::UserFullInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "userPrivacySettingRules" => Some(TdType::UserPrivacySettingRules(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "users" => Some(TdType::Users(serde_json::from_value(rtd_trait_value)?)),
        "validatedOrderInfo" => Some(TdType::ValidatedOrderInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        "webAppInfo" => Some(TdType::WebAppInfo(serde_json::from_value(rtd_trait_value)?)),
        "webPage" => Some(TdType::WebPage(serde_json::from_value(rtd_trait_value)?)),
        "webPageInstantView" => Some(TdType::WebPageInstantView(serde_json::from_value(
            rtd_trait_value,
        )?)),
        _ => None,
    })
}

const AUTHORIZATIONSTATE_MEMBERS: &[&str] = &[
    "authorizationStateClosed",
    "authorizationStateClosing",
    "authorizationStateLoggingOut",
    "authorizationStateReady",
    "authorizationStateWaitCode",
    "authorizationStateWaitEncryptionKey",
    "authorizationStateWaitOtherDeviceConfirmation",
    "authorizationStateWaitPassword",
    "authorizationStateWaitPhoneNumber",
    "authorizationStateWaitRegistration",
    "authorizationStateWaitTdlibParameters",
    "getAuthorizationState",
];

fn deserialize_authorization_state(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match AUTHORIZATIONSTATE_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::AuthorizationState(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const CANTRANSFEROWNERSHIPRESULT_MEMBERS: &[&str] = &[
    "canTransferOwnership",
    "canTransferOwnershipResultOk",
    "canTransferOwnershipResultPasswordNeeded",
    "canTransferOwnershipResultPasswordTooFresh",
    "canTransferOwnershipResultSessionTooFresh",
];

fn deserialize_can_transfer_ownership_result(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(
        match CANTRANSFEROWNERSHIPRESULT_MEMBERS.contains(&rtd_trait_type) {
            true => Some(TdType::CanTransferOwnershipResult(serde_json::from_value(
                rtd_trait_value,
            )?)),
            false => None,
        },
    )
}

const CHATSTATISTICS_MEMBERS: &[&str] = &[
    "chatStatisticsChannel",
    "chatStatisticsSupergroup",
    "getChatStatistics",
];

fn deserialize_chat_statistics(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match CHATSTATISTICS_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::ChatStatistics(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const CHECKCHATUSERNAMERESULT_MEMBERS: &[&str] = &[
    "checkChatUsername",
    "checkChatUsernameResultOk",
    "checkChatUsernameResultPublicChatsTooMuch",
    "checkChatUsernameResultPublicGroupsUnavailable",
    "checkChatUsernameResultUsernameInvalid",
    "checkChatUsernameResultUsernameOccupied",
];

fn deserialize_check_chat_username_result(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(
        match CHECKCHATUSERNAMERESULT_MEMBERS.contains(&rtd_trait_type) {
            true => Some(TdType::CheckChatUsernameResult(serde_json::from_value(
                rtd_trait_value,
            )?)),
            false => None,
        },
    )
}

const CHECKSTICKERSETNAMERESULT_MEMBERS: &[&str] = &[
    "checkStickerSetName",
    "checkStickerSetNameResultNameInvalid",
    "checkStickerSetNameResultNameOccupied",
    "checkStickerSetNameResultOk",
];

fn deserialize_check_sticker_set_name_result(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(
        match CHECKSTICKERSETNAMERESULT_MEMBERS.contains(&rtd_trait_type) {
            true => Some(TdType::CheckStickerSetNameResult(serde_json::from_value(
                rtd_trait_value,
            )?)),
            false => None,
        },
    )
}

const INTERNALLINKTYPE_MEMBERS: &[&str] = &[
    "getInternalLinkType",
    "internalLinkTypeActiveSessions",
    "internalLinkTypeAttachmentMenuBot",
    "internalLinkTypeAuthenticationCode",
    "internalLinkTypeBackground",
    "internalLinkTypeBotAddToChannel",
    "internalLinkTypeBotStart",
    "internalLinkTypeBotStartInGroup",
    "internalLinkTypeChangePhoneNumber",
    "internalLinkTypeChatInvite",
    "internalLinkTypeFilterSettings",
    "internalLinkTypeGame",
    "internalLinkTypeInvoice",
    "internalLinkTypeLanguagePack",
    "internalLinkTypeLanguageSettings",
    "internalLinkTypeMessage",
    "internalLinkTypeMessageDraft",
    "internalLinkTypePassportDataRequest",
    "internalLinkTypePhoneNumberConfirmation",
    "internalLinkTypePremiumFeatures",
    "internalLinkTypePrivacyAndSecuritySettings",
    "internalLinkTypeProxy",
    "internalLinkTypePublicChat",
    "internalLinkTypeQrCodeAuthentication",
    "internalLinkTypeSettings",
    "internalLinkTypeStickerSet",
    "internalLinkTypeTheme",
    "internalLinkTypeThemeSettings",
    "internalLinkTypeUnknownDeepLink",
    "internalLinkTypeUnsupportedProxy",
    "internalLinkTypeUserPhoneNumber",
    "internalLinkTypeVideoChat",
];

fn deserialize_internal_link_type(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match INTERNALLINKTYPE_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::InternalLinkType(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const JSONVALUE_MEMBERS: &[&str] = &[
    "getApplicationConfig",
    "getJsonValue",
    "jsonValueArray",
    "jsonValueBoolean",
    "jsonValueNull",
    "jsonValueNumber",
    "jsonValueObject",
    "jsonValueString",
];

fn deserialize_json_value(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match JSONVALUE_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::JsonValue(serde_json::from_value(rtd_trait_value)?)),
        false => None,
    })
}

const LANGUAGEPACKSTRINGVALUE_MEMBERS: &[&str] = &[
    "getLanguagePackString",
    "languagePackStringValueDeleted",
    "languagePackStringValueOrdinary",
    "languagePackStringValuePluralized",
];

fn deserialize_language_pack_string_value(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(
        match LANGUAGEPACKSTRINGVALUE_MEMBERS.contains(&rtd_trait_type) {
            true => Some(TdType::LanguagePackStringValue(serde_json::from_value(
                rtd_trait_value,
            )?)),
            false => None,
        },
    )
}

const LOGSTREAM_MEMBERS: &[&str] = &[
    "getLogStream",
    "logStreamDefault",
    "logStreamEmpty",
    "logStreamFile",
];

fn deserialize_log_stream(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match LOGSTREAM_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::LogStream(serde_json::from_value(rtd_trait_value)?)),
        false => None,
    })
}

const LOGINURLINFO_MEMBERS: &[&str] = &[
    "getExternalLinkInfo",
    "getLoginUrlInfo",
    "loginUrlInfoOpen",
    "loginUrlInfoRequestConfirmation",
];

fn deserialize_login_url_info(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match LOGINURLINFO_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::LoginUrlInfo(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const MESSAGEFILETYPE_MEMBERS: &[&str] = &[
    "getMessageFileType",
    "messageFileTypeGroup",
    "messageFileTypePrivate",
    "messageFileTypeUnknown",
];

fn deserialize_message_file_type(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match MESSAGEFILETYPE_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::MessageFileType(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const OPTIONVALUE_MEMBERS: &[&str] = &[
    "getOption",
    "optionValueBoolean",
    "optionValueEmpty",
    "optionValueInteger",
    "optionValueString",
];

fn deserialize_option_value(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match OPTIONVALUE_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::OptionValue(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const PASSPORTELEMENT_MEMBERS: &[&str] = &[
    "getPassportElement",
    "passportElementAddress",
    "passportElementBankStatement",
    "passportElementDriverLicense",
    "passportElementEmailAddress",
    "passportElementIdentityCard",
    "passportElementInternalPassport",
    "passportElementPassport",
    "passportElementPassportRegistration",
    "passportElementPersonalDetails",
    "passportElementPhoneNumber",
    "passportElementRentalAgreement",
    "passportElementTemporaryRegistration",
    "passportElementUtilityBill",
    "setPassportElement",
];

fn deserialize_passport_element(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match PASSPORTELEMENT_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::PassportElement(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const RESETPASSWORDRESULT_MEMBERS: &[&str] = &[
    "resetPassword",
    "resetPasswordResultDeclined",
    "resetPasswordResultOk",
    "resetPasswordResultPending",
];

fn deserialize_reset_password_result(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(
        match RESETPASSWORDRESULT_MEMBERS.contains(&rtd_trait_type) {
            true => Some(TdType::ResetPasswordResult(serde_json::from_value(
                rtd_trait_value,
            )?)),
            false => None,
        },
    )
}

const STATISTICALGRAPH_MEMBERS: &[&str] = &[
    "getStatisticalGraph",
    "statisticalGraphAsync",
    "statisticalGraphData",
    "statisticalGraphError",
];

fn deserialize_statistical_graph(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match STATISTICALGRAPH_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::StatisticalGraph(serde_json::from_value(
            rtd_trait_value,
        )?)),
        false => None,
    })
}

const UPDATE_MEMBERS: &[&str] = &[
    "testUseUpdate",
    "updateActiveNotifications",
    "updateAnimatedEmojiMessageClicked",
    "updateAnimationSearchParameters",
    "updateAttachmentMenuBots",
    "updateAuthorizationState",
    "updateBasicGroup",
    "updateBasicGroupFullInfo",
    "updateCall",
    "updateChatAction",
    "updateChatActionBar",
    "updateChatAvailableReactions",
    "updateChatDefaultDisableNotification",
    "updateChatDraftMessage",
    "updateChatFilters",
    "updateChatHasProtectedContent",
    "updateChatHasScheduledMessages",
    "updateChatIsBlocked",
    "updateChatIsMarkedAsUnread",
    "updateChatLastMessage",
    "updateChatMember",
    "updateChatMessageSender",
    "updateChatMessageTtl",
    "updateChatNotificationSettings",
    "updateChatOnlineMemberCount",
    "updateChatPendingJoinRequests",
    "updateChatPermissions",
    "updateChatPhoto",
    "updateChatPosition",
    "updateChatReadInbox",
    "updateChatReadOutbox",
    "updateChatReplyMarkup",
    "updateChatTheme",
    "updateChatThemes",
    "updateChatTitle",
    "updateChatUnreadMentionCount",
    "updateChatUnreadReactionCount",
    "updateChatVideoChat",
    "updateConnectionState",
    "updateDeleteMessages",
    "updateDiceEmojis",
    "updateFavoriteStickers",
    "updateFile",
    "updateFileAddedToDownloads",
    "updateFileDownload",
    "updateFileDownloads",
    "updateFileGenerationStart",
    "updateFileGenerationStop",
    "updateFileRemovedFromDownloads",
    "updateGroupCall",
    "updateGroupCallParticipant",
    "updateHavePendingNotifications",
    "updateInstalledStickerSets",
    "updateLanguagePackStrings",
    "updateMessageContent",
    "updateMessageContentOpened",
    "updateMessageEdited",
    "updateMessageInteractionInfo",
    "updateMessageIsPinned",
    "updateMessageLiveLocationViewed",
    "updateMessageMentionRead",
    "updateMessageSendAcknowledged",
    "updateMessageSendFailed",
    "updateMessageSendSucceeded",
    "updateMessageUnreadReactions",
    "updateNewCallSignalingData",
    "updateNewCallbackQuery",
    "updateNewChat",
    "updateNewChatJoinRequest",
    "updateNewChosenInlineResult",
    "updateNewCustomEvent",
    "updateNewCustomQuery",
    "updateNewInlineCallbackQuery",
    "updateNewInlineQuery",
    "updateNewMessage",
    "updateNewPreCheckoutQuery",
    "updateNewShippingQuery",
    "updateNotification",
    "updateNotificationGroup",
    "updateOption",
    "updatePoll",
    "updatePollAnswer",
    "updateReactions",
    "updateRecentStickers",
    "updateSavedAnimations",
    "updateSavedNotificationSounds",
    "updateScopeNotificationSettings",
    "updateSecretChat",
    "updateSelectedBackground",
    "updateServiceNotification",
    "updateStickerSet",
    "updateSuggestedActions",
    "updateSupergroup",
    "updateSupergroupFullInfo",
    "updateTermsOfService",
    "updateTrendingStickerSets",
    "updateUnreadChatCount",
    "updateUnreadMessageCount",
    "updateUser",
    "updateUserFullInfo",
    "updateUserPrivacySettingRules",
    "updateUserStatus",
    "updateUsersNearby",
    "updateWebAppMessageSent",
];

fn deserialize_update(
    rtd_trait_type: &str,
    rtd_trait_value: serde_json::Value,
) -> Result<Option<TdType>, serde_json::Error> {
    Ok(match UPDATE_MEMBERS.contains(&rtd_trait_type) {
        true => Some(TdType::Update(serde_json::from_value(rtd_trait_value)?)),
        false => None,
    })
}

impl TdType {
    pub fn extra(&self) -> Option<&str> {
        match self {
            TdType::AuthorizationState(value) => value.extra(),

            TdType::CanTransferOwnershipResult(value) => value.extra(),

            TdType::ChatStatistics(value) => value.extra(),

            TdType::CheckChatUsernameResult(value) => value.extra(),

            TdType::CheckStickerSetNameResult(value) => value.extra(),

            TdType::InternalLinkType(value) => value.extra(),

            TdType::JsonValue(value) => value.extra(),

            TdType::LanguagePackStringValue(value) => value.extra(),

            TdType::LogStream(value) => value.extra(),

            TdType::LoginUrlInfo(value) => value.extra(),

            TdType::MessageFileType(value) => value.extra(),

            TdType::OptionValue(value) => value.extra(),

            TdType::PassportElement(value) => value.extra(),

            TdType::ResetPasswordResult(value) => value.extra(),

            TdType::StatisticalGraph(value) => value.extra(),

            TdType::Update(value) => value.extra(),

            TdType::AccountTtl(value) => value.extra(),

            TdType::AddedReactions(value) => value.extra(),

            TdType::AnimatedEmoji(value) => value.extra(),

            TdType::Animations(value) => value.extra(),

            TdType::AttachmentMenuBot(value) => value.extra(),

            TdType::AuthenticationCodeInfo(value) => value.extra(),

            TdType::AutoDownloadSettingsPresets(value) => value.extra(),

            TdType::AvailableReactions(value) => value.extra(),

            TdType::Background(value) => value.extra(),

            TdType::Backgrounds(value) => value.extra(),

            TdType::BankCardInfo(value) => value.extra(),

            TdType::BasicGroup(value) => value.extra(),

            TdType::BasicGroupFullInfo(value) => value.extra(),

            TdType::BotCommands(value) => value.extra(),

            TdType::BotMenuButton(value) => value.extra(),

            TdType::CallId(value) => value.extra(),

            TdType::CallbackQueryAnswer(value) => value.extra(),

            TdType::Chat(value) => value.extra(),

            TdType::ChatAdministrators(value) => value.extra(),

            TdType::ChatEvents(value) => value.extra(),

            TdType::ChatFilter(value) => value.extra(),

            TdType::ChatFilterInfo(value) => value.extra(),

            TdType::ChatInviteLink(value) => value.extra(),

            TdType::ChatInviteLinkCounts(value) => value.extra(),

            TdType::ChatInviteLinkInfo(value) => value.extra(),

            TdType::ChatInviteLinkMembers(value) => value.extra(),

            TdType::ChatInviteLinks(value) => value.extra(),

            TdType::ChatJoinRequests(value) => value.extra(),

            TdType::ChatLists(value) => value.extra(),

            TdType::ChatMember(value) => value.extra(),

            TdType::ChatMembers(value) => value.extra(),

            TdType::ChatPhotos(value) => value.extra(),

            TdType::Chats(value) => value.extra(),

            TdType::ChatsNearby(value) => value.extra(),

            TdType::ConnectedWebsites(value) => value.extra(),

            TdType::Count(value) => value.extra(),

            TdType::Countries(value) => value.extra(),

            TdType::CustomRequestResult(value) => value.extra(),

            TdType::DatabaseStatistics(value) => value.extra(),

            TdType::DeepLinkInfo(value) => value.extra(),

            TdType::EmailAddressAuthenticationCodeInfo(value) => value.extra(),

            TdType::Emojis(value) => value.extra(),

            TdType::Error(value) => value.extra(),

            TdType::File(value) => value.extra(),

            TdType::FileDownloadedPrefixSize(value) => value.extra(),

            TdType::FilePart(value) => value.extra(),

            TdType::FormattedText(value) => value.extra(),

            TdType::FoundFileDownloads(value) => value.extra(),

            TdType::FoundMessages(value) => value.extra(),

            TdType::GameHighScores(value) => value.extra(),

            TdType::GroupCall(value) => value.extra(),

            TdType::GroupCallId(value) => value.extra(),

            TdType::GroupCallStreams(value) => value.extra(),

            TdType::Hashtags(value) => value.extra(),

            TdType::HttpUrl(value) => value.extra(),

            TdType::ImportedContacts(value) => value.extra(),

            TdType::InlineQueryResults(value) => value.extra(),

            TdType::LanguagePackInfo(value) => value.extra(),

            TdType::LanguagePackStrings(value) => value.extra(),

            TdType::LocalizationTargetInfo(value) => value.extra(),

            TdType::LogTags(value) => value.extra(),

            TdType::LogVerbosityLevel(value) => value.extra(),

            TdType::Message(value) => value.extra(),

            TdType::MessageCalendar(value) => value.extra(),

            TdType::MessageLink(value) => value.extra(),

            TdType::MessageLinkInfo(value) => value.extra(),

            TdType::MessagePositions(value) => value.extra(),

            TdType::MessageSenders(value) => value.extra(),

            TdType::MessageStatistics(value) => value.extra(),

            TdType::MessageThreadInfo(value) => value.extra(),

            TdType::Messages(value) => value.extra(),

            TdType::NetworkStatistics(value) => value.extra(),

            TdType::NotificationSound(value) => value.extra(),

            TdType::NotificationSounds(value) => value.extra(),

            TdType::Ok(value) => value.extra(),

            TdType::OrderInfo(value) => value.extra(),

            TdType::PassportAuthorizationForm(value) => value.extra(),

            TdType::PassportElements(value) => value.extra(),

            TdType::PassportElementsWithErrors(value) => value.extra(),

            TdType::PasswordState(value) => value.extra(),

            TdType::PaymentForm(value) => value.extra(),

            TdType::PaymentReceipt(value) => value.extra(),

            TdType::PaymentResult(value) => value.extra(),

            TdType::PhoneNumberInfo(value) => value.extra(),

            TdType::PremiumFeatures(value) => value.extra(),

            TdType::PremiumLimit(value) => value.extra(),

            TdType::PremiumState(value) => value.extra(),

            TdType::Proxies(value) => value.extra(),

            TdType::Proxy(value) => value.extra(),

            TdType::PushReceiverId(value) => value.extra(),

            TdType::RecommendedChatFilters(value) => value.extra(),

            TdType::RecoveryEmailAddress(value) => value.extra(),

            TdType::RtmpUrl(value) => value.extra(),

            TdType::ScopeNotificationSettings(value) => value.extra(),

            TdType::Seconds(value) => value.extra(),

            TdType::SecretChat(value) => value.extra(),

            TdType::SentWebAppMessage(value) => value.extra(),

            TdType::Session(value) => value.extra(),

            TdType::Sessions(value) => value.extra(),

            TdType::SponsoredMessage(value) => value.extra(),

            TdType::Sticker(value) => value.extra(),

            TdType::StickerSet(value) => value.extra(),

            TdType::StickerSets(value) => value.extra(),

            TdType::Stickers(value) => value.extra(),

            TdType::StorageStatistics(value) => value.extra(),

            TdType::StorageStatisticsFast(value) => value.extra(),

            TdType::Supergroup(value) => value.extra(),

            TdType::SupergroupFullInfo(value) => value.extra(),

            TdType::TMeUrls(value) => value.extra(),

            TdType::TemporaryPasswordState(value) => value.extra(),

            TdType::TestBytes(value) => value.extra(),

            TdType::TestInt(value) => value.extra(),

            TdType::TestString(value) => value.extra(),

            TdType::TestVectorInt(value) => value.extra(),

            TdType::TestVectorIntObject(value) => value.extra(),

            TdType::TestVectorString(value) => value.extra(),

            TdType::TestVectorStringObject(value) => value.extra(),

            TdType::Text(value) => value.extra(),

            TdType::TextEntities(value) => value.extra(),

            TdType::TrendingStickerSets(value) => value.extra(),

            TdType::Updates(value) => value.extra(),

            TdType::User(value) => value.extra(),

            TdType::UserFullInfo(value) => value.extra(),

            TdType::UserPrivacySettingRules(value) => value.extra(),

            TdType::Users(value) => value.extra(),

            TdType::ValidatedOrderInfo(value) => value.extra(),

            TdType::WebAppInfo(value) => value.extra(),

            TdType::WebPage(value) => value.extra(),

            TdType::WebPageInstantView(value) => value.extra(),
        }
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
    use crate::types::_common::deserialize_update;
    use crate::types::{from_json, AuthorizationState, TdType, Update};

    #[test]
    fn test_deserialize_enums() {
        match deserialize_update(
            "updateAuthorizationState", serde_json::from_str::<serde_json::Value>(r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#).unwrap(),
        ) {
            Ok(v) => {match v {
                Some(v) => {
                    match v {
                        TdType::Update(_) => {},

                        _ => {panic!("serialization failed")},
                    }
                },
                None => panic!("serialization failed")
            }}
            Err(e) => {
                panic!("{}", e)
            }
        };

        match from_json::<TdType>(
            r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#,
        ) {
            Ok(t) => match t {
                TdType::Update(Update::AuthorizationState(state)) => {
                    match state.authorization_state() {
                        AuthorizationState::WaitTdlibParameters(_) => {}
                        _ => {
                            panic!("invalid serialized data")
                        }
                    }
                }
                _ => panic!("from_json failed: {:?}", t),
            },
            Err(e) => {
                panic!("{}", e)
            }
        };
    }
}
