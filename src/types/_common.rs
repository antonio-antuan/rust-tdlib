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
    fn extra(&self) -> Option<String>;
    fn client_id(&self) -> Option<i32>;
}

pub trait RFunction: Debug + RObject + Serialize {
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl<'a, RObj: RObject> RObject for &'a RObj {
    fn extra(&self) -> Option<String> {
        (*self).extra()
    }
    fn client_id(&self) -> Option<i32> {
        (*self).client_id()
    }
}

impl<'a, RObj: RObject> RObject for &'a mut RObj {
    fn extra(&self) -> Option<String> {
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

        let obj = match &rtd_trait_type[..] {

      "authorizationStateClosed" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateClosed deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "authorizationStateClosing" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateClosing deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "authorizationStateLoggingOut" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateLoggingOut deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "authorizationStateReady" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateReady deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "authorizationStateWaitCode" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateWaitCode deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "authorizationStateWaitEncryptionKey" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateWaitEncryptionKey deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "authorizationStateWaitOtherDeviceConfirmation" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateWaitOtherDeviceConfirmation deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "authorizationStateWaitPassword" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateWaitPassword deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "authorizationStateWaitPhoneNumber" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateWaitPhoneNumber deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "authorizationStateWaitRegistration" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateWaitRegistration deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "authorizationStateWaitTdlibParameters" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthorizationStateWaitTdlibParameters deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "getAuthorizationState" => TdType::AuthorizationState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GetAuthorizationState deserialize to TdType::AuthorizationState with error: {}", _e)))
        }),

      "canTransferOwnership" => TdType::CanTransferOwnershipResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CanTransferOwnership deserialize to TdType::CanTransferOwnershipResult with error: {}", _e)))
        }),

      "canTransferOwnershipResultOk" => TdType::CanTransferOwnershipResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CanTransferOwnershipResultOk deserialize to TdType::CanTransferOwnershipResult with error: {}", _e)))
        }),

      "canTransferOwnershipResultPasswordNeeded" => TdType::CanTransferOwnershipResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CanTransferOwnershipResultPasswordNeeded deserialize to TdType::CanTransferOwnershipResult with error: {}", _e)))
        }),

      "canTransferOwnershipResultPasswordTooFresh" => TdType::CanTransferOwnershipResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CanTransferOwnershipResultPasswordTooFresh deserialize to TdType::CanTransferOwnershipResult with error: {}", _e)))
        }),

      "canTransferOwnershipResultSessionTooFresh" => TdType::CanTransferOwnershipResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CanTransferOwnershipResultSessionTooFresh deserialize to TdType::CanTransferOwnershipResult with error: {}", _e)))
        }),

      "chatStatisticsChannel" => TdType::ChatStatistics(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatStatisticsChannel deserialize to TdType::ChatStatistics with error: {}", _e)))
        }),

      "chatStatisticsSupergroup" => TdType::ChatStatistics(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatStatisticsSupergroup deserialize to TdType::ChatStatistics with error: {}", _e)))
        }),

      "getChatStatistics" => TdType::ChatStatistics(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GetChatStatistics deserialize to TdType::ChatStatistics with error: {}", _e)))
        }),

      "checkChatUsername" => TdType::CheckChatUsernameResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CheckChatUsername deserialize to TdType::CheckChatUsernameResult with error: {}", _e)))
        }),

      "checkChatUsernameResultOk" => TdType::CheckChatUsernameResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CheckChatUsernameResultOk deserialize to TdType::CheckChatUsernameResult with error: {}", _e)))
        }),

      "checkChatUsernameResultPublicChatsTooMuch" => TdType::CheckChatUsernameResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CheckChatUsernameResultPublicChatsTooMuch deserialize to TdType::CheckChatUsernameResult with error: {}", _e)))
        }),

      "checkChatUsernameResultPublicGroupsUnavailable" => TdType::CheckChatUsernameResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CheckChatUsernameResultPublicGroupsUnavailable deserialize to TdType::CheckChatUsernameResult with error: {}", _e)))
        }),

      "checkChatUsernameResultUsernameInvalid" => TdType::CheckChatUsernameResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CheckChatUsernameResultUsernameInvalid deserialize to TdType::CheckChatUsernameResult with error: {}", _e)))
        }),

      "checkChatUsernameResultUsernameOccupied" => TdType::CheckChatUsernameResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CheckChatUsernameResultUsernameOccupied deserialize to TdType::CheckChatUsernameResult with error: {}", _e)))
        }),

      "getApplicationConfig" => TdType::JsonValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GetApplicationConfig deserialize to TdType::JsonValue with error: {}", _e)))
        }),

      "getJsonValue" => TdType::JsonValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GetJsonValue deserialize to TdType::JsonValue with error: {}", _e)))
        }),

      "jsonValueArray" => TdType::JsonValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("JsonValueArray deserialize to TdType::JsonValue with error: {}", _e)))
        }),

      "jsonValueBoolean" => TdType::JsonValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("JsonValueBoolean deserialize to TdType::JsonValue with error: {}", _e)))
        }),

      "jsonValueNull" => TdType::JsonValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("JsonValueNull deserialize to TdType::JsonValue with error: {}", _e)))
        }),

      "jsonValueNumber" => TdType::JsonValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("JsonValueNumber deserialize to TdType::JsonValue with error: {}", _e)))
        }),

      "jsonValueObject" => TdType::JsonValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("JsonValueObject deserialize to TdType::JsonValue with error: {}", _e)))
        }),

      "jsonValueString" => TdType::JsonValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("JsonValueString deserialize to TdType::JsonValue with error: {}", _e)))
        }),

      "getLanguagePackString" => TdType::LanguagePackStringValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GetLanguagePackString deserialize to TdType::LanguagePackStringValue with error: {}", _e)))
        }),

      "languagePackStringValueDeleted" => TdType::LanguagePackStringValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LanguagePackStringValueDeleted deserialize to TdType::LanguagePackStringValue with error: {}", _e)))
        }),

      "languagePackStringValueOrdinary" => TdType::LanguagePackStringValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LanguagePackStringValueOrdinary deserialize to TdType::LanguagePackStringValue with error: {}", _e)))
        }),

      "languagePackStringValuePluralized" => TdType::LanguagePackStringValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LanguagePackStringValuePluralized deserialize to TdType::LanguagePackStringValue with error: {}", _e)))
        }),

      "getLogStream" => TdType::LogStream(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GetLogStream deserialize to TdType::LogStream with error: {}", _e)))
        }),

      "logStreamDefault" => TdType::LogStream(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LogStreamDefault deserialize to TdType::LogStream with error: {}", _e)))
        }),

      "logStreamEmpty" => TdType::LogStream(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LogStreamEmpty deserialize to TdType::LogStream with error: {}", _e)))
        }),

      "logStreamFile" => TdType::LogStream(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LogStreamFile deserialize to TdType::LogStream with error: {}", _e)))
        }),

      "getLoginUrlInfo" => TdType::LoginUrlInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GetLoginUrlInfo deserialize to TdType::LoginUrlInfo with error: {}", _e)))
        }),

      "loginUrlInfoOpen" => TdType::LoginUrlInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LoginUrlInfoOpen deserialize to TdType::LoginUrlInfo with error: {}", _e)))
        }),

      "loginUrlInfoRequestConfirmation" => TdType::LoginUrlInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LoginUrlInfoRequestConfirmation deserialize to TdType::LoginUrlInfo with error: {}", _e)))
        }),

      "getOption" => TdType::OptionValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GetOption deserialize to TdType::OptionValue with error: {}", _e)))
        }),

      "optionValueBoolean" => TdType::OptionValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("OptionValueBoolean deserialize to TdType::OptionValue with error: {}", _e)))
        }),

      "optionValueEmpty" => TdType::OptionValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("OptionValueEmpty deserialize to TdType::OptionValue with error: {}", _e)))
        }),

      "optionValueInteger" => TdType::OptionValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("OptionValueInteger deserialize to TdType::OptionValue with error: {}", _e)))
        }),

      "optionValueString" => TdType::OptionValue(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("OptionValueString deserialize to TdType::OptionValue with error: {}", _e)))
        }),

      "getPassportElement" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GetPassportElement deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementAddress" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementAddress deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementBankStatement" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementBankStatement deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementDriverLicense" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementDriverLicense deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementEmailAddress" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementEmailAddress deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementIdentityCard" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementIdentityCard deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementInternalPassport" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementInternalPassport deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementPassport" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementPassport deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementPassportRegistration" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementPassportRegistration deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementPersonalDetails" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementPersonalDetails deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementPhoneNumber" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementPhoneNumber deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementRentalAgreement" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementRentalAgreement deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementTemporaryRegistration" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementTemporaryRegistration deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "passportElementUtilityBill" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementUtilityBill deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "setPassportElement" => TdType::PassportElement(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("SetPassportElement deserialize to TdType::PassportElement with error: {}", _e)))
        }),

      "getStatisticalGraph" => TdType::StatisticalGraph(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GetStatisticalGraph deserialize to TdType::StatisticalGraph with error: {}", _e)))
        }),

      "statisticalGraphAsync" => TdType::StatisticalGraph(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("StatisticalGraphAsync deserialize to TdType::StatisticalGraph with error: {}", _e)))
        }),

      "statisticalGraphData" => TdType::StatisticalGraph(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("StatisticalGraphData deserialize to TdType::StatisticalGraph with error: {}", _e)))
        }),

      "statisticalGraphError" => TdType::StatisticalGraph(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("StatisticalGraphError deserialize to TdType::StatisticalGraph with error: {}", _e)))
        }),

      "testUseUpdate" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TestUseUpdate deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateActiveNotifications" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateActiveNotifications deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateAnimationSearchParameters" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateAnimationSearchParameters deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateAuthorizationState" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateAuthorizationState deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateBasicGroup" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateBasicGroup deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateBasicGroupFullInfo" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateBasicGroupFullInfo deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateCall" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateCall deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatActionBar" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatActionBar deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatDefaultDisableNotification" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatDefaultDisableNotification deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatDraftMessage" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatDraftMessage deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatFilters" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatFilters deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatHasScheduledMessages" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatHasScheduledMessages deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatIsBlocked" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatIsBlocked deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatIsMarkedAsUnread" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatIsMarkedAsUnread deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatLastMessage" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatLastMessage deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatNotificationSettings" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatNotificationSettings deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatOnlineMemberCount" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatOnlineMemberCount deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatPermissions" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatPermissions deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatPhoto" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatPhoto deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatPosition" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatPosition deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatReadInbox" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatReadInbox deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatReadOutbox" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatReadOutbox deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatReplyMarkup" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatReplyMarkup deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatTitle" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatTitle deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateChatUnreadMentionCount" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateChatUnreadMentionCount deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateConnectionState" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateConnectionState deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateDeleteMessages" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateDeleteMessages deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateDiceEmojis" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateDiceEmojis deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateFavoriteStickers" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateFavoriteStickers deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateFile" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateFile deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateFileGenerationStart" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateFileGenerationStart deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateFileGenerationStop" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateFileGenerationStop deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateHavePendingNotifications" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateHavePendingNotifications deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateInstalledStickerSets" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateInstalledStickerSets deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateLanguagePackStrings" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateLanguagePackStrings deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateMessageContent" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateMessageContent deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateMessageContentOpened" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateMessageContentOpened deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateMessageEdited" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateMessageEdited deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateMessageInteractionInfo" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateMessageInteractionInfo deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateMessageIsPinned" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateMessageIsPinned deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateMessageLiveLocationViewed" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateMessageLiveLocationViewed deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateMessageMentionRead" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateMessageMentionRead deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateMessageSendAcknowledged" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateMessageSendAcknowledged deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateMessageSendFailed" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateMessageSendFailed deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateMessageSendSucceeded" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateMessageSendSucceeded deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewCallSignalingData" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewCallSignalingData deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewCallbackQuery" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewCallbackQuery deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewChat" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewChat deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewChosenInlineResult" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewChosenInlineResult deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewCustomEvent" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewCustomEvent deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewCustomQuery" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewCustomQuery deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewInlineCallbackQuery" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewInlineCallbackQuery deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewInlineQuery" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewInlineQuery deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewMessage" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewMessage deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewPreCheckoutQuery" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewPreCheckoutQuery deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNewShippingQuery" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNewShippingQuery deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNotification" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNotification deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateNotificationGroup" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateNotificationGroup deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateOption" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateOption deserialize to TdType::Update with error: {}", _e)))
        }),

      "updatePoll" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdatePoll deserialize to TdType::Update with error: {}", _e)))
        }),

      "updatePollAnswer" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdatePollAnswer deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateRecentStickers" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateRecentStickers deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateSavedAnimations" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateSavedAnimations deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateScopeNotificationSettings" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateScopeNotificationSettings deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateSecretChat" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateSecretChat deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateSelectedBackground" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateSelectedBackground deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateServiceNotification" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateServiceNotification deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateStickerSet" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateStickerSet deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateSuggestedActions" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateSuggestedActions deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateSupergroup" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateSupergroup deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateSupergroupFullInfo" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateSupergroupFullInfo deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateTermsOfService" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateTermsOfService deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateTrendingStickerSets" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateTrendingStickerSets deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateUnreadChatCount" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateUnreadChatCount deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateUnreadMessageCount" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateUnreadMessageCount deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateUser" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateUser deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateUserChatAction" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateUserChatAction deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateUserFullInfo" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateUserFullInfo deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateUserPrivacySettingRules" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateUserPrivacySettingRules deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateUserStatus" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateUserStatus deserialize to TdType::Update with error: {}", _e)))
        }),

      "updateUsersNearby" => TdType::Update(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UpdateUsersNearby deserialize to TdType::Update with error: {}", _e)))
        }),

      "accountTtl" => TdType::AccountTtl(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AccountTtl deserialize to TdType::AccountTtl with error: {}", _e)))
        }),

      "animations" => TdType::Animations(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Animations deserialize to TdType::Animations with error: {}", _e)))
        }),

      "authenticationCodeInfo" => TdType::AuthenticationCodeInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AuthenticationCodeInfo deserialize to TdType::AuthenticationCodeInfo with error: {}", _e)))
        }),

      "autoDownloadSettingsPresets" => TdType::AutoDownloadSettingsPresets(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("AutoDownloadSettingsPresets deserialize to TdType::AutoDownloadSettingsPresets with error: {}", _e)))
        }),

      "background" => TdType::Background(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Background deserialize to TdType::Background with error: {}", _e)))
        }),

      "backgrounds" => TdType::Backgrounds(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Backgrounds deserialize to TdType::Backgrounds with error: {}", _e)))
        }),

      "bankCardInfo" => TdType::BankCardInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("BankCardInfo deserialize to TdType::BankCardInfo with error: {}", _e)))
        }),

      "basicGroup" => TdType::BasicGroup(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("BasicGroup deserialize to TdType::BasicGroup with error: {}", _e)))
        }),

      "basicGroupFullInfo" => TdType::BasicGroupFullInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("BasicGroupFullInfo deserialize to TdType::BasicGroupFullInfo with error: {}", _e)))
        }),

      "callId" => TdType::CallId(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CallId deserialize to TdType::CallId with error: {}", _e)))
        }),

      "callbackQueryAnswer" => TdType::CallbackQueryAnswer(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CallbackQueryAnswer deserialize to TdType::CallbackQueryAnswer with error: {}", _e)))
        }),

      "chat" => TdType::Chat(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Chat deserialize to TdType::Chat with error: {}", _e)))
        }),

      "chatAdministrators" => TdType::ChatAdministrators(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatAdministrators deserialize to TdType::ChatAdministrators with error: {}", _e)))
        }),

      "chatEvents" => TdType::ChatEvents(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatEvents deserialize to TdType::ChatEvents with error: {}", _e)))
        }),

      "chatFilter" => TdType::ChatFilter(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatFilter deserialize to TdType::ChatFilter with error: {}", _e)))
        }),

      "chatFilterInfo" => TdType::ChatFilterInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatFilterInfo deserialize to TdType::ChatFilterInfo with error: {}", _e)))
        }),

      "chatInviteLink" => TdType::ChatInviteLink(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatInviteLink deserialize to TdType::ChatInviteLink with error: {}", _e)))
        }),

      "chatInviteLinkInfo" => TdType::ChatInviteLinkInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatInviteLinkInfo deserialize to TdType::ChatInviteLinkInfo with error: {}", _e)))
        }),

      "chatLists" => TdType::ChatLists(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatLists deserialize to TdType::ChatLists with error: {}", _e)))
        }),

      "chatMember" => TdType::ChatMember(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatMember deserialize to TdType::ChatMember with error: {}", _e)))
        }),

      "chatMembers" => TdType::ChatMembers(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatMembers deserialize to TdType::ChatMembers with error: {}", _e)))
        }),

      "chatPhotos" => TdType::ChatPhotos(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatPhotos deserialize to TdType::ChatPhotos with error: {}", _e)))
        }),

      "chats" => TdType::Chats(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Chats deserialize to TdType::Chats with error: {}", _e)))
        }),

      "chatsNearby" => TdType::ChatsNearby(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ChatsNearby deserialize to TdType::ChatsNearby with error: {}", _e)))
        }),

      "connectedWebsites" => TdType::ConnectedWebsites(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ConnectedWebsites deserialize to TdType::ConnectedWebsites with error: {}", _e)))
        }),

      "count" => TdType::Count(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Count deserialize to TdType::Count with error: {}", _e)))
        }),

      "countries" => TdType::Countries(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Countries deserialize to TdType::Countries with error: {}", _e)))
        }),

      "customRequestResult" => TdType::CustomRequestResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("CustomRequestResult deserialize to TdType::CustomRequestResult with error: {}", _e)))
        }),

      "databaseStatistics" => TdType::DatabaseStatistics(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("DatabaseStatistics deserialize to TdType::DatabaseStatistics with error: {}", _e)))
        }),

      "deepLinkInfo" => TdType::DeepLinkInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("DeepLinkInfo deserialize to TdType::DeepLinkInfo with error: {}", _e)))
        }),

      "emailAddressAuthenticationCodeInfo" => TdType::EmailAddressAuthenticationCodeInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("EmailAddressAuthenticationCodeInfo deserialize to TdType::EmailAddressAuthenticationCodeInfo with error: {}", _e)))
        }),

      "emojis" => TdType::Emojis(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Emojis deserialize to TdType::Emojis with error: {}", _e)))
        }),

      "error" => TdType::Error(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Error deserialize to TdType::Error with error: {}", _e)))
        }),

      "file" => TdType::File(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("File deserialize to TdType::File with error: {}", _e)))
        }),

      "filePart" => TdType::FilePart(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("FilePart deserialize to TdType::FilePart with error: {}", _e)))
        }),

      "formattedText" => TdType::FormattedText(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("FormattedText deserialize to TdType::FormattedText with error: {}", _e)))
        }),

      "foundMessages" => TdType::FoundMessages(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("FoundMessages deserialize to TdType::FoundMessages with error: {}", _e)))
        }),

      "gameHighScores" => TdType::GameHighScores(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("GameHighScores deserialize to TdType::GameHighScores with error: {}", _e)))
        }),

      "hashtags" => TdType::Hashtags(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Hashtags deserialize to TdType::Hashtags with error: {}", _e)))
        }),

      "httpUrl" => TdType::HttpUrl(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("HttpUrl deserialize to TdType::HttpUrl with error: {}", _e)))
        }),

      "importedContacts" => TdType::ImportedContacts(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ImportedContacts deserialize to TdType::ImportedContacts with error: {}", _e)))
        }),

      "inlineQueryResults" => TdType::InlineQueryResults(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("InlineQueryResults deserialize to TdType::InlineQueryResults with error: {}", _e)))
        }),

      "languagePackInfo" => TdType::LanguagePackInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LanguagePackInfo deserialize to TdType::LanguagePackInfo with error: {}", _e)))
        }),

      "languagePackStrings" => TdType::LanguagePackStrings(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LanguagePackStrings deserialize to TdType::LanguagePackStrings with error: {}", _e)))
        }),

      "localizationTargetInfo" => TdType::LocalizationTargetInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LocalizationTargetInfo deserialize to TdType::LocalizationTargetInfo with error: {}", _e)))
        }),

      "logTags" => TdType::LogTags(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LogTags deserialize to TdType::LogTags with error: {}", _e)))
        }),

      "logVerbosityLevel" => TdType::LogVerbosityLevel(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("LogVerbosityLevel deserialize to TdType::LogVerbosityLevel with error: {}", _e)))
        }),

      "message" => TdType::Message(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Message deserialize to TdType::Message with error: {}", _e)))
        }),

      "messageLink" => TdType::MessageLink(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("MessageLink deserialize to TdType::MessageLink with error: {}", _e)))
        }),

      "messageLinkInfo" => TdType::MessageLinkInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("MessageLinkInfo deserialize to TdType::MessageLinkInfo with error: {}", _e)))
        }),

      "messageSenders" => TdType::MessageSenders(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("MessageSenders deserialize to TdType::MessageSenders with error: {}", _e)))
        }),

      "messageStatistics" => TdType::MessageStatistics(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("MessageStatistics deserialize to TdType::MessageStatistics with error: {}", _e)))
        }),

      "messageThreadInfo" => TdType::MessageThreadInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("MessageThreadInfo deserialize to TdType::MessageThreadInfo with error: {}", _e)))
        }),

      "messages" => TdType::Messages(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Messages deserialize to TdType::Messages with error: {}", _e)))
        }),

      "networkStatistics" => TdType::NetworkStatistics(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("NetworkStatistics deserialize to TdType::NetworkStatistics with error: {}", _e)))
        }),

      "ok" => TdType::Ok(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Ok deserialize to TdType::Ok with error: {}", _e)))
        }),

      "orderInfo" => TdType::OrderInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("OrderInfo deserialize to TdType::OrderInfo with error: {}", _e)))
        }),

      "passportAuthorizationForm" => TdType::PassportAuthorizationForm(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportAuthorizationForm deserialize to TdType::PassportAuthorizationForm with error: {}", _e)))
        }),

      "passportElements" => TdType::PassportElements(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElements deserialize to TdType::PassportElements with error: {}", _e)))
        }),

      "passportElementsWithErrors" => TdType::PassportElementsWithErrors(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PassportElementsWithErrors deserialize to TdType::PassportElementsWithErrors with error: {}", _e)))
        }),

      "passwordState" => TdType::PasswordState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PasswordState deserialize to TdType::PasswordState with error: {}", _e)))
        }),

      "paymentForm" => TdType::PaymentForm(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PaymentForm deserialize to TdType::PaymentForm with error: {}", _e)))
        }),

      "paymentReceipt" => TdType::PaymentReceipt(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PaymentReceipt deserialize to TdType::PaymentReceipt with error: {}", _e)))
        }),

      "paymentResult" => TdType::PaymentResult(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PaymentResult deserialize to TdType::PaymentResult with error: {}", _e)))
        }),

      "phoneNumberInfo" => TdType::PhoneNumberInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PhoneNumberInfo deserialize to TdType::PhoneNumberInfo with error: {}", _e)))
        }),

      "proxies" => TdType::Proxies(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Proxies deserialize to TdType::Proxies with error: {}", _e)))
        }),

      "proxy" => TdType::Proxy(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Proxy deserialize to TdType::Proxy with error: {}", _e)))
        }),

      "pushReceiverId" => TdType::PushReceiverId(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("PushReceiverId deserialize to TdType::PushReceiverId with error: {}", _e)))
        }),

      "recommendedChatFilters" => TdType::RecommendedChatFilters(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("RecommendedChatFilters deserialize to TdType::RecommendedChatFilters with error: {}", _e)))
        }),

      "recoveryEmailAddress" => TdType::RecoveryEmailAddress(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("RecoveryEmailAddress deserialize to TdType::RecoveryEmailAddress with error: {}", _e)))
        }),

      "scopeNotificationSettings" => TdType::ScopeNotificationSettings(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ScopeNotificationSettings deserialize to TdType::ScopeNotificationSettings with error: {}", _e)))
        }),

      "seconds" => TdType::Seconds(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Seconds deserialize to TdType::Seconds with error: {}", _e)))
        }),

      "secretChat" => TdType::SecretChat(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("SecretChat deserialize to TdType::SecretChat with error: {}", _e)))
        }),

      "session" => TdType::Session(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Session deserialize to TdType::Session with error: {}", _e)))
        }),

      "sessions" => TdType::Sessions(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Sessions deserialize to TdType::Sessions with error: {}", _e)))
        }),

      "stickerSet" => TdType::StickerSet(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("StickerSet deserialize to TdType::StickerSet with error: {}", _e)))
        }),

      "stickerSets" => TdType::StickerSets(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("StickerSets deserialize to TdType::StickerSets with error: {}", _e)))
        }),

      "stickers" => TdType::Stickers(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Stickers deserialize to TdType::Stickers with error: {}", _e)))
        }),

      "storageStatistics" => TdType::StorageStatistics(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("StorageStatistics deserialize to TdType::StorageStatistics with error: {}", _e)))
        }),

      "storageStatisticsFast" => TdType::StorageStatisticsFast(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("StorageStatisticsFast deserialize to TdType::StorageStatisticsFast with error: {}", _e)))
        }),

      "supergroup" => TdType::Supergroup(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Supergroup deserialize to TdType::Supergroup with error: {}", _e)))
        }),

      "supergroupFullInfo" => TdType::SupergroupFullInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("SupergroupFullInfo deserialize to TdType::SupergroupFullInfo with error: {}", _e)))
        }),

      "tMeUrls" => TdType::TMeUrls(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TMeUrls deserialize to TdType::TMeUrls with error: {}", _e)))
        }),

      "temporaryPasswordState" => TdType::TemporaryPasswordState(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TemporaryPasswordState deserialize to TdType::TemporaryPasswordState with error: {}", _e)))
        }),

      "testBytes" => TdType::TestBytes(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TestBytes deserialize to TdType::TestBytes with error: {}", _e)))
        }),

      "testInt" => TdType::TestInt(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TestInt deserialize to TdType::TestInt with error: {}", _e)))
        }),

      "testString" => TdType::TestString(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TestString deserialize to TdType::TestString with error: {}", _e)))
        }),

      "testVectorInt" => TdType::TestVectorInt(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TestVectorInt deserialize to TdType::TestVectorInt with error: {}", _e)))
        }),

      "testVectorIntObject" => TdType::TestVectorIntObject(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TestVectorIntObject deserialize to TdType::TestVectorIntObject with error: {}", _e)))
        }),

      "testVectorString" => TdType::TestVectorString(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TestVectorString deserialize to TdType::TestVectorString with error: {}", _e)))
        }),

      "testVectorStringObject" => TdType::TestVectorStringObject(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TestVectorStringObject deserialize to TdType::TestVectorStringObject with error: {}", _e)))
        }),

      "text" => TdType::Text(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Text deserialize to TdType::Text with error: {}", _e)))
        }),

      "textEntities" => TdType::TextEntities(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("TextEntities deserialize to TdType::TextEntities with error: {}", _e)))
        }),

      "updates" => TdType::Updates(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Updates deserialize to TdType::Updates with error: {}", _e)))
        }),

      "user" => TdType::User(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("User deserialize to TdType::User with error: {}", _e)))
        }),

      "userFullInfo" => TdType::UserFullInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UserFullInfo deserialize to TdType::UserFullInfo with error: {}", _e)))
        }),

      "userPrivacySettingRules" => TdType::UserPrivacySettingRules(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("UserPrivacySettingRules deserialize to TdType::UserPrivacySettingRules with error: {}", _e)))
        }),

      "users" => TdType::Users(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("Users deserialize to TdType::Users with error: {}", _e)))
        }),

      "validatedOrderInfo" => TdType::ValidatedOrderInfo(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("ValidatedOrderInfo deserialize to TdType::ValidatedOrderInfo with error: {}", _e)))
        }),

      "webPage" => TdType::WebPage(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("WebPage deserialize to TdType::WebPage with error: {}", _e)))
        }),

      "webPageInstantView" => TdType::WebPageInstantView(match serde_json::from_value(rtd_trait_value.clone()) {
            Ok(t) => t,
            Err(_e) => return Err(D::Error::custom(format!("WebPageInstantView deserialize to TdType::WebPageInstantView with error: {}", _e)))
        }),

      _ => return Err(D::Error::custom(format!("got {} @type with unavailable variant", rtd_trait_type)))
    };
        Ok(obj)
    }
}

impl TdType {
    pub fn client_id(&self) -> Option<i32> {
        match self {
            TdType::AuthorizationState(value) => value.client_id(),

            TdType::CanTransferOwnershipResult(value) => value.client_id(),

            TdType::ChatStatistics(value) => value.client_id(),

            TdType::CheckChatUsernameResult(value) => value.client_id(),

            TdType::JsonValue(value) => value.client_id(),

            TdType::LanguagePackStringValue(value) => value.client_id(),

            TdType::LogStream(value) => value.client_id(),

            TdType::LoginUrlInfo(value) => value.client_id(),

            TdType::OptionValue(value) => value.client_id(),

            TdType::PassportElement(value) => value.client_id(),

            TdType::StatisticalGraph(value) => value.client_id(),

            TdType::Update(value) => value.client_id(),

            TdType::AccountTtl(value) => value.client_id(),

            TdType::Animations(value) => value.client_id(),

            TdType::AuthenticationCodeInfo(value) => value.client_id(),

            TdType::AutoDownloadSettingsPresets(value) => value.client_id(),

            TdType::Background(value) => value.client_id(),

            TdType::Backgrounds(value) => value.client_id(),

            TdType::BankCardInfo(value) => value.client_id(),

            TdType::BasicGroup(value) => value.client_id(),

            TdType::BasicGroupFullInfo(value) => value.client_id(),

            TdType::CallId(value) => value.client_id(),

            TdType::CallbackQueryAnswer(value) => value.client_id(),

            TdType::Chat(value) => value.client_id(),

            TdType::ChatAdministrators(value) => value.client_id(),

            TdType::ChatEvents(value) => value.client_id(),

            TdType::ChatFilter(value) => value.client_id(),

            TdType::ChatFilterInfo(value) => value.client_id(),

            TdType::ChatInviteLink(value) => value.client_id(),

            TdType::ChatInviteLinkInfo(value) => value.client_id(),

            TdType::ChatLists(value) => value.client_id(),

            TdType::ChatMember(value) => value.client_id(),

            TdType::ChatMembers(value) => value.client_id(),

            TdType::ChatPhotos(value) => value.client_id(),

            TdType::Chats(value) => value.client_id(),

            TdType::ChatsNearby(value) => value.client_id(),

            TdType::ConnectedWebsites(value) => value.client_id(),

            TdType::Count(value) => value.client_id(),

            TdType::Countries(value) => value.client_id(),

            TdType::CustomRequestResult(value) => value.client_id(),

            TdType::DatabaseStatistics(value) => value.client_id(),

            TdType::DeepLinkInfo(value) => value.client_id(),

            TdType::EmailAddressAuthenticationCodeInfo(value) => value.client_id(),

            TdType::Emojis(value) => value.client_id(),

            TdType::Error(value) => value.client_id(),

            TdType::File(value) => value.client_id(),

            TdType::FilePart(value) => value.client_id(),

            TdType::FormattedText(value) => value.client_id(),

            TdType::FoundMessages(value) => value.client_id(),

            TdType::GameHighScores(value) => value.client_id(),

            TdType::Hashtags(value) => value.client_id(),

            TdType::HttpUrl(value) => value.client_id(),

            TdType::ImportedContacts(value) => value.client_id(),

            TdType::InlineQueryResults(value) => value.client_id(),

            TdType::LanguagePackInfo(value) => value.client_id(),

            TdType::LanguagePackStrings(value) => value.client_id(),

            TdType::LocalizationTargetInfo(value) => value.client_id(),

            TdType::LogTags(value) => value.client_id(),

            TdType::LogVerbosityLevel(value) => value.client_id(),

            TdType::Message(value) => value.client_id(),

            TdType::MessageLink(value) => value.client_id(),

            TdType::MessageLinkInfo(value) => value.client_id(),

            TdType::MessageSenders(value) => value.client_id(),

            TdType::MessageStatistics(value) => value.client_id(),

            TdType::MessageThreadInfo(value) => value.client_id(),

            TdType::Messages(value) => value.client_id(),

            TdType::NetworkStatistics(value) => value.client_id(),

            TdType::Ok(value) => value.client_id(),

            TdType::OrderInfo(value) => value.client_id(),

            TdType::PassportAuthorizationForm(value) => value.client_id(),

            TdType::PassportElements(value) => value.client_id(),

            TdType::PassportElementsWithErrors(value) => value.client_id(),

            TdType::PasswordState(value) => value.client_id(),

            TdType::PaymentForm(value) => value.client_id(),

            TdType::PaymentReceipt(value) => value.client_id(),

            TdType::PaymentResult(value) => value.client_id(),

            TdType::PhoneNumberInfo(value) => value.client_id(),

            TdType::Proxies(value) => value.client_id(),

            TdType::Proxy(value) => value.client_id(),

            TdType::PushReceiverId(value) => value.client_id(),

            TdType::RecommendedChatFilters(value) => value.client_id(),

            TdType::RecoveryEmailAddress(value) => value.client_id(),

            TdType::ScopeNotificationSettings(value) => value.client_id(),

            TdType::Seconds(value) => value.client_id(),

            TdType::SecretChat(value) => value.client_id(),

            TdType::Session(value) => value.client_id(),

            TdType::Sessions(value) => value.client_id(),

            TdType::StickerSet(value) => value.client_id(),

            TdType::StickerSets(value) => value.client_id(),

            TdType::Stickers(value) => value.client_id(),

            TdType::StorageStatistics(value) => value.client_id(),

            TdType::StorageStatisticsFast(value) => value.client_id(),

            TdType::Supergroup(value) => value.client_id(),

            TdType::SupergroupFullInfo(value) => value.client_id(),

            TdType::TMeUrls(value) => value.client_id(),

            TdType::TemporaryPasswordState(value) => value.client_id(),

            TdType::TestBytes(value) => value.client_id(),

            TdType::TestInt(value) => value.client_id(),

            TdType::TestString(value) => value.client_id(),

            TdType::TestVectorInt(value) => value.client_id(),

            TdType::TestVectorIntObject(value) => value.client_id(),

            TdType::TestVectorString(value) => value.client_id(),

            TdType::TestVectorStringObject(value) => value.client_id(),

            TdType::Text(value) => value.client_id(),

            TdType::TextEntities(value) => value.client_id(),

            TdType::Updates(value) => value.client_id(),

            TdType::User(value) => value.client_id(),

            TdType::UserFullInfo(value) => value.client_id(),

            TdType::UserPrivacySettingRules(value) => value.client_id(),

            TdType::Users(value) => value.client_id(),

            TdType::ValidatedOrderInfo(value) => value.client_id(),

            TdType::WebPage(value) => value.client_id(),

            TdType::WebPageInstantView(value) => value.client_id(),
        }
    }

    pub fn extra(&self) -> Option<String> {
        match self {
            TdType::AuthorizationState(value) => value.extra(),

            TdType::CanTransferOwnershipResult(value) => value.extra(),

            TdType::ChatStatistics(value) => value.extra(),

            TdType::CheckChatUsernameResult(value) => value.extra(),

            TdType::JsonValue(value) => value.extra(),

            TdType::LanguagePackStringValue(value) => value.extra(),

            TdType::LogStream(value) => value.extra(),

            TdType::LoginUrlInfo(value) => value.extra(),

            TdType::OptionValue(value) => value.extra(),

            TdType::PassportElement(value) => value.extra(),

            TdType::StatisticalGraph(value) => value.extra(),

            TdType::Update(value) => value.extra(),

            TdType::AccountTtl(value) => value.extra(),

            TdType::Animations(value) => value.extra(),

            TdType::AuthenticationCodeInfo(value) => value.extra(),

            TdType::AutoDownloadSettingsPresets(value) => value.extra(),

            TdType::Background(value) => value.extra(),

            TdType::Backgrounds(value) => value.extra(),

            TdType::BankCardInfo(value) => value.extra(),

            TdType::BasicGroup(value) => value.extra(),

            TdType::BasicGroupFullInfo(value) => value.extra(),

            TdType::CallId(value) => value.extra(),

            TdType::CallbackQueryAnswer(value) => value.extra(),

            TdType::Chat(value) => value.extra(),

            TdType::ChatAdministrators(value) => value.extra(),

            TdType::ChatEvents(value) => value.extra(),

            TdType::ChatFilter(value) => value.extra(),

            TdType::ChatFilterInfo(value) => value.extra(),

            TdType::ChatInviteLink(value) => value.extra(),

            TdType::ChatInviteLinkInfo(value) => value.extra(),

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

            TdType::FilePart(value) => value.extra(),

            TdType::FormattedText(value) => value.extra(),

            TdType::FoundMessages(value) => value.extra(),

            TdType::GameHighScores(value) => value.extra(),

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

            TdType::MessageLink(value) => value.extra(),

            TdType::MessageLinkInfo(value) => value.extra(),

            TdType::MessageSenders(value) => value.extra(),

            TdType::MessageStatistics(value) => value.extra(),

            TdType::MessageThreadInfo(value) => value.extra(),

            TdType::Messages(value) => value.extra(),

            TdType::NetworkStatistics(value) => value.extra(),

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

            TdType::Proxies(value) => value.extra(),

            TdType::Proxy(value) => value.extra(),

            TdType::PushReceiverId(value) => value.extra(),

            TdType::RecommendedChatFilters(value) => value.extra(),

            TdType::RecoveryEmailAddress(value) => value.extra(),

            TdType::ScopeNotificationSettings(value) => value.extra(),

            TdType::Seconds(value) => value.extra(),

            TdType::SecretChat(value) => value.extra(),

            TdType::Session(value) => value.extra(),

            TdType::Sessions(value) => value.extra(),

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

            TdType::Updates(value) => value.extra(),

            TdType::User(value) => value.extra(),

            TdType::UserFullInfo(value) => value.extra(),

            TdType::UserPrivacySettingRules(value) => value.extra(),

            TdType::Users(value) => value.extra(),

            TdType::ValidatedOrderInfo(value) => value.extra(),

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

#[cfg(test)]
mod tests {
    use crate::types::{from_json, AuthorizationState, TdType, Update, UpdateAuthorizationState};

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
