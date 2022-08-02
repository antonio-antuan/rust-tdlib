use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes an internal https://t.me or tg: link, which must be processed by the app in a special way
pub trait TDInternalLinkType: Debug + RObject {}

/// Describes an internal https://t.me or tg: link, which must be processed by the app in a special way
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InternalLinkType {
    #[doc(hidden)]
    _Default,
    /// Returns information about the type of an internal link. Returns a 404 error if the link is not internal. Can be called before authorization
    #[serde(rename = "getInternalLinkType")]
    GetInternalLinkType(GetInternalLinkType),
    /// The link is a link to the active sessions section of the app. Use getActiveSessions to handle the link
    #[serde(rename = "internalLinkTypeActiveSessions")]
    ActiveSessions(InternalLinkTypeActiveSessions),
    /// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
    #[serde(rename = "internalLinkTypeAuthenticationCode")]
    AuthenticationCode(InternalLinkTypeAuthenticationCode),
    /// The link is a link to a background. Call searchBackground with the given background name to process the link
    #[serde(rename = "internalLinkTypeBackground")]
    Background(InternalLinkTypeBackground),
    /// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot, and then call sendBotStartMessage with the given start parameter after the button is pressed
    #[serde(rename = "internalLinkTypeBotStart")]
    BotStart(InternalLinkTypeBotStart),
    /// The link is a link to a Telegram bot, which is supposed to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups, ask the current user to select a group to add the bot to, and then call sendBotStartMessage with the given start parameter and the chosen group chat. Bots can be added to a public group only by administrators of the group
    #[serde(rename = "internalLinkTypeBotStartInGroup")]
    BotStartInGroup(InternalLinkTypeBotStartInGroup),
    /// The link is a link to the change phone number section of the app
    #[serde(rename = "internalLinkTypeChangePhoneNumber")]
    ChangePhoneNumber(InternalLinkTypeChangePhoneNumber),
    /// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link
    #[serde(rename = "internalLinkTypeChatInvite")]
    ChatInvite(InternalLinkTypeChatInvite),
    /// The link is a link to the filter settings section of the app
    #[serde(rename = "internalLinkTypeFilterSettings")]
    FilterSettings(InternalLinkTypeFilterSettings),
    /// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot, ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
    #[serde(rename = "internalLinkTypeGame")]
    Game(InternalLinkTypeGame),
    /// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link
    #[serde(rename = "internalLinkTypeLanguagePack")]
    LanguagePack(InternalLinkTypeLanguagePack),
    /// The link is a link to a Telegram message. Call getMessageLinkInfo with the given URL to process the link
    #[serde(rename = "internalLinkTypeMessage")]
    Message(InternalLinkTypeMessage),
    /// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
    #[serde(rename = "internalLinkTypeMessageDraft")]
    MessageDraft(InternalLinkTypeMessageDraft),
    /// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the app, otherwise ignore it
    #[serde(rename = "internalLinkTypePassportDataRequest")]
    PassportDataRequest(InternalLinkTypePassportDataRequest),
    /// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberConfirmationCode with the given hash and phone number to process the link
    #[serde(rename = "internalLinkTypePhoneNumberConfirmation")]
    PhoneNumberConfirmation(InternalLinkTypePhoneNumberConfirmation),
    /// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
    #[serde(rename = "internalLinkTypeProxy")]
    Proxy(InternalLinkTypeProxy),
    /// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link
    #[serde(rename = "internalLinkTypePublicChat")]
    PublicChat(InternalLinkTypePublicChat),
    /// The link can be used to login the current user on another device, but it must be scanned from QR-code using in-app camera. An alert similar to "This code can be used to allow someone to log in to your Telegram account. To confirm Telegram login, please go to Settings > Devices > Scan QR and scan the code" needs to be shown
    #[serde(rename = "internalLinkTypeQrCodeAuthentication")]
    QrCodeAuthentication(InternalLinkTypeQrCodeAuthentication),
    /// The link is a link to app settings
    #[serde(rename = "internalLinkTypeSettings")]
    Settings(InternalLinkTypeSettings),
    /// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set
    #[serde(rename = "internalLinkTypeStickerSet")]
    StickerSet(InternalLinkTypeStickerSet),
    /// The link is a link to a theme. TDLib has no theme support yet
    #[serde(rename = "internalLinkTypeTheme")]
    Theme(InternalLinkTypeTheme),
    /// The link is a link to the theme settings section of the app
    #[serde(rename = "internalLinkTypeThemeSettings")]
    ThemeSettings(InternalLinkTypeThemeSettings),
    /// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
    #[serde(rename = "internalLinkTypeUnknownDeepLink")]
    UnknownDeepLink(InternalLinkTypeUnknownDeepLink),
    /// The link is a link to an unsupported proxy. An alert can be shown to the user
    #[serde(rename = "internalLinkTypeUnsupportedProxy")]
    UnsupportedProxy(InternalLinkTypeUnsupportedProxy),
    /// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinGoupCall with the given invite hash to process the link
    #[serde(rename = "internalLinkTypeVideoChat")]
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
            InternalLinkType::AuthenticationCode(t) => t.extra(),
            InternalLinkType::Background(t) => t.extra(),
            InternalLinkType::BotStart(t) => t.extra(),
            InternalLinkType::BotStartInGroup(t) => t.extra(),
            InternalLinkType::ChangePhoneNumber(t) => t.extra(),
            InternalLinkType::ChatInvite(t) => t.extra(),
            InternalLinkType::FilterSettings(t) => t.extra(),
            InternalLinkType::Game(t) => t.extra(),
            InternalLinkType::LanguagePack(t) => t.extra(),
            InternalLinkType::Message(t) => t.extra(),
            InternalLinkType::MessageDraft(t) => t.extra(),
            InternalLinkType::PassportDataRequest(t) => t.extra(),
            InternalLinkType::PhoneNumberConfirmation(t) => t.extra(),
            InternalLinkType::Proxy(t) => t.extra(),
            InternalLinkType::PublicChat(t) => t.extra(),
            InternalLinkType::QrCodeAuthentication(t) => t.extra(),
            InternalLinkType::Settings(t) => t.extra(),
            InternalLinkType::StickerSet(t) => t.extra(),
            InternalLinkType::Theme(t) => t.extra(),
            InternalLinkType::ThemeSettings(t) => t.extra(),
            InternalLinkType::UnknownDeepLink(t) => t.extra(),
            InternalLinkType::UnsupportedProxy(t) => t.extra(),
            InternalLinkType::VideoChat(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InternalLinkType::GetInternalLinkType(t) => t.client_id(),
            InternalLinkType::ActiveSessions(t) => t.client_id(),
            InternalLinkType::AuthenticationCode(t) => t.client_id(),
            InternalLinkType::Background(t) => t.client_id(),
            InternalLinkType::BotStart(t) => t.client_id(),
            InternalLinkType::BotStartInGroup(t) => t.client_id(),
            InternalLinkType::ChangePhoneNumber(t) => t.client_id(),
            InternalLinkType::ChatInvite(t) => t.client_id(),
            InternalLinkType::FilterSettings(t) => t.client_id(),
            InternalLinkType::Game(t) => t.client_id(),
            InternalLinkType::LanguagePack(t) => t.client_id(),
            InternalLinkType::Message(t) => t.client_id(),
            InternalLinkType::MessageDraft(t) => t.client_id(),
            InternalLinkType::PassportDataRequest(t) => t.client_id(),
            InternalLinkType::PhoneNumberConfirmation(t) => t.client_id(),
            InternalLinkType::Proxy(t) => t.client_id(),
            InternalLinkType::PublicChat(t) => t.client_id(),
            InternalLinkType::QrCodeAuthentication(t) => t.client_id(),
            InternalLinkType::Settings(t) => t.client_id(),
            InternalLinkType::StickerSet(t) => t.client_id(),
            InternalLinkType::Theme(t) => t.client_id(),
            InternalLinkType::ThemeSettings(t) => t.client_id(),
            InternalLinkType::UnknownDeepLink(t) => t.client_id(),
            InternalLinkType::UnsupportedProxy(t) => t.client_id(),
            InternalLinkType::VideoChat(t) => t.client_id(),

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

/// The link is a link to the active sessions section of the app. Use getActiveSessions to handle the link
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

/// The link is a link to a Telegram bot, which is supposed to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups, ask the current user to select a group to add the bot to, and then call sendBotStartMessage with the given start parameter and the chosen group chat. Bots can be added to a public group only by administrators of the group
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InternalLinkTypeFilterSettingsBuilder {
        let mut inner = InternalLinkTypeFilterSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InternalLinkTypeFilterSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InternalLinkTypeFilterSettingsBuilder {
    inner: InternalLinkTypeFilterSettings,
}

#[deprecated]
pub type RTDInternalLinkTypeFilterSettingsBuilder = InternalLinkTypeFilterSettingsBuilder;

impl InternalLinkTypeFilterSettingsBuilder {
    pub fn build(&self) -> InternalLinkTypeFilterSettings {
        self.inner.clone()
    }
}

impl AsRef<InternalLinkTypeFilterSettings> for InternalLinkTypeFilterSettings {
    fn as_ref(&self) -> &InternalLinkTypeFilterSettings {
        self
    }
}

impl AsRef<InternalLinkTypeFilterSettings> for InternalLinkTypeFilterSettingsBuilder {
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

/// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the app, otherwise ignore it
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

/// The link is a link to app settings
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

/// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinGoupCall with the given invite hash to process the link
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
