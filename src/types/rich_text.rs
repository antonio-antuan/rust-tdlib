use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a text object inside an instant-view web page
pub trait TDRichText: Debug + RObject {}

/// Describes a text object inside an instant-view web page
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RichText {
    #[doc(hidden)]
    _Default,
    /// An anchor
    #[serde(rename = "richTextAnchor")]
    Anchor(RichTextAnchor),
    /// A link to an anchor on the same web page
    #[serde(rename = "richTextAnchorLink")]
    AnchorLink(RichTextAnchorLink),
    /// A bold rich text
    #[serde(rename = "richTextBold")]
    Bold(RichTextBold),
    /// A rich text email link
    #[serde(rename = "richTextEmailAddress")]
    EmailAddress(RichTextEmailAddress),
    /// A fixed-width rich text
    #[serde(rename = "richTextFixed")]
    Fixed(RichTextFixed),
    /// A small image inside the text
    #[serde(rename = "richTextIcon")]
    Icon(Box<RichTextIcon>),
    /// An italicized rich text
    #[serde(rename = "richTextItalic")]
    Italic(RichTextItalic),
    /// A marked rich text
    #[serde(rename = "richTextMarked")]
    Marked(RichTextMarked),
    /// A rich text phone number
    #[serde(rename = "richTextPhoneNumber")]
    PhoneNumber(RichTextPhoneNumber),
    /// A plain text
    #[serde(rename = "richTextPlain")]
    Plain(RichTextPlain),
    /// A reference to a richTexts object on the same web page
    #[serde(rename = "richTextReference")]
    Reference(RichTextReference),
    /// A strikethrough rich text
    #[serde(rename = "richTextStrikethrough")]
    Strikethrough(RichTextStrikethrough),
    /// A subscript rich text
    #[serde(rename = "richTextSubscript")]
    Subscript(RichTextSubscript),
    /// A superscript rich text
    #[serde(rename = "richTextSuperscript")]
    Superscript(RichTextSuperscript),
    /// An underlined rich text
    #[serde(rename = "richTextUnderline")]
    Underline(RichTextUnderline),
    /// A rich text URL link
    #[serde(rename = "richTextUrl")]
    Url(RichTextUrl),
    /// A concatenation of rich texts
    #[serde(rename = "richTexts")]
    RichTexts(RichTexts),
}

impl Default for RichText {
    fn default() -> Self {
        RichText::_Default
    }
}

impl RObject for RichText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            RichText::Anchor(t) => t.extra(),
            RichText::AnchorLink(t) => t.extra(),
            RichText::Bold(t) => t.extra(),
            RichText::EmailAddress(t) => t.extra(),
            RichText::Fixed(t) => t.extra(),
            RichText::Icon(t) => t.extra(),
            RichText::Italic(t) => t.extra(),
            RichText::Marked(t) => t.extra(),
            RichText::PhoneNumber(t) => t.extra(),
            RichText::Plain(t) => t.extra(),
            RichText::Reference(t) => t.extra(),
            RichText::Strikethrough(t) => t.extra(),
            RichText::Subscript(t) => t.extra(),
            RichText::Superscript(t) => t.extra(),
            RichText::Underline(t) => t.extra(),
            RichText::Url(t) => t.extra(),
            RichText::RichTexts(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            RichText::Anchor(t) => t.client_id(),
            RichText::AnchorLink(t) => t.client_id(),
            RichText::Bold(t) => t.client_id(),
            RichText::EmailAddress(t) => t.client_id(),
            RichText::Fixed(t) => t.client_id(),
            RichText::Icon(t) => t.client_id(),
            RichText::Italic(t) => t.client_id(),
            RichText::Marked(t) => t.client_id(),
            RichText::PhoneNumber(t) => t.client_id(),
            RichText::Plain(t) => t.client_id(),
            RichText::Reference(t) => t.client_id(),
            RichText::Strikethrough(t) => t.client_id(),
            RichText::Subscript(t) => t.client_id(),
            RichText::Superscript(t) => t.client_id(),
            RichText::Underline(t) => t.client_id(),
            RichText::Url(t) => t.client_id(),
            RichText::RichTexts(t) => t.client_id(),

            _ => None,
        }
    }
}

impl RichText {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, RichText::_Default)
    }
}

impl AsRef<RichText> for RichText {
    fn as_ref(&self) -> &RichText {
        self
    }
}

/// An anchor
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextAnchor {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Anchor name

    #[serde(default)]
    name: String,
}

impl RObject for RichTextAnchor {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextAnchor {}

impl RichTextAnchor {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextAnchorBuilder {
        let mut inner = RichTextAnchor::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextAnchorBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct RichTextAnchorBuilder {
    inner: RichTextAnchor,
}

#[deprecated]
pub type RTDRichTextAnchorBuilder = RichTextAnchorBuilder;

impl RichTextAnchorBuilder {
    pub fn build(&self) -> RichTextAnchor {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<RichTextAnchor> for RichTextAnchor {
    fn as_ref(&self) -> &RichTextAnchor {
        self
    }
}

impl AsRef<RichTextAnchor> for RichTextAnchorBuilder {
    fn as_ref(&self) -> &RichTextAnchor {
        &self.inner
    }
}

/// A link to an anchor on the same web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextAnchorLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The link text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
    /// The anchor name. If the name is empty, the link must bring back to top

    #[serde(default)]
    anchor_name: String,
    /// An HTTP URL, opening the anchor

    #[serde(default)]
    url: String,
}

impl RObject for RichTextAnchorLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextAnchorLink {}

impl RichTextAnchorLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextAnchorLinkBuilder {
        let mut inner = RichTextAnchorLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextAnchorLinkBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }

    pub fn anchor_name(&self) -> &String {
        &self.anchor_name
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RichTextAnchorLinkBuilder {
    inner: RichTextAnchorLink,
}

#[deprecated]
pub type RTDRichTextAnchorLinkBuilder = RichTextAnchorLinkBuilder;

impl RichTextAnchorLinkBuilder {
    pub fn build(&self) -> RichTextAnchorLink {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn anchor_name<T: AsRef<str>>(&mut self, anchor_name: T) -> &mut Self {
        self.inner.anchor_name = anchor_name.as_ref().to_string();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<RichTextAnchorLink> for RichTextAnchorLink {
    fn as_ref(&self) -> &RichTextAnchorLink {
        self
    }
}

impl AsRef<RichTextAnchorLink> for RichTextAnchorLinkBuilder {
    fn as_ref(&self) -> &RichTextAnchorLink {
        &self.inner
    }
}

/// A bold rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextBold {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
}

impl RObject for RichTextBold {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextBold {}

impl RichTextBold {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextBoldBuilder {
        let mut inner = RichTextBold::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextBoldBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RichTextBoldBuilder {
    inner: RichTextBold,
}

#[deprecated]
pub type RTDRichTextBoldBuilder = RichTextBoldBuilder;

impl RichTextBoldBuilder {
    pub fn build(&self) -> RichTextBold {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<RichTextBold> for RichTextBold {
    fn as_ref(&self) -> &RichTextBold {
        self
    }
}

impl AsRef<RichTextBold> for RichTextBoldBuilder {
    fn as_ref(&self) -> &RichTextBold {
        &self.inner
    }
}

/// A rich text email link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
    /// Email address

    #[serde(default)]
    email_address: String,
}

impl RObject for RichTextEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextEmailAddress {}

impl RichTextEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextEmailAddressBuilder {
        let mut inner = RichTextEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextEmailAddressBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }

    pub fn email_address(&self) -> &String {
        &self.email_address
    }
}

#[doc(hidden)]
pub struct RichTextEmailAddressBuilder {
    inner: RichTextEmailAddress,
}

#[deprecated]
pub type RTDRichTextEmailAddressBuilder = RichTextEmailAddressBuilder;

impl RichTextEmailAddressBuilder {
    pub fn build(&self) -> RichTextEmailAddress {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn email_address<T: AsRef<str>>(&mut self, email_address: T) -> &mut Self {
        self.inner.email_address = email_address.as_ref().to_string();
        self
    }
}

impl AsRef<RichTextEmailAddress> for RichTextEmailAddress {
    fn as_ref(&self) -> &RichTextEmailAddress {
        self
    }
}

impl AsRef<RichTextEmailAddress> for RichTextEmailAddressBuilder {
    fn as_ref(&self) -> &RichTextEmailAddress {
        &self.inner
    }
}

/// A fixed-width rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextFixed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
}

impl RObject for RichTextFixed {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextFixed {}

impl RichTextFixed {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextFixedBuilder {
        let mut inner = RichTextFixed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextFixedBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RichTextFixedBuilder {
    inner: RichTextFixed,
}

#[deprecated]
pub type RTDRichTextFixedBuilder = RichTextFixedBuilder;

impl RichTextFixedBuilder {
    pub fn build(&self) -> RichTextFixed {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<RichTextFixed> for RichTextFixed {
    fn as_ref(&self) -> &RichTextFixed {
        self
    }
}

impl AsRef<RichTextFixed> for RichTextFixedBuilder {
    fn as_ref(&self) -> &RichTextFixed {
        &self.inner
    }
}

/// A small image inside the text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextIcon {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The image represented as a document. The image can be in GIF, JPEG or PNG format
    document: Document,
    /// Width of a bounding box in which the image must be shown; 0 if unknown

    #[serde(default)]
    width: i32,
    /// Height of a bounding box in which the image must be shown; 0 if unknown

    #[serde(default)]
    height: i32,
}

impl RObject for RichTextIcon {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextIcon {}

impl RichTextIcon {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextIconBuilder {
        let mut inner = RichTextIcon::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextIconBuilder { inner }
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

#[doc(hidden)]
pub struct RichTextIconBuilder {
    inner: RichTextIcon,
}

#[deprecated]
pub type RTDRichTextIconBuilder = RichTextIconBuilder;

impl RichTextIconBuilder {
    pub fn build(&self) -> RichTextIcon {
        self.inner.clone()
    }

    pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
        self.inner.document = document.as_ref().clone();
        self
    }

    pub fn width(&mut self, width: i32) -> &mut Self {
        self.inner.width = width;
        self
    }

    pub fn height(&mut self, height: i32) -> &mut Self {
        self.inner.height = height;
        self
    }
}

impl AsRef<RichTextIcon> for RichTextIcon {
    fn as_ref(&self) -> &RichTextIcon {
        self
    }
}

impl AsRef<RichTextIcon> for RichTextIconBuilder {
    fn as_ref(&self) -> &RichTextIcon {
        &self.inner
    }
}

/// An italicized rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextItalic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
}

impl RObject for RichTextItalic {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextItalic {}

impl RichTextItalic {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextItalicBuilder {
        let mut inner = RichTextItalic::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextItalicBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RichTextItalicBuilder {
    inner: RichTextItalic,
}

#[deprecated]
pub type RTDRichTextItalicBuilder = RichTextItalicBuilder;

impl RichTextItalicBuilder {
    pub fn build(&self) -> RichTextItalic {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<RichTextItalic> for RichTextItalic {
    fn as_ref(&self) -> &RichTextItalic {
        self
    }
}

impl AsRef<RichTextItalic> for RichTextItalicBuilder {
    fn as_ref(&self) -> &RichTextItalic {
        &self.inner
    }
}

/// A marked rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextMarked {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
}

impl RObject for RichTextMarked {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextMarked {}

impl RichTextMarked {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextMarkedBuilder {
        let mut inner = RichTextMarked::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextMarkedBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RichTextMarkedBuilder {
    inner: RichTextMarked,
}

#[deprecated]
pub type RTDRichTextMarkedBuilder = RichTextMarkedBuilder;

impl RichTextMarkedBuilder {
    pub fn build(&self) -> RichTextMarked {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<RichTextMarked> for RichTextMarked {
    fn as_ref(&self) -> &RichTextMarked {
        self
    }
}

impl AsRef<RichTextMarked> for RichTextMarkedBuilder {
    fn as_ref(&self) -> &RichTextMarked {
        &self.inner
    }
}

/// A rich text phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
    /// Phone number

    #[serde(default)]
    phone_number: String,
}

impl RObject for RichTextPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextPhoneNumber {}

impl RichTextPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextPhoneNumberBuilder {
        let mut inner = RichTextPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextPhoneNumberBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }
}

#[doc(hidden)]
pub struct RichTextPhoneNumberBuilder {
    inner: RichTextPhoneNumber,
}

#[deprecated]
pub type RTDRichTextPhoneNumberBuilder = RichTextPhoneNumberBuilder;

impl RichTextPhoneNumberBuilder {
    pub fn build(&self) -> RichTextPhoneNumber {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
        self.inner.phone_number = phone_number.as_ref().to_string();
        self
    }
}

impl AsRef<RichTextPhoneNumber> for RichTextPhoneNumber {
    fn as_ref(&self) -> &RichTextPhoneNumber {
        self
    }
}

impl AsRef<RichTextPhoneNumber> for RichTextPhoneNumberBuilder {
    fn as_ref(&self) -> &RichTextPhoneNumber {
        &self.inner
    }
}

/// A plain text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextPlain {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(default)]
    text: Box<RichText>,
}

impl RObject for RichTextPlain {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextPlain {}

impl RichTextPlain {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextPlainBuilder {
        let mut inner = RichTextPlain::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextPlainBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RichTextPlainBuilder {
    inner: RichTextPlain,
}

#[deprecated]
pub type RTDRichTextPlainBuilder = RichTextPlainBuilder;

impl RichTextPlainBuilder {
    pub fn build(&self) -> RichTextPlain {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<RichTextPlain> for RichTextPlain {
    fn as_ref(&self) -> &RichTextPlain {
        self
    }
}

impl AsRef<RichTextPlain> for RichTextPlainBuilder {
    fn as_ref(&self) -> &RichTextPlain {
        &self.inner
    }
}

/// A reference to a richTexts object on the same web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextReference {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
    /// The name of a richTextAnchor object, which is the first element of the target richTexts object

    #[serde(default)]
    anchor_name: String,
    /// An HTTP URL, opening the reference

    #[serde(default)]
    url: String,
}

impl RObject for RichTextReference {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextReference {}

impl RichTextReference {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextReferenceBuilder {
        let mut inner = RichTextReference::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextReferenceBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }

    pub fn anchor_name(&self) -> &String {
        &self.anchor_name
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RichTextReferenceBuilder {
    inner: RichTextReference,
}

#[deprecated]
pub type RTDRichTextReferenceBuilder = RichTextReferenceBuilder;

impl RichTextReferenceBuilder {
    pub fn build(&self) -> RichTextReference {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn anchor_name<T: AsRef<str>>(&mut self, anchor_name: T) -> &mut Self {
        self.inner.anchor_name = anchor_name.as_ref().to_string();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<RichTextReference> for RichTextReference {
    fn as_ref(&self) -> &RichTextReference {
        self
    }
}

impl AsRef<RichTextReference> for RichTextReferenceBuilder {
    fn as_ref(&self) -> &RichTextReference {
        &self.inner
    }
}

/// A strikethrough rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextStrikethrough {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
}

impl RObject for RichTextStrikethrough {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextStrikethrough {}

impl RichTextStrikethrough {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextStrikethroughBuilder {
        let mut inner = RichTextStrikethrough::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextStrikethroughBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RichTextStrikethroughBuilder {
    inner: RichTextStrikethrough,
}

#[deprecated]
pub type RTDRichTextStrikethroughBuilder = RichTextStrikethroughBuilder;

impl RichTextStrikethroughBuilder {
    pub fn build(&self) -> RichTextStrikethrough {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<RichTextStrikethrough> for RichTextStrikethrough {
    fn as_ref(&self) -> &RichTextStrikethrough {
        self
    }
}

impl AsRef<RichTextStrikethrough> for RichTextStrikethroughBuilder {
    fn as_ref(&self) -> &RichTextStrikethrough {
        &self.inner
    }
}

/// A subscript rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextSubscript {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
}

impl RObject for RichTextSubscript {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextSubscript {}

impl RichTextSubscript {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextSubscriptBuilder {
        let mut inner = RichTextSubscript::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextSubscriptBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RichTextSubscriptBuilder {
    inner: RichTextSubscript,
}

#[deprecated]
pub type RTDRichTextSubscriptBuilder = RichTextSubscriptBuilder;

impl RichTextSubscriptBuilder {
    pub fn build(&self) -> RichTextSubscript {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<RichTextSubscript> for RichTextSubscript {
    fn as_ref(&self) -> &RichTextSubscript {
        self
    }
}

impl AsRef<RichTextSubscript> for RichTextSubscriptBuilder {
    fn as_ref(&self) -> &RichTextSubscript {
        &self.inner
    }
}

/// A superscript rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextSuperscript {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
}

impl RObject for RichTextSuperscript {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextSuperscript {}

impl RichTextSuperscript {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextSuperscriptBuilder {
        let mut inner = RichTextSuperscript::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextSuperscriptBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RichTextSuperscriptBuilder {
    inner: RichTextSuperscript,
}

#[deprecated]
pub type RTDRichTextSuperscriptBuilder = RichTextSuperscriptBuilder;

impl RichTextSuperscriptBuilder {
    pub fn build(&self) -> RichTextSuperscript {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<RichTextSuperscript> for RichTextSuperscript {
    fn as_ref(&self) -> &RichTextSuperscript {
        self
    }
}

impl AsRef<RichTextSuperscript> for RichTextSuperscriptBuilder {
    fn as_ref(&self) -> &RichTextSuperscript {
        &self.inner
    }
}

/// An underlined rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextUnderline {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
}

impl RObject for RichTextUnderline {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextUnderline {}

impl RichTextUnderline {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextUnderlineBuilder {
        let mut inner = RichTextUnderline::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextUnderlineBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RichTextUnderlineBuilder {
    inner: RichTextUnderline,
}

#[deprecated]
pub type RTDRichTextUnderlineBuilder = RichTextUnderlineBuilder;

impl RichTextUnderlineBuilder {
    pub fn build(&self) -> RichTextUnderline {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }
}

impl AsRef<RichTextUnderline> for RichTextUnderline {
    fn as_ref(&self) -> &RichTextUnderline {
        self
    }
}

impl AsRef<RichTextUnderline> for RichTextUnderlineBuilder {
    fn as_ref(&self) -> &RichTextUnderline {
        &self.inner
    }
}

/// A rich text URL link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(skip_serializing_if = "RichText::_is_default")]
    text: Box<RichText>,
    /// URL

    #[serde(default)]
    url: String,
    /// True, if the URL has cached instant view server-side

    #[serde(default)]
    is_cached: bool,
}

impl RObject for RichTextUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTextUrl {}

impl RichTextUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextUrlBuilder {
        let mut inner = RichTextUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextUrlBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn is_cached(&self) -> bool {
        self.is_cached
    }
}

#[doc(hidden)]
pub struct RichTextUrlBuilder {
    inner: RichTextUrl,
}

#[deprecated]
pub type RTDRichTextUrlBuilder = RichTextUrlBuilder;

impl RichTextUrlBuilder {
    pub fn build(&self) -> RichTextUrl {
        self.inner.clone()
    }

    pub fn text<T: AsRef<Box<RichText>>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn is_cached(&mut self, is_cached: bool) -> &mut Self {
        self.inner.is_cached = is_cached;
        self
    }
}

impl AsRef<RichTextUrl> for RichTextUrl {
    fn as_ref(&self) -> &RichTextUrl {
        self
    }
}

impl AsRef<RichTextUrl> for RichTextUrlBuilder {
    fn as_ref(&self) -> &RichTextUrl {
        &self.inner
    }
}

/// A concatenation of rich texts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTexts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Texts

    #[serde(default)]
    texts: Vec<RichText>,
}

impl RObject for RichTexts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDRichText for RichTexts {}

impl RichTexts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RichTextsBuilder {
        let mut inner = RichTexts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RichTextsBuilder { inner }
    }

    pub fn texts(&self) -> &Vec<RichText> {
        &self.texts
    }
}

#[doc(hidden)]
pub struct RichTextsBuilder {
    inner: RichTexts,
}

#[deprecated]
pub type RTDRichTextsBuilder = RichTextsBuilder;

impl RichTextsBuilder {
    pub fn build(&self) -> RichTexts {
        self.inner.clone()
    }

    pub fn texts(&mut self, texts: Vec<RichText>) -> &mut Self {
        self.inner.texts = texts;
        self
    }
}

impl AsRef<RichTexts> for RichTexts {
    fn as_ref(&self) -> &RichTexts {
        self
    }
}

impl AsRef<RichTexts> for RichTextsBuilder {
    fn as_ref(&self) -> &RichTexts {
        &self.inner
    }
}
