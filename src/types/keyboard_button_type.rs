use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a keyboard button type
pub trait TDKeyboardButtonType: Debug + RObject {}

/// Describes a keyboard button type
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum KeyboardButtonType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// A button that requests a chat to be shared by the current user; available only in private chats. Use the method shareChatWithBot to complete the request
    #[serde(rename = "keyboardButtonTypeRequestChat")]
    RequestChat(KeyboardButtonTypeRequestChat),
    /// A button that sends the user's location when pressed; available only in private chats
    #[serde(rename = "keyboardButtonTypeRequestLocation")]
    RequestLocation(KeyboardButtonTypeRequestLocation),
    /// A button that sends the user's phone number when pressed; available only in private chats
    #[serde(rename = "keyboardButtonTypeRequestPhoneNumber")]
    RequestPhoneNumber(KeyboardButtonTypeRequestPhoneNumber),
    /// A button that allows the user to create and send a poll when pressed; available only in private chats
    #[serde(rename = "keyboardButtonTypeRequestPoll")]
    RequestPoll(KeyboardButtonTypeRequestPoll),
    /// A button that requests a user to be shared by the current user; available only in private chats. Use the method shareUserWithBot to complete the request
    #[serde(rename = "keyboardButtonTypeRequestUser")]
    RequestUser(KeyboardButtonTypeRequestUser),
    /// A simple button, with text that must be sent when the button is pressed
    #[serde(rename = "keyboardButtonTypeText")]
    Text(KeyboardButtonTypeText),
    /// A button that opens a Web App by calling getWebAppUrl
    #[serde(rename = "keyboardButtonTypeWebApp")]
    WebApp(KeyboardButtonTypeWebApp),
}

impl RObject for KeyboardButtonType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            KeyboardButtonType::RequestChat(t) => t.extra(),
            KeyboardButtonType::RequestLocation(t) => t.extra(),
            KeyboardButtonType::RequestPhoneNumber(t) => t.extra(),
            KeyboardButtonType::RequestPoll(t) => t.extra(),
            KeyboardButtonType::RequestUser(t) => t.extra(),
            KeyboardButtonType::Text(t) => t.extra(),
            KeyboardButtonType::WebApp(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            KeyboardButtonType::RequestChat(t) => t.client_id(),
            KeyboardButtonType::RequestLocation(t) => t.client_id(),
            KeyboardButtonType::RequestPhoneNumber(t) => t.client_id(),
            KeyboardButtonType::RequestPoll(t) => t.client_id(),
            KeyboardButtonType::RequestUser(t) => t.client_id(),
            KeyboardButtonType::Text(t) => t.client_id(),
            KeyboardButtonType::WebApp(t) => t.client_id(),

            _ => None,
        }
    }
}

impl KeyboardButtonType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, KeyboardButtonType::_Default)
    }
}

impl AsRef<KeyboardButtonType> for KeyboardButtonType {
    fn as_ref(&self) -> &KeyboardButtonType {
        self
    }
}

/// A button that requests a chat to be shared by the current user; available only in private chats. Use the method shareChatWithBot to complete the request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique button identifier

    #[serde(default)]
    id: i32,
    /// True, if the chat must be a channel; otherwise, a basic group or a supergroup chat is shared

    #[serde(default)]
    chat_is_channel: bool,
    /// True, if the chat must or must not be a forum supergroup

    #[serde(default)]
    restrict_chat_is_forum: bool,
    /// True, if the chat must be a forum supergroup; otherwise, the chat must not be a forum supergroup. Ignored if restrict_chat_is_forum is false

    #[serde(default)]
    chat_is_forum: bool,
    /// True, if the chat must or must not have a username

    #[serde(default)]
    restrict_chat_has_username: bool,
    /// True, if the chat must have a username; otherwise, the chat must not have a username. Ignored if restrict_chat_has_username is false

    #[serde(default)]
    chat_has_username: bool,
    /// True, if the chat must be created by the current user

    #[serde(default)]
    chat_is_created: bool,
    /// Expected user administrator rights in the chat; may be null if they aren't restricted
    user_administrator_rights: Option<ChatAdministratorRights>,
    /// Expected bot administrator rights in the chat; may be null if they aren't restricted
    bot_administrator_rights: Option<ChatAdministratorRights>,
    /// True, if the bot must be a member of the chat; for basic group and supergroup chats only

    #[serde(default)]
    bot_is_member: bool,
}

impl RObject for KeyboardButtonTypeRequestChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestChat {}

impl KeyboardButtonTypeRequestChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> KeyboardButtonTypeRequestChatBuilder {
        let mut inner = KeyboardButtonTypeRequestChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        KeyboardButtonTypeRequestChatBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn chat_is_channel(&self) -> bool {
        self.chat_is_channel
    }

    pub fn restrict_chat_is_forum(&self) -> bool {
        self.restrict_chat_is_forum
    }

    pub fn chat_is_forum(&self) -> bool {
        self.chat_is_forum
    }

    pub fn restrict_chat_has_username(&self) -> bool {
        self.restrict_chat_has_username
    }

    pub fn chat_has_username(&self) -> bool {
        self.chat_has_username
    }

    pub fn chat_is_created(&self) -> bool {
        self.chat_is_created
    }

    pub fn user_administrator_rights(&self) -> &Option<ChatAdministratorRights> {
        &self.user_administrator_rights
    }

    pub fn bot_administrator_rights(&self) -> &Option<ChatAdministratorRights> {
        &self.bot_administrator_rights
    }

    pub fn bot_is_member(&self) -> bool {
        self.bot_is_member
    }
}

#[doc(hidden)]
pub struct KeyboardButtonTypeRequestChatBuilder {
    inner: KeyboardButtonTypeRequestChat,
}

#[deprecated]
pub type RTDKeyboardButtonTypeRequestChatBuilder = KeyboardButtonTypeRequestChatBuilder;

impl KeyboardButtonTypeRequestChatBuilder {
    pub fn build(&self) -> KeyboardButtonTypeRequestChat {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn chat_is_channel(&mut self, chat_is_channel: bool) -> &mut Self {
        self.inner.chat_is_channel = chat_is_channel;
        self
    }

    pub fn restrict_chat_is_forum(&mut self, restrict_chat_is_forum: bool) -> &mut Self {
        self.inner.restrict_chat_is_forum = restrict_chat_is_forum;
        self
    }

    pub fn chat_is_forum(&mut self, chat_is_forum: bool) -> &mut Self {
        self.inner.chat_is_forum = chat_is_forum;
        self
    }

    pub fn restrict_chat_has_username(&mut self, restrict_chat_has_username: bool) -> &mut Self {
        self.inner.restrict_chat_has_username = restrict_chat_has_username;
        self
    }

    pub fn chat_has_username(&mut self, chat_has_username: bool) -> &mut Self {
        self.inner.chat_has_username = chat_has_username;
        self
    }

    pub fn chat_is_created(&mut self, chat_is_created: bool) -> &mut Self {
        self.inner.chat_is_created = chat_is_created;
        self
    }

    pub fn user_administrator_rights<T: AsRef<ChatAdministratorRights>>(
        &mut self,
        user_administrator_rights: T,
    ) -> &mut Self {
        self.inner.user_administrator_rights = Some(user_administrator_rights.as_ref().clone());
        self
    }

    pub fn bot_administrator_rights<T: AsRef<ChatAdministratorRights>>(
        &mut self,
        bot_administrator_rights: T,
    ) -> &mut Self {
        self.inner.bot_administrator_rights = Some(bot_administrator_rights.as_ref().clone());
        self
    }

    pub fn bot_is_member(&mut self, bot_is_member: bool) -> &mut Self {
        self.inner.bot_is_member = bot_is_member;
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestChat> for KeyboardButtonTypeRequestChat {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestChat {
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestChat> for KeyboardButtonTypeRequestChatBuilder {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestChat {
        &self.inner
    }
}

/// A button that sends the user's location when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for KeyboardButtonTypeRequestLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestLocation {}

impl KeyboardButtonTypeRequestLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> KeyboardButtonTypeRequestLocationBuilder {
        let mut inner = KeyboardButtonTypeRequestLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        KeyboardButtonTypeRequestLocationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct KeyboardButtonTypeRequestLocationBuilder {
    inner: KeyboardButtonTypeRequestLocation,
}

#[deprecated]
pub type RTDKeyboardButtonTypeRequestLocationBuilder = KeyboardButtonTypeRequestLocationBuilder;

impl KeyboardButtonTypeRequestLocationBuilder {
    pub fn build(&self) -> KeyboardButtonTypeRequestLocation {
        self.inner.clone()
    }
}

impl AsRef<KeyboardButtonTypeRequestLocation> for KeyboardButtonTypeRequestLocation {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestLocation {
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestLocation> for KeyboardButtonTypeRequestLocationBuilder {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestLocation {
        &self.inner
    }
}

/// A button that sends the user's phone number when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for KeyboardButtonTypeRequestPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestPhoneNumber {}

impl KeyboardButtonTypeRequestPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> KeyboardButtonTypeRequestPhoneNumberBuilder {
        let mut inner = KeyboardButtonTypeRequestPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        KeyboardButtonTypeRequestPhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct KeyboardButtonTypeRequestPhoneNumberBuilder {
    inner: KeyboardButtonTypeRequestPhoneNumber,
}

#[deprecated]
pub type RTDKeyboardButtonTypeRequestPhoneNumberBuilder =
    KeyboardButtonTypeRequestPhoneNumberBuilder;

impl KeyboardButtonTypeRequestPhoneNumberBuilder {
    pub fn build(&self) -> KeyboardButtonTypeRequestPhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<KeyboardButtonTypeRequestPhoneNumber> for KeyboardButtonTypeRequestPhoneNumber {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestPhoneNumber {
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestPhoneNumber> for KeyboardButtonTypeRequestPhoneNumberBuilder {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestPhoneNumber {
        &self.inner
    }
}

/// A button that allows the user to create and send a poll when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestPoll {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If true, only regular polls must be allowed to create

    #[serde(default)]
    force_regular: bool,
    /// If true, only polls in quiz mode must be allowed to create

    #[serde(default)]
    force_quiz: bool,
}

impl RObject for KeyboardButtonTypeRequestPoll {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestPoll {}

impl KeyboardButtonTypeRequestPoll {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> KeyboardButtonTypeRequestPollBuilder {
        let mut inner = KeyboardButtonTypeRequestPoll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        KeyboardButtonTypeRequestPollBuilder { inner }
    }

    pub fn force_regular(&self) -> bool {
        self.force_regular
    }

    pub fn force_quiz(&self) -> bool {
        self.force_quiz
    }
}

#[doc(hidden)]
pub struct KeyboardButtonTypeRequestPollBuilder {
    inner: KeyboardButtonTypeRequestPoll,
}

#[deprecated]
pub type RTDKeyboardButtonTypeRequestPollBuilder = KeyboardButtonTypeRequestPollBuilder;

impl KeyboardButtonTypeRequestPollBuilder {
    pub fn build(&self) -> KeyboardButtonTypeRequestPoll {
        self.inner.clone()
    }

    pub fn force_regular(&mut self, force_regular: bool) -> &mut Self {
        self.inner.force_regular = force_regular;
        self
    }

    pub fn force_quiz(&mut self, force_quiz: bool) -> &mut Self {
        self.inner.force_quiz = force_quiz;
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestPoll> for KeyboardButtonTypeRequestPoll {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestPoll {
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestPoll> for KeyboardButtonTypeRequestPollBuilder {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestPoll {
        &self.inner
    }
}

/// A button that requests a user to be shared by the current user; available only in private chats. Use the method shareUserWithBot to complete the request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique button identifier

    #[serde(default)]
    id: i32,
    /// True, if the shared user must or must not be a bot

    #[serde(default)]
    restrict_user_is_bot: bool,
    /// True, if the shared user must be a bot; otherwise, the shared user must no be a bot. Ignored if restrict_user_is_bot is false

    #[serde(default)]
    user_is_bot: bool,
    /// True, if the shared user must or must not be a Telegram Premium user

    #[serde(default)]
    restrict_user_is_premium: bool,
    /// True, if the shared user must be a Telegram Premium user; otherwise, the shared user must no be a Telegram Premium user. Ignored if restrict_user_is_premium is false

    #[serde(default)]
    user_is_premium: bool,
}

impl RObject for KeyboardButtonTypeRequestUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestUser {}

impl KeyboardButtonTypeRequestUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> KeyboardButtonTypeRequestUserBuilder {
        let mut inner = KeyboardButtonTypeRequestUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        KeyboardButtonTypeRequestUserBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn restrict_user_is_bot(&self) -> bool {
        self.restrict_user_is_bot
    }

    pub fn user_is_bot(&self) -> bool {
        self.user_is_bot
    }

    pub fn restrict_user_is_premium(&self) -> bool {
        self.restrict_user_is_premium
    }

    pub fn user_is_premium(&self) -> bool {
        self.user_is_premium
    }
}

#[doc(hidden)]
pub struct KeyboardButtonTypeRequestUserBuilder {
    inner: KeyboardButtonTypeRequestUser,
}

#[deprecated]
pub type RTDKeyboardButtonTypeRequestUserBuilder = KeyboardButtonTypeRequestUserBuilder;

impl KeyboardButtonTypeRequestUserBuilder {
    pub fn build(&self) -> KeyboardButtonTypeRequestUser {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn restrict_user_is_bot(&mut self, restrict_user_is_bot: bool) -> &mut Self {
        self.inner.restrict_user_is_bot = restrict_user_is_bot;
        self
    }

    pub fn user_is_bot(&mut self, user_is_bot: bool) -> &mut Self {
        self.inner.user_is_bot = user_is_bot;
        self
    }

    pub fn restrict_user_is_premium(&mut self, restrict_user_is_premium: bool) -> &mut Self {
        self.inner.restrict_user_is_premium = restrict_user_is_premium;
        self
    }

    pub fn user_is_premium(&mut self, user_is_premium: bool) -> &mut Self {
        self.inner.user_is_premium = user_is_premium;
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestUser> for KeyboardButtonTypeRequestUser {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestUser {
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestUser> for KeyboardButtonTypeRequestUserBuilder {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestUser {
        &self.inner
    }
}

/// A simple button, with text that must be sent when the button is pressed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for KeyboardButtonTypeText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeText {}

impl KeyboardButtonTypeText {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> KeyboardButtonTypeTextBuilder {
        let mut inner = KeyboardButtonTypeText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        KeyboardButtonTypeTextBuilder { inner }
    }
}

#[doc(hidden)]
pub struct KeyboardButtonTypeTextBuilder {
    inner: KeyboardButtonTypeText,
}

#[deprecated]
pub type RTDKeyboardButtonTypeTextBuilder = KeyboardButtonTypeTextBuilder;

impl KeyboardButtonTypeTextBuilder {
    pub fn build(&self) -> KeyboardButtonTypeText {
        self.inner.clone()
    }
}

impl AsRef<KeyboardButtonTypeText> for KeyboardButtonTypeText {
    fn as_ref(&self) -> &KeyboardButtonTypeText {
        self
    }
}

impl AsRef<KeyboardButtonTypeText> for KeyboardButtonTypeTextBuilder {
    fn as_ref(&self) -> &KeyboardButtonTypeText {
        &self.inner
    }
}

/// A button that opens a Web App by calling getWebAppUrl
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeWebApp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// An HTTP URL to pass to getWebAppUrl

    #[serde(default)]
    url: String,
}

impl RObject for KeyboardButtonTypeWebApp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeWebApp {}

impl KeyboardButtonTypeWebApp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> KeyboardButtonTypeWebAppBuilder {
        let mut inner = KeyboardButtonTypeWebApp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        KeyboardButtonTypeWebAppBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct KeyboardButtonTypeWebAppBuilder {
    inner: KeyboardButtonTypeWebApp,
}

#[deprecated]
pub type RTDKeyboardButtonTypeWebAppBuilder = KeyboardButtonTypeWebAppBuilder;

impl KeyboardButtonTypeWebAppBuilder {
    pub fn build(&self) -> KeyboardButtonTypeWebApp {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<KeyboardButtonTypeWebApp> for KeyboardButtonTypeWebApp {
    fn as_ref(&self) -> &KeyboardButtonTypeWebApp {
        self
    }
}

impl AsRef<KeyboardButtonTypeWebApp> for KeyboardButtonTypeWebAppBuilder {
    fn as_ref(&self) -> &KeyboardButtonTypeWebApp {
        &self.inner
    }
}
