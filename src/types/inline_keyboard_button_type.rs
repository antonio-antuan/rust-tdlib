use crate::errors::*;
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
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeBuy",
        deserialize = "inlineKeyboardButtonTypeBuy"
    ))]
    Buy(InlineKeyboardButtonTypeBuy),
    /// A button that sends a callback query to a bot
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeCallback",
        deserialize = "inlineKeyboardButtonTypeCallback"
    ))]
    Callback(InlineKeyboardButtonTypeCallback),
    /// A button with a game that sends a callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeCallbackGame",
        deserialize = "inlineKeyboardButtonTypeCallbackGame"
    ))]
    CallbackGame(InlineKeyboardButtonTypeCallbackGame),
    /// A button that asks for password of the current user and then sends a callback query to a bot
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeCallbackWithPassword",
        deserialize = "inlineKeyboardButtonTypeCallbackWithPassword"
    ))]
    CallbackWithPassword(InlineKeyboardButtonTypeCallbackWithPassword),
    /// A button that opens a specified URL and automatically logs in in current user if they allowed to do that
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeLoginUrl",
        deserialize = "inlineKeyboardButtonTypeLoginUrl"
    ))]
    LoginUrl(InlineKeyboardButtonTypeLoginUrl),
    /// A button that forces an inline query to the bot to be inserted in the input field
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeSwitchInline",
        deserialize = "inlineKeyboardButtonTypeSwitchInline"
    ))]
    SwitchInline(InlineKeyboardButtonTypeSwitchInline),
    /// A button that opens a specified URL
    #[serde(rename(
        serialize = "inlineKeyboardButtonTypeUrl",
        deserialize = "inlineKeyboardButtonTypeUrl"
    ))]
    Url(InlineKeyboardButtonTypeUrl),
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

            _ => None,
        }
    }
}

impl InlineKeyboardButtonType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeBuyBuilder {
        let mut inner = InlineKeyboardButtonTypeBuy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineKeyboardButtonTypeBuyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeBuyBuilder {
    inner: InlineKeyboardButtonTypeBuy,
}

impl RTDInlineKeyboardButtonTypeBuyBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeBuy {
        self.inner.clone()
    }
}

impl AsRef<InlineKeyboardButtonTypeBuy> for InlineKeyboardButtonTypeBuy {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeBuy {
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeBuy> for RTDInlineKeyboardButtonTypeBuyBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackBuilder {
        let mut inner = InlineKeyboardButtonTypeCallback::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineKeyboardButtonTypeCallbackBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeCallbackBuilder {
    inner: InlineKeyboardButtonTypeCallback,
}

impl RTDInlineKeyboardButtonTypeCallbackBuilder {
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

impl AsRef<InlineKeyboardButtonTypeCallback> for RTDInlineKeyboardButtonTypeCallbackBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackGameBuilder {
        let mut inner = InlineKeyboardButtonTypeCallbackGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineKeyboardButtonTypeCallbackGameBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeCallbackGameBuilder {
    inner: InlineKeyboardButtonTypeCallbackGame,
}

impl RTDInlineKeyboardButtonTypeCallbackGameBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeCallbackGame {
        self.inner.clone()
    }
}

impl AsRef<InlineKeyboardButtonTypeCallbackGame> for InlineKeyboardButtonTypeCallbackGame {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackGame {
        self
    }
}

impl AsRef<InlineKeyboardButtonTypeCallbackGame>
    for RTDInlineKeyboardButtonTypeCallbackGameBuilder
{
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder {
        let mut inner = InlineKeyboardButtonTypeCallbackWithPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder {
    inner: InlineKeyboardButtonTypeCallbackWithPassword,
}

impl RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder {
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
    for RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder
{
    fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackWithPassword {
        &self.inner
    }
}

/// A button that opens a specified URL and automatically logs in in current user if they allowed to do that
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeLoginUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// An HTTP URL to open
    url: String,
    /// Unique button identifier
    id: i32,
    /// If non-empty, new text of the button in forwarded messages
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeLoginUrlBuilder {
        let mut inner = InlineKeyboardButtonTypeLoginUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineKeyboardButtonTypeLoginUrlBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn forward_text(&self) -> &String {
        &self.forward_text
    }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeLoginUrlBuilder {
    inner: InlineKeyboardButtonTypeLoginUrl,
}

impl RTDInlineKeyboardButtonTypeLoginUrlBuilder {
    pub fn build(&self) -> InlineKeyboardButtonTypeLoginUrl {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
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

impl AsRef<InlineKeyboardButtonTypeLoginUrl> for RTDInlineKeyboardButtonTypeLoginUrlBuilder {
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
    query: String,
    /// True, if the inline query should be sent from the current chat
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
        let mut inner = InlineKeyboardButtonTypeSwitchInline::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineKeyboardButtonTypeSwitchInlineBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn in_current_chat(&self) -> bool {
        self.in_current_chat
    }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
    inner: InlineKeyboardButtonTypeSwitchInline,
}

impl RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
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

impl AsRef<InlineKeyboardButtonTypeSwitchInline>
    for RTDInlineKeyboardButtonTypeSwitchInlineBuilder
{
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeUrlBuilder {
        let mut inner = InlineKeyboardButtonTypeUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineKeyboardButtonTypeUrlBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeUrlBuilder {
    inner: InlineKeyboardButtonTypeUrl,
}

impl RTDInlineKeyboardButtonTypeUrlBuilder {
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

impl AsRef<InlineKeyboardButtonTypeUrl> for RTDInlineKeyboardButtonTypeUrlBuilder {
    fn as_ref(&self) -> &InlineKeyboardButtonTypeUrl {
        &self.inner
    }
}
