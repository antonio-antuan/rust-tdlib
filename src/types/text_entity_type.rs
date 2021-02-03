use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a part of the text which must be formatted differently
pub trait TDTextEntityType: Debug + RObject {}

/// Represents a part of the text which must be formatted differently
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TextEntityType {
    #[doc(hidden)]
    _Default(()),
    /// A bank card number. The getBankCardInfo method can be used to get information about the bank card
    BankCardNumber(TextEntityTypeBankCardNumber),
    /// A bold text
    Bold(TextEntityTypeBold),
    /// A bot command, beginning with "/". This shouldn't be highlighted if there are no bots in the chat
    BotCommand(TextEntityTypeBotCommand),
    /// A cashtag text, beginning with "$" and consisting of capital english letters (i.e. "$USD")
    Cashtag(TextEntityTypeCashtag),
    /// Text that must be formatted as if inside a code HTML tag
    Code(TextEntityTypeCode),
    /// An email address
    EmailAddress(TextEntityTypeEmailAddress),
    /// A hashtag text, beginning with "#"
    Hashtag(TextEntityTypeHashtag),
    /// An italic text
    Italic(TextEntityTypeItalic),
    /// A mention of a user by their username
    Mention(TextEntityTypeMention),
    /// A text shows instead of a raw mention of the user (e.g., when the user has no username)
    MentionName(TextEntityTypeMentionName),
    /// A phone number
    PhoneNumber(TextEntityTypePhoneNumber),
    /// Text that must be formatted as if inside a pre HTML tag
    Pre(TextEntityTypePre),
    /// Text that must be formatted as if inside pre, and code HTML tags
    PreCode(TextEntityTypePreCode),
    /// A strikethrough text
    Strikethrough(TextEntityTypeStrikethrough),
    /// A text description shown instead of a raw URL
    TextUrl(TextEntityTypeTextUrl),
    /// An underlined text
    Underline(TextEntityTypeUnderline),
    /// An HTTP URL
    Url(TextEntityTypeUrl),
}

impl Default for TextEntityType {
    fn default() -> Self {
        TextEntityType::_Default(())
    }
}

impl<'de> Deserialize<'de> for TextEntityType {
    fn deserialize<D>(deserializer: D) -> Result<TextEntityType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          TextEntityType,
          (textEntityTypeBankCardNumber, BankCardNumber);
          (textEntityTypeBold, Bold);
          (textEntityTypeBotCommand, BotCommand);
          (textEntityTypeCashtag, Cashtag);
          (textEntityTypeCode, Code);
          (textEntityTypeEmailAddress, EmailAddress);
          (textEntityTypeHashtag, Hashtag);
          (textEntityTypeItalic, Italic);
          (textEntityTypeMention, Mention);
          (textEntityTypeMentionName, MentionName);
          (textEntityTypePhoneNumber, PhoneNumber);
          (textEntityTypePre, Pre);
          (textEntityTypePreCode, PreCode);
          (textEntityTypeStrikethrough, Strikethrough);
          (textEntityTypeTextUrl, TextUrl);
          (textEntityTypeUnderline, Underline);
          (textEntityTypeUrl, Url);

        )(deserializer)
    }
}

impl RObject for TextEntityType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            TextEntityType::BankCardNumber(t) => t.td_name(),
            TextEntityType::Bold(t) => t.td_name(),
            TextEntityType::BotCommand(t) => t.td_name(),
            TextEntityType::Cashtag(t) => t.td_name(),
            TextEntityType::Code(t) => t.td_name(),
            TextEntityType::EmailAddress(t) => t.td_name(),
            TextEntityType::Hashtag(t) => t.td_name(),
            TextEntityType::Italic(t) => t.td_name(),
            TextEntityType::Mention(t) => t.td_name(),
            TextEntityType::MentionName(t) => t.td_name(),
            TextEntityType::PhoneNumber(t) => t.td_name(),
            TextEntityType::Pre(t) => t.td_name(),
            TextEntityType::PreCode(t) => t.td_name(),
            TextEntityType::Strikethrough(t) => t.td_name(),
            TextEntityType::TextUrl(t) => t.td_name(),
            TextEntityType::Underline(t) => t.td_name(),
            TextEntityType::Url(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            TextEntityType::BankCardNumber(t) => t.extra(),
            TextEntityType::Bold(t) => t.extra(),
            TextEntityType::BotCommand(t) => t.extra(),
            TextEntityType::Cashtag(t) => t.extra(),
            TextEntityType::Code(t) => t.extra(),
            TextEntityType::EmailAddress(t) => t.extra(),
            TextEntityType::Hashtag(t) => t.extra(),
            TextEntityType::Italic(t) => t.extra(),
            TextEntityType::Mention(t) => t.extra(),
            TextEntityType::MentionName(t) => t.extra(),
            TextEntityType::PhoneNumber(t) => t.extra(),
            TextEntityType::Pre(t) => t.extra(),
            TextEntityType::PreCode(t) => t.extra(),
            TextEntityType::Strikethrough(t) => t.extra(),
            TextEntityType::TextUrl(t) => t.extra(),
            TextEntityType::Underline(t) => t.extra(),
            TextEntityType::Url(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            TextEntityType::BankCardNumber(t) => t.client_id(),
            TextEntityType::Bold(t) => t.client_id(),
            TextEntityType::BotCommand(t) => t.client_id(),
            TextEntityType::Cashtag(t) => t.client_id(),
            TextEntityType::Code(t) => t.client_id(),
            TextEntityType::EmailAddress(t) => t.client_id(),
            TextEntityType::Hashtag(t) => t.client_id(),
            TextEntityType::Italic(t) => t.client_id(),
            TextEntityType::Mention(t) => t.client_id(),
            TextEntityType::MentionName(t) => t.client_id(),
            TextEntityType::PhoneNumber(t) => t.client_id(),
            TextEntityType::Pre(t) => t.client_id(),
            TextEntityType::PreCode(t) => t.client_id(),
            TextEntityType::Strikethrough(t) => t.client_id(),
            TextEntityType::TextUrl(t) => t.client_id(),
            TextEntityType::Underline(t) => t.client_id(),
            TextEntityType::Url(t) => t.client_id(),

            _ => None,
        }
    }
}

impl TextEntityType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, TextEntityType::_Default(_))
    }
}

impl AsRef<TextEntityType> for TextEntityType {
    fn as_ref(&self) -> &TextEntityType {
        self
    }
}

/// A bank card number. The getBankCardInfo method can be used to get information about the bank card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeBankCardNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeBankCardNumber {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeBankCardNumber"
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

impl TDTextEntityType for TextEntityTypeBankCardNumber {}

impl TextEntityTypeBankCardNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeBankCardNumberBuilder {
        let mut inner = TextEntityTypeBankCardNumber::default();
        inner.td_name = "textEntityTypeBankCardNumber".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeBankCardNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeBankCardNumberBuilder {
    inner: TextEntityTypeBankCardNumber,
}

impl RTDTextEntityTypeBankCardNumberBuilder {
    pub fn build(&self) -> TextEntityTypeBankCardNumber {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeBankCardNumber> for TextEntityTypeBankCardNumber {
    fn as_ref(&self) -> &TextEntityTypeBankCardNumber {
        self
    }
}

impl AsRef<TextEntityTypeBankCardNumber> for RTDTextEntityTypeBankCardNumberBuilder {
    fn as_ref(&self) -> &TextEntityTypeBankCardNumber {
        &self.inner
    }
}

/// A bold text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeBold {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeBold {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeBold"
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

impl TDTextEntityType for TextEntityTypeBold {}

impl TextEntityTypeBold {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeBoldBuilder {
        let mut inner = TextEntityTypeBold::default();
        inner.td_name = "textEntityTypeBold".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeBoldBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeBoldBuilder {
    inner: TextEntityTypeBold,
}

impl RTDTextEntityTypeBoldBuilder {
    pub fn build(&self) -> TextEntityTypeBold {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeBold> for TextEntityTypeBold {
    fn as_ref(&self) -> &TextEntityTypeBold {
        self
    }
}

impl AsRef<TextEntityTypeBold> for RTDTextEntityTypeBoldBuilder {
    fn as_ref(&self) -> &TextEntityTypeBold {
        &self.inner
    }
}

/// A bot command, beginning with "/". This shouldn't be highlighted if there are no bots in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeBotCommand {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeBotCommand {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeBotCommand"
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

impl TDTextEntityType for TextEntityTypeBotCommand {}

impl TextEntityTypeBotCommand {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeBotCommandBuilder {
        let mut inner = TextEntityTypeBotCommand::default();
        inner.td_name = "textEntityTypeBotCommand".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeBotCommandBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeBotCommandBuilder {
    inner: TextEntityTypeBotCommand,
}

impl RTDTextEntityTypeBotCommandBuilder {
    pub fn build(&self) -> TextEntityTypeBotCommand {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeBotCommand> for TextEntityTypeBotCommand {
    fn as_ref(&self) -> &TextEntityTypeBotCommand {
        self
    }
}

impl AsRef<TextEntityTypeBotCommand> for RTDTextEntityTypeBotCommandBuilder {
    fn as_ref(&self) -> &TextEntityTypeBotCommand {
        &self.inner
    }
}

/// A cashtag text, beginning with "$" and consisting of capital english letters (i.e. "$USD")
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeCashtag {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeCashtag {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeCashtag"
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

impl TDTextEntityType for TextEntityTypeCashtag {}

impl TextEntityTypeCashtag {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeCashtagBuilder {
        let mut inner = TextEntityTypeCashtag::default();
        inner.td_name = "textEntityTypeCashtag".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeCashtagBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeCashtagBuilder {
    inner: TextEntityTypeCashtag,
}

impl RTDTextEntityTypeCashtagBuilder {
    pub fn build(&self) -> TextEntityTypeCashtag {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeCashtag> for TextEntityTypeCashtag {
    fn as_ref(&self) -> &TextEntityTypeCashtag {
        self
    }
}

impl AsRef<TextEntityTypeCashtag> for RTDTextEntityTypeCashtagBuilder {
    fn as_ref(&self) -> &TextEntityTypeCashtag {
        &self.inner
    }
}

/// Text that must be formatted as if inside a code HTML tag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeCode {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeCode"
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

impl TDTextEntityType for TextEntityTypeCode {}

impl TextEntityTypeCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeCodeBuilder {
        let mut inner = TextEntityTypeCode::default();
        inner.td_name = "textEntityTypeCode".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeCodeBuilder {
    inner: TextEntityTypeCode,
}

impl RTDTextEntityTypeCodeBuilder {
    pub fn build(&self) -> TextEntityTypeCode {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeCode> for TextEntityTypeCode {
    fn as_ref(&self) -> &TextEntityTypeCode {
        self
    }
}

impl AsRef<TextEntityTypeCode> for RTDTextEntityTypeCodeBuilder {
    fn as_ref(&self) -> &TextEntityTypeCode {
        &self.inner
    }
}

/// An email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeEmailAddress {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeEmailAddress"
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

impl TDTextEntityType for TextEntityTypeEmailAddress {}

impl TextEntityTypeEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeEmailAddressBuilder {
        let mut inner = TextEntityTypeEmailAddress::default();
        inner.td_name = "textEntityTypeEmailAddress".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeEmailAddressBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeEmailAddressBuilder {
    inner: TextEntityTypeEmailAddress,
}

impl RTDTextEntityTypeEmailAddressBuilder {
    pub fn build(&self) -> TextEntityTypeEmailAddress {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeEmailAddress> for TextEntityTypeEmailAddress {
    fn as_ref(&self) -> &TextEntityTypeEmailAddress {
        self
    }
}

impl AsRef<TextEntityTypeEmailAddress> for RTDTextEntityTypeEmailAddressBuilder {
    fn as_ref(&self) -> &TextEntityTypeEmailAddress {
        &self.inner
    }
}

/// A hashtag text, beginning with "#"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeHashtag {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeHashtag {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeHashtag"
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

impl TDTextEntityType for TextEntityTypeHashtag {}

impl TextEntityTypeHashtag {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeHashtagBuilder {
        let mut inner = TextEntityTypeHashtag::default();
        inner.td_name = "textEntityTypeHashtag".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeHashtagBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeHashtagBuilder {
    inner: TextEntityTypeHashtag,
}

impl RTDTextEntityTypeHashtagBuilder {
    pub fn build(&self) -> TextEntityTypeHashtag {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeHashtag> for TextEntityTypeHashtag {
    fn as_ref(&self) -> &TextEntityTypeHashtag {
        self
    }
}

impl AsRef<TextEntityTypeHashtag> for RTDTextEntityTypeHashtagBuilder {
    fn as_ref(&self) -> &TextEntityTypeHashtag {
        &self.inner
    }
}

/// An italic text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeItalic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeItalic {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeItalic"
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

impl TDTextEntityType for TextEntityTypeItalic {}

impl TextEntityTypeItalic {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeItalicBuilder {
        let mut inner = TextEntityTypeItalic::default();
        inner.td_name = "textEntityTypeItalic".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeItalicBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeItalicBuilder {
    inner: TextEntityTypeItalic,
}

impl RTDTextEntityTypeItalicBuilder {
    pub fn build(&self) -> TextEntityTypeItalic {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeItalic> for TextEntityTypeItalic {
    fn as_ref(&self) -> &TextEntityTypeItalic {
        self
    }
}

impl AsRef<TextEntityTypeItalic> for RTDTextEntityTypeItalicBuilder {
    fn as_ref(&self) -> &TextEntityTypeItalic {
        &self.inner
    }
}

/// A mention of a user by their username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeMention {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeMention {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeMention"
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

impl TDTextEntityType for TextEntityTypeMention {}

impl TextEntityTypeMention {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeMentionBuilder {
        let mut inner = TextEntityTypeMention::default();
        inner.td_name = "textEntityTypeMention".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeMentionBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeMentionBuilder {
    inner: TextEntityTypeMention,
}

impl RTDTextEntityTypeMentionBuilder {
    pub fn build(&self) -> TextEntityTypeMention {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeMention> for TextEntityTypeMention {
    fn as_ref(&self) -> &TextEntityTypeMention {
        self
    }
}

impl AsRef<TextEntityTypeMention> for RTDTextEntityTypeMentionBuilder {
    fn as_ref(&self) -> &TextEntityTypeMention {
        &self.inner
    }
}

/// A text shows instead of a raw mention of the user (e.g., when the user has no username)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeMentionName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the mentioned user
    user_id: i32,
}

impl RObject for TextEntityTypeMentionName {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeMentionName"
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

impl TDTextEntityType for TextEntityTypeMentionName {}

impl TextEntityTypeMentionName {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeMentionNameBuilder {
        let mut inner = TextEntityTypeMentionName::default();
        inner.td_name = "textEntityTypeMentionName".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeMentionNameBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeMentionNameBuilder {
    inner: TextEntityTypeMentionName,
}

impl RTDTextEntityTypeMentionNameBuilder {
    pub fn build(&self) -> TextEntityTypeMentionName {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<TextEntityTypeMentionName> for TextEntityTypeMentionName {
    fn as_ref(&self) -> &TextEntityTypeMentionName {
        self
    }
}

impl AsRef<TextEntityTypeMentionName> for RTDTextEntityTypeMentionNameBuilder {
    fn as_ref(&self) -> &TextEntityTypeMentionName {
        &self.inner
    }
}

/// A phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypePhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypePhoneNumber {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypePhoneNumber"
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

impl TDTextEntityType for TextEntityTypePhoneNumber {}

impl TextEntityTypePhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypePhoneNumberBuilder {
        let mut inner = TextEntityTypePhoneNumber::default();
        inner.td_name = "textEntityTypePhoneNumber".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypePhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypePhoneNumberBuilder {
    inner: TextEntityTypePhoneNumber,
}

impl RTDTextEntityTypePhoneNumberBuilder {
    pub fn build(&self) -> TextEntityTypePhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypePhoneNumber> for TextEntityTypePhoneNumber {
    fn as_ref(&self) -> &TextEntityTypePhoneNumber {
        self
    }
}

impl AsRef<TextEntityTypePhoneNumber> for RTDTextEntityTypePhoneNumberBuilder {
    fn as_ref(&self) -> &TextEntityTypePhoneNumber {
        &self.inner
    }
}

/// Text that must be formatted as if inside a pre HTML tag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypePre {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypePre {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypePre"
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

impl TDTextEntityType for TextEntityTypePre {}

impl TextEntityTypePre {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypePreBuilder {
        let mut inner = TextEntityTypePre::default();
        inner.td_name = "textEntityTypePre".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypePreBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypePreBuilder {
    inner: TextEntityTypePre,
}

impl RTDTextEntityTypePreBuilder {
    pub fn build(&self) -> TextEntityTypePre {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypePre> for TextEntityTypePre {
    fn as_ref(&self) -> &TextEntityTypePre {
        self
    }
}

impl AsRef<TextEntityTypePre> for RTDTextEntityTypePreBuilder {
    fn as_ref(&self) -> &TextEntityTypePre {
        &self.inner
    }
}

/// Text that must be formatted as if inside pre, and code HTML tags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypePreCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Programming language of the code; as defined by the sender
    language: String,
}

impl RObject for TextEntityTypePreCode {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypePreCode"
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

impl TDTextEntityType for TextEntityTypePreCode {}

impl TextEntityTypePreCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypePreCodeBuilder {
        let mut inner = TextEntityTypePreCode::default();
        inner.td_name = "textEntityTypePreCode".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypePreCodeBuilder { inner }
    }

    pub fn language(&self) -> &String {
        &self.language
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypePreCodeBuilder {
    inner: TextEntityTypePreCode,
}

impl RTDTextEntityTypePreCodeBuilder {
    pub fn build(&self) -> TextEntityTypePreCode {
        self.inner.clone()
    }

    pub fn language<T: AsRef<str>>(&mut self, language: T) -> &mut Self {
        self.inner.language = language.as_ref().to_string();
        self
    }
}

impl AsRef<TextEntityTypePreCode> for TextEntityTypePreCode {
    fn as_ref(&self) -> &TextEntityTypePreCode {
        self
    }
}

impl AsRef<TextEntityTypePreCode> for RTDTextEntityTypePreCodeBuilder {
    fn as_ref(&self) -> &TextEntityTypePreCode {
        &self.inner
    }
}

/// A strikethrough text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeStrikethrough {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeStrikethrough {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeStrikethrough"
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

impl TDTextEntityType for TextEntityTypeStrikethrough {}

impl TextEntityTypeStrikethrough {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeStrikethroughBuilder {
        let mut inner = TextEntityTypeStrikethrough::default();
        inner.td_name = "textEntityTypeStrikethrough".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeStrikethroughBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeStrikethroughBuilder {
    inner: TextEntityTypeStrikethrough,
}

impl RTDTextEntityTypeStrikethroughBuilder {
    pub fn build(&self) -> TextEntityTypeStrikethrough {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeStrikethrough> for TextEntityTypeStrikethrough {
    fn as_ref(&self) -> &TextEntityTypeStrikethrough {
        self
    }
}

impl AsRef<TextEntityTypeStrikethrough> for RTDTextEntityTypeStrikethroughBuilder {
    fn as_ref(&self) -> &TextEntityTypeStrikethrough {
        &self.inner
    }
}

/// A text description shown instead of a raw URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeTextUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// HTTP or tg:// URL to be opened when the link is clicked
    url: String,
}

impl RObject for TextEntityTypeTextUrl {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeTextUrl"
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

impl TDTextEntityType for TextEntityTypeTextUrl {}

impl TextEntityTypeTextUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeTextUrlBuilder {
        let mut inner = TextEntityTypeTextUrl::default();
        inner.td_name = "textEntityTypeTextUrl".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeTextUrlBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeTextUrlBuilder {
    inner: TextEntityTypeTextUrl,
}

impl RTDTextEntityTypeTextUrlBuilder {
    pub fn build(&self) -> TextEntityTypeTextUrl {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<TextEntityTypeTextUrl> for TextEntityTypeTextUrl {
    fn as_ref(&self) -> &TextEntityTypeTextUrl {
        self
    }
}

impl AsRef<TextEntityTypeTextUrl> for RTDTextEntityTypeTextUrlBuilder {
    fn as_ref(&self) -> &TextEntityTypeTextUrl {
        &self.inner
    }
}

/// An underlined text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeUnderline {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeUnderline {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeUnderline"
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

impl TDTextEntityType for TextEntityTypeUnderline {}

impl TextEntityTypeUnderline {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeUnderlineBuilder {
        let mut inner = TextEntityTypeUnderline::default();
        inner.td_name = "textEntityTypeUnderline".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeUnderlineBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeUnderlineBuilder {
    inner: TextEntityTypeUnderline,
}

impl RTDTextEntityTypeUnderlineBuilder {
    pub fn build(&self) -> TextEntityTypeUnderline {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeUnderline> for TextEntityTypeUnderline {
    fn as_ref(&self) -> &TextEntityTypeUnderline {
        self
    }
}

impl AsRef<TextEntityTypeUnderline> for RTDTextEntityTypeUnderlineBuilder {
    fn as_ref(&self) -> &TextEntityTypeUnderline {
        &self.inner
    }
}

/// An HTTP URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeUrl {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "textEntityTypeUrl"
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

impl TDTextEntityType for TextEntityTypeUrl {}

impl TextEntityTypeUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeUrlBuilder {
        let mut inner = TextEntityTypeUrl::default();
        inner.td_name = "textEntityTypeUrl".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTextEntityTypeUrlBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTextEntityTypeUrlBuilder {
    inner: TextEntityTypeUrl,
}

impl RTDTextEntityTypeUrlBuilder {
    pub fn build(&self) -> TextEntityTypeUrl {
        self.inner.clone()
    }
}

impl AsRef<TextEntityTypeUrl> for TextEntityTypeUrl {
    fn as_ref(&self) -> &TextEntityTypeUrl {
        self
    }
}

impl AsRef<TextEntityTypeUrl> for RTDTextEntityTypeUrlBuilder {
    fn as_ref(&self) -> &TextEntityTypeUrl {
        &self.inner
    }
}
