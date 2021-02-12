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

        let obj = match rtd_trait_type {
      

      "authorizationStateClosed" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateClosed deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "authorizationStateClosing" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateClosing deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "authorizationStateLoggingOut" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateLoggingOut deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "authorizationStateReady" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateReady deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "authorizationStateWaitCode" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateWaitCode deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "authorizationStateWaitEncryptionKey" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateWaitEncryptionKey deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "authorizationStateWaitOtherDeviceConfirmation" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateWaitOtherDeviceConfirmation deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "authorizationStateWaitPassword" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateWaitPassword deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "authorizationStateWaitPhoneNumber" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateWaitPhoneNumber deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "authorizationStateWaitRegistration" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateWaitRegistration deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "authorizationStateWaitTdlibParameters" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthorizationStateWaitTdlibParameters deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "getAuthorizationState" => TdType::AuthorizationState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GetAuthorizationState deserialize to TdType::AuthorizationState with error: {}", e))
          )?
      ),
      

      "canTransferOwnership" => TdType::CanTransferOwnershipResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CanTransferOwnership deserialize to TdType::CanTransferOwnershipResult with error: {}", e))
          )?
      ),
      

      "canTransferOwnershipResultOk" => TdType::CanTransferOwnershipResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CanTransferOwnershipResultOk deserialize to TdType::CanTransferOwnershipResult with error: {}", e))
          )?
      ),
      

      "canTransferOwnershipResultPasswordNeeded" => TdType::CanTransferOwnershipResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CanTransferOwnershipResultPasswordNeeded deserialize to TdType::CanTransferOwnershipResult with error: {}", e))
          )?
      ),
      

      "canTransferOwnershipResultPasswordTooFresh" => TdType::CanTransferOwnershipResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CanTransferOwnershipResultPasswordTooFresh deserialize to TdType::CanTransferOwnershipResult with error: {}", e))
          )?
      ),
      

      "canTransferOwnershipResultSessionTooFresh" => TdType::CanTransferOwnershipResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CanTransferOwnershipResultSessionTooFresh deserialize to TdType::CanTransferOwnershipResult with error: {}", e))
          )?
      ),
      

      "chatStatisticsChannel" => TdType::ChatStatistics(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatStatisticsChannel deserialize to TdType::ChatStatistics with error: {}", e))
          )?
      ),
      

      "chatStatisticsSupergroup" => TdType::ChatStatistics(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatStatisticsSupergroup deserialize to TdType::ChatStatistics with error: {}", e))
          )?
      ),
      

      "getChatStatistics" => TdType::ChatStatistics(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GetChatStatistics deserialize to TdType::ChatStatistics with error: {}", e))
          )?
      ),
      

      "checkChatUsername" => TdType::CheckChatUsernameResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CheckChatUsername deserialize to TdType::CheckChatUsernameResult with error: {}", e))
          )?
      ),
      

      "checkChatUsernameResultOk" => TdType::CheckChatUsernameResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CheckChatUsernameResultOk deserialize to TdType::CheckChatUsernameResult with error: {}", e))
          )?
      ),
      

      "checkChatUsernameResultPublicChatsTooMuch" => TdType::CheckChatUsernameResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CheckChatUsernameResultPublicChatsTooMuch deserialize to TdType::CheckChatUsernameResult with error: {}", e))
          )?
      ),
      

      "checkChatUsernameResultPublicGroupsUnavailable" => TdType::CheckChatUsernameResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CheckChatUsernameResultPublicGroupsUnavailable deserialize to TdType::CheckChatUsernameResult with error: {}", e))
          )?
      ),
      

      "checkChatUsernameResultUsernameInvalid" => TdType::CheckChatUsernameResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CheckChatUsernameResultUsernameInvalid deserialize to TdType::CheckChatUsernameResult with error: {}", e))
          )?
      ),
      

      "checkChatUsernameResultUsernameOccupied" => TdType::CheckChatUsernameResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CheckChatUsernameResultUsernameOccupied deserialize to TdType::CheckChatUsernameResult with error: {}", e))
          )?
      ),
      

      "getApplicationConfig" => TdType::JsonValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GetApplicationConfig deserialize to TdType::JsonValue with error: {}", e))
          )?
      ),
      

      "getJsonValue" => TdType::JsonValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GetJsonValue deserialize to TdType::JsonValue with error: {}", e))
          )?
      ),
      

      "jsonValueArray" => TdType::JsonValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("JsonValueArray deserialize to TdType::JsonValue with error: {}", e))
          )?
      ),
      

      "jsonValueBoolean" => TdType::JsonValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("JsonValueBoolean deserialize to TdType::JsonValue with error: {}", e))
          )?
      ),
      

      "jsonValueNull" => TdType::JsonValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("JsonValueNull deserialize to TdType::JsonValue with error: {}", e))
          )?
      ),
      

      "jsonValueNumber" => TdType::JsonValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("JsonValueNumber deserialize to TdType::JsonValue with error: {}", e))
          )?
      ),
      

      "jsonValueObject" => TdType::JsonValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("JsonValueObject deserialize to TdType::JsonValue with error: {}", e))
          )?
      ),
      

      "jsonValueString" => TdType::JsonValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("JsonValueString deserialize to TdType::JsonValue with error: {}", e))
          )?
      ),
      

      "getLanguagePackString" => TdType::LanguagePackStringValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GetLanguagePackString deserialize to TdType::LanguagePackStringValue with error: {}", e))
          )?
      ),
      

      "languagePackStringValueDeleted" => TdType::LanguagePackStringValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LanguagePackStringValueDeleted deserialize to TdType::LanguagePackStringValue with error: {}", e))
          )?
      ),
      

      "languagePackStringValueOrdinary" => TdType::LanguagePackStringValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LanguagePackStringValueOrdinary deserialize to TdType::LanguagePackStringValue with error: {}", e))
          )?
      ),
      

      "languagePackStringValuePluralized" => TdType::LanguagePackStringValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LanguagePackStringValuePluralized deserialize to TdType::LanguagePackStringValue with error: {}", e))
          )?
      ),
      

      "getLogStream" => TdType::LogStream(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GetLogStream deserialize to TdType::LogStream with error: {}", e))
          )?
      ),
      

      "logStreamDefault" => TdType::LogStream(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LogStreamDefault deserialize to TdType::LogStream with error: {}", e))
          )?
      ),
      

      "logStreamEmpty" => TdType::LogStream(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LogStreamEmpty deserialize to TdType::LogStream with error: {}", e))
          )?
      ),
      

      "logStreamFile" => TdType::LogStream(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LogStreamFile deserialize to TdType::LogStream with error: {}", e))
          )?
      ),
      

      "getLoginUrlInfo" => TdType::LoginUrlInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GetLoginUrlInfo deserialize to TdType::LoginUrlInfo with error: {}", e))
          )?
      ),
      

      "loginUrlInfoOpen" => TdType::LoginUrlInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LoginUrlInfoOpen deserialize to TdType::LoginUrlInfo with error: {}", e))
          )?
      ),
      

      "loginUrlInfoRequestConfirmation" => TdType::LoginUrlInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LoginUrlInfoRequestConfirmation deserialize to TdType::LoginUrlInfo with error: {}", e))
          )?
      ),
      

      "getOption" => TdType::OptionValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GetOption deserialize to TdType::OptionValue with error: {}", e))
          )?
      ),
      

      "optionValueBoolean" => TdType::OptionValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("OptionValueBoolean deserialize to TdType::OptionValue with error: {}", e))
          )?
      ),
      

      "optionValueEmpty" => TdType::OptionValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("OptionValueEmpty deserialize to TdType::OptionValue with error: {}", e))
          )?
      ),
      

      "optionValueInteger" => TdType::OptionValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("OptionValueInteger deserialize to TdType::OptionValue with error: {}", e))
          )?
      ),
      

      "optionValueString" => TdType::OptionValue(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("OptionValueString deserialize to TdType::OptionValue with error: {}", e))
          )?
      ),
      

      "getPassportElement" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GetPassportElement deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementAddress" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementAddress deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementBankStatement" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementBankStatement deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementDriverLicense" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementDriverLicense deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementEmailAddress" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementEmailAddress deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementIdentityCard" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementIdentityCard deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementInternalPassport" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementInternalPassport deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementPassport" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementPassport deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementPassportRegistration" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementPassportRegistration deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementPersonalDetails" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementPersonalDetails deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementPhoneNumber" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementPhoneNumber deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementRentalAgreement" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementRentalAgreement deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementTemporaryRegistration" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementTemporaryRegistration deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "passportElementUtilityBill" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementUtilityBill deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "setPassportElement" => TdType::PassportElement(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("SetPassportElement deserialize to TdType::PassportElement with error: {}", e))
          )?
      ),
      

      "getStatisticalGraph" => TdType::StatisticalGraph(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GetStatisticalGraph deserialize to TdType::StatisticalGraph with error: {}", e))
          )?
      ),
      

      "statisticalGraphAsync" => TdType::StatisticalGraph(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("StatisticalGraphAsync deserialize to TdType::StatisticalGraph with error: {}", e))
          )?
      ),
      

      "statisticalGraphData" => TdType::StatisticalGraph(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("StatisticalGraphData deserialize to TdType::StatisticalGraph with error: {}", e))
          )?
      ),
      

      "statisticalGraphError" => TdType::StatisticalGraph(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("StatisticalGraphError deserialize to TdType::StatisticalGraph with error: {}", e))
          )?
      ),
      

      "testUseUpdate" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TestUseUpdate deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateActiveNotifications" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateActiveNotifications deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateAnimationSearchParameters" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateAnimationSearchParameters deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateAuthorizationState" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateAuthorizationState deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateBasicGroup" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateBasicGroup deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateBasicGroupFullInfo" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateBasicGroupFullInfo deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateCall" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateCall deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatActionBar" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatActionBar deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatDefaultDisableNotification" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatDefaultDisableNotification deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatDraftMessage" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatDraftMessage deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatFilters" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatFilters deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatHasScheduledMessages" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatHasScheduledMessages deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatIsBlocked" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatIsBlocked deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatIsMarkedAsUnread" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatIsMarkedAsUnread deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatLastMessage" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatLastMessage deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatNotificationSettings" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatNotificationSettings deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatOnlineMemberCount" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatOnlineMemberCount deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatPermissions" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatPermissions deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatPhoto" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatPhoto deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatPosition" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatPosition deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatReadInbox" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatReadInbox deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatReadOutbox" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatReadOutbox deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatReplyMarkup" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatReplyMarkup deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatTitle" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatTitle deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateChatUnreadMentionCount" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateChatUnreadMentionCount deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateConnectionState" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateConnectionState deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateDeleteMessages" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateDeleteMessages deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateDiceEmojis" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateDiceEmojis deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateFavoriteStickers" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateFavoriteStickers deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateFile" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateFile deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateFileGenerationStart" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateFileGenerationStart deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateFileGenerationStop" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateFileGenerationStop deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateHavePendingNotifications" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateHavePendingNotifications deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateInstalledStickerSets" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateInstalledStickerSets deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateLanguagePackStrings" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateLanguagePackStrings deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateMessageContent" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateMessageContent deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateMessageContentOpened" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateMessageContentOpened deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateMessageEdited" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateMessageEdited deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateMessageInteractionInfo" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateMessageInteractionInfo deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateMessageIsPinned" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateMessageIsPinned deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateMessageLiveLocationViewed" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateMessageLiveLocationViewed deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateMessageMentionRead" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateMessageMentionRead deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateMessageSendAcknowledged" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateMessageSendAcknowledged deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateMessageSendFailed" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateMessageSendFailed deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateMessageSendSucceeded" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateMessageSendSucceeded deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewCallSignalingData" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewCallSignalingData deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewCallbackQuery" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewCallbackQuery deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewChat" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewChat deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewChosenInlineResult" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewChosenInlineResult deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewCustomEvent" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewCustomEvent deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewCustomQuery" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewCustomQuery deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewInlineCallbackQuery" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewInlineCallbackQuery deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewInlineQuery" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewInlineQuery deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewMessage" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewMessage deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewPreCheckoutQuery" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewPreCheckoutQuery deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNewShippingQuery" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNewShippingQuery deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNotification" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNotification deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateNotificationGroup" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateNotificationGroup deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateOption" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateOption deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updatePoll" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdatePoll deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updatePollAnswer" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdatePollAnswer deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateRecentStickers" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateRecentStickers deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateSavedAnimations" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateSavedAnimations deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateScopeNotificationSettings" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateScopeNotificationSettings deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateSecretChat" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateSecretChat deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateSelectedBackground" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateSelectedBackground deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateServiceNotification" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateServiceNotification deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateStickerSet" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateStickerSet deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateSuggestedActions" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateSuggestedActions deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateSupergroup" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateSupergroup deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateSupergroupFullInfo" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateSupergroupFullInfo deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateTermsOfService" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateTermsOfService deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateTrendingStickerSets" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateTrendingStickerSets deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateUnreadChatCount" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateUnreadChatCount deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateUnreadMessageCount" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateUnreadMessageCount deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateUser" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateUser deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateUserChatAction" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateUserChatAction deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateUserFullInfo" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateUserFullInfo deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateUserPrivacySettingRules" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateUserPrivacySettingRules deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateUserStatus" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateUserStatus deserialize to TdType::Update with error: {}", e))
          )?
      ),
      

      "updateUsersNearby" => TdType::Update(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UpdateUsersNearby deserialize to TdType::Update with error: {}", e))
          )?
      ),
      
      "accountTtl" => TdType::AccountTtl(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AccountTtl deserialize to TdType::AccountTtl with error: {}", e))
          )?
      ),
      
      "animations" => TdType::Animations(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Animations deserialize to TdType::Animations with error: {}", e))
          )?
      ),
      
      "authenticationCodeInfo" => TdType::AuthenticationCodeInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AuthenticationCodeInfo deserialize to TdType::AuthenticationCodeInfo with error: {}", e))
          )?
      ),
      
      "autoDownloadSettingsPresets" => TdType::AutoDownloadSettingsPresets(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("AutoDownloadSettingsPresets deserialize to TdType::AutoDownloadSettingsPresets with error: {}", e))
          )?
      ),
      
      "background" => TdType::Background(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Background deserialize to TdType::Background with error: {}", e))
          )?
      ),
      
      "backgrounds" => TdType::Backgrounds(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Backgrounds deserialize to TdType::Backgrounds with error: {}", e))
          )?
      ),
      
      "bankCardInfo" => TdType::BankCardInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("BankCardInfo deserialize to TdType::BankCardInfo with error: {}", e))
          )?
      ),
      
      "basicGroup" => TdType::BasicGroup(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("BasicGroup deserialize to TdType::BasicGroup with error: {}", e))
          )?
      ),
      
      "basicGroupFullInfo" => TdType::BasicGroupFullInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("BasicGroupFullInfo deserialize to TdType::BasicGroupFullInfo with error: {}", e))
          )?
      ),
      
      "callId" => TdType::CallId(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CallId deserialize to TdType::CallId with error: {}", e))
          )?
      ),
      
      "callbackQueryAnswer" => TdType::CallbackQueryAnswer(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CallbackQueryAnswer deserialize to TdType::CallbackQueryAnswer with error: {}", e))
          )?
      ),
      
      "chat" => TdType::Chat(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Chat deserialize to TdType::Chat with error: {}", e))
          )?
      ),
      
      "chatAdministrators" => TdType::ChatAdministrators(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatAdministrators deserialize to TdType::ChatAdministrators with error: {}", e))
          )?
      ),
      
      "chatEvents" => TdType::ChatEvents(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatEvents deserialize to TdType::ChatEvents with error: {}", e))
          )?
      ),
      
      "chatFilter" => TdType::ChatFilter(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatFilter deserialize to TdType::ChatFilter with error: {}", e))
          )?
      ),
      
      "chatFilterInfo" => TdType::ChatFilterInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatFilterInfo deserialize to TdType::ChatFilterInfo with error: {}", e))
          )?
      ),
      
      "chatInviteLink" => TdType::ChatInviteLink(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatInviteLink deserialize to TdType::ChatInviteLink with error: {}", e))
          )?
      ),
      
      "chatInviteLinkInfo" => TdType::ChatInviteLinkInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatInviteLinkInfo deserialize to TdType::ChatInviteLinkInfo with error: {}", e))
          )?
      ),
      
      "chatLists" => TdType::ChatLists(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatLists deserialize to TdType::ChatLists with error: {}", e))
          )?
      ),
      
      "chatMember" => TdType::ChatMember(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatMember deserialize to TdType::ChatMember with error: {}", e))
          )?
      ),
      
      "chatMembers" => TdType::ChatMembers(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatMembers deserialize to TdType::ChatMembers with error: {}", e))
          )?
      ),
      
      "chatPhotos" => TdType::ChatPhotos(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatPhotos deserialize to TdType::ChatPhotos with error: {}", e))
          )?
      ),
      
      "chats" => TdType::Chats(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Chats deserialize to TdType::Chats with error: {}", e))
          )?
      ),
      
      "chatsNearby" => TdType::ChatsNearby(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ChatsNearby deserialize to TdType::ChatsNearby with error: {}", e))
          )?
      ),
      
      "connectedWebsites" => TdType::ConnectedWebsites(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ConnectedWebsites deserialize to TdType::ConnectedWebsites with error: {}", e))
          )?
      ),
      
      "count" => TdType::Count(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Count deserialize to TdType::Count with error: {}", e))
          )?
      ),
      
      "countries" => TdType::Countries(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Countries deserialize to TdType::Countries with error: {}", e))
          )?
      ),
      
      "customRequestResult" => TdType::CustomRequestResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("CustomRequestResult deserialize to TdType::CustomRequestResult with error: {}", e))
          )?
      ),
      
      "databaseStatistics" => TdType::DatabaseStatistics(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("DatabaseStatistics deserialize to TdType::DatabaseStatistics with error: {}", e))
          )?
      ),
      
      "deepLinkInfo" => TdType::DeepLinkInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("DeepLinkInfo deserialize to TdType::DeepLinkInfo with error: {}", e))
          )?
      ),
      
      "emailAddressAuthenticationCodeInfo" => TdType::EmailAddressAuthenticationCodeInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("EmailAddressAuthenticationCodeInfo deserialize to TdType::EmailAddressAuthenticationCodeInfo with error: {}", e))
          )?
      ),
      
      "emojis" => TdType::Emojis(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Emojis deserialize to TdType::Emojis with error: {}", e))
          )?
      ),
      
      "error" => TdType::Error(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Error deserialize to TdType::Error with error: {}", e))
          )?
      ),
      
      "file" => TdType::File(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("File deserialize to TdType::File with error: {}", e))
          )?
      ),
      
      "filePart" => TdType::FilePart(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("FilePart deserialize to TdType::FilePart with error: {}", e))
          )?
      ),
      
      "formattedText" => TdType::FormattedText(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("FormattedText deserialize to TdType::FormattedText with error: {}", e))
          )?
      ),
      
      "foundMessages" => TdType::FoundMessages(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("FoundMessages deserialize to TdType::FoundMessages with error: {}", e))
          )?
      ),
      
      "gameHighScores" => TdType::GameHighScores(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("GameHighScores deserialize to TdType::GameHighScores with error: {}", e))
          )?
      ),
      
      "hashtags" => TdType::Hashtags(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Hashtags deserialize to TdType::Hashtags with error: {}", e))
          )?
      ),
      
      "httpUrl" => TdType::HttpUrl(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("HttpUrl deserialize to TdType::HttpUrl with error: {}", e))
          )?
      ),
      
      "importedContacts" => TdType::ImportedContacts(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ImportedContacts deserialize to TdType::ImportedContacts with error: {}", e))
          )?
      ),
      
      "inlineQueryResults" => TdType::InlineQueryResults(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("InlineQueryResults deserialize to TdType::InlineQueryResults with error: {}", e))
          )?
      ),
      
      "languagePackInfo" => TdType::LanguagePackInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LanguagePackInfo deserialize to TdType::LanguagePackInfo with error: {}", e))
          )?
      ),
      
      "languagePackStrings" => TdType::LanguagePackStrings(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LanguagePackStrings deserialize to TdType::LanguagePackStrings with error: {}", e))
          )?
      ),
      
      "localizationTargetInfo" => TdType::LocalizationTargetInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LocalizationTargetInfo deserialize to TdType::LocalizationTargetInfo with error: {}", e))
          )?
      ),
      
      "logTags" => TdType::LogTags(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LogTags deserialize to TdType::LogTags with error: {}", e))
          )?
      ),
      
      "logVerbosityLevel" => TdType::LogVerbosityLevel(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("LogVerbosityLevel deserialize to TdType::LogVerbosityLevel with error: {}", e))
          )?
      ),
      
      "message" => TdType::Message(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Message deserialize to TdType::Message with error: {}", e))
          )?
      ),
      
      "messageLink" => TdType::MessageLink(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("MessageLink deserialize to TdType::MessageLink with error: {}", e))
          )?
      ),
      
      "messageLinkInfo" => TdType::MessageLinkInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("MessageLinkInfo deserialize to TdType::MessageLinkInfo with error: {}", e))
          )?
      ),
      
      "messageSenders" => TdType::MessageSenders(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("MessageSenders deserialize to TdType::MessageSenders with error: {}", e))
          )?
      ),
      
      "messageStatistics" => TdType::MessageStatistics(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("MessageStatistics deserialize to TdType::MessageStatistics with error: {}", e))
          )?
      ),
      
      "messageThreadInfo" => TdType::MessageThreadInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("MessageThreadInfo deserialize to TdType::MessageThreadInfo with error: {}", e))
          )?
      ),
      
      "messages" => TdType::Messages(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Messages deserialize to TdType::Messages with error: {}", e))
          )?
      ),
      
      "networkStatistics" => TdType::NetworkStatistics(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("NetworkStatistics deserialize to TdType::NetworkStatistics with error: {}", e))
          )?
      ),
      
      "ok" => TdType::Ok(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Ok deserialize to TdType::Ok with error: {}", e))
          )?
      ),
      
      "orderInfo" => TdType::OrderInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("OrderInfo deserialize to TdType::OrderInfo with error: {}", e))
          )?
      ),
      
      "passportAuthorizationForm" => TdType::PassportAuthorizationForm(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportAuthorizationForm deserialize to TdType::PassportAuthorizationForm with error: {}", e))
          )?
      ),
      
      "passportElements" => TdType::PassportElements(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElements deserialize to TdType::PassportElements with error: {}", e))
          )?
      ),
      
      "passportElementsWithErrors" => TdType::PassportElementsWithErrors(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PassportElementsWithErrors deserialize to TdType::PassportElementsWithErrors with error: {}", e))
          )?
      ),
      
      "passwordState" => TdType::PasswordState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PasswordState deserialize to TdType::PasswordState with error: {}", e))
          )?
      ),
      
      "paymentForm" => TdType::PaymentForm(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PaymentForm deserialize to TdType::PaymentForm with error: {}", e))
          )?
      ),
      
      "paymentReceipt" => TdType::PaymentReceipt(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PaymentReceipt deserialize to TdType::PaymentReceipt with error: {}", e))
          )?
      ),
      
      "paymentResult" => TdType::PaymentResult(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PaymentResult deserialize to TdType::PaymentResult with error: {}", e))
          )?
      ),
      
      "phoneNumberInfo" => TdType::PhoneNumberInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PhoneNumberInfo deserialize to TdType::PhoneNumberInfo with error: {}", e))
          )?
      ),
      
      "proxies" => TdType::Proxies(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Proxies deserialize to TdType::Proxies with error: {}", e))
          )?
      ),
      
      "proxy" => TdType::Proxy(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Proxy deserialize to TdType::Proxy with error: {}", e))
          )?
      ),
      
      "pushReceiverId" => TdType::PushReceiverId(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("PushReceiverId deserialize to TdType::PushReceiverId with error: {}", e))
          )?
      ),
      
      "recommendedChatFilters" => TdType::RecommendedChatFilters(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("RecommendedChatFilters deserialize to TdType::RecommendedChatFilters with error: {}", e))
          )?
      ),
      
      "recoveryEmailAddress" => TdType::RecoveryEmailAddress(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("RecoveryEmailAddress deserialize to TdType::RecoveryEmailAddress with error: {}", e))
          )?
      ),
      
      "scopeNotificationSettings" => TdType::ScopeNotificationSettings(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ScopeNotificationSettings deserialize to TdType::ScopeNotificationSettings with error: {}", e))
          )?
      ),
      
      "seconds" => TdType::Seconds(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Seconds deserialize to TdType::Seconds with error: {}", e))
          )?
      ),
      
      "secretChat" => TdType::SecretChat(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("SecretChat deserialize to TdType::SecretChat with error: {}", e))
          )?
      ),
      
      "session" => TdType::Session(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Session deserialize to TdType::Session with error: {}", e))
          )?
      ),
      
      "sessions" => TdType::Sessions(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Sessions deserialize to TdType::Sessions with error: {}", e))
          )?
      ),
      
      "stickerSet" => TdType::StickerSet(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("StickerSet deserialize to TdType::StickerSet with error: {}", e))
          )?
      ),
      
      "stickerSets" => TdType::StickerSets(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("StickerSets deserialize to TdType::StickerSets with error: {}", e))
          )?
      ),
      
      "stickers" => TdType::Stickers(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Stickers deserialize to TdType::Stickers with error: {}", e))
          )?
      ),
      
      "storageStatistics" => TdType::StorageStatistics(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("StorageStatistics deserialize to TdType::StorageStatistics with error: {}", e))
          )?
      ),
      
      "storageStatisticsFast" => TdType::StorageStatisticsFast(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("StorageStatisticsFast deserialize to TdType::StorageStatisticsFast with error: {}", e))
          )?
      ),
      
      "supergroup" => TdType::Supergroup(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Supergroup deserialize to TdType::Supergroup with error: {}", e))
          )?
      ),
      
      "supergroupFullInfo" => TdType::SupergroupFullInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("SupergroupFullInfo deserialize to TdType::SupergroupFullInfo with error: {}", e))
          )?
      ),
      
      "tMeUrls" => TdType::TMeUrls(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TMeUrls deserialize to TdType::TMeUrls with error: {}", e))
          )?
      ),
      
      "temporaryPasswordState" => TdType::TemporaryPasswordState(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TemporaryPasswordState deserialize to TdType::TemporaryPasswordState with error: {}", e))
          )?
      ),
      
      "testBytes" => TdType::TestBytes(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TestBytes deserialize to TdType::TestBytes with error: {}", e))
          )?
      ),
      
      "testInt" => TdType::TestInt(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TestInt deserialize to TdType::TestInt with error: {}", e))
          )?
      ),
      
      "testString" => TdType::TestString(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TestString deserialize to TdType::TestString with error: {}", e))
          )?
      ),
      
      "testVectorInt" => TdType::TestVectorInt(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TestVectorInt deserialize to TdType::TestVectorInt with error: {}", e))
          )?
      ),
      
      "testVectorIntObject" => TdType::TestVectorIntObject(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TestVectorIntObject deserialize to TdType::TestVectorIntObject with error: {}", e))
          )?
      ),
      
      "testVectorString" => TdType::TestVectorString(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TestVectorString deserialize to TdType::TestVectorString with error: {}", e))
          )?
      ),
      
      "testVectorStringObject" => TdType::TestVectorStringObject(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TestVectorStringObject deserialize to TdType::TestVectorStringObject with error: {}", e))
          )?
      ),
      
      "text" => TdType::Text(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Text deserialize to TdType::Text with error: {}", e))
          )?
      ),
      
      "textEntities" => TdType::TextEntities(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("TextEntities deserialize to TdType::TextEntities with error: {}", e))
          )?
      ),
      
      "updates" => TdType::Updates(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Updates deserialize to TdType::Updates with error: {}", e))
          )?
      ),
      
      "user" => TdType::User(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("User deserialize to TdType::User with error: {}", e))
          )?
      ),
      
      "userFullInfo" => TdType::UserFullInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UserFullInfo deserialize to TdType::UserFullInfo with error: {}", e))
          )?
      ),
      
      "userPrivacySettingRules" => TdType::UserPrivacySettingRules(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("UserPrivacySettingRules deserialize to TdType::UserPrivacySettingRules with error: {}", e))
          )?
      ),
      
      "users" => TdType::Users(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("Users deserialize to TdType::Users with error: {}", e))
          )?
      ),
      
      "validatedOrderInfo" => TdType::ValidatedOrderInfo(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("ValidatedOrderInfo deserialize to TdType::ValidatedOrderInfo with error: {}", e))
          )?
      ),
      
      "webPage" => TdType::WebPage(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("WebPage deserialize to TdType::WebPage with error: {}", e))
          )?
      ),
      
      "webPageInstantView" => TdType::WebPageInstantView(
          serde_json::from_value(
              rtd_trait_value.clone()
          ).map_err(|e|
              D::Error::custom(format!("WebPageInstantView deserialize to TdType::WebPageInstantView with error: {}", e))
          )?
      ),
      
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
