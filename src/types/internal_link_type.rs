use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes an internal https://t.me or tg: link, which must be processed by the application in a special way
pub trait TDInternalLinkType: Debug + RObject {}

/// Describes an internal https://t.me or tg: link, which must be processed by the application in a special way
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum InternalLinkType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Returns information about the type of an internal link. Returns a 404 error if the link is not internal. Can be called before authorization
    #[serde(rename = "getInternalLinkType")]
    GetInternalLinkType(GetInternalLinkType),
    /// The link is a link to the active sessions section of the application. Use getActiveSessions to handle the link
    #[serde(rename = "internalLinkTypeActiveSessions")]
    ActiveSessions(InternalLinkTypeActiveSessions),
    /// The link is a link to an attachment menu bot to be opened in the specified or a chosen chat. Process given target_chat to open the chat. Then, call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then, use getAttachmentMenuBot to receive information about the bot. If the bot isn't added to attachment menu, then show a disclaimer about Mini Apps being a third-party apps, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu. If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot. If the attachment menu bot can't be used in the opened chat, show an error to the user. If the bot is added to attachment menu and can be used in the chat, then use openWebApp with the given URL
    #[serde(rename = "internalLinkTypeAttachmentMenuBot")]
    AttachmentMenuBot(InternalLinkTypeAttachmentMenuBot),
    /// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
    #[serde(rename = "internalLinkTypeAuthenticationCode")]
    AuthenticationCode(InternalLinkTypeAuthenticationCode),
    /// The link is a link to a background. Call searchBackground with the given background name to process the link
    #[serde(rename = "internalLinkTypeBackground")]
    Background(InternalLinkTypeBackground),
    /// The link is a link to a Telegram bot, which is supposed to be added to a channel chat as an administrator. Call searchPublicChat with the given bot username and check that the user is a bot, ask the current user to select a channel chat to add the bot to as an administrator. Then, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights and combine received rights with the requested administrator rights. Then, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed rights
    #[serde(rename = "internalLinkTypeBotAddToChannel")]
    BotAddToChannel(InternalLinkTypeBotAddToChannel),
    /// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot, and then call sendBotStartMessage with the given start parameter after the button is pressed
    #[serde(rename = "internalLinkTypeBotStart")]
    BotStart(InternalLinkTypeBotStart),
    /// The link is a link to a Telegram bot, which is supposed to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups, ask the current user to select a basic group or a supergroup chat to add the bot to, taking into account that bots can be added to a public supergroup only by administrators of the supergroup. If administrator rights are provided by the link, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights, combine received rights with the requested administrator rights, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed administrator rights. Before call to setChatMemberStatus it may be required to upgrade the chosen basic group chat to a supergroup chat. Then, if start_parameter isn't empty, call sendBotStartMessage with the given start parameter and the chosen chat; otherwise, just send /start message with bot's username added to the chat.
    #[serde(rename = "internalLinkTypeBotStartInGroup")]
    BotStartInGroup(InternalLinkTypeBotStartInGroup),
    /// The link is a link to the change phone number section of the app
    #[serde(rename = "internalLinkTypeChangePhoneNumber")]
    ChangePhoneNumber(InternalLinkTypeChangePhoneNumber),
    /// The link is a link to boost a Telegram chat. Call getChatBoostLinkInfo with the given URL to process the link. If the chat is found, then call getChatBoostStatus and getAvailableChatBoostSlots to get the current boost status and check whether the chat can be boosted. If the user wants to boost the chat and the chat can be boosted, then call boostChat
    #[serde(rename = "internalLinkTypeChatBoost")]
    ChatBoost(InternalLinkTypeChatBoost),
    /// The link is an invite link to a chat folder. Call checkChatFolderInviteLink with the given invite link to process the link
    #[serde(rename = "internalLinkTypeChatFolderInvite")]
    ChatFolderInvite(InternalLinkTypeChatFolderInvite),
    /// The link is a link to the folder section of the app settings
    #[serde(rename = "internalLinkTypeChatFolderSettings")]
    ChatFolderSettings(InternalLinkTypeChatFolderSettings),
    /// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link
    #[serde(rename = "internalLinkTypeChatInvite")]
    ChatInvite(InternalLinkTypeChatInvite),
    /// The link is a link to the default message auto-delete timer settings section of the app settings
    #[serde(rename = "internalLinkTypeDefaultMessageAutoDeleteTimerSettings")]
    DefaultMessageAutoDeleteTimerSettings(InternalLinkTypeDefaultMessageAutoDeleteTimerSettings),
    /// The link is a link to the edit profile section of the app settings
    #[serde(rename = "internalLinkTypeEditProfileSettings")]
    EditProfileSettings(InternalLinkTypeEditProfileSettings),
    /// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot, ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
    #[serde(rename = "internalLinkTypeGame")]
    Game(InternalLinkTypeGame),
    /// The link must be opened in an Instant View. Call getWebPageInstantView with the given URL to process the link
    #[serde(rename = "internalLinkTypeInstantView")]
    InstantView(InternalLinkTypeInstantView),
    /// The link is a link to an invoice. Call getPaymentForm with the given invoice name to process the link
    #[serde(rename = "internalLinkTypeInvoice")]
    Invoice(InternalLinkTypeInvoice),
    /// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link
    #[serde(rename = "internalLinkTypeLanguagePack")]
    LanguagePack(InternalLinkTypeLanguagePack),
    /// The link is a link to the language section of the app settings
    #[serde(rename = "internalLinkTypeLanguageSettings")]
    LanguageSettings(InternalLinkTypeLanguageSettings),
    /// The link is a link to a Telegram message or a forum topic. Call getMessageLinkInfo with the given URL to process the link
    #[serde(rename = "internalLinkTypeMessage")]
    Message(InternalLinkTypeMessage),
    /// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
    #[serde(rename = "internalLinkTypeMessageDraft")]
    MessageDraft(InternalLinkTypeMessageDraft),
    /// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the application; otherwise, ignore it
    #[serde(rename = "internalLinkTypePassportDataRequest")]
    PassportDataRequest(InternalLinkTypePassportDataRequest),
    /// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberConfirmationCode with the given hash and phone number to process the link
    #[serde(rename = "internalLinkTypePhoneNumberConfirmation")]
    PhoneNumberConfirmation(InternalLinkTypePhoneNumberConfirmation),
    /// The link is a link to the Premium features screen of the application from which the user can subscribe to Telegram Premium. Call getPremiumFeatures with the given referrer to process the link
    #[serde(rename = "internalLinkTypePremiumFeatures")]
    PremiumFeatures(InternalLinkTypePremiumFeatures),
    /// The link is a link with a Telegram Premium gift code. Call checkPremiumGiftCode with the given code to process the link. If the code is valid and the user wants to apply it, then call applyPremiumGiftCode
    #[serde(rename = "internalLinkTypePremiumGiftCode")]
    PremiumGiftCode(InternalLinkTypePremiumGiftCode),
    /// The link is a link to the privacy and security section of the app settings
    #[serde(rename = "internalLinkTypePrivacyAndSecuritySettings")]
    PrivacyAndSecuritySettings(InternalLinkTypePrivacyAndSecuritySettings),
    /// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
    #[serde(rename = "internalLinkTypeProxy")]
    Proxy(InternalLinkTypeProxy),
    /// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link
    #[serde(rename = "internalLinkTypePublicChat")]
    PublicChat(InternalLinkTypePublicChat),
    /// The link can be used to login the current user on another device, but it must be scanned from QR-code using in-app camera. An alert similar to "This code can be used to allow someone to log in to your Telegram account. To confirm Telegram login, please go to Settings > Devices > Scan QR and scan the code" needs to be shown
    #[serde(rename = "internalLinkTypeQrCodeAuthentication")]
    QrCodeAuthentication(InternalLinkTypeQrCodeAuthentication),
    /// The link forces restore of App Store purchases when opened. For official iOS application only
    #[serde(rename = "internalLinkTypeRestorePurchases")]
    RestorePurchases(InternalLinkTypeRestorePurchases),
    /// The link is a link to application settings
    #[serde(rename = "internalLinkTypeSettings")]
    Settings(InternalLinkTypeSettings),
    /// The link is a link to a bot, which can be installed to the side menu. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then, use getAttachmentMenuBot to receive information about the bot. If the bot isn't added to side menu, then show a disclaimer about Mini Apps being a third-party apps, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu. If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot. If the bot is added to side menu, then use getWebAppUrl with the given URL
    #[serde(rename = "internalLinkTypeSideMenuBot")]
    SideMenuBot(InternalLinkTypeSideMenuBot),
    /// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set
    #[serde(rename = "internalLinkTypeStickerSet")]
    StickerSet(InternalLinkTypeStickerSet),
    /// The link is a link to a story. Call searchPublicChat with the given sender username, then call getStory with the received chat identifier and the given story identifier
    #[serde(rename = "internalLinkTypeStory")]
    Story(InternalLinkTypeStory),
    /// The link is a link to a theme. TDLib has no theme support yet
    #[serde(rename = "internalLinkTypeTheme")]
    Theme(InternalLinkTypeTheme),
    /// The link is a link to the theme section of the app settings
    #[serde(rename = "internalLinkTypeThemeSettings")]
    ThemeSettings(InternalLinkTypeThemeSettings),
    /// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
    #[serde(rename = "internalLinkTypeUnknownDeepLink")]
    UnknownDeepLink(InternalLinkTypeUnknownDeepLink),
    /// The link is a link to an unsupported proxy. An alert can be shown to the user
    #[serde(rename = "internalLinkTypeUnsupportedProxy")]
    UnsupportedProxy(InternalLinkTypeUnsupportedProxy),
    /// The link is a link to a user by its phone number. Call searchUserByPhoneNumber with the given phone number to process the link
    #[serde(rename = "internalLinkTypeUserPhoneNumber")]
    UserPhoneNumber(InternalLinkTypeUserPhoneNumber),
    /// The link is a link to a user by a temporary token. Call searchUserByToken with the given token to process the link
    #[serde(rename = "internalLinkTypeUserToken")]
    UserToken(InternalLinkTypeUserToken),
    /// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinGroupCall with the given invite hash to process the link
    #[serde(rename = "internalLinkTypeVideoChat")]
    VideoChat(InternalLinkTypeVideoChat),
    /// The link is a link to a Web App. Call searchPublicChat with the given bot username, check that the user is a bot, then call searchWebApp with the received bot and the given web_app_short_name. Process received foundWebApp by showing a confirmation dialog if needed. If the bot can be added to attachment or side menu, but isn't added yet, then show a disclaimer about Mini Apps being a third-party apps instead of the dialog and ask the user to accept their Terms of service. If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot. Then, call getWebAppLinkUrl and open the returned URL as a Web App
    #[serde(rename = "internalLinkTypeWebApp")]
    WebApp(InternalLinkTypeWebApp),
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
            InternalLinkType::ChatBoost(t) => t.extra(),
            InternalLinkType::ChatFolderInvite(t) => t.extra(),
            InternalLinkType::ChatFolderSettings(t) => t.extra(),
            InternalLinkType::ChatInvite(t) => t.extra(),
            InternalLinkType::DefaultMessageAutoDeleteTimerSettings(t) => t.extra(),
            InternalLinkType::EditProfileSettings(t) => t.extra(),
            InternalLinkType::Game(t) => t.extra(),
            InternalLinkType::InstantView(t) => t.extra(),
            InternalLinkType::Invoice(t) => t.extra(),
            InternalLinkType::LanguagePack(t) => t.extra(),
            InternalLinkType::LanguageSettings(t) => t.extra(),
            InternalLinkType::Message(t) => t.extra(),
            InternalLinkType::MessageDraft(t) => t.extra(),
            InternalLinkType::PassportDataRequest(t) => t.extra(),
            InternalLinkType::PhoneNumberConfirmation(t) => t.extra(),
            InternalLinkType::PremiumFeatures(t) => t.extra(),
            InternalLinkType::PremiumGiftCode(t) => t.extra(),
            InternalLinkType::PrivacyAndSecuritySettings(t) => t.extra(),
            InternalLinkType::Proxy(t) => t.extra(),
            InternalLinkType::PublicChat(t) => t.extra(),
            InternalLinkType::QrCodeAuthentication(t) => t.extra(),
            InternalLinkType::RestorePurchases(t) => t.extra(),
            InternalLinkType::Settings(t) => t.extra(),
            InternalLinkType::SideMenuBot(t) => t.extra(),
            InternalLinkType::StickerSet(t) => t.extra(),
            InternalLinkType::Story(t) => t.extra(),
            InternalLinkType::Theme(t) => t.extra(),
            InternalLinkType::ThemeSettings(t) => t.extra(),
            InternalLinkType::UnknownDeepLink(t) => t.extra(),
            InternalLinkType::UnsupportedProxy(t) => t.extra(),
            InternalLinkType::UserPhoneNumber(t) => t.extra(),
            InternalLinkType::UserToken(t) => t.extra(),
            InternalLinkType::VideoChat(t) => t.extra(),
            InternalLinkType::WebApp(t) => t.extra(),

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
            InternalLinkType::ChatBoost(t) => t.client_id(),
            InternalLinkType::ChatFolderInvite(t) => t.client_id(),
            InternalLinkType::ChatFolderSettings(t) => t.client_id(),
            InternalLinkType::ChatInvite(t) => t.client_id(),
            InternalLinkType::DefaultMessageAutoDeleteTimerSettings(t) => t.client_id(),
            InternalLinkType::EditProfileSettings(t) => t.client_id(),
            InternalLinkType::Game(t) => t.client_id(),
            InternalLinkType::InstantView(t) => t.client_id(),
            InternalLinkType::Invoice(t) => t.client_id(),
            InternalLinkType::LanguagePack(t) => t.client_id(),
            InternalLinkType::LanguageSettings(t) => t.client_id(),
            InternalLinkType::Message(t) => t.client_id(),
            InternalLinkType::MessageDraft(t) => t.client_id(),
            InternalLinkType::PassportDataRequest(t) => t.client_id(),
            InternalLinkType::PhoneNumberConfirmation(t) => t.client_id(),
            InternalLinkType::PremiumFeatures(t) => t.client_id(),
            InternalLinkType::PremiumGiftCode(t) => t.client_id(),
            InternalLinkType::PrivacyAndSecuritySettings(t) => t.client_id(),
            InternalLinkType::Proxy(t) => t.client_id(),
            InternalLinkType::PublicChat(t) => t.client_id(),
            InternalLinkType::QrCodeAuthentication(t) => t.client_id(),
            InternalLinkType::RestorePurchases(t) => t.client_id(),
            InternalLinkType::Settings(t) => t.client_id(),
            InternalLinkType::SideMenuBot(t) => t.client_id(),
            InternalLinkType::StickerSet(t) => t.client_id(),
            InternalLinkType::Story(t) => t.client_id(),
            InternalLinkType::Theme(t) => t.client_id(),
            InternalLinkType::ThemeSettings(t) => t.client_id(),
            InternalLinkType::UnknownDeepLink(t) => t.client_id(),
            InternalLinkType::UnsupportedProxy(t) => t.client_id(),
            InternalLinkType::UserPhoneNumber(t) => t.client_id(),
            InternalLinkType::UserToken(t) => t.client_id(),
            InternalLinkType::VideoChat(t) => t.client_id(),
            InternalLinkType::WebApp(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InternalLinkType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeActiveSessionsBuilder {
        let mut inner = InternalLinkTypeActiveSessions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeActiveSessionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeActiveSessionsBuilder {
    inner: InternalLinkTypeActiveSessions,
}

#[deprecated]
pub type RTDInternalLinkTypeActiveSessionsBuilder = InternalLinkTypeActiveSessionsBuilder;

impl InternalLinkTypeActiveSessionsBuilder {
    pub fn build(&self) -> InternalLinkTypeActiveSessions {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeActiveSessions> for InternalLinkTypeActiveSessions {
    fn as_ref(&self) -> &InternalLinkTypeActiveSessions {
        self
    }
}

impl AsRef<InternalLinkTypeActiveSessions> for InternalLinkTypeActiveSessionsBuilder {
    fn as_ref(&self) -> &InternalLinkTypeActiveSessions {
        &self.inner
    }
}

/// The link is a link to an attachment menu bot to be opened in the specified or a chosen chat. Process given target_chat to open the chat. Then, call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then, use getAttachmentMenuBot to receive information about the bot. If the bot isn't added to attachment menu, then show a disclaimer about Mini Apps being a third-party apps, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu. If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot. If the attachment menu bot can't be used in the opened chat, show an error to the user. If the bot is added to attachment menu and can be used in the chat, then use openWebApp with the given URL
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeAttachmentMenuBotBuilder {
        let mut inner = InternalLinkTypeAttachmentMenuBot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeAttachmentMenuBotBuilder { inner }
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
pub struct InternalLinkTypeAttachmentMenuBotBuilder {
    inner: InternalLinkTypeAttachmentMenuBot,
}

#[deprecated]
pub type RTDInternalLinkTypeAttachmentMenuBotBuilder = InternalLinkTypeAttachmentMenuBotBuilder;

impl InternalLinkTypeAttachmentMenuBotBuilder {
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

impl AsRef<InternalLinkTypeAttachmentMenuBot> for InternalLinkTypeAttachmentMenuBotBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeAuthenticationCodeBuilder {
        let mut inner = InternalLinkTypeAuthenticationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeAuthenticationCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeAuthenticationCodeBuilder {
    inner: InternalLinkTypeAuthenticationCode,
}

#[deprecated]
pub type RTDInternalLinkTypeAuthenticationCodeBuilder = InternalLinkTypeAuthenticationCodeBuilder;

impl InternalLinkTypeAuthenticationCodeBuilder {
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

impl AsRef<InternalLinkTypeAuthenticationCode> for InternalLinkTypeAuthenticationCodeBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeBackgroundBuilder {
        let mut inner = InternalLinkTypeBackground::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeBackgroundBuilder { inner }
    }

    pub fn background_name(&self) -> &String {
        &self.background_name
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeBackgroundBuilder {
    inner: InternalLinkTypeBackground,
}

#[deprecated]
pub type RTDInternalLinkTypeBackgroundBuilder = InternalLinkTypeBackgroundBuilder;

impl InternalLinkTypeBackgroundBuilder {
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

impl AsRef<InternalLinkTypeBackground> for InternalLinkTypeBackgroundBuilder {
    fn as_ref(&self) -> &InternalLinkTypeBackground {
        &self.inner
    }
}

/// The link is a link to a Telegram bot, which is supposed to be added to a channel chat as an administrator. Call searchPublicChat with the given bot username and check that the user is a bot, ask the current user to select a channel chat to add the bot to as an administrator. Then, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights and combine received rights with the requested administrator rights. Then, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed rights
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeBotAddToChannelBuilder {
        let mut inner = InternalLinkTypeBotAddToChannel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeBotAddToChannelBuilder { inner }
    }

    pub fn bot_username(&self) -> &String {
        &self.bot_username
    }

    pub fn administrator_rights(&self) -> &ChatAdministratorRights {
        &self.administrator_rights
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeBotAddToChannelBuilder {
    inner: InternalLinkTypeBotAddToChannel,
}

#[deprecated]
pub type RTDInternalLinkTypeBotAddToChannelBuilder = InternalLinkTypeBotAddToChannelBuilder;

impl InternalLinkTypeBotAddToChannelBuilder {
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

impl AsRef<InternalLinkTypeBotAddToChannel> for InternalLinkTypeBotAddToChannelBuilder {
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
    /// True, if sendBotStartMessage must be called automatically without showing the START button

    #[serde(default)]
    autostart: bool,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeBotStartBuilder {
        let mut inner = InternalLinkTypeBotStart::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeBotStartBuilder { inner }
    }

    pub fn bot_username(&self) -> &String {
        &self.bot_username
    }

    pub fn start_parameter(&self) -> &String {
        &self.start_parameter
    }

    pub fn autostart(&self) -> bool {
        self.autostart
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeBotStartBuilder {
    inner: InternalLinkTypeBotStart,
}

#[deprecated]
pub type RTDInternalLinkTypeBotStartBuilder = InternalLinkTypeBotStartBuilder;

impl InternalLinkTypeBotStartBuilder {
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

    pub fn autostart(&mut self, autostart: bool) -> &mut Self {
        self.inner.autostart = autostart;
        self
    }
}

impl AsRef<InternalLinkTypeBotStart> for InternalLinkTypeBotStart {
    fn as_ref(&self) -> &InternalLinkTypeBotStart {
        self
    }
}

impl AsRef<InternalLinkTypeBotStart> for InternalLinkTypeBotStartBuilder {
    fn as_ref(&self) -> &InternalLinkTypeBotStart {
        &self.inner
    }
}

/// The link is a link to a Telegram bot, which is supposed to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups, ask the current user to select a basic group or a supergroup chat to add the bot to, taking into account that bots can be added to a public supergroup only by administrators of the supergroup. If administrator rights are provided by the link, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator, check that the current user can edit its administrator rights, combine received rights with the requested administrator rights, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed administrator rights. Before call to setChatMemberStatus it may be required to upgrade the chosen basic group chat to a supergroup chat. Then, if start_parameter isn't empty, call sendBotStartMessage with the given start parameter and the chosen chat; otherwise, just send /start message with bot's username added to the chat.
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeBotStartInGroupBuilder {
        let mut inner = InternalLinkTypeBotStartInGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeBotStartInGroupBuilder { inner }
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
pub struct InternalLinkTypeBotStartInGroupBuilder {
    inner: InternalLinkTypeBotStartInGroup,
}

#[deprecated]
pub type RTDInternalLinkTypeBotStartInGroupBuilder = InternalLinkTypeBotStartInGroupBuilder;

impl InternalLinkTypeBotStartInGroupBuilder {
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

impl AsRef<InternalLinkTypeBotStartInGroup> for InternalLinkTypeBotStartInGroupBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeChangePhoneNumberBuilder {
        let mut inner = InternalLinkTypeChangePhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeChangePhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeChangePhoneNumberBuilder {
    inner: InternalLinkTypeChangePhoneNumber,
}

#[deprecated]
pub type RTDInternalLinkTypeChangePhoneNumberBuilder = InternalLinkTypeChangePhoneNumberBuilder;

impl InternalLinkTypeChangePhoneNumberBuilder {
    pub fn build(&self) -> InternalLinkTypeChangePhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeChangePhoneNumber> for InternalLinkTypeChangePhoneNumber {
    fn as_ref(&self) -> &InternalLinkTypeChangePhoneNumber {
        self
    }
}

impl AsRef<InternalLinkTypeChangePhoneNumber> for InternalLinkTypeChangePhoneNumberBuilder {
    fn as_ref(&self) -> &InternalLinkTypeChangePhoneNumber {
        &self.inner
    }
}

/// The link is a link to boost a Telegram chat. Call getChatBoostLinkInfo with the given URL to process the link. If the chat is found, then call getChatBoostStatus and getAvailableChatBoostSlots to get the current boost status and check whether the chat can be boosted. If the user wants to boost the chat and the chat can be boosted, then call boostChat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChatBoost {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// URL to be passed to getChatBoostLinkInfo

    #[serde(default)]
    url: String,
}

impl RObject for InternalLinkTypeChatBoost {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeChatBoost {}

impl InternalLinkTypeChatBoost {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeChatBoostBuilder {
        let mut inner = InternalLinkTypeChatBoost::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeChatBoostBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeChatBoostBuilder {
    inner: InternalLinkTypeChatBoost,
}

#[deprecated]
pub type RTDInternalLinkTypeChatBoostBuilder = InternalLinkTypeChatBoostBuilder;

impl InternalLinkTypeChatBoostBuilder {
    pub fn build(&self) -> InternalLinkTypeChatBoost {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeChatBoost> for InternalLinkTypeChatBoost {
    fn as_ref(&self) -> &InternalLinkTypeChatBoost {
        self
    }
}

impl AsRef<InternalLinkTypeChatBoost> for InternalLinkTypeChatBoostBuilder {
    fn as_ref(&self) -> &InternalLinkTypeChatBoost {
        &self.inner
    }
}

/// The link is an invite link to a chat folder. Call checkChatFolderInviteLink with the given invite link to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChatFolderInvite {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Internal representation of the invite link

    #[serde(default)]
    invite_link: String,
}

impl RObject for InternalLinkTypeChatFolderInvite {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeChatFolderInvite {}

impl InternalLinkTypeChatFolderInvite {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeChatFolderInviteBuilder {
        let mut inner = InternalLinkTypeChatFolderInvite::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeChatFolderInviteBuilder { inner }
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeChatFolderInviteBuilder {
    inner: InternalLinkTypeChatFolderInvite,
}

#[deprecated]
pub type RTDInternalLinkTypeChatFolderInviteBuilder = InternalLinkTypeChatFolderInviteBuilder;

impl InternalLinkTypeChatFolderInviteBuilder {
    pub fn build(&self) -> InternalLinkTypeChatFolderInvite {
        self.inner.clone()
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeChatFolderInvite> for InternalLinkTypeChatFolderInvite {
    fn as_ref(&self) -> &InternalLinkTypeChatFolderInvite {
        self
    }
}

impl AsRef<InternalLinkTypeChatFolderInvite> for InternalLinkTypeChatFolderInviteBuilder {
    fn as_ref(&self) -> &InternalLinkTypeChatFolderInvite {
        &self.inner
    }
}

/// The link is a link to the folder section of the app settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeChatFolderSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeChatFolderSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeChatFolderSettings {}

impl InternalLinkTypeChatFolderSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeChatFolderSettingsBuilder {
        let mut inner = InternalLinkTypeChatFolderSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeChatFolderSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeChatFolderSettingsBuilder {
    inner: InternalLinkTypeChatFolderSettings,
}

#[deprecated]
pub type RTDInternalLinkTypeChatFolderSettingsBuilder = InternalLinkTypeChatFolderSettingsBuilder;

impl InternalLinkTypeChatFolderSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeChatFolderSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeChatFolderSettings> for InternalLinkTypeChatFolderSettings {
    fn as_ref(&self) -> &InternalLinkTypeChatFolderSettings {
        self
    }
}

impl AsRef<InternalLinkTypeChatFolderSettings> for InternalLinkTypeChatFolderSettingsBuilder {
    fn as_ref(&self) -> &InternalLinkTypeChatFolderSettings {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeChatInviteBuilder {
        let mut inner = InternalLinkTypeChatInvite::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeChatInviteBuilder { inner }
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeChatInviteBuilder {
    inner: InternalLinkTypeChatInvite,
}

#[deprecated]
pub type RTDInternalLinkTypeChatInviteBuilder = InternalLinkTypeChatInviteBuilder;

impl InternalLinkTypeChatInviteBuilder {
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

impl AsRef<InternalLinkTypeChatInvite> for InternalLinkTypeChatInviteBuilder {
    fn as_ref(&self) -> &InternalLinkTypeChatInvite {
        &self.inner
    }
}

/// The link is a link to the default message auto-delete timer settings section of the app settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeDefaultMessageAutoDeleteTimerSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeDefaultMessageAutoDeleteTimerSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeDefaultMessageAutoDeleteTimerSettings {}

impl InternalLinkTypeDefaultMessageAutoDeleteTimerSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeDefaultMessageAutoDeleteTimerSettingsBuilder {
        let mut inner = InternalLinkTypeDefaultMessageAutoDeleteTimerSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeDefaultMessageAutoDeleteTimerSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeDefaultMessageAutoDeleteTimerSettingsBuilder {
    inner: InternalLinkTypeDefaultMessageAutoDeleteTimerSettings,
}

#[deprecated]
pub type RTDInternalLinkTypeDefaultMessageAutoDeleteTimerSettingsBuilder =
    InternalLinkTypeDefaultMessageAutoDeleteTimerSettingsBuilder;

impl InternalLinkTypeDefaultMessageAutoDeleteTimerSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeDefaultMessageAutoDeleteTimerSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeDefaultMessageAutoDeleteTimerSettings>
    for InternalLinkTypeDefaultMessageAutoDeleteTimerSettings
{
    fn as_ref(&self) -> &InternalLinkTypeDefaultMessageAutoDeleteTimerSettings {
        self
    }
}

impl AsRef<InternalLinkTypeDefaultMessageAutoDeleteTimerSettings>
    for InternalLinkTypeDefaultMessageAutoDeleteTimerSettingsBuilder
{
    fn as_ref(&self) -> &InternalLinkTypeDefaultMessageAutoDeleteTimerSettings {
        &self.inner
    }
}

/// The link is a link to the edit profile section of the app settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeEditProfileSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeEditProfileSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeEditProfileSettings {}

impl InternalLinkTypeEditProfileSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeEditProfileSettingsBuilder {
        let mut inner = InternalLinkTypeEditProfileSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeEditProfileSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeEditProfileSettingsBuilder {
    inner: InternalLinkTypeEditProfileSettings,
}

#[deprecated]
pub type RTDInternalLinkTypeEditProfileSettingsBuilder = InternalLinkTypeEditProfileSettingsBuilder;

impl InternalLinkTypeEditProfileSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeEditProfileSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeEditProfileSettings> for InternalLinkTypeEditProfileSettings {
    fn as_ref(&self) -> &InternalLinkTypeEditProfileSettings {
        self
    }
}

impl AsRef<InternalLinkTypeEditProfileSettings> for InternalLinkTypeEditProfileSettingsBuilder {
    fn as_ref(&self) -> &InternalLinkTypeEditProfileSettings {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeGameBuilder {
        let mut inner = InternalLinkTypeGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeGameBuilder { inner }
    }

    pub fn bot_username(&self) -> &String {
        &self.bot_username
    }

    pub fn game_short_name(&self) -> &String {
        &self.game_short_name
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeGameBuilder {
    inner: InternalLinkTypeGame,
}

#[deprecated]
pub type RTDInternalLinkTypeGameBuilder = InternalLinkTypeGameBuilder;

impl InternalLinkTypeGameBuilder {
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

impl AsRef<InternalLinkTypeGame> for InternalLinkTypeGameBuilder {
    fn as_ref(&self) -> &InternalLinkTypeGame {
        &self.inner
    }
}

/// The link must be opened in an Instant View. Call getWebPageInstantView with the given URL to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeInstantView {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// URL to be passed to getWebPageInstantView

    #[serde(default)]
    url: String,
    /// An URL to open if getWebPageInstantView fails

    #[serde(default)]
    fallback_url: String,
}

impl RObject for InternalLinkTypeInstantView {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeInstantView {}

impl InternalLinkTypeInstantView {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeInstantViewBuilder {
        let mut inner = InternalLinkTypeInstantView::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeInstantViewBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn fallback_url(&self) -> &String {
        &self.fallback_url
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeInstantViewBuilder {
    inner: InternalLinkTypeInstantView,
}

#[deprecated]
pub type RTDInternalLinkTypeInstantViewBuilder = InternalLinkTypeInstantViewBuilder;

impl InternalLinkTypeInstantViewBuilder {
    pub fn build(&self) -> InternalLinkTypeInstantView {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn fallback_url<T: AsRef<str>>(&mut self, fallback_url: T) -> &mut Self {
        self.inner.fallback_url = fallback_url.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeInstantView> for InternalLinkTypeInstantView {
    fn as_ref(&self) -> &InternalLinkTypeInstantView {
        self
    }
}

impl AsRef<InternalLinkTypeInstantView> for InternalLinkTypeInstantViewBuilder {
    fn as_ref(&self) -> &InternalLinkTypeInstantView {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeInvoiceBuilder {
        let mut inner = InternalLinkTypeInvoice::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeInvoiceBuilder { inner }
    }

    pub fn invoice_name(&self) -> &String {
        &self.invoice_name
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeInvoiceBuilder {
    inner: InternalLinkTypeInvoice,
}

#[deprecated]
pub type RTDInternalLinkTypeInvoiceBuilder = InternalLinkTypeInvoiceBuilder;

impl InternalLinkTypeInvoiceBuilder {
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

impl AsRef<InternalLinkTypeInvoice> for InternalLinkTypeInvoiceBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeLanguagePackBuilder {
        let mut inner = InternalLinkTypeLanguagePack::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeLanguagePackBuilder { inner }
    }

    pub fn language_pack_id(&self) -> &String {
        &self.language_pack_id
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeLanguagePackBuilder {
    inner: InternalLinkTypeLanguagePack,
}

#[deprecated]
pub type RTDInternalLinkTypeLanguagePackBuilder = InternalLinkTypeLanguagePackBuilder;

impl InternalLinkTypeLanguagePackBuilder {
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

impl AsRef<InternalLinkTypeLanguagePack> for InternalLinkTypeLanguagePackBuilder {
    fn as_ref(&self) -> &InternalLinkTypeLanguagePack {
        &self.inner
    }
}

/// The link is a link to the language section of the app settings
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeLanguageSettingsBuilder {
        let mut inner = InternalLinkTypeLanguageSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeLanguageSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeLanguageSettingsBuilder {
    inner: InternalLinkTypeLanguageSettings,
}

#[deprecated]
pub type RTDInternalLinkTypeLanguageSettingsBuilder = InternalLinkTypeLanguageSettingsBuilder;

impl InternalLinkTypeLanguageSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeLanguageSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeLanguageSettings> for InternalLinkTypeLanguageSettings {
    fn as_ref(&self) -> &InternalLinkTypeLanguageSettings {
        self
    }
}

impl AsRef<InternalLinkTypeLanguageSettings> for InternalLinkTypeLanguageSettingsBuilder {
    fn as_ref(&self) -> &InternalLinkTypeLanguageSettings {
        &self.inner
    }
}

/// The link is a link to a Telegram message or a forum topic. Call getMessageLinkInfo with the given URL to process the link
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeMessageBuilder {
        let mut inner = InternalLinkTypeMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeMessageBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeMessageBuilder {
    inner: InternalLinkTypeMessage,
}

#[deprecated]
pub type RTDInternalLinkTypeMessageBuilder = InternalLinkTypeMessageBuilder;

impl InternalLinkTypeMessageBuilder {
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

impl AsRef<InternalLinkTypeMessage> for InternalLinkTypeMessageBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeMessageDraftBuilder {
        let mut inner = InternalLinkTypeMessageDraft::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeMessageDraftBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    pub fn contains_link(&self) -> bool {
        self.contains_link
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeMessageDraftBuilder {
    inner: InternalLinkTypeMessageDraft,
}

#[deprecated]
pub type RTDInternalLinkTypeMessageDraftBuilder = InternalLinkTypeMessageDraftBuilder;

impl InternalLinkTypeMessageDraftBuilder {
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

impl AsRef<InternalLinkTypeMessageDraft> for InternalLinkTypeMessageDraftBuilder {
    fn as_ref(&self) -> &InternalLinkTypeMessageDraft {
        &self.inner
    }
}

/// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the application; otherwise, ignore it
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
    /// An HTTP URL to open once the request is finished, canceled, or failed with the parameters tg_passport=success, tg_passport=cancel, or tg_passport=error&error=... respectively. If empty, then onActivityResult method must be used to return response on Android, or the link tgbot{bot_user_id}://passport/success or tgbot{bot_user_id}://passport/cancel must be opened otherwise

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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypePassportDataRequestBuilder {
        let mut inner = InternalLinkTypePassportDataRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypePassportDataRequestBuilder { inner }
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
pub struct InternalLinkTypePassportDataRequestBuilder {
    inner: InternalLinkTypePassportDataRequest,
}

#[deprecated]
pub type RTDInternalLinkTypePassportDataRequestBuilder = InternalLinkTypePassportDataRequestBuilder;

impl InternalLinkTypePassportDataRequestBuilder {
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

impl AsRef<InternalLinkTypePassportDataRequest> for InternalLinkTypePassportDataRequestBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypePhoneNumberConfirmationBuilder {
        let mut inner = InternalLinkTypePhoneNumberConfirmation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypePhoneNumberConfirmationBuilder { inner }
    }

    pub fn hash(&self) -> &String {
        &self.hash
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }
}

#[doc(hidden)]
pub struct InternalLinkTypePhoneNumberConfirmationBuilder {
    inner: InternalLinkTypePhoneNumberConfirmation,
}

#[deprecated]
pub type RTDInternalLinkTypePhoneNumberConfirmationBuilder =
    InternalLinkTypePhoneNumberConfirmationBuilder;

impl InternalLinkTypePhoneNumberConfirmationBuilder {
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
    for InternalLinkTypePhoneNumberConfirmationBuilder
{
    fn as_ref(&self) -> &InternalLinkTypePhoneNumberConfirmation {
        &self.inner
    }
}

/// The link is a link to the Premium features screen of the application from which the user can subscribe to Telegram Premium. Call getPremiumFeatures with the given referrer to process the link
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypePremiumFeaturesBuilder {
        let mut inner = InternalLinkTypePremiumFeatures::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypePremiumFeaturesBuilder { inner }
    }

    pub fn referrer(&self) -> &String {
        &self.referrer
    }
}

#[doc(hidden)]
pub struct InternalLinkTypePremiumFeaturesBuilder {
    inner: InternalLinkTypePremiumFeatures,
}

#[deprecated]
pub type RTDInternalLinkTypePremiumFeaturesBuilder = InternalLinkTypePremiumFeaturesBuilder;

impl InternalLinkTypePremiumFeaturesBuilder {
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

impl AsRef<InternalLinkTypePremiumFeatures> for InternalLinkTypePremiumFeaturesBuilder {
    fn as_ref(&self) -> &InternalLinkTypePremiumFeatures {
        &self.inner
    }
}

/// The link is a link with a Telegram Premium gift code. Call checkPremiumGiftCode with the given code to process the link. If the code is valid and the user wants to apply it, then call applyPremiumGiftCode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypePremiumGiftCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The Telegram Premium gift code

    #[serde(default)]
    code: String,
}

impl RObject for InternalLinkTypePremiumGiftCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypePremiumGiftCode {}

impl InternalLinkTypePremiumGiftCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypePremiumGiftCodeBuilder {
        let mut inner = InternalLinkTypePremiumGiftCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypePremiumGiftCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct InternalLinkTypePremiumGiftCodeBuilder {
    inner: InternalLinkTypePremiumGiftCode,
}

#[deprecated]
pub type RTDInternalLinkTypePremiumGiftCodeBuilder = InternalLinkTypePremiumGiftCodeBuilder;

impl InternalLinkTypePremiumGiftCodeBuilder {
    pub fn build(&self) -> InternalLinkTypePremiumGiftCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypePremiumGiftCode> for InternalLinkTypePremiumGiftCode {
    fn as_ref(&self) -> &InternalLinkTypePremiumGiftCode {
        self
    }
}

impl AsRef<InternalLinkTypePremiumGiftCode> for InternalLinkTypePremiumGiftCodeBuilder {
    fn as_ref(&self) -> &InternalLinkTypePremiumGiftCode {
        &self.inner
    }
}

/// The link is a link to the privacy and security section of the app settings
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypePrivacyAndSecuritySettingsBuilder {
        let mut inner = InternalLinkTypePrivacyAndSecuritySettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypePrivacyAndSecuritySettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypePrivacyAndSecuritySettingsBuilder {
    inner: InternalLinkTypePrivacyAndSecuritySettings,
}

#[deprecated]
pub type RTDInternalLinkTypePrivacyAndSecuritySettingsBuilder =
    InternalLinkTypePrivacyAndSecuritySettingsBuilder;

impl InternalLinkTypePrivacyAndSecuritySettingsBuilder {
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
    for InternalLinkTypePrivacyAndSecuritySettingsBuilder
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
    /// Proxy server domain or IP address

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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeProxyBuilder {
        let mut inner = InternalLinkTypeProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeProxyBuilder { inner }
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
pub struct InternalLinkTypeProxyBuilder {
    inner: InternalLinkTypeProxy,
}

#[deprecated]
pub type RTDInternalLinkTypeProxyBuilder = InternalLinkTypeProxyBuilder;

impl InternalLinkTypeProxyBuilder {
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

impl AsRef<InternalLinkTypeProxy> for InternalLinkTypeProxyBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypePublicChatBuilder {
        let mut inner = InternalLinkTypePublicChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypePublicChatBuilder { inner }
    }

    pub fn chat_username(&self) -> &String {
        &self.chat_username
    }
}

#[doc(hidden)]
pub struct InternalLinkTypePublicChatBuilder {
    inner: InternalLinkTypePublicChat,
}

#[deprecated]
pub type RTDInternalLinkTypePublicChatBuilder = InternalLinkTypePublicChatBuilder;

impl InternalLinkTypePublicChatBuilder {
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

impl AsRef<InternalLinkTypePublicChat> for InternalLinkTypePublicChatBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeQrCodeAuthenticationBuilder {
        let mut inner = InternalLinkTypeQrCodeAuthentication::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeQrCodeAuthenticationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeQrCodeAuthenticationBuilder {
    inner: InternalLinkTypeQrCodeAuthentication,
}

#[deprecated]
pub type RTDInternalLinkTypeQrCodeAuthenticationBuilder =
    InternalLinkTypeQrCodeAuthenticationBuilder;

impl InternalLinkTypeQrCodeAuthenticationBuilder {
    pub fn build(&self) -> InternalLinkTypeQrCodeAuthentication {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeQrCodeAuthentication> for InternalLinkTypeQrCodeAuthentication {
    fn as_ref(&self) -> &InternalLinkTypeQrCodeAuthentication {
        self
    }
}

impl AsRef<InternalLinkTypeQrCodeAuthentication> for InternalLinkTypeQrCodeAuthenticationBuilder {
    fn as_ref(&self) -> &InternalLinkTypeQrCodeAuthentication {
        &self.inner
    }
}

/// The link forces restore of App Store purchases when opened. For official iOS application only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeRestorePurchases {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InternalLinkTypeRestorePurchases {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeRestorePurchases {}

impl InternalLinkTypeRestorePurchases {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeRestorePurchasesBuilder {
        let mut inner = InternalLinkTypeRestorePurchases::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeRestorePurchasesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeRestorePurchasesBuilder {
    inner: InternalLinkTypeRestorePurchases,
}

#[deprecated]
pub type RTDInternalLinkTypeRestorePurchasesBuilder = InternalLinkTypeRestorePurchasesBuilder;

impl InternalLinkTypeRestorePurchasesBuilder {
    pub fn build(&self) -> InternalLinkTypeRestorePurchases {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeRestorePurchases> for InternalLinkTypeRestorePurchases {
    fn as_ref(&self) -> &InternalLinkTypeRestorePurchases {
        self
    }
}

impl AsRef<InternalLinkTypeRestorePurchases> for InternalLinkTypeRestorePurchasesBuilder {
    fn as_ref(&self) -> &InternalLinkTypeRestorePurchases {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeSettingsBuilder {
        let mut inner = InternalLinkTypeSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeSettingsBuilder {
    inner: InternalLinkTypeSettings,
}

#[deprecated]
pub type RTDInternalLinkTypeSettingsBuilder = InternalLinkTypeSettingsBuilder;

impl InternalLinkTypeSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeSettings> for InternalLinkTypeSettings {
    fn as_ref(&self) -> &InternalLinkTypeSettings {
        self
    }
}

impl AsRef<InternalLinkTypeSettings> for InternalLinkTypeSettingsBuilder {
    fn as_ref(&self) -> &InternalLinkTypeSettings {
        &self.inner
    }
}

/// The link is a link to a bot, which can be installed to the side menu. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then, use getAttachmentMenuBot to receive information about the bot. If the bot isn't added to side menu, then show a disclaimer about Mini Apps being a third-party apps, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu. If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot. If the bot is added to side menu, then use getWebAppUrl with the given URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeSideMenuBot {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username of the bot

    #[serde(default)]
    bot_username: String,
    /// URL to be passed to getWebAppUrl

    #[serde(default)]
    url: String,
}

impl RObject for InternalLinkTypeSideMenuBot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeSideMenuBot {}

impl InternalLinkTypeSideMenuBot {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeSideMenuBotBuilder {
        let mut inner = InternalLinkTypeSideMenuBot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeSideMenuBotBuilder { inner }
    }

    pub fn bot_username(&self) -> &String {
        &self.bot_username
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeSideMenuBotBuilder {
    inner: InternalLinkTypeSideMenuBot,
}

#[deprecated]
pub type RTDInternalLinkTypeSideMenuBotBuilder = InternalLinkTypeSideMenuBotBuilder;

impl InternalLinkTypeSideMenuBotBuilder {
    pub fn build(&self) -> InternalLinkTypeSideMenuBot {
        self.inner.clone()
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

impl AsRef<InternalLinkTypeSideMenuBot> for InternalLinkTypeSideMenuBot {
    fn as_ref(&self) -> &InternalLinkTypeSideMenuBot {
        self
    }
}

impl AsRef<InternalLinkTypeSideMenuBot> for InternalLinkTypeSideMenuBotBuilder {
    fn as_ref(&self) -> &InternalLinkTypeSideMenuBot {
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
    /// True, if the sticker set is expected to contain custom emoji

    #[serde(default)]
    expect_custom_emoji: bool,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeStickerSetBuilder {
        let mut inner = InternalLinkTypeStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeStickerSetBuilder { inner }
    }

    pub fn sticker_set_name(&self) -> &String {
        &self.sticker_set_name
    }

    pub fn expect_custom_emoji(&self) -> bool {
        self.expect_custom_emoji
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeStickerSetBuilder {
    inner: InternalLinkTypeStickerSet,
}

#[deprecated]
pub type RTDInternalLinkTypeStickerSetBuilder = InternalLinkTypeStickerSetBuilder;

impl InternalLinkTypeStickerSetBuilder {
    pub fn build(&self) -> InternalLinkTypeStickerSet {
        self.inner.clone()
    }

    pub fn sticker_set_name<T: AsRef<str>>(&mut self, sticker_set_name: T) -> &mut Self {
        self.inner.sticker_set_name = sticker_set_name.as_ref().to_string();
        self
    }

    pub fn expect_custom_emoji(&mut self, expect_custom_emoji: bool) -> &mut Self {
        self.inner.expect_custom_emoji = expect_custom_emoji;
        self
    }
}

impl AsRef<InternalLinkTypeStickerSet> for InternalLinkTypeStickerSet {
    fn as_ref(&self) -> &InternalLinkTypeStickerSet {
        self
    }
}

impl AsRef<InternalLinkTypeStickerSet> for InternalLinkTypeStickerSetBuilder {
    fn as_ref(&self) -> &InternalLinkTypeStickerSet {
        &self.inner
    }
}

/// The link is a link to a story. Call searchPublicChat with the given sender username, then call getStory with the received chat identifier and the given story identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeStory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username of the sender of the story

    #[serde(default)]
    story_sender_username: String,
    /// Story identifier

    #[serde(default)]
    story_id: i32,
}

impl RObject for InternalLinkTypeStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeStory {}

impl InternalLinkTypeStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeStoryBuilder {
        let mut inner = InternalLinkTypeStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeStoryBuilder { inner }
    }

    pub fn story_sender_username(&self) -> &String {
        &self.story_sender_username
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeStoryBuilder {
    inner: InternalLinkTypeStory,
}

#[deprecated]
pub type RTDInternalLinkTypeStoryBuilder = InternalLinkTypeStoryBuilder;

impl InternalLinkTypeStoryBuilder {
    pub fn build(&self) -> InternalLinkTypeStory {
        self.inner.clone()
    }

    pub fn story_sender_username<T: AsRef<str>>(&mut self, story_sender_username: T) -> &mut Self {
        self.inner.story_sender_username = story_sender_username.as_ref().to_string();
        self
    }

    pub fn story_id(&mut self, story_id: i32) -> &mut Self {
        self.inner.story_id = story_id;
        self
    }
}

impl AsRef<InternalLinkTypeStory> for InternalLinkTypeStory {
    fn as_ref(&self) -> &InternalLinkTypeStory {
        self
    }
}

impl AsRef<InternalLinkTypeStory> for InternalLinkTypeStoryBuilder {
    fn as_ref(&self) -> &InternalLinkTypeStory {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeThemeBuilder {
        let mut inner = InternalLinkTypeTheme::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeThemeBuilder { inner }
    }

    pub fn theme_name(&self) -> &String {
        &self.theme_name
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeThemeBuilder {
    inner: InternalLinkTypeTheme,
}

#[deprecated]
pub type RTDInternalLinkTypeThemeBuilder = InternalLinkTypeThemeBuilder;

impl InternalLinkTypeThemeBuilder {
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

impl AsRef<InternalLinkTypeTheme> for InternalLinkTypeThemeBuilder {
    fn as_ref(&self) -> &InternalLinkTypeTheme {
        &self.inner
    }
}

/// The link is a link to the theme section of the app settings
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeThemeSettingsBuilder {
        let mut inner = InternalLinkTypeThemeSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeThemeSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeThemeSettingsBuilder {
    inner: InternalLinkTypeThemeSettings,
}

#[deprecated]
pub type RTDInternalLinkTypeThemeSettingsBuilder = InternalLinkTypeThemeSettingsBuilder;

impl InternalLinkTypeThemeSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeThemeSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeThemeSettings> for InternalLinkTypeThemeSettings {
    fn as_ref(&self) -> &InternalLinkTypeThemeSettings {
        self
    }
}

impl AsRef<InternalLinkTypeThemeSettings> for InternalLinkTypeThemeSettingsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeUnknownDeepLinkBuilder {
        let mut inner = InternalLinkTypeUnknownDeepLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeUnknownDeepLinkBuilder { inner }
    }

    pub fn link(&self) -> &String {
        &self.link
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeUnknownDeepLinkBuilder {
    inner: InternalLinkTypeUnknownDeepLink,
}

#[deprecated]
pub type RTDInternalLinkTypeUnknownDeepLinkBuilder = InternalLinkTypeUnknownDeepLinkBuilder;

impl InternalLinkTypeUnknownDeepLinkBuilder {
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

impl AsRef<InternalLinkTypeUnknownDeepLink> for InternalLinkTypeUnknownDeepLinkBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeUnsupportedProxyBuilder {
        let mut inner = InternalLinkTypeUnsupportedProxy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeUnsupportedProxyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeUnsupportedProxyBuilder {
    inner: InternalLinkTypeUnsupportedProxy,
}

#[deprecated]
pub type RTDInternalLinkTypeUnsupportedProxyBuilder = InternalLinkTypeUnsupportedProxyBuilder;

impl InternalLinkTypeUnsupportedProxyBuilder {
    pub fn build(&self) -> InternalLinkTypeUnsupportedProxy {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeUnsupportedProxy> for InternalLinkTypeUnsupportedProxy {
    fn as_ref(&self) -> &InternalLinkTypeUnsupportedProxy {
        self
    }
}

impl AsRef<InternalLinkTypeUnsupportedProxy> for InternalLinkTypeUnsupportedProxyBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeUserPhoneNumberBuilder {
        let mut inner = InternalLinkTypeUserPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeUserPhoneNumberBuilder { inner }
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeUserPhoneNumberBuilder {
    inner: InternalLinkTypeUserPhoneNumber,
}

#[deprecated]
pub type RTDInternalLinkTypeUserPhoneNumberBuilder = InternalLinkTypeUserPhoneNumberBuilder;

impl InternalLinkTypeUserPhoneNumberBuilder {
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

impl AsRef<InternalLinkTypeUserPhoneNumber> for InternalLinkTypeUserPhoneNumberBuilder {
    fn as_ref(&self) -> &InternalLinkTypeUserPhoneNumber {
        &self.inner
    }
}

/// The link is a link to a user by a temporary token. Call searchUserByToken with the given token to process the link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeUserToken {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The token

    #[serde(default)]
    token: String,
}

impl RObject for InternalLinkTypeUserToken {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeUserToken {}

impl InternalLinkTypeUserToken {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeUserTokenBuilder {
        let mut inner = InternalLinkTypeUserToken::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeUserTokenBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeUserTokenBuilder {
    inner: InternalLinkTypeUserToken,
}

#[deprecated]
pub type RTDInternalLinkTypeUserTokenBuilder = InternalLinkTypeUserTokenBuilder;

impl InternalLinkTypeUserTokenBuilder {
    pub fn build(&self) -> InternalLinkTypeUserToken {
        self.inner.clone()
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeUserToken> for InternalLinkTypeUserToken {
    fn as_ref(&self) -> &InternalLinkTypeUserToken {
        self
    }
}

impl AsRef<InternalLinkTypeUserToken> for InternalLinkTypeUserTokenBuilder {
    fn as_ref(&self) -> &InternalLinkTypeUserToken {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeVideoChatBuilder {
        let mut inner = InternalLinkTypeVideoChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeVideoChatBuilder { inner }
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
pub struct InternalLinkTypeVideoChatBuilder {
    inner: InternalLinkTypeVideoChat,
}

#[deprecated]
pub type RTDInternalLinkTypeVideoChatBuilder = InternalLinkTypeVideoChatBuilder;

impl InternalLinkTypeVideoChatBuilder {
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

impl AsRef<InternalLinkTypeVideoChat> for InternalLinkTypeVideoChatBuilder {
    fn as_ref(&self) -> &InternalLinkTypeVideoChat {
        &self.inner
    }
}

/// The link is a link to a Web App. Call searchPublicChat with the given bot username, check that the user is a bot, then call searchWebApp with the received bot and the given web_app_short_name. Process received foundWebApp by showing a confirmation dialog if needed. If the bot can be added to attachment or side menu, but isn't added yet, then show a disclaimer about Mini Apps being a third-party apps instead of the dialog and ask the user to accept their Terms of service. If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot. Then, call getWebAppLinkUrl and open the returned URL as a Web App
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InternalLinkTypeWebApp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username of the bot that owns the Web App

    #[serde(default)]
    bot_username: String,
    /// Short name of the Web App

    #[serde(default)]
    web_app_short_name: String,
    /// Start parameter to be passed to getWebAppLinkUrl

    #[serde(default)]
    start_parameter: String,
}

impl RObject for InternalLinkTypeWebApp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInternalLinkType for InternalLinkTypeWebApp {}

impl InternalLinkTypeWebApp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeWebAppBuilder {
        let mut inner = InternalLinkTypeWebApp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeWebAppBuilder { inner }
    }

    pub fn bot_username(&self) -> &String {
        &self.bot_username
    }

    pub fn web_app_short_name(&self) -> &String {
        &self.web_app_short_name
    }

    pub fn start_parameter(&self) -> &String {
        &self.start_parameter
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeWebAppBuilder {
    inner: InternalLinkTypeWebApp,
}

#[deprecated]
pub type RTDInternalLinkTypeWebAppBuilder = InternalLinkTypeWebAppBuilder;

impl InternalLinkTypeWebAppBuilder {
    pub fn build(&self) -> InternalLinkTypeWebApp {
        self.inner.clone()
    }

    pub fn bot_username<T: AsRef<str>>(&mut self, bot_username: T) -> &mut Self {
        self.inner.bot_username = bot_username.as_ref().to_string();
        self
    }

    pub fn web_app_short_name<T: AsRef<str>>(&mut self, web_app_short_name: T) -> &mut Self {
        self.inner.web_app_short_name = web_app_short_name.as_ref().to_string();
        self
    }

    pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
        self.inner.start_parameter = start_parameter.as_ref().to_string();
        self
    }
}

impl AsRef<InternalLinkTypeWebApp> for InternalLinkTypeWebApp {
    fn as_ref(&self) -> &InternalLinkTypeWebApp {
        self
    }
}

impl AsRef<InternalLinkTypeWebApp> for InternalLinkTypeWebAppBuilder {
    fn as_ref(&self) -> &InternalLinkTypeWebApp {
        &self.inner
    }
}
