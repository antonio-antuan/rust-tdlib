use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains a description of a custom keyboard and actions that can be done with it to quickly reply to bots
pub trait TDReplyMarkup: Debug + RObject {}

/// Contains a description of a custom keyboard and actions that can be done with it to quickly reply to bots
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReplyMarkup {
    #[doc(hidden)]
    _Default,
    /// Instructs application to force a reply to this message
    #[serde(rename = "replyMarkupForceReply")]
    ForceReply(ReplyMarkupForceReply),
    /// Contains an inline keyboard layout
    #[serde(rename = "replyMarkupInlineKeyboard")]
    InlineKeyboard(ReplyMarkupInlineKeyboard),
    /// Instructs application to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, UpdateChatReplyMarkup with message_id == 0 will be sent
    #[serde(rename = "replyMarkupRemoveKeyboard")]
    RemoveKeyboard(ReplyMarkupRemoveKeyboard),
    /// Contains a custom keyboard layout to quickly reply to bots
    #[serde(rename = "replyMarkupShowKeyboard")]
    ShowKeyboard(ReplyMarkupShowKeyboard),
}

impl Default for ReplyMarkup {
    fn default() -> Self {
        ReplyMarkup::_Default
    }
}

impl RObject for ReplyMarkup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ReplyMarkup::ForceReply(t) => t.extra(),
            ReplyMarkup::InlineKeyboard(t) => t.extra(),
            ReplyMarkup::RemoveKeyboard(t) => t.extra(),
            ReplyMarkup::ShowKeyboard(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ReplyMarkup::ForceReply(t) => t.client_id(),
            ReplyMarkup::InlineKeyboard(t) => t.client_id(),
            ReplyMarkup::RemoveKeyboard(t) => t.client_id(),
            ReplyMarkup::ShowKeyboard(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ReplyMarkup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ReplyMarkup::_Default)
    }
}

impl AsRef<ReplyMarkup> for ReplyMarkup {
    fn as_ref(&self) -> &ReplyMarkup {
        self
    }
}

/// Instructs application to force a reply to this message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyMarkupForceReply {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if a forced reply must automatically be shown to the current user. For outgoing messages, specify true to show the forced reply only for the mentioned users and for the target user of a reply

    #[serde(default)]
    is_personal: bool,
    /// If non-empty, the placeholder to be shown in the input field when the reply is active; 0-64 characters

    #[serde(default)]
    input_field_placeholder: String,
}

impl RObject for ReplyMarkupForceReply {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReplyMarkup for ReplyMarkupForceReply {}

impl ReplyMarkupForceReply {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReplyMarkupForceReplyBuilder {
        let mut inner = ReplyMarkupForceReply::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReplyMarkupForceReplyBuilder { inner }
    }

    pub fn is_personal(&self) -> bool {
        self.is_personal
    }

    pub fn input_field_placeholder(&self) -> &String {
        &self.input_field_placeholder
    }
}

#[doc(hidden)]
pub struct ReplyMarkupForceReplyBuilder {
    inner: ReplyMarkupForceReply,
}

#[deprecated]
pub type RTDReplyMarkupForceReplyBuilder = ReplyMarkupForceReplyBuilder;

impl ReplyMarkupForceReplyBuilder {
    pub fn build(&self) -> ReplyMarkupForceReply {
        self.inner.clone()
    }

    pub fn is_personal(&mut self, is_personal: bool) -> &mut Self {
        self.inner.is_personal = is_personal;
        self
    }

    pub fn input_field_placeholder<T: AsRef<str>>(
        &mut self,
        input_field_placeholder: T,
    ) -> &mut Self {
        self.inner.input_field_placeholder = input_field_placeholder.as_ref().to_string();
        self
    }
}

impl AsRef<ReplyMarkupForceReply> for ReplyMarkupForceReply {
    fn as_ref(&self) -> &ReplyMarkupForceReply {
        self
    }
}

impl AsRef<ReplyMarkupForceReply> for ReplyMarkupForceReplyBuilder {
    fn as_ref(&self) -> &ReplyMarkupForceReply {
        &self.inner
    }
}

/// Contains an inline keyboard layout
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyMarkupInlineKeyboard {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A list of rows of inline keyboard buttons

    #[serde(default)]
    rows: Vec<Vec<InlineKeyboardButton>>,
}

impl RObject for ReplyMarkupInlineKeyboard {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReplyMarkup for ReplyMarkupInlineKeyboard {}

impl ReplyMarkupInlineKeyboard {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReplyMarkupInlineKeyboardBuilder {
        let mut inner = ReplyMarkupInlineKeyboard::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReplyMarkupInlineKeyboardBuilder { inner }
    }

    pub fn rows(&self) -> &Vec<Vec<InlineKeyboardButton>> {
        &self.rows
    }
}

#[doc(hidden)]
pub struct ReplyMarkupInlineKeyboardBuilder {
    inner: ReplyMarkupInlineKeyboard,
}

#[deprecated]
pub type RTDReplyMarkupInlineKeyboardBuilder = ReplyMarkupInlineKeyboardBuilder;

impl ReplyMarkupInlineKeyboardBuilder {
    pub fn build(&self) -> ReplyMarkupInlineKeyboard {
        self.inner.clone()
    }

    pub fn rows(&mut self, rows: Vec<Vec<InlineKeyboardButton>>) -> &mut Self {
        self.inner.rows = rows;
        self
    }
}

impl AsRef<ReplyMarkupInlineKeyboard> for ReplyMarkupInlineKeyboard {
    fn as_ref(&self) -> &ReplyMarkupInlineKeyboard {
        self
    }
}

impl AsRef<ReplyMarkupInlineKeyboard> for ReplyMarkupInlineKeyboardBuilder {
    fn as_ref(&self) -> &ReplyMarkupInlineKeyboard {
        &self.inner
    }
}

/// Instructs application to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, UpdateChatReplyMarkup with message_id == 0 will be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyMarkupRemoveKeyboard {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the keyboard is removed only for the mentioned users or the target user of a reply

    #[serde(default)]
    is_personal: bool,
}

impl RObject for ReplyMarkupRemoveKeyboard {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReplyMarkup for ReplyMarkupRemoveKeyboard {}

impl ReplyMarkupRemoveKeyboard {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReplyMarkupRemoveKeyboardBuilder {
        let mut inner = ReplyMarkupRemoveKeyboard::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReplyMarkupRemoveKeyboardBuilder { inner }
    }

    pub fn is_personal(&self) -> bool {
        self.is_personal
    }
}

#[doc(hidden)]
pub struct ReplyMarkupRemoveKeyboardBuilder {
    inner: ReplyMarkupRemoveKeyboard,
}

#[deprecated]
pub type RTDReplyMarkupRemoveKeyboardBuilder = ReplyMarkupRemoveKeyboardBuilder;

impl ReplyMarkupRemoveKeyboardBuilder {
    pub fn build(&self) -> ReplyMarkupRemoveKeyboard {
        self.inner.clone()
    }

    pub fn is_personal(&mut self, is_personal: bool) -> &mut Self {
        self.inner.is_personal = is_personal;
        self
    }
}

impl AsRef<ReplyMarkupRemoveKeyboard> for ReplyMarkupRemoveKeyboard {
    fn as_ref(&self) -> &ReplyMarkupRemoveKeyboard {
        self
    }
}

impl AsRef<ReplyMarkupRemoveKeyboard> for ReplyMarkupRemoveKeyboardBuilder {
    fn as_ref(&self) -> &ReplyMarkupRemoveKeyboard {
        &self.inner
    }
}

/// Contains a custom keyboard layout to quickly reply to bots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyMarkupShowKeyboard {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A list of rows of bot keyboard buttons

    #[serde(default)]
    rows: Vec<Vec<KeyboardButton>>,
    /// True, if the application needs to resize the keyboard vertically

    #[serde(default)]
    resize_keyboard: bool,
    /// True, if the application needs to hide the keyboard after use

    #[serde(default)]
    one_time: bool,
    /// True, if the keyboard must automatically be shown to the current user. For outgoing messages, specify true to show the keyboard only for the mentioned users and for the target user of a reply

    #[serde(default)]
    is_personal: bool,
    /// If non-empty, the placeholder to be shown in the input field when the keyboard is active; 0-64 characters

    #[serde(default)]
    input_field_placeholder: String,
}

impl RObject for ReplyMarkupShowKeyboard {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReplyMarkup for ReplyMarkupShowKeyboard {}

impl ReplyMarkupShowKeyboard {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReplyMarkupShowKeyboardBuilder {
        let mut inner = ReplyMarkupShowKeyboard::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReplyMarkupShowKeyboardBuilder { inner }
    }

    pub fn rows(&self) -> &Vec<Vec<KeyboardButton>> {
        &self.rows
    }

    pub fn resize_keyboard(&self) -> bool {
        self.resize_keyboard
    }

    pub fn one_time(&self) -> bool {
        self.one_time
    }

    pub fn is_personal(&self) -> bool {
        self.is_personal
    }

    pub fn input_field_placeholder(&self) -> &String {
        &self.input_field_placeholder
    }
}

#[doc(hidden)]
pub struct ReplyMarkupShowKeyboardBuilder {
    inner: ReplyMarkupShowKeyboard,
}

#[deprecated]
pub type RTDReplyMarkupShowKeyboardBuilder = ReplyMarkupShowKeyboardBuilder;

impl ReplyMarkupShowKeyboardBuilder {
    pub fn build(&self) -> ReplyMarkupShowKeyboard {
        self.inner.clone()
    }

    pub fn rows(&mut self, rows: Vec<Vec<KeyboardButton>>) -> &mut Self {
        self.inner.rows = rows;
        self
    }

    pub fn resize_keyboard(&mut self, resize_keyboard: bool) -> &mut Self {
        self.inner.resize_keyboard = resize_keyboard;
        self
    }

    pub fn one_time(&mut self, one_time: bool) -> &mut Self {
        self.inner.one_time = one_time;
        self
    }

    pub fn is_personal(&mut self, is_personal: bool) -> &mut Self {
        self.inner.is_personal = is_personal;
        self
    }

    pub fn input_field_placeholder<T: AsRef<str>>(
        &mut self,
        input_field_placeholder: T,
    ) -> &mut Self {
        self.inner.input_field_placeholder = input_field_placeholder.as_ref().to_string();
        self
    }
}

impl AsRef<ReplyMarkupShowKeyboard> for ReplyMarkupShowKeyboard {
    fn as_ref(&self) -> &ReplyMarkupShowKeyboard {
        self
    }
}

impl AsRef<ReplyMarkupShowKeyboard> for ReplyMarkupShowKeyboardBuilder {
    fn as_ref(&self) -> &ReplyMarkupShowKeyboard {
        &self.inner
    }
}
