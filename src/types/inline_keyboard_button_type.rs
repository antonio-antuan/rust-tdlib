use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the type of an inline keyboard button
pub trait TDInlineKeyboardButtonType: Debug + RObject {}

/// Describes the type of an inline keyboard button
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InlineKeyboardButtonType {
    #[doc(hidden)]
    _Default(()),
    /// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice
    Buy(InlineKeyboardButtonTypeBuy),
    /// A button that sends a callback query to a bot
    Callback(InlineKeyboardButtonTypeCallback),
    /// A button with a game that sends a callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
    CallbackGame(InlineKeyboardButtonTypeCallbackGame),
    /// A button that asks for password of the current user and then sends a callback query to a bot
    CallbackWithPassword(InlineKeyboardButtonTypeCallbackWithPassword),
    /// A button that opens a specified URL and automatically logs in in current user if they allowed to do that
    LoginUrl(InlineKeyboardButtonTypeLoginUrl),
    /// A button that forces an inline query to the bot to be inserted in the input field
    SwitchInline(InlineKeyboardButtonTypeSwitchInline),
    /// A button that opens a specified URL
    Url(InlineKeyboardButtonTypeUrl),
}

impl Default for InlineKeyboardButtonType {
    fn default() -> Self {
        InlineKeyboardButtonType::_Default(())
    }
}

impl<'de> Deserialize<'de> for InlineKeyboardButtonType {
    fn deserialize<D>(deserializer: D) -> Result<InlineKeyboardButtonType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          InlineKeyboardButtonType,
          (inlineKeyboardButtonTypeBuy, Buy);
          (inlineKeyboardButtonTypeCallback, Callback);
          (inlineKeyboardButtonTypeCallbackGame, CallbackGame);
          (inlineKeyboardButtonTypeCallbackWithPassword, CallbackWithPassword);
          (inlineKeyboardButtonTypeLoginUrl, LoginUrl);
          (inlineKeyboardButtonTypeSwitchInline, SwitchInline);
          (inlineKeyboardButtonTypeUrl, Url);

        )(deserializer)
    }
}

impl RObject for InlineKeyboardButtonType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            InlineKeyboardButtonType::Buy(t) => t.td_name(),
            InlineKeyboardButtonType::Callback(t) => t.td_name(),
            InlineKeyboardButtonType::CallbackGame(t) => t.td_name(),
            InlineKeyboardButtonType::CallbackWithPassword(t) => t.td_name(),
            InlineKeyboardButtonType::LoginUrl(t) => t.td_name(),
            InlineKeyboardButtonType::SwitchInline(t) => t.td_name(),
            InlineKeyboardButtonType::Url(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
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
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
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
        matches!(self, InlineKeyboardButtonType::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InlineKeyboardButtonTypeBuy {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inlineKeyboardButtonTypeBuy"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeBuy {}

impl InlineKeyboardButtonTypeBuy {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeBuyBuilder {
        let mut inner = InlineKeyboardButtonTypeBuy::default();
        inner.td_name = "inlineKeyboardButtonTypeBuy".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "inlineKeyboardButtonTypeCallback"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallback {}

impl InlineKeyboardButtonTypeCallback {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackBuilder {
        let mut inner = InlineKeyboardButtonTypeCallback::default();
        inner.td_name = "inlineKeyboardButtonTypeCallback".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for InlineKeyboardButtonTypeCallbackGame {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inlineKeyboardButtonTypeCallbackGame"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallbackGame {}

impl InlineKeyboardButtonTypeCallbackGame {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackGameBuilder {
        let mut inner = InlineKeyboardButtonTypeCallbackGame::default();
        inner.td_name = "inlineKeyboardButtonTypeCallbackGame".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "inlineKeyboardButtonTypeCallbackWithPassword"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallbackWithPassword {}

impl InlineKeyboardButtonTypeCallbackWithPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackWithPasswordBuilder {
        let mut inner = InlineKeyboardButtonTypeCallbackWithPassword::default();
        inner.td_name = "inlineKeyboardButtonTypeCallbackWithPassword".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "inlineKeyboardButtonTypeLoginUrl"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeLoginUrl {}

impl InlineKeyboardButtonTypeLoginUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeLoginUrlBuilder {
        let mut inner = InlineKeyboardButtonTypeLoginUrl::default();
        inner.td_name = "inlineKeyboardButtonTypeLoginUrl".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "inlineKeyboardButtonTypeSwitchInline"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeSwitchInline {}

impl InlineKeyboardButtonTypeSwitchInline {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
        let mut inner = InlineKeyboardButtonTypeSwitchInline::default();
        inner.td_name = "inlineKeyboardButtonTypeSwitchInline".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "inlineKeyboardButtonTypeUrl"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeUrl {}

impl InlineKeyboardButtonTypeUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonTypeUrlBuilder {
        let mut inner = InlineKeyboardButtonTypeUrl::default();
        inner.td_name = "inlineKeyboardButtonTypeUrl".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
