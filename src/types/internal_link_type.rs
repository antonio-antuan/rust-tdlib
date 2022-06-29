use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes an internal https://t.me or tg: link, which must be processed by the application in a special way
pub trait TDInternalLinkType: Debug + RObject {}

/// Describes an internal https://t.me or tg: link, which must be processed by the application in a special way
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InternalLinkType {
    #[doc(hidden)]
    _Default,
    /// Returns information about the type of an internal link. Returns a 404 error if the link is not internal. Can be called before authorization
    #[serde(rename(deserialize = "getInternalLinkType"))]
    GetInternalLinkType(GetInternalLinkType),
    /// The link is a link to the active sessions section of the application. Use getActiveSessions to handle the link
    #[serde(rename(deserialize = "internalLinkTypeActiveSessions"))]
    ActiveSessions(InternalLinkTypeActiveSessions),
    /// The link is a link to an attachment menu bot to be opened in the specified or a chosen chat. Process given target_chat to open the chat. Then call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then use getAttachmentMenuBot to receive information about the bot. If the bot isn't added to attachment menu, then user needs to confirm adding the bot to attachment menu. If user confirms adding, then use toggleBotIsAddedToAttachmentMenu to add it. If the attachment menu bot can't be used in the opened chat, show an error to the user. If the bot is added to attachment menu and can be used in the chat, then use openWebApp with the given URL
    #[serde(rename(deserialize = "internalLinkTypeAttachmentMenuBot"))]
    AttachmentMenuBot(InternalLinkTypeAttachmentMenuBot),
    /// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
    #[serde(rename(deserialize = "internalLinkTypeAuthenticationCode"))]
    AuthenticationCode(InternalLinkTypeAuthenticationCode),
    /// The link is a link to a background. Call searchBackground with the given background name to process the link
    #[serde(rename(deserialize = "internalLinkTypeBackground"))]
    Background(InternalLinkTypeBackground),
    /// The link is a link to a Telegram bot, which is supposed to be added to a channel chat as an administrator. Call searchPublicChat with the given bot username and check that the user is a bot, ask the current user to select a channel chat to add the bot to as an administrator. Then call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights and combine received rights with the requested administrator rights. Then show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed rights
    #[serde(rename(deserialize = "internalLinkTypeBotAddToChannel"))]
    BotAddToChannel(InternalLinkTypeBotAddToChannel),
    /// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot, and then call sendBotStartMessage with the given start parameter after the button is pressed
    #[serde(rename(deserialize = "internalLinkTypeBotStart"))]
    BotStart(InternalLinkTypeBotStart),
    /// The link is a link to a Telegram bot, which is supposed to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups, ask the current user to select a basic group or a supergroup chat to add the bot to, taking into account that bots can be added to a public supergroup only by administrators of the supergroup. If administrator rights are provided by the link, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights, combine received rights with the requested administrator rights, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed administrator rights. Before call to setChatMemberStatus it may be required to upgrade the chosen basic group chat to a supergroup chat. Then if start_parameter isn't empty, call sendBotStartMessage with the given start parameter and the chosen chat, otherwise just send /start message with bot's username added to the chat.
    #[serde(rename(deserialize = "internalLinkTypeBotStartInGroup"))]
    BotStartInGroup(InternalLinkTypeBotStartInGroup),
    /// The link is a link to the change phone number section of the app
    #[serde(rename(deserialize = "internalLinkTypeChangePhoneNumber"))]
    ChangePhoneNumber(InternalLinkTypeChangePhoneNumber),
    /// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link
    #[serde(rename(deserialize = "internalLinkTypeChatInvite"))]
    ChatInvite(InternalLinkTypeChatInvite),
    /// The link is a link to the filter settings section of the app
    #[serde(rename(deserialize = "internalLinkTypeFilterSettings"))]
    FilterSettings(InternalLinkTypeFilterSettings),
    /// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot, ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
    #[serde(rename(deserialize = "internalLinkTypeGame"))]
    Game(InternalLinkTypeGame),
    /// The link is a link to an invoice. Call getPaymentForm with the given invoice name to process the link
    #[serde(rename(deserialize = "internalLinkTypeInvoice"))]
    Invoice(InternalLinkTypeInvoice),
    /// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link
    #[serde(rename(deserialize = "internalLinkTypeLanguagePack"))]
    LanguagePack(InternalLinkTypeLanguagePack),
    /// The link is a link to the language settings section of the app
    #[serde(rename(deserialize = "internalLinkTypeLanguageSettings"))]
    LanguageSettings(InternalLinkTypeLanguageSettings),
    /// The link is a link to a Telegram message. Call getMessageLinkInfo with the given URL to process the link
    #[serde(rename(deserialize = "internalLinkTypeMessage"))]
    Message(InternalLinkTypeMessage),
    /// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
    #[serde(rename(deserialize = "internalLinkTypeMessageDraft"))]
    MessageDraft(InternalLinkTypeMessageDraft),
    /// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the application, otherwise ignore it
    #[serde(rename(deserialize = "internalLinkTypePassportDataRequest"))]
    PassportDataRequest(InternalLinkTypePassportDataRequest),
    /// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberConfirmationCode with the given hash and phone number to process the link
    #[serde(rename(deserialize = "internalLinkTypePhoneNumberConfirmation"))]
    PhoneNumberConfirmation(InternalLinkTypePhoneNumberConfirmation),
    /// The link is a link to the Premium features screen of the applcation from which the user can subscribe to Telegram Premium. Call getPremiumFeatures with the given referrer to process the link
    #[serde(rename(deserialize = "internalLinkTypePremiumFeatures"))]
    PremiumFeatures(InternalLinkTypePremiumFeatures),
    /// The link is a link to the privacy and security settings section of the app
    #[serde(rename(deserialize = "internalLinkTypePrivacyAndSecuritySettings"))]
    PrivacyAndSecuritySettings(InternalLinkTypePrivacyAndSecuritySettings),
    /// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
    #[serde(rename(deserialize = "internalLinkTypeProxy"))]
    Proxy(InternalLinkTypeProxy),
    /// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link
    #[serde(rename(deserialize = "internalLinkTypePublicChat"))]
    PublicChat(InternalLinkTypePublicChat),
    /// The link can be used to login the current user on another device, but it must be scanned from QR-code using in-app camera. An alert similar to "This code can be used to allow someone to log in to your Telegram account. To confirm Telegram login, please go to Settings > Devices > Scan QR and scan the code" needs to be shown
    #[serde(rename(deserialize = "internalLinkTypeQrCodeAuthentication"))]
    QrCodeAuthentication(InternalLinkTypeQrCodeAuthentication),
    /// The link is a link to application settings
    #[serde(rename(deserialize = "internalLinkTypeSettings"))]
    Settings(InternalLinkTypeSettings),
    /// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set
    #[serde(rename(deserialize = "internalLinkTypeStickerSet"))]
    StickerSet(InternalLinkTypeStickerSet),
    /// The link is a link to a theme. TDLib has no theme support yet
    #[serde(rename(deserialize = "internalLinkTypeTheme"))]
    Theme(InternalLinkTypeTheme),
    /// The link is a link to the theme settings section of the app
    #[serde(rename(deserialize = "internalLinkTypeThemeSettings"))]
    ThemeSettings(InternalLinkTypeThemeSettings),
    /// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
    #[serde(rename(deserialize = "internalLinkTypeUnknownDeepLink"))]
    UnknownDeepLink(InternalLinkTypeUnknownDeepLink),
    /// The link is a link to an unsupported proxy. An alert can be shown to the user
    #[serde(rename(deserialize = "internalLinkTypeUnsupportedProxy"))]
    UnsupportedProxy(InternalLinkTypeUnsupportedProxy),
    /// The link is a link to a user by its phone number. Call searchUserByPhoneNumber with the given phone number to process the link
    #[serde(rename(deserialize = "internalLinkTypeUserPhoneNumber"))]
    UserPhoneNumber(InternalLinkTypeUserPhoneNumber),
    /// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinGroupCall with the given invite hash to process the link
    #[serde(rename(deserialize = "internalLinkTypeVideoChat"))]
    VideoChat(InternalLinkTypeVideoChat),
}

impl Default for InternalLinkType {
    fn default() -> Self {
        InternalLinkType::_Default
    }
}

impl RObject for InternalLinkType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InternalLinkType::GetInternalLinkType(t) => t.extra(),
            InternalLinkType::ActiveSessions(t) => t.extra(),
            InternalLinkType::AttachmentMenuBot(t) => t.extra(),
            InternalLinkType::AuthenticationCode(t) => t.extra(),
            InternalLinkType::Background(t) => t.extra(),
            InternalLinkType::BotAddToChannel(t) => t.extra(),
            InternalLinkType::BotStart(t) => t.extra(),
            InternalLinkType::BotStartInGroup(t) => t.extra(),
            InternalLinkType::ChangePhoneNumber(t) => t.extra(),
            InternalLinkType::ChatInvite(t) => t.extra(),
            InternalLinkType::FilterSettings(t) => t.extra(),
            InternalLinkType::Game(t) => t.extra(),
            InternalLinkType::Invoice(t) => t.extra(),
            InternalLinkType::LanguagePack(t) => t.extra(),
            InternalLinkType::LanguageSettings(t) => t.extra(),
            InternalLinkType::Message(t) => t.extra(),
            InternalLinkType::MessageDraft(t) => t.extra(),
            InternalLinkType::PassportDataRequest(t) => t.extra(),
            InternalLinkType::PhoneNumberConfirmation(t) => t.extra(),
            InternalLinkType::PremiumFeatures(t) => t.extra(),
            InternalLinkType::PrivacyAndSecuritySettings(t) => t.extra(),
            InternalLinkType::Proxy(t) => t.extra(),
            InternalLinkType::PublicChat(t) => t.extra(),
            InternalLinkType::QrCodeAuthentication(t) => t.extra(),
            InternalLinkType::Settings(t) => t.extra(),
            InternalLinkType::StickerSet(t) => t.extra(),
            InternalLinkType::Theme(t) => t.extra(),
            InternalLinkType::ThemeSettings(t) => t.extra(),
            InternalLinkType::UnknownDeepLink(t) => t.extra(),
            InternalLinkType::UnsupportedProxy(t) => t.extra(),
            InternalLinkType::UserPhoneNumber(t) => t.extra(),
            InternalLinkType::VideoChat(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InternalLinkType::GetInternalLinkType(t) => t.client_id(),
            InternalLinkType::ActiveSessions(t) => t.client_id(),
            InternalLinkType::AttachmentMenuBot(t) => t.client_id(),
            InternalLinkType::AuthenticationCode(t) => t.client_id(),
            InternalLinkType::Background(t) => t.client_id(),
            InternalLinkType::BotAddToChannel(t) => t.client_id(),
            InternalLinkType::BotStart(t) => t.client_id(),
            InternalLinkType::BotStartInGroup(t) => t.client_id(),
            InternalLinkType::ChangePhoneNumber(t) => t.client_id(),
            InternalLinkType::ChatInvite(t) => t.client_id(),
            InternalLinkType::FilterSettings(t) => t.client_id(),
            InternalLinkType::Game(t) => t.client_id(),
            InternalLinkType::Invoice(t) => t.client_id(),
            InternalLinkType::LanguagePack(t) => t.client_id(),
            InternalLinkType::LanguageSettings(t) => t.client_id(),
            InternalLinkType::Message(t) => t.client_id(),
            InternalLinkType::MessageDraft(t) => t.client_id(),
            InternalLinkType::PassportDataRequest(t) => t.client_id(),
            InternalLinkType::PhoneNumberConfirmation(t) => t.client_id(),
            InternalLinkType::PremiumFeatures(t) => t.client_id(),
            InternalLinkType::PrivacyAndSecuritySettings(t) => t.client_id(),
            InternalLinkType::Proxy(t) => t.client_id(),
            InternalLinkType::PublicChat(t) => t.client_id(),
            InternalLinkType::QrCodeAuthentication(t) => t.client_id(),
            InternalLinkType::Settings(t) => t.client_id(),
            InternalLinkType::StickerSet(t) => t.client_id(),
            InternalLinkType::Theme(t) => t.client_id(),
            InternalLinkType::ThemeSettings(t) => t.client_id(),
            InternalLinkType::UnknownDeepLink(t) => t.client_id(),
            InternalLinkType::UnsupportedProxy(t) => t.client_id(),
            InternalLinkType::UserPhoneNumber(t) => t.client_id(),
            InternalLinkType::VideoChat(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InternalLinkType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InternalLinkType::_Default)
    }
}

impl AsRef<InternalLinkType> for InternalLinkType {
    fn as_ref(&self) -> &InternalLinkType {
        self
    }
}

/// The link is a link to the active sessions section of the application. Use getActiveSessions to handle the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeActiveSessions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeActiveSessions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeActiveSessions {}

impl InternalLinkTypeActiveSessions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeActiveSessionsBuilder {
        let mut inner = InternalLinkTypeActiveSessions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeActiveSessionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeActiveSessionsBuilder {
    inner: InternalLinkTypeActiveSessions,
}

impl RTDInternalLinkTypeActiveSessionsBuilder {
    pub fn build(&self) -> InternalLinkTypeActiveSessions {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeActiveSessions> for InternalLinkTypeActiveSessions {
    fn as_ref(&self) -> &InternalLinkTypeActiveSessions {
        self
    }
}

impl AsRef<InternalLinkTypeActiveSessions> for RTDInternalLinkTypeActiveSessionsBuilder {
    fn as_ref(&self) -> &InternalLinkTypeActiveSessions {
        &self.inner
    }
}

/// The link is a link to an attachment menu bot to be opened in the specified or a chosen chat. Process given target_chat to open the chat. Then call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then use getAttachmentMenuBot to receive information about the bot. If the bot isn't added to attachment menu, then user needs to confirm adding the bot to attachment menu. If user confirms adding, then use toggleBotIsAddedToAttachmentMenu to add it. If the attachment menu bot can't be used in the opened chat, show an error to the user. If the bot is added to attachment menu and can be used in the chat, then use openWebApp with the given URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeAttachmentMenuBot {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Target chat to be opened

    #[serde(skip_serializing_if = "TargetChat::_is_default")]
    target_chat: TargetChat,
    /// Username of the bot

    #[serde(default)]
    bot_username: String,
    /// URL to be passed to openWebApp

    #[serde(default)]
    url: String,
}

impl RObject for InternalLinkTypeAttachmentMenuBot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeAttachmentMenuBot {}

impl InternalLinkTypeAttachmentMenuBot {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeAttachmentMenuBotBuilder {
        let mut inner = InternalLinkTypeAttachmentMenuBot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeAttachmentMenuBotBuilder { inner }
    }

    pub fn target_chat(&self) -> &TargetChat {
        &self.target_chat
    }

    pub fn bot_username(&self) -> &String {
        &self.bot_username
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeAttachmentMenuBotBuilder {
    inner: InternalLinkTypeAttachmentMenuBot,
}

impl RTDInternalLinkTypeAttachmentMenuBotBuilder {
    pub fn build(&self) -> InternalLinkTypeAttachmentMenuBot {
        self.inner.clone()
    }

    pub fn target_chat<T: AsRef<TargetChat>>(&mut self, target_chat: T) -> &mut Self {
        self.inner.target_chat = target_chat.as_ref().clone();
        self
    }

    pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
        self.inner.bot_username = bot_username.as_ref().to_string();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeAttachmentMenuBot> for InternalLinkTypeAttachmentMenuBot {
    fn as_ref(&self) -> &InternalLinkTypeAttachmentMenuBot {
        self
    }
}

impl AsRef<InternalLinkTypeAttachmentMenuBot> for RTDInternalLinkTypeAttachmentMenuBotBuilder {
    fn as_ref(&self) -> &InternalLinkTypeAttachmentMenuBot {
        &self.inner
    }
}

/// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeAuthenticationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The authentication code

    #[serde(default)]
    code: String,
}

impl RObject for InternalLinkTypeAuthenticationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeAuthenticationCode {}

impl InternalLinkTypeAuthenticationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeAuthenticationCodeBuilder {
        let mut inner = InternalLinkTypeAuthenticationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeAuthenticationCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeAuthenticationCodeBuilder {
    inner: InternalLinkTypeAuthenticationCode,
}

impl RTDInternalLinkTypeAuthenticationCodeBuilder {
    pub fn build(&self) -> InternalLinkTypeAuthenticationCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeAuthenticationCode> for InternalLinkTypeAuthenticationCode {
    fn as_ref(&self) -> &InternalLinkTypeAuthenticationCode {
        self
    }
}

impl AsRef<InternalLinkTypeAuthenticationCode> for RTDInternalLinkTypeAuthenticationCodeBuilder {
    fn as_ref(&self) -> &InternalLinkTypeAuthenticationCode {
        &self.inner
    }
}

/// The link is a link to a background. Call searchBackground with the given background name to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeBackground {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the background

    #[serde(default)]
    background_name: String,
}

impl RObject for InternalLinkTypeBackground {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeBackground {}

impl InternalLinkTypeBackground {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeBackgroundBuilder {
        let mut inner = InternalLinkTypeBackground::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeBackgroundBuilder { inner }
    }

    pub fn background_name(&self) -> &String {
        &self.background_name
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBackgroundBuilder {
    inner: InternalLinkTypeBackground,
}

impl RTDInternalLinkTypeBackgroundBuilder {
    pub fn build(&self) -> InternalLinkTypeBackground {
        self.inner.clone()
    }

    pub fn background_name<T: AsRef<str>>(&mut self, background_name: T) -> &mut Self {
        self.inner.background_name = background_name.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeBackground> for InternalLinkTypeBackground {
    fn as_ref(&self) -> &InternalLinkTypeBackground {
        self
    }
}

impl AsRef<InternalLinkTypeBackground> for RTDInternalLinkTypeBackgroundBuilder {
    fn as_ref(&self) -> &InternalLinkTypeBackground {
        &self.inner
    }
}

/// The link is a link to a Telegram bot, which is supposed to be added to a channel chat as an administrator. Call searchPublicChat with the given bot username and check that the user is a bot, ask the current user to select a channel chat to add the bot to as an administrator. Then call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights and combine received rights with the requested administrator rights. Then show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeBotAddToChannel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username of the bot

    #[serde(default)]
    bot_username: String,
    /// Expected administrator rights for the bot
    administrator_rights: ChatAdministratorRights,
}

impl RObject for InternalLinkTypeBotAddToChannel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeBotAddToChannel {}

impl InternalLinkTypeBotAddToChannel {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeBotAddToChannelBuilder {
        let mut inner = InternalLinkTypeBotAddToChannel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeBotAddToChannelBuilder { inner }
    }

    pub fn bot_username(&self) -> &String {
        &self.bot_username
    }

    pub fn administrator_rights(&self) -> &ChatAdministratorRights {
        &self.administrator_rights
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBotAddToChannelBuilder {
    inner: InternalLinkTypeBotAddToChannel,
}

impl RTDInternalLinkTypeBotAddToChannelBuilder {
    pub fn build(&self) -> InternalLinkTypeBotAddToChannel {
        self.inner.clone()
    }

    pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
        self.inner.bot_username = bot_username.as_ref().to_string();
        self
    }

    pub fn administrator_rights<T: AsRef<ChatAdministratorRights>>(
        &mut self,
        administrator_rights: T,
    ) -> &mut Self {
        self.inner.administrator_rights = administrator_rights.as_ref().clone();
        self
    }
}

impl AsRef<InternalLinkTypeBotAddToChannel> for InternalLinkTypeBotAddToChannel {
    fn as_ref(&self) -> &InternalLinkTypeBotAddToChannel {
        self
    }
}

impl AsRef<InternalLinkTypeBotAddToChannel> for RTDInternalLinkTypeBotAddToChannelBuilder {
    fn as_ref(&self) -> &InternalLinkTypeBotAddToChannel {
        &self.inner
    }
}

/// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot, and then call sendBotStartMessage with the given start parameter after the button is pressed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeBotStart {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username of the bot

    #[serde(default)]
    bot_username: String,
    /// The parameter to be passed to sendBotStartMessage

    #[serde(default)]
    start_parameter: String,
}

impl RObject for InternalLinkTypeBotStart {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeBotStart {}

impl InternalLinkTypeBotStart {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeBotStartBuilder {
        let mut inner = InternalLinkTypeBotStart::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeBotStartBuilder { inner }
    }

    pub fn bot_username(&self) -> &String {
        &self.bot_username
    }

    pub fn start_parameter(&self) -> &String {
        &self.start_parameter
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBotStartBuilder {
    inner: InternalLinkTypeBotStart,
}

impl RTDInternalLinkTypeBotStartBuilder {
    pub fn build(&self) -> InternalLinkTypeBotStart {
        self.inner.clone()
    }

    pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
        self.inner.bot_username = bot_username.as_ref().to_string();
        self
    }

    pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
        self.inner.start_parameter = start_parameter.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeBotStart> for InternalLinkTypeBotStart {
    fn as_ref(&self) -> &InternalLinkTypeBotStart {
        self
    }
}

impl AsRef<InternalLinkTypeBotStart> for RTDInternalLinkTypeBotStartBuilder {
    fn as_ref(&self) -> &InternalLinkTypeBotStart {
        &self.inner
    }
}

/// The link is a link to a Telegram bot, which is supposed to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups, ask the current user to select a basic group or a supergroup chat to add the bot to, taking into account that bots can be added to a public supergroup only by administrators of the supergroup. If administrator rights are provided by the link, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights, combine received rights with the requested administrator rights, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed administrator rights. Before call to setChatMemberStatus it may be required to upgrade the chosen basic group chat to a supergroup chat. Then if start_parameter isn't empty, call sendBotStartMessage with the given start parameter and the chosen chat, otherwise just send /start message with bot's username added to the chat.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeBotStartInGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username of the bot

    #[serde(default)]
    bot_username: String,
    /// The parameter to be passed to sendBotStartMessage

    #[serde(default)]
    start_parameter: String,
    /// Expected administrator rights for the bot; may be null
    administrator_rights: Option<ChatAdministratorRights>,
}

impl RObject for InternalLinkTypeBotStartInGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeBotStartInGroup {}

impl InternalLinkTypeBotStartInGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeBotStartInGroupBuilder {
        let mut inner = InternalLinkTypeBotStartInGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeBotStartInGroupBuilder { inner }
    }

    pub fn bot_username(&self) -> &String {
        &self.bot_username
    }

    pub fn start_parameter(&self) -> &String {
        &self.start_parameter
    }

    pub fn administrator_rights(&self) -> &Option<ChatAdministratorRights> {
        &self.administrator_rights
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeBotStartInGroupBuilder {
    inner: InternalLinkTypeBotStartInGroup,
}

impl RTDInternalLinkTypeBotStartInGroupBuilder {
    pub fn build(&self) -> InternalLinkTypeBotStartInGroup {
        self.inner.clone()
    }

    pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
        self.inner.bot_username = bot_username.as_ref().to_string();
        self
    }

    pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
        self.inner.start_parameter = start_parameter.as_ref().to_string();
        self
    }

    pub fn administrator_rights<T: AsRef<ChatAdministratorRights>>(
        &mut self,
        administrator_rights: T,
    ) -> &mut Self {
        self.inner.administrator_rights = Some(administrator_rights.as_ref().clone());
        self
    }
}

impl AsRef<InternalLinkTypeBotStartInGroup> for InternalLinkTypeBotStartInGroup {
    fn as_ref(&self) -> &InternalLinkTypeBotStartInGroup {
        self
    }
}

impl AsRef<InternalLinkTypeBotStartInGroup> for RTDInternalLinkTypeBotStartInGroupBuilder {
    fn as_ref(&self) -> &InternalLinkTypeBotStartInGroup {
        &self.inner
    }
}

/// The link is a link to the change phone number section of the app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChangePhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeChangePhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeChangePhoneNumber {}

impl InternalLinkTypeChangePhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeChangePhoneNumberBuilder {
        let mut inner = InternalLinkTypeChangePhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeChangePhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeChangePhoneNumberBuilder {
    inner: InternalLinkTypeChangePhoneNumber,
}

impl RTDInternalLinkTypeChangePhoneNumberBuilder {
    pub fn build(&self) -> InternalLinkTypeChangePhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeChangePhoneNumber> for InternalLinkTypeChangePhoneNumber {
    fn as_ref(&self) -> &InternalLinkTypeChangePhoneNumber {
        self
    }
}

impl AsRef<InternalLinkTypeChangePhoneNumber> for RTDInternalLinkTypeChangePhoneNumberBuilder {
    fn as_ref(&self) -> &InternalLinkTypeChangePhoneNumber {
        &self.inner
    }
}

/// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChatInvite {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Internal representation of the invite link

    #[serde(default)]
    invite_link: String,
}

impl RObject for InternalLinkTypeChatInvite {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeChatInvite {}

impl InternalLinkTypeChatInvite {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeChatInviteBuilder {
        let mut inner = InternalLinkTypeChatInvite::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeChatInviteBuilder { inner }
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeChatInviteBuilder {
    inner: InternalLinkTypeChatInvite,
}

impl RTDInternalLinkTypeChatInviteBuilder {
    pub fn build(&self) -> InternalLinkTypeChatInvite {
        self.inner.clone()
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeChatInvite> for InternalLinkTypeChatInvite {
    fn as_ref(&self) -> &InternalLinkTypeChatInvite {
        self
    }
}

impl AsRef<InternalLinkTypeChatInvite> for RTDInternalLinkTypeChatInviteBuilder {
    fn as_ref(&self) -> &InternalLinkTypeChatInvite {
        &self.inner
    }
}

/// The link is a link to the filter settings section of the app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeFilterSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeFilterSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeFilterSettings {}

impl InternalLinkTypeFilterSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeFilterSettingsBuilder {
        let mut inner = InternalLinkTypeFilterSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeFilterSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeFilterSettingsBuilder {
    inner: InternalLinkTypeFilterSettings,
}

impl RTDInternalLinkTypeFilterSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeFilterSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeFilterSettings> for InternalLinkTypeFilterSettings {
    fn as_ref(&self) -> &InternalLinkTypeFilterSettings {
        self
    }
}

impl AsRef<InternalLinkTypeFilterSettings> for RTDInternalLinkTypeFilterSettingsBuilder {
    fn as_ref(&self) -> &InternalLinkTypeFilterSettings {
        &self.inner
    }
}

/// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot, ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeGame {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username of the bot that owns the game

    #[serde(default)]
    bot_username: String,
    /// Short name of the game

    #[serde(default)]
    game_short_name: String,
}

impl RObject for InternalLinkTypeGame {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeGame {}

impl InternalLinkTypeGame {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeGameBuilder {
        let mut inner = InternalLinkTypeGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeGameBuilder { inner }
    }

    pub fn bot_username(&self) -> &String {
        &self.bot_username
    }

    pub fn game_short_name(&self) -> &String {
        &self.game_short_name
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeGameBuilder {
    inner: InternalLinkTypeGame,
}

impl RTDInternalLinkTypeGameBuilder {
    pub fn build(&self) -> InternalLinkTypeGame {
        self.inner.clone()
    }

    pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
        self.inner.bot_username = bot_username.as_ref().to_string();
        self
    }

    pub fn game_short_name<T: AsRef<str>>(&mut self, game_short_name: T) -> &mut Self {
        self.inner.game_short_name = game_short_name.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeGame> for InternalLinkTypeGame {
    fn as_ref(&self) -> &InternalLinkTypeGame {
        self
    }
}

impl AsRef<InternalLinkTypeGame> for RTDInternalLinkTypeGameBuilder {
    fn as_ref(&self) -> &InternalLinkTypeGame {
        &self.inner
    }
}

/// The link is a link to an invoice. Call getPaymentForm with the given invoice name to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeInvoice {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the invoice

    #[serde(default)]
    invoice_name: String,
}

impl RObject for InternalLinkTypeInvoice {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeInvoice {}

impl InternalLinkTypeInvoice {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeInvoiceBuilder {
        let mut inner = InternalLinkTypeInvoice::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeInvoiceBuilder { inner }
    }

    pub fn invoice_name(&self) -> &String {
        &self.invoice_name
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeInvoiceBuilder {
    inner: InternalLinkTypeInvoice,
}

impl RTDInternalLinkTypeInvoiceBuilder {
    pub fn build(&self) -> InternalLinkTypeInvoice {
        self.inner.clone()
    }

    pub fn invoice_name<T: AsRef<str>>(&mut self, invoice_name: T) -> &mut Self {
        self.inner.invoice_name = invoice_name.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeInvoice> for InternalLinkTypeInvoice {
    fn as_ref(&self) -> &InternalLinkTypeInvoice {
        self
    }
}

impl AsRef<InternalLinkTypeInvoice> for RTDInternalLinkTypeInvoiceBuilder {
    fn as_ref(&self) -> &InternalLinkTypeInvoice {
        &self.inner
    }
}

/// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeLanguagePack {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Language pack identifier

    #[serde(default)]
    language_pack_id: String,
}

impl RObject for InternalLinkTypeLanguagePack {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeLanguagePack {}

impl InternalLinkTypeLanguagePack {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeLanguagePackBuilder {
        let mut inner = InternalLinkTypeLanguagePack::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeLanguagePackBuilder { inner }
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeLanguagePackBuilder {
    inner: InternalLinkTypeLanguagePack,
}

impl RTDInternalLinkTypeLanguagePackBuilder {
    pub fn build(&self) -> InternalLinkTypeLanguagePack {
        self.inner.clone()
    }

    pub fn language_pack_id<T: AsRef<str>>(&mut self, language_pack_id: T) -> &mut Self {
        self.inner.language_pack_id = language_pack_id.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeLanguagePack> for InternalLinkTypeLanguagePack {
    fn as_ref(&self) -> &InternalLinkTypeLanguagePack {
        self
    }
}

impl AsRef<InternalLinkTypeLanguagePack> for RTDInternalLinkTypeLanguagePackBuilder {
    fn as_ref(&self) -> &InternalLinkTypeLanguagePack {
        &self.inner
    }
}

/// The link is a link to the language settings section of the app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeLanguageSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeLanguageSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeLanguageSettings {}

impl InternalLinkTypeLanguageSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeLanguageSettingsBuilder {
        let mut inner = InternalLinkTypeLanguageSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeLanguageSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeLanguageSettingsBuilder {
    inner: InternalLinkTypeLanguageSettings,
}

impl RTDInternalLinkTypeLanguageSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeLanguageSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeLanguageSettings> for InternalLinkTypeLanguageSettings {
    fn as_ref(&self) -> &InternalLinkTypeLanguageSettings {
        self
    }
}

impl AsRef<InternalLinkTypeLanguageSettings> for RTDInternalLinkTypeLanguageSettingsBuilder {
    fn as_ref(&self) -> &InternalLinkTypeLanguageSettings {
        &self.inner
    }
}

/// The link is a link to a Telegram message. Call getMessageLinkInfo with the given URL to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// URL to be passed to getMessageLinkInfo

    #[serde(default)]
    url: String,
}

impl RObject for InternalLinkTypeMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeMessage {}

impl InternalLinkTypeMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeMessageBuilder {
        let mut inner = InternalLinkTypeMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeMessageBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeMessageBuilder {
    inner: InternalLinkTypeMessage,
}

impl RTDInternalLinkTypeMessageBuilder {
    pub fn build(&self) -> InternalLinkTypeMessage {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeMessage> for InternalLinkTypeMessage {
    fn as_ref(&self) -> &InternalLinkTypeMessage {
        self
    }
}

impl AsRef<InternalLinkTypeMessage> for RTDInternalLinkTypeMessageBuilder {
    fn as_ref(&self) -> &InternalLinkTypeMessage {
        &self.inner
    }
}

/// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeMessageDraft {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message draft text
    text: FormattedText,
    /// True, if the first line of the text contains a link. If true, the input field needs to be focused and the text after the link must be selected

    #[serde(default)]
    contains_link: bool,
}

impl RObject for InternalLinkTypeMessageDraft {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeMessageDraft {}

impl InternalLinkTypeMessageDraft {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeMessageDraftBuilder {
        let mut inner = InternalLinkTypeMessageDraft::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeMessageDraftBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    pub fn contains_link(&self) -> bool {
        self.contains_link
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeMessageDraftBuilder {
    inner: InternalLinkTypeMessageDraft,
}

impl RTDInternalLinkTypeMessageDraftBuilder {
    pub fn build(&self) -> InternalLinkTypeMessageDraft {
        self.inner.clone()
    }

    pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn contains_link(&mut self, contains_link: bool) -> &mut Self {
        self.inner.contains_link = contains_link;
        self
    }
}

impl AsRef<InternalLinkTypeMessageDraft> for InternalLinkTypeMessageDraft {
    fn as_ref(&self) -> &InternalLinkTypeMessageDraft {
        self
    }
}

impl AsRef<InternalLinkTypeMessageDraft> for RTDInternalLinkTypeMessageDraftBuilder {
    fn as_ref(&self) -> &InternalLinkTypeMessageDraft {
        &self.inner
    }
}

/// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the application, otherwise ignore it
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePassportDataRequest {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier of the service's bot

    #[serde(default)]
    bot_user_id: i64,
    /// Telegram Passport element types requested by the service

    #[serde(default)]
    scope: String,
    /// Service's public key

    #[serde(default)]
    public_key: String,
    /// Unique request identifier provided by the service

    #[serde(default)]
    nonce: String,
    /// An HTTP URL to open once the request is finished or canceled with the parameter tg_passport=success or tg_passport=cancel respectively. If empty, then the link tgbot{bot_user_id}://passport/success or tgbot{bot_user_id}://passport/cancel needs to be opened instead

    #[serde(default)]
    callback_url: String,
}

impl RObject for InternalLinkTypePassportDataRequest {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypePassportDataRequest {}

impl InternalLinkTypePassportDataRequest {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypePassportDataRequestBuilder {
        let mut inner = InternalLinkTypePassportDataRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypePassportDataRequestBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn scope(&self) -> &String {
        &self.scope
    }

    pub fn public_key(&self) -> &String {
        &self.public_key
    }

    pub fn nonce(&self) -> &String {
        &self.nonce
    }

    pub fn callback_url(&self) -> &String {
        &self.callback_url
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePassportDataRequestBuilder {
    inner: InternalLinkTypePassportDataRequest,
}

impl RTDInternalLinkTypePassportDataRequestBuilder {
    pub fn build(&self) -> InternalLinkTypePassportDataRequest {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn scope<T: AsRef<str>>(&mut self, scope: T) -> &mut Self {
        self.inner.scope = scope.as_ref().to_string();
        self
    }

    pub fn public_key<T: AsRef<str>>(&mut self, public_key: T) -> &mut Self {
        self.inner.public_key = public_key.as_ref().to_string();
        self
    }

    pub fn nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
        self.inner.nonce = nonce.as_ref().to_string();
        self
    }

    pub fn callback_url<T: AsRef<str>>(&mut self, callback_url: T) -> &mut Self {
        self.inner.callback_url = callback_url.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypePassportDataRequest> for InternalLinkTypePassportDataRequest {
    fn as_ref(&self) -> &InternalLinkTypePassportDataRequest {
        self
    }
}

impl AsRef<InternalLinkTypePassportDataRequest> for RTDInternalLinkTypePassportDataRequestBuilder {
    fn as_ref(&self) -> &InternalLinkTypePassportDataRequest {
        &self.inner
    }
}

/// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberConfirmationCode with the given hash and phone number to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePhoneNumberConfirmation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Hash value from the link

    #[serde(default)]
    hash: String,
    /// Phone number value from the link

    #[serde(default)]
    phone_number: String,
}

impl RObject for InternalLinkTypePhoneNumberConfirmation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypePhoneNumberConfirmation {}

impl InternalLinkTypePhoneNumberConfirmation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypePhoneNumberConfirmationBuilder {
        let mut inner = InternalLinkTypePhoneNumberConfirmation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypePhoneNumberConfirmationBuilder { inner }
    }

    pub fn hash(&self) -> &String {
        &self.hash
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePhoneNumberConfirmationBuilder {
    inner: InternalLinkTypePhoneNumberConfirmation,
}

impl RTDInternalLinkTypePhoneNumberConfirmationBuilder {
    pub fn build(&self) -> InternalLinkTypePhoneNumberConfirmation {
        self.inner.clone()
    }

    pub fn hash<T: AsRef<str>>(&mut self, hash: T) -> &mut Self {
        self.inner.hash = hash.as_ref().to_string();
        self
    }

    pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
        self.inner.phone_number = phone_number.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypePhoneNumberConfirmation> for InternalLinkTypePhoneNumberConfirmation {
    fn as_ref(&self) -> &InternalLinkTypePhoneNumberConfirmation {
        self
    }
}

impl AsRef<InternalLinkTypePhoneNumberConfirmation>
    for RTDInternalLinkTypePhoneNumberConfirmationBuilder
{
    fn as_ref(&self) -> &InternalLinkTypePhoneNumberConfirmation {
        &self.inner
    }
}

/// The link is a link to the Premium features screen of the applcation from which the user can subscribe to Telegram Premium. Call getPremiumFeatures with the given referrer to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePremiumFeatures {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Referrer specified in the link

    #[serde(default)]
    referrer: String,
}

impl RObject for InternalLinkTypePremiumFeatures {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypePremiumFeatures {}

impl InternalLinkTypePremiumFeatures {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypePremiumFeaturesBuilder {
        let mut inner = InternalLinkTypePremiumFeatures::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypePremiumFeaturesBuilder { inner }
    }

    pub fn referrer(&self) -> &String {
        &self.referrer
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePremiumFeaturesBuilder {
    inner: InternalLinkTypePremiumFeatures,
}

impl RTDInternalLinkTypePremiumFeaturesBuilder {
    pub fn build(&self) -> InternalLinkTypePremiumFeatures {
        self.inner.clone()
    }

    pub fn referrer<T: AsRef<str>>(&mut self, referrer: T) -> &mut Self {
        self.inner.referrer = referrer.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypePremiumFeatures> for InternalLinkTypePremiumFeatures {
    fn as_ref(&self) -> &InternalLinkTypePremiumFeatures {
        self
    }
}

impl AsRef<InternalLinkTypePremiumFeatures> for RTDInternalLinkTypePremiumFeaturesBuilder {
    fn as_ref(&self) -> &InternalLinkTypePremiumFeatures {
        &self.inner
    }
}

/// The link is a link to the privacy and security settings section of the app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePrivacyAndSecuritySettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypePrivacyAndSecuritySettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypePrivacyAndSecuritySettings {}

impl InternalLinkTypePrivacyAndSecuritySettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypePrivacyAndSecuritySettingsBuilder {
        let mut inner = InternalLinkTypePrivacyAndSecuritySettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypePrivacyAndSecuritySettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePrivacyAndSecuritySettingsBuilder {
    inner: InternalLinkTypePrivacyAndSecuritySettings,
}

impl RTDInternalLinkTypePrivacyAndSecuritySettingsBuilder {
    pub fn build(&self) -> InternalLinkTypePrivacyAndSecuritySettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypePrivacyAndSecuritySettings>
    for InternalLinkTypePrivacyAndSecuritySettings
{
    fn as_ref(&self) -> &InternalLinkTypePrivacyAndSecuritySettings {
        self
    }
}

impl AsRef<InternalLinkTypePrivacyAndSecuritySettings>
    for RTDInternalLinkTypePrivacyAndSecuritySettingsBuilder
{
    fn as_ref(&self) -> &InternalLinkTypePrivacyAndSecuritySettings {
        &self.inner
    }
}

/// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Proxy server IP address

    #[serde(default)]
    server: String,
    /// Proxy server port

    #[serde(default)]
    port: i32,
    /// Type of the proxy

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "ProxyType::_is_default")]
    type_: ProxyType,
}

impl RObject for InternalLinkTypeProxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeProxy {}

impl InternalLinkTypeProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeProxyBuilder {
        let mut inner = InternalLinkTypeProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeProxyBuilder { inner }
    }

    pub fn server(&self) -> &String {
        &self.server
    }

    pub fn port(&self) -> i32 {
        self.port
    }

    pub fn type_(&self) -> &ProxyType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeProxyBuilder {
    inner: InternalLinkTypeProxy,
}

impl RTDInternalLinkTypeProxyBuilder {
    pub fn build(&self) -> InternalLinkTypeProxy {
        self.inner.clone()
    }

    pub fn server<T: AsRef<str>>(&mut self, server: T) -> &mut Self {
        self.inner.server = server.as_ref().to_string();
        self
    }

    pub fn port(&mut self, port: i32) -> &mut Self {
        self.inner.port = port;
        self
    }

    pub fn type_<T: AsRef<ProxyType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<InternalLinkTypeProxy> for InternalLinkTypeProxy {
    fn as_ref(&self) -> &InternalLinkTypeProxy {
        self
    }
}

impl AsRef<InternalLinkTypeProxy> for RTDInternalLinkTypeProxyBuilder {
    fn as_ref(&self) -> &InternalLinkTypeProxy {
        &self.inner
    }
}

/// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePublicChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username of the chat

    #[serde(default)]
    chat_username: String,
}

impl RObject for InternalLinkTypePublicChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypePublicChat {}

impl InternalLinkTypePublicChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypePublicChatBuilder {
        let mut inner = InternalLinkTypePublicChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypePublicChatBuilder { inner }
    }

    pub fn chat_username(&self) -> &String {
        &self.chat_username
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypePublicChatBuilder {
    inner: InternalLinkTypePublicChat,
}

impl RTDInternalLinkTypePublicChatBuilder {
    pub fn build(&self) -> InternalLinkTypePublicChat {
        self.inner.clone()
    }

    pub fn chat_username<T: AsRef<str>>(&mut self, chat_username: T) -> &mut Self {
        self.inner.chat_username = chat_username.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypePublicChat> for InternalLinkTypePublicChat {
    fn as_ref(&self) -> &InternalLinkTypePublicChat {
        self
    }
}

impl AsRef<InternalLinkTypePublicChat> for RTDInternalLinkTypePublicChatBuilder {
    fn as_ref(&self) -> &InternalLinkTypePublicChat {
        &self.inner
    }
}

/// The link can be used to login the current user on another device, but it must be scanned from QR-code using in-app camera. An alert similar to "This code can be used to allow someone to log in to your Telegram account. To confirm Telegram login, please go to Settings > Devices > Scan QR and scan the code" needs to be shown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeQrCodeAuthentication {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeQrCodeAuthentication {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeQrCodeAuthentication {}

impl InternalLinkTypeQrCodeAuthentication {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeQrCodeAuthenticationBuilder {
        let mut inner = InternalLinkTypeQrCodeAuthentication::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeQrCodeAuthenticationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeQrCodeAuthenticationBuilder {
    inner: InternalLinkTypeQrCodeAuthentication,
}

impl RTDInternalLinkTypeQrCodeAuthenticationBuilder {
    pub fn build(&self) -> InternalLinkTypeQrCodeAuthentication {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeQrCodeAuthentication> for InternalLinkTypeQrCodeAuthentication {
    fn as_ref(&self) -> &InternalLinkTypeQrCodeAuthentication {
        self
    }
}

impl AsRef<InternalLinkTypeQrCodeAuthentication>
    for RTDInternalLinkTypeQrCodeAuthenticationBuilder
{
    fn as_ref(&self) -> &InternalLinkTypeQrCodeAuthentication {
        &self.inner
    }
}

/// The link is a link to application settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeSettings {}

impl InternalLinkTypeSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeSettingsBuilder {
        let mut inner = InternalLinkTypeSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeSettingsBuilder {
    inner: InternalLinkTypeSettings,
}

impl RTDInternalLinkTypeSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeSettings> for InternalLinkTypeSettings {
    fn as_ref(&self) -> &InternalLinkTypeSettings {
        self
    }
}

impl AsRef<InternalLinkTypeSettings> for RTDInternalLinkTypeSettingsBuilder {
    fn as_ref(&self) -> &InternalLinkTypeSettings {
        &self.inner
    }
}

/// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeStickerSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the sticker set

    #[serde(default)]
    sticker_set_name: String,
}

impl RObject for InternalLinkTypeStickerSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeStickerSet {}

impl InternalLinkTypeStickerSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeStickerSetBuilder {
        let mut inner = InternalLinkTypeStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeStickerSetBuilder { inner }
    }

    pub fn sticker_set_name(&self) -> &String {
        &self.sticker_set_name
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeStickerSetBuilder {
    inner: InternalLinkTypeStickerSet,
}

impl RTDInternalLinkTypeStickerSetBuilder {
    pub fn build(&self) -> InternalLinkTypeStickerSet {
        self.inner.clone()
    }

    pub fn sticker_set_name<T: AsRef<str>>(&mut self, sticker_set_name: T) -> &mut Self {
        self.inner.sticker_set_name = sticker_set_name.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeStickerSet> for InternalLinkTypeStickerSet {
    fn as_ref(&self) -> &InternalLinkTypeStickerSet {
        self
    }
}

impl AsRef<InternalLinkTypeStickerSet> for RTDInternalLinkTypeStickerSetBuilder {
    fn as_ref(&self) -> &InternalLinkTypeStickerSet {
        &self.inner
    }
}

/// The link is a link to a theme. TDLib has no theme support yet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeTheme {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the theme

    #[serde(default)]
    theme_name: String,
}

impl RObject for InternalLinkTypeTheme {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeTheme {}

impl InternalLinkTypeTheme {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeThemeBuilder {
        let mut inner = InternalLinkTypeTheme::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeThemeBuilder { inner }
    }

    pub fn theme_name(&self) -> &String {
        &self.theme_name
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeThemeBuilder {
    inner: InternalLinkTypeTheme,
}

impl RTDInternalLinkTypeThemeBuilder {
    pub fn build(&self) -> InternalLinkTypeTheme {
        self.inner.clone()
    }

    pub fn theme_name<T: AsRef<str>>(&mut self, theme_name: T) -> &mut Self {
        self.inner.theme_name = theme_name.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeTheme> for InternalLinkTypeTheme {
    fn as_ref(&self) -> &InternalLinkTypeTheme {
        self
    }
}

impl AsRef<InternalLinkTypeTheme> for RTDInternalLinkTypeThemeBuilder {
    fn as_ref(&self) -> &InternalLinkTypeTheme {
        &self.inner
    }
}

/// The link is a link to the theme settings section of the app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeThemeSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeThemeSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeThemeSettings {}

impl InternalLinkTypeThemeSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeThemeSettingsBuilder {
        let mut inner = InternalLinkTypeThemeSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeThemeSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeThemeSettingsBuilder {
    inner: InternalLinkTypeThemeSettings,
}

impl RTDInternalLinkTypeThemeSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeThemeSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeThemeSettings> for InternalLinkTypeThemeSettings {
    fn as_ref(&self) -> &InternalLinkTypeThemeSettings {
        self
    }
}

impl AsRef<InternalLinkTypeThemeSettings> for RTDInternalLinkTypeThemeSettingsBuilder {
    fn as_ref(&self) -> &InternalLinkTypeThemeSettings {
        &self.inner
    }
}

/// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeUnknownDeepLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Link to be passed to getDeepLinkInfo

    #[serde(default)]
    link: String,
}

impl RObject for InternalLinkTypeUnknownDeepLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeUnknownDeepLink {}

impl InternalLinkTypeUnknownDeepLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeUnknownDeepLinkBuilder {
        let mut inner = InternalLinkTypeUnknownDeepLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeUnknownDeepLinkBuilder { inner }
    }

    pub fn link(&self) -> &String {
        &self.link
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeUnknownDeepLinkBuilder {
    inner: InternalLinkTypeUnknownDeepLink,
}

impl RTDInternalLinkTypeUnknownDeepLinkBuilder {
    pub fn build(&self) -> InternalLinkTypeUnknownDeepLink {
        self.inner.clone()
    }

    pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
        self.inner.link = link.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeUnknownDeepLink> for InternalLinkTypeUnknownDeepLink {
    fn as_ref(&self) -> &InternalLinkTypeUnknownDeepLink {
        self
    }
}

impl AsRef<InternalLinkTypeUnknownDeepLink> for RTDInternalLinkTypeUnknownDeepLinkBuilder {
    fn as_ref(&self) -> &InternalLinkTypeUnknownDeepLink {
        &self.inner
    }
}

/// The link is a link to an unsupported proxy. An alert can be shown to the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeUnsupportedProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeUnsupportedProxy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeUnsupportedProxy {}

impl InternalLinkTypeUnsupportedProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeUnsupportedProxyBuilder {
        let mut inner = InternalLinkTypeUnsupportedProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeUnsupportedProxyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeUnsupportedProxyBuilder {
    inner: InternalLinkTypeUnsupportedProxy,
}

impl RTDInternalLinkTypeUnsupportedProxyBuilder {
    pub fn build(&self) -> InternalLinkTypeUnsupportedProxy {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeUnsupportedProxy> for InternalLinkTypeUnsupportedProxy {
    fn as_ref(&self) -> &InternalLinkTypeUnsupportedProxy {
        self
    }
}

impl AsRef<InternalLinkTypeUnsupportedProxy> for RTDInternalLinkTypeUnsupportedProxyBuilder {
    fn as_ref(&self) -> &InternalLinkTypeUnsupportedProxy {
        &self.inner
    }
}

/// The link is a link to a user by its phone number. Call searchUserByPhoneNumber with the given phone number to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeUserPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Phone number of the user

    #[serde(default)]
    phone_number: String,
}

impl RObject for InternalLinkTypeUserPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeUserPhoneNumber {}

impl InternalLinkTypeUserPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeUserPhoneNumberBuilder {
        let mut inner = InternalLinkTypeUserPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeUserPhoneNumberBuilder { inner }
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeUserPhoneNumberBuilder {
    inner: InternalLinkTypeUserPhoneNumber,
}

impl RTDInternalLinkTypeUserPhoneNumberBuilder {
    pub fn build(&self) -> InternalLinkTypeUserPhoneNumber {
        self.inner.clone()
    }

    pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
        self.inner.phone_number = phone_number.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeUserPhoneNumber> for InternalLinkTypeUserPhoneNumber {
    fn as_ref(&self) -> &InternalLinkTypeUserPhoneNumber {
        self
    }
}

impl AsRef<InternalLinkTypeUserPhoneNumber> for RTDInternalLinkTypeUserPhoneNumberBuilder {
    fn as_ref(&self) -> &InternalLinkTypeUserPhoneNumber {
        &self.inner
    }
}

/// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinGroupCall with the given invite hash to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeVideoChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username of the chat with the video chat

    #[serde(default)]
    chat_username: String,
    /// If non-empty, invite hash to be used to join the video chat without being muted by administrators

    #[serde(default)]
    invite_hash: String,
    /// True, if the video chat is expected to be a live stream in a channel or a broadcast group

    #[serde(default)]
    is_live_stream: bool,
}

impl RObject for InternalLinkTypeVideoChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeVideoChat {}

impl InternalLinkTypeVideoChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInternalLinkTypeVideoChatBuilder {
        let mut inner = InternalLinkTypeVideoChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInternalLinkTypeVideoChatBuilder { inner }
    }

    pub fn chat_username(&self) -> &String {
        &self.chat_username
    }

    pub fn invite_hash(&self) -> &String {
        &self.invite_hash
    }

    pub fn is_live_stream(&self) -> bool {
        self.is_live_stream
    }
}

#[doc(hidden)]
pub struct RTDInternalLinkTypeVideoChatBuilder {
    inner: InternalLinkTypeVideoChat,
}

impl RTDInternalLinkTypeVideoChatBuilder {
    pub fn build(&self) -> InternalLinkTypeVideoChat {
        self.inner.clone()
    }

    pub fn chat_username<T: AsRef<str>>(&mut self, chat_username: T) -> &mut Self {
        self.inner.chat_username = chat_username.as_ref().to_string();
        self
    }

    pub fn invite_hash<T: AsRef<str>>(&mut self, invite_hash: T) -> &mut Self {
        self.inner.invite_hash = invite_hash.as_ref().to_string();
        self
    }

    pub fn is_live_stream(&mut self, is_live_stream: bool) -> &mut Self {
        self.inner.is_live_stream = is_live_stream;
        self
    }
}

impl AsRef<InternalLinkTypeVideoChat> for InternalLinkTypeVideoChat {
    fn as_ref(&self) -> &InternalLinkTypeVideoChat {
        self
    }
}

impl AsRef<InternalLinkTypeVideoChat> for RTDInternalLinkTypeVideoChatBuilder {
    fn as_ref(&self) -> &InternalLinkTypeVideoChat {
        &self.inner
    }
}
