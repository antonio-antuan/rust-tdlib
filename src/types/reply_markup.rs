use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains a description of a custom keyboard and actions that can be done with it to quickly reply to bots
pub trait TDReplyMarkup: Debug + RObject {}

/// Contains a description of a custom keyboard and actions that can be done with it to quickly reply to bots
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    #[doc(hidden)]
    _Default(()),
    /// Instructs application to force a reply to this message
    ForceReply(ReplyMarkupForceReply),
    /// Contains an inline keyboard layout
    InlineKeyboard(ReplyMarkupInlineKeyboard),
    /// Instructs application to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, UpdateChatReplyMarkup with message_id == 0 will be sent
    RemoveKeyboard(ReplyMarkupRemoveKeyboard),
    /// Contains a custom keyboard layout to quickly reply to bots
    ShowKeyboard(ReplyMarkupShowKeyboard),
}

impl Default for ReplyMarkup {
    fn default() -> Self {
        ReplyMarkup::_Default(())
    }
}

impl<'de> Deserialize<'de> for ReplyMarkup {
    fn deserialize<D>(deserializer: D) -> Result<ReplyMarkup, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          ReplyMarkup,
          (replyMarkupForceReply, ForceReply);
          (replyMarkupInlineKeyboard, InlineKeyboard);
          (replyMarkupRemoveKeyboard, RemoveKeyboard);
          (replyMarkupShowKeyboard, ShowKeyboard);

        )(deserializer)
    }
}

impl RObject for ReplyMarkup {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            ReplyMarkup::ForceReply(t) => t.td_name(),
            ReplyMarkup::InlineKeyboard(t) => t.td_name(),
            ReplyMarkup::RemoveKeyboard(t) => t.td_name(),
            ReplyMarkup::ShowKeyboard(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            ReplyMarkup::ForceReply(t) => t.extra(),
            ReplyMarkup::InlineKeyboard(t) => t.extra(),
            ReplyMarkup::RemoveKeyboard(t) => t.extra(),
            ReplyMarkup::ShowKeyboard(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl ReplyMarkup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ReplyMarkup::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// True, if a forced reply must automatically be shown to the current user. For outgoing messages, specify true to show the forced reply only for the mentioned users and for the target user of a reply
    is_personal: bool,
}

impl RObject for ReplyMarkupForceReply {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "replyMarkupForceReply"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDReplyMarkup for ReplyMarkupForceReply {}

impl ReplyMarkupForceReply {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReplyMarkupForceReplyBuilder {
        let mut inner = ReplyMarkupForceReply::default();
        inner.td_name = "replyMarkupForceReply".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDReplyMarkupForceReplyBuilder { inner }
    }

    pub fn is_personal(&self) -> bool {
        self.is_personal
    }
}

#[doc(hidden)]
pub struct RTDReplyMarkupForceReplyBuilder {
    inner: ReplyMarkupForceReply,
}

impl RTDReplyMarkupForceReplyBuilder {
    pub fn build(&self) -> ReplyMarkupForceReply {
        self.inner.clone()
    }

    pub fn is_personal(&mut self, is_personal: bool) -> &mut Self {
        self.inner.is_personal = is_personal;
        self
    }
}

impl AsRef<ReplyMarkupForceReply> for ReplyMarkupForceReply {
    fn as_ref(&self) -> &ReplyMarkupForceReply {
        self
    }
}

impl AsRef<ReplyMarkupForceReply> for RTDReplyMarkupForceReplyBuilder {
    fn as_ref(&self) -> &ReplyMarkupForceReply {
        &self.inner
    }
}

/// Contains an inline keyboard layout
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyMarkupInlineKeyboard {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// A list of rows of inline keyboard buttons
    rows: Vec<Vec<InlineKeyboardButton>>,
}

impl RObject for ReplyMarkupInlineKeyboard {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "replyMarkupInlineKeyboard"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDReplyMarkup for ReplyMarkupInlineKeyboard {}

impl ReplyMarkupInlineKeyboard {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReplyMarkupInlineKeyboardBuilder {
        let mut inner = ReplyMarkupInlineKeyboard::default();
        inner.td_name = "replyMarkupInlineKeyboard".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDReplyMarkupInlineKeyboardBuilder { inner }
    }

    pub fn rows(&self) -> &Vec<Vec<InlineKeyboardButton>> {
        &self.rows
    }
}

#[doc(hidden)]
pub struct RTDReplyMarkupInlineKeyboardBuilder {
    inner: ReplyMarkupInlineKeyboard,
}

impl RTDReplyMarkupInlineKeyboardBuilder {
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

impl AsRef<ReplyMarkupInlineKeyboard> for RTDReplyMarkupInlineKeyboardBuilder {
    fn as_ref(&self) -> &ReplyMarkupInlineKeyboard {
        &self.inner
    }
}

/// Instructs application to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, UpdateChatReplyMarkup with message_id == 0 will be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyMarkupRemoveKeyboard {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// True, if the keyboard is removed only for the mentioned users or the target user of a reply
    is_personal: bool,
}

impl RObject for ReplyMarkupRemoveKeyboard {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "replyMarkupRemoveKeyboard"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDReplyMarkup for ReplyMarkupRemoveKeyboard {}

impl ReplyMarkupRemoveKeyboard {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReplyMarkupRemoveKeyboardBuilder {
        let mut inner = ReplyMarkupRemoveKeyboard::default();
        inner.td_name = "replyMarkupRemoveKeyboard".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDReplyMarkupRemoveKeyboardBuilder { inner }
    }

    pub fn is_personal(&self) -> bool {
        self.is_personal
    }
}

#[doc(hidden)]
pub struct RTDReplyMarkupRemoveKeyboardBuilder {
    inner: ReplyMarkupRemoveKeyboard,
}

impl RTDReplyMarkupRemoveKeyboardBuilder {
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

impl AsRef<ReplyMarkupRemoveKeyboard> for RTDReplyMarkupRemoveKeyboardBuilder {
    fn as_ref(&self) -> &ReplyMarkupRemoveKeyboard {
        &self.inner
    }
}

/// Contains a custom keyboard layout to quickly reply to bots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplyMarkupShowKeyboard {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// A list of rows of bot keyboard buttons
    rows: Vec<Vec<KeyboardButton>>,
    /// True, if the application needs to resize the keyboard vertically
    resize_keyboard: bool,
    /// True, if the application needs to hide the keyboard after use
    one_time: bool,
    /// True, if the keyboard must automatically be shown to the current user. For outgoing messages, specify true to show the keyboard only for the mentioned users and for the target user of a reply
    is_personal: bool,
}

impl RObject for ReplyMarkupShowKeyboard {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "replyMarkupShowKeyboard"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDReplyMarkup for ReplyMarkupShowKeyboard {}

impl ReplyMarkupShowKeyboard {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReplyMarkupShowKeyboardBuilder {
        let mut inner = ReplyMarkupShowKeyboard::default();
        inner.td_name = "replyMarkupShowKeyboard".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDReplyMarkupShowKeyboardBuilder { inner }
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
}

#[doc(hidden)]
pub struct RTDReplyMarkupShowKeyboardBuilder {
    inner: ReplyMarkupShowKeyboard,
}

impl RTDReplyMarkupShowKeyboardBuilder {
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
}

impl AsRef<ReplyMarkupShowKeyboard> for ReplyMarkupShowKeyboard {
    fn as_ref(&self) -> &ReplyMarkupShowKeyboard {
        self
    }
}

impl AsRef<ReplyMarkupShowKeyboard> for RTDReplyMarkupShowKeyboardBuilder {
    fn as_ref(&self) -> &ReplyMarkupShowKeyboard {
        &self.inner
    }
}
