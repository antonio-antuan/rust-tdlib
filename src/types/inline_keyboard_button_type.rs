use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the type of an inline keyboard button
pub trait TDInlineKeyboardButtonType: Debug + RObject {}

/// Describes the type of an inline keyboard button
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineKeyboardButtonType {
    #[doc(hidden)]
    _Default,
    /// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice
    #[serde(rename = "inlineKeyboardButtonTypeBuy")]
    Buy(InlineKeyboardButtonTypeBuy),
    /// A button that sends a callback query to a bot
    #[serde(rename = "inlineKeyboardButtonTypeCallback")]
    Callback(InlineKeyboardButtonTypeCallback),
    /// A button with a game that sends a callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
    #[serde(rename = "inlineKeyboardButtonTypeCallbackGame")]
    CallbackGame(InlineKeyboardButtonTypeCallbackGame),
    /// A button that asks for password of the current user and then sends a callback query to a bot
    #[serde(rename = "inlineKeyboardButtonTypeCallbackWithPassword")]
    CallbackWithPassword(InlineKeyboardButtonTypeCallbackWithPassword),
    /// A button that opens a specified URL and automatically authorize the current user if allowed to do so
    #[serde(rename = "inlineKeyboardButtonTypeLoginUrl")]
    LoginUrl(InlineKeyboardButtonTypeLoginUrl),
    /// A button that forces an inline query to the bot to be inserted in the input field
    #[serde(rename = "inlineKeyboardButtonTypeSwitchInline")]
    SwitchInline(InlineKeyboardButtonTypeSwitchInline),
    /// A button that opens a specified URL
    #[serde(rename = "inlineKeyboardButtonTypeUrl")]
    Url(InlineKeyboardButtonTypeUrl),
    /// A button with a user reference to be handled in the same way as textEntityTypeMentionName entities
    #[serde(rename = "inlineKeyboardButtonTypeUser")]
    User(InlineKeyboardButtonTypeUser),
}

impl Default for InlineKeyboardButtonType {
    fn default() -> Self {
        InlineKeyboardButtonType::_Default
    }
}

impl RObject for InlineKeyboardButtonType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InlineKeyboardButtonType::Buy(t) => t.extra(),
            InlineKeyboardButtonType::Callback(t) => t.extra(),
            InlineKeyboardButtonType::CallbackGame(t) => t.extra(),
            InlineKeyboardButtonType::CallbackWithPassword(t) => t.extra(),
            InlineKeyboardButtonType::LoginUrl(t) => t.extra(),
            InlineKeyboardButtonType::SwitchInline(t) => t.extra(),
            InlineKeyboardButtonType::Url(t) => t.extra(),
            InlineKeyboardButtonType::User(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InlineKeyboardButtonType::Buy(t) => t.client_id(),
            InlineKeyboardButtonType::Callback(t) => t.client_id(),
            InlineKeyboardButtonType::CallbackGame(t) => t.client_id(),
            InlineKeyboardButtonType::CallbackWithPassword(t) => t.client_id(),
            InlineKeyboardButtonType::LoginUrl(t) => t.client_id(),
            InlineKeyboardButtonType::SwitchInline(t) => t.client_id(),
            InlineKeyboardButtonType::Url(t) => t.client_id(),
            InlineKeyboardButtonType::User(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InlineKeyboardButtonType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InlineKeyboardButtonType::_Default)
    }
}

impl AsRef<InlineKeyboardButtonType> for InlineKeyboardButtonType {
    fn as_ref(&self) -> &InlineKeyboardButtonType {
        self
    }
}

/// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeBuy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InlineKeyboardButtonTypeBuy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeBuy {}

impl InlineKeyboardButtonTypeBuy {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineKeyboardButtonTypeBuyBuilder {
        let mut inner = InlineKeyboardButtonTypeBuy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineKeyboardButtonTypeBuyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InlineKeyboardButtonTypeBuyBuilder {
    inner: InlineKeyboardButtonTypeBuy,
}

#[deprecated]
pub type RTDInlineKeyboardButtonTypeBuyBuilder = InlineKeyboardButtonTypeBuyBuilder;

impl InlineKeyboardButtonTypeBuyBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeBuy {
        self.inner.clone()
    }
}

impl AsRef<InlineKeyboardButtonTypeBuy> for InlineKeyboardButtonTypeBuy {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeBuy {
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeBuy> for InlineKeyboardButtonTypeBuyBuilder {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeBuy {
        &self.inner
    }
}

/// A button that sends a callback query to a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCallback {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Data to be sent to the bot via a callback query

    #[serde(default)]
    data: String,
}

impl RObject for InlineKeyboardButtonTypeCallback {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallback {}

impl InlineKeyboardButtonTypeCallback {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineKeyboardButtonTypeCallbackBuilder {
        let mut inner = InlineKeyboardButtonTypeCallback::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineKeyboardButtonTypeCallbackBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct InlineKeyboardButtonTypeCallbackBuilder {
    inner: InlineKeyboardButtonTypeCallback,
}

#[deprecated]
pub type RTDInlineKeyboardButtonTypeCallbackBuilder = InlineKeyboardButtonTypeCallbackBuilder;

impl InlineKeyboardButtonTypeCallbackBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeCallback {
        self.inner.clone()
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeCallback> for InlineKeyboardButtonTypeCallback {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeCallback {
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeCallback> for InlineKeyboardButtonTypeCallbackBuilder {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeCallback {
        &self.inner
    }
}

/// A button with a game that sends a callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCallbackGame {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InlineKeyboardButtonTypeCallbackGame {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallbackGame {}

impl InlineKeyboardButtonTypeCallbackGame {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineKeyboardButtonTypeCallbackGameBuilder {
        let mut inner = InlineKeyboardButtonTypeCallbackGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineKeyboardButtonTypeCallbackGameBuilder { inner }
    }
}

#[doc(hidden)]
pub struct InlineKeyboardButtonTypeCallbackGameBuilder {
    inner: InlineKeyboardButtonTypeCallbackGame,
}

#[deprecated]
pub type RTDInlineKeyboardButtonTypeCallbackGameBuilder =
    InlineKeyboardButtonTypeCallbackGameBuilder;

impl InlineKeyboardButtonTypeCallbackGameBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeCallbackGame {
        self.inner.clone()
    }
}

impl AsRef<InlineKeyboardButtonTypeCallbackGame> for InlineKeyboardButtonTypeCallbackGame {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackGame {
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeCallbackGame> for InlineKeyboardButtonTypeCallbackGameBuilder {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackGame {
        &self.inner
    }
}

/// A button that asks for password of the current user and then sends a callback query to a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCallbackWithPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Data to be sent to the bot via a callback query

    #[serde(default)]
    data: String,
}

impl RObject for InlineKeyboardButtonTypeCallbackWithPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallbackWithPassword {}

impl InlineKeyboardButtonTypeCallbackWithPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineKeyboardButtonTypeCallbackWithPasswordBuilder {
        let mut inner = InlineKeyboardButtonTypeCallbackWithPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineKeyboardButtonTypeCallbackWithPasswordBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct InlineKeyboardButtonTypeCallbackWithPasswordBuilder {
    inner: InlineKeyboardButtonTypeCallbackWithPassword,
}

#[deprecated]
pub type RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder =
    InlineKeyboardButtonTypeCallbackWithPasswordBuilder;

impl InlineKeyboardButtonTypeCallbackWithPasswordBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeCallbackWithPassword {
        self.inner.clone()
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeCallbackWithPassword>
    for InlineKeyboardButtonTypeCallbackWithPassword
{
    fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackWithPassword {
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeCallbackWithPassword>
    for InlineKeyboardButtonTypeCallbackWithPasswordBuilder
{
    fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackWithPassword {
        &self.inner
    }
}

/// A button that opens a specified URL and automatically authorize the current user if allowed to do so
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeLoginUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// An HTTP URL to open

    #[serde(default)]
    url: String,
    /// Unique button identifier

    #[serde(default)]
    id: i64,
    /// If non-empty, new text of the button in forwarded messages

    #[serde(default)]
    forward_text: String,
}

impl RObject for InlineKeyboardButtonTypeLoginUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeLoginUrl {}

impl InlineKeyboardButtonTypeLoginUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineKeyboardButtonTypeLoginUrlBuilder {
        let mut inner = InlineKeyboardButtonTypeLoginUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineKeyboardButtonTypeLoginUrlBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn forward_text(&self) -> &String {
        &self.forward_text
    }
}

#[doc(hidden)]
pub struct InlineKeyboardButtonTypeLoginUrlBuilder {
    inner: InlineKeyboardButtonTypeLoginUrl,
}

#[deprecated]
pub type RTDInlineKeyboardButtonTypeLoginUrlBuilder = InlineKeyboardButtonTypeLoginUrlBuilder;

impl InlineKeyboardButtonTypeLoginUrlBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeLoginUrl {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn forward_text<T: AsRef<str>>(&mut self, forward_text: T) -> &mut Self {
        self.inner.forward_text = forward_text.as_ref().to_string();
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeLoginUrl> for InlineKeyboardButtonTypeLoginUrl {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeLoginUrl {
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeLoginUrl> for InlineKeyboardButtonTypeLoginUrlBuilder {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeLoginUrl {
        &self.inner
    }
}

/// A button that forces an inline query to the bot to be inserted in the input field
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeSwitchInline {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Inline query to be sent to the bot

    #[serde(default)]
    query: String,
    /// True, if the inline query must be sent from the current chat

    #[serde(default)]
    in_current_chat: bool,
}

impl RObject for InlineKeyboardButtonTypeSwitchInline {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeSwitchInline {}

impl InlineKeyboardButtonTypeSwitchInline {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineKeyboardButtonTypeSwitchInlineBuilder {
        let mut inner = InlineKeyboardButtonTypeSwitchInline::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineKeyboardButtonTypeSwitchInlineBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn in_current_chat(&self) -> bool {
        self.in_current_chat
    }
}

#[doc(hidden)]
pub struct InlineKeyboardButtonTypeSwitchInlineBuilder {
    inner: InlineKeyboardButtonTypeSwitchInline,
}

#[deprecated]
pub type RTDInlineKeyboardButtonTypeSwitchInlineBuilder =
    InlineKeyboardButtonTypeSwitchInlineBuilder;

impl InlineKeyboardButtonTypeSwitchInlineBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeSwitchInline {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn in_current_chat(&mut self, in_current_chat: bool) -> &mut Self {
        self.inner.in_current_chat = in_current_chat;
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeSwitchInline> for InlineKeyboardButtonTypeSwitchInline {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeSwitchInline {
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeSwitchInline> for InlineKeyboardButtonTypeSwitchInlineBuilder {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeSwitchInline {
        &self.inner
    }
}

/// A button that opens a specified URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// HTTP or tg:// URL to open

    #[serde(default)]
    url: String,
}

impl RObject for InlineKeyboardButtonTypeUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeUrl {}

impl InlineKeyboardButtonTypeUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineKeyboardButtonTypeUrlBuilder {
        let mut inner = InlineKeyboardButtonTypeUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineKeyboardButtonTypeUrlBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct InlineKeyboardButtonTypeUrlBuilder {
    inner: InlineKeyboardButtonTypeUrl,
}

#[deprecated]
pub type RTDInlineKeyboardButtonTypeUrlBuilder = InlineKeyboardButtonTypeUrlBuilder;

impl InlineKeyboardButtonTypeUrlBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeUrl {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeUrl> for InlineKeyboardButtonTypeUrl {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeUrl {
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeUrl> for InlineKeyboardButtonTypeUrlBuilder {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeUrl {
        &self.inner
    }
}

/// A button with a user reference to be handled in the same way as textEntityTypeMentionName entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,
}

impl RObject for InlineKeyboardButtonTypeUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeUser {}

impl InlineKeyboardButtonTypeUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineKeyboardButtonTypeUserBuilder {
        let mut inner = InlineKeyboardButtonTypeUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineKeyboardButtonTypeUserBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct InlineKeyboardButtonTypeUserBuilder {
    inner: InlineKeyboardButtonTypeUser,
}

#[deprecated]
pub type RTDInlineKeyboardButtonTypeUserBuilder = InlineKeyboardButtonTypeUserBuilder;

impl InlineKeyboardButtonTypeUserBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeUser {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeUser> for InlineKeyboardButtonTypeUser {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeUser {
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeUser> for InlineKeyboardButtonTypeUserBuilder {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeUser {
        &self.inner
    }
}
