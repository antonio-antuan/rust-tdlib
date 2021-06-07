use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents a part of the text which must be formatted differently
pub trait TDTextEntityType: Debug + RObject {}

/// Represents a part of the text which must be formatted differently
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextEntityType {
    #[doc(hidden)]
    _Default,
    /// A bank card number. The getBankCardInfo method can be used to get information about the bank card
    #[serde(rename(
        serialize = "textEntityTypeBankCardNumber",
        deserialize = "textEntityTypeBankCardNumber"
    ))]
    BankCardNumber(TextEntityTypeBankCardNumber),
    /// A bold text
    #[serde(rename(serialize = "textEntityTypeBold", deserialize = "textEntityTypeBold"))]
    Bold(TextEntityTypeBold),
    /// A bot command, beginning with "/". This shouldn't be highlighted if there are no bots in the chat
    #[serde(rename(
        serialize = "textEntityTypeBotCommand",
        deserialize = "textEntityTypeBotCommand"
    ))]
    BotCommand(TextEntityTypeBotCommand),
    /// A cashtag text, beginning with "$" and consisting of capital english letters (i.e. "$USD")
    #[serde(rename(
        serialize = "textEntityTypeCashtag",
        deserialize = "textEntityTypeCashtag"
    ))]
    Cashtag(TextEntityTypeCashtag),
    /// Text that must be formatted as if inside a code HTML tag
    #[serde(rename(serialize = "textEntityTypeCode", deserialize = "textEntityTypeCode"))]
    Code(TextEntityTypeCode),
    /// An email address
    #[serde(rename(
        serialize = "textEntityTypeEmailAddress",
        deserialize = "textEntityTypeEmailAddress"
    ))]
    EmailAddress(TextEntityTypeEmailAddress),
    /// A hashtag text, beginning with "#"
    #[serde(rename(
        serialize = "textEntityTypeHashtag",
        deserialize = "textEntityTypeHashtag"
    ))]
    Hashtag(TextEntityTypeHashtag),
    /// An italic text
    #[serde(rename(
        serialize = "textEntityTypeItalic",
        deserialize = "textEntityTypeItalic"
    ))]
    Italic(TextEntityTypeItalic),
    /// A mention of a user by their username
    #[serde(rename(
        serialize = "textEntityTypeMention",
        deserialize = "textEntityTypeMention"
    ))]
    Mention(TextEntityTypeMention),
    /// A text shows instead of a raw mention of the user (e.g., when the user has no username)
    #[serde(rename(
        serialize = "textEntityTypeMentionName",
        deserialize = "textEntityTypeMentionName"
    ))]
    MentionName(TextEntityTypeMentionName),
    /// A phone number
    #[serde(rename(
        serialize = "textEntityTypePhoneNumber",
        deserialize = "textEntityTypePhoneNumber"
    ))]
    PhoneNumber(TextEntityTypePhoneNumber),
    /// Text that must be formatted as if inside a pre HTML tag
    #[serde(rename(serialize = "textEntityTypePre", deserialize = "textEntityTypePre"))]
    Pre(TextEntityTypePre),
    /// Text that must be formatted as if inside pre, and code HTML tags
    #[serde(rename(
        serialize = "textEntityTypePreCode",
        deserialize = "textEntityTypePreCode"
    ))]
    PreCode(TextEntityTypePreCode),
    /// A strikethrough text
    #[serde(rename(
        serialize = "textEntityTypeStrikethrough",
        deserialize = "textEntityTypeStrikethrough"
    ))]
    Strikethrough(TextEntityTypeStrikethrough),
    /// A text description shown instead of a raw URL
    #[serde(rename(
        serialize = "textEntityTypeTextUrl",
        deserialize = "textEntityTypeTextUrl"
    ))]
    TextUrl(TextEntityTypeTextUrl),
    /// An underlined text
    #[serde(rename(
        serialize = "textEntityTypeUnderline",
        deserialize = "textEntityTypeUnderline"
    ))]
    Underline(TextEntityTypeUnderline),
    /// An HTTP URL
    #[serde(rename(serialize = "textEntityTypeUrl", deserialize = "textEntityTypeUrl"))]
    Url(TextEntityTypeUrl),
}

impl Default for TextEntityType {
    fn default() -> Self {
        TextEntityType::_Default
    }
}

impl RObject for TextEntityType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
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
        matches!(self, TextEntityType::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeBankCardNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeBankCardNumber {}

impl TextEntityTypeBankCardNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeBankCardNumberBuilder {
        let mut inner = TextEntityTypeBankCardNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeBold {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeBold {}

impl TextEntityTypeBold {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeBoldBuilder {
        let mut inner = TextEntityTypeBold::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeBotCommand {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeBotCommand {}

impl TextEntityTypeBotCommand {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeBotCommandBuilder {
        let mut inner = TextEntityTypeBotCommand::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeCashtag {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeCashtag {}

impl TextEntityTypeCashtag {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeCashtagBuilder {
        let mut inner = TextEntityTypeCashtag::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeCode {}

impl TextEntityTypeCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeCodeBuilder {
        let mut inner = TextEntityTypeCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeEmailAddress {}

impl TextEntityTypeEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeEmailAddressBuilder {
        let mut inner = TextEntityTypeEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeHashtag {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeHashtag {}

impl TextEntityTypeHashtag {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeHashtagBuilder {
        let mut inner = TextEntityTypeHashtag::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeItalic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeItalic {}

impl TextEntityTypeItalic {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeItalicBuilder {
        let mut inner = TextEntityTypeItalic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeMention {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeMention {}

impl TextEntityTypeMention {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeMentionBuilder {
        let mut inner = TextEntityTypeMention::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the mentioned user
    user_id: i32,
}

impl RObject for TextEntityTypeMentionName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeMentionName {}

impl TextEntityTypeMentionName {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeMentionNameBuilder {
        let mut inner = TextEntityTypeMentionName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypePhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypePhoneNumber {}

impl TextEntityTypePhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypePhoneNumberBuilder {
        let mut inner = TextEntityTypePhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypePre {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypePre {}

impl TextEntityTypePre {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypePreBuilder {
        let mut inner = TextEntityTypePre::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Programming language of the code; as defined by the sender
    language: String,
}

impl RObject for TextEntityTypePreCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypePreCode {}

impl TextEntityTypePreCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypePreCodeBuilder {
        let mut inner = TextEntityTypePreCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeStrikethrough {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeStrikethrough {}

impl TextEntityTypeStrikethrough {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeStrikethroughBuilder {
        let mut inner = TextEntityTypeStrikethrough::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// HTTP or tg:// URL to be opened when the link is clicked
    url: String,
}

impl RObject for TextEntityTypeTextUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeTextUrl {}

impl TextEntityTypeTextUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeTextUrlBuilder {
        let mut inner = TextEntityTypeTextUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeUnderline {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeUnderline {}

impl TextEntityTypeUnderline {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeUnderlineBuilder {
        let mut inner = TextEntityTypeUnderline::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for TextEntityTypeUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTextEntityType for TextEntityTypeUrl {}

impl TextEntityTypeUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntityTypeUrlBuilder {
        let mut inner = TextEntityTypeUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
