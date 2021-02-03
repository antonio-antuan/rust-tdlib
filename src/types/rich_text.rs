use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a text object inside an instant-view web page
pub trait TDRichText: Debug + RObject {}

/// Describes a text object inside an instant-view web page
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RichText {
    #[doc(hidden)]
    _Default(()),
    /// An anchor
    Anchor(RichTextAnchor),
    /// A link to an anchor on the same web page
    AnchorLink(RichTextAnchorLink),
    /// A bold rich text
    Bold(RichTextBold),
    /// A rich text email link
    EmailAddress(RichTextEmailAddress),
    /// A fixed-width rich text
    Fixed(RichTextFixed),
    /// A small image inside the text
    Icon(RichTextIcon),
    /// An italicized rich text
    Italic(RichTextItalic),
    /// A marked rich text
    Marked(RichTextMarked),
    /// A rich text phone number
    PhoneNumber(RichTextPhoneNumber),
    /// A plain text
    Plain(RichTextPlain),
    /// A reference to a richTexts object on the same web page
    Reference(RichTextReference),
    /// A strikethrough rich text
    Strikethrough(RichTextStrikethrough),
    /// A subscript rich text
    Subscript(RichTextSubscript),
    /// A superscript rich text
    Superscript(RichTextSuperscript),
    /// An underlined rich text
    Underline(RichTextUnderline),
    /// A rich text URL link
    Url(RichTextUrl),
    /// A concatenation of rich texts
    RichTexts(RichTexts),
}

impl Default for RichText {
    fn default() -> Self {
        RichText::_Default(())
    }
}

impl<'de> Deserialize<'de> for RichText {
    fn deserialize<D>(deserializer: D) -> Result<RichText, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          RichText,
          (richTextAnchor, Anchor);
          (richTextAnchorLink, AnchorLink);
          (richTextBold, Bold);
          (richTextEmailAddress, EmailAddress);
          (richTextFixed, Fixed);
          (richTextIcon, Icon);
          (richTextItalic, Italic);
          (richTextMarked, Marked);
          (richTextPhoneNumber, PhoneNumber);
          (richTextPlain, Plain);
          (richTextReference, Reference);
          (richTextStrikethrough, Strikethrough);
          (richTextSubscript, Subscript);
          (richTextSuperscript, Superscript);
          (richTextUnderline, Underline);
          (richTextUrl, Url);
          (richTexts, RichTexts);

        )(deserializer)
    }
}

impl RObject for RichText {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            RichText::Anchor(t) => t.td_name(),
            RichText::AnchorLink(t) => t.td_name(),
            RichText::Bold(t) => t.td_name(),
            RichText::EmailAddress(t) => t.td_name(),
            RichText::Fixed(t) => t.td_name(),
            RichText::Icon(t) => t.td_name(),
            RichText::Italic(t) => t.td_name(),
            RichText::Marked(t) => t.td_name(),
            RichText::PhoneNumber(t) => t.td_name(),
            RichText::Plain(t) => t.td_name(),
            RichText::Reference(t) => t.td_name(),
            RichText::Strikethrough(t) => t.td_name(),
            RichText::Subscript(t) => t.td_name(),
            RichText::Superscript(t) => t.td_name(),
            RichText::Underline(t) => t.td_name(),
            RichText::Url(t) => t.td_name(),
            RichText::RichTexts(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
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
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, RichText::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Anchor name
    name: String,
}

impl RObject for RichTextAnchor {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextAnchor"
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

impl TDRichText for RichTextAnchor {}

impl RichTextAnchor {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextAnchorBuilder {
        let mut inner = RichTextAnchor::default();
        inner.td_name = "richTextAnchor".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextAnchorBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct RTDRichTextAnchorBuilder {
    inner: RichTextAnchor,
}

impl RTDRichTextAnchorBuilder {
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

impl AsRef<RichTextAnchor> for RTDRichTextAnchorBuilder {
    fn as_ref(&self) -> &RichTextAnchor {
        &self.inner
    }
}

/// A link to an anchor on the same web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextAnchorLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The link text
    text: Box<RichText>,
    /// The anchor name. If the name is empty, the link should bring back to top
    anchor_name: String,
    /// An HTTP URL, opening the anchor
    url: String,
}

impl RObject for RichTextAnchorLink {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextAnchorLink"
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

impl TDRichText for RichTextAnchorLink {}

impl RichTextAnchorLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextAnchorLinkBuilder {
        let mut inner = RichTextAnchorLink::default();
        inner.td_name = "richTextAnchorLink".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextAnchorLinkBuilder { inner }
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
pub struct RTDRichTextAnchorLinkBuilder {
    inner: RichTextAnchorLink,
}

impl RTDRichTextAnchorLinkBuilder {
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

impl AsRef<RichTextAnchorLink> for RTDRichTextAnchorLinkBuilder {
    fn as_ref(&self) -> &RichTextAnchorLink {
        &self.inner
    }
}

/// A bold rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextBold {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
}

impl RObject for RichTextBold {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextBold"
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

impl TDRichText for RichTextBold {}

impl RichTextBold {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextBoldBuilder {
        let mut inner = RichTextBold::default();
        inner.td_name = "richTextBold".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextBoldBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDRichTextBoldBuilder {
    inner: RichTextBold,
}

impl RTDRichTextBoldBuilder {
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

impl AsRef<RichTextBold> for RTDRichTextBoldBuilder {
    fn as_ref(&self) -> &RichTextBold {
        &self.inner
    }
}

/// A rich text email link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
    /// Email address
    email_address: String,
}

impl RObject for RichTextEmailAddress {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextEmailAddress"
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

impl TDRichText for RichTextEmailAddress {}

impl RichTextEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextEmailAddressBuilder {
        let mut inner = RichTextEmailAddress::default();
        inner.td_name = "richTextEmailAddress".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextEmailAddressBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }

    pub fn email_address(&self) -> &String {
        &self.email_address
    }
}

#[doc(hidden)]
pub struct RTDRichTextEmailAddressBuilder {
    inner: RichTextEmailAddress,
}

impl RTDRichTextEmailAddressBuilder {
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

impl AsRef<RichTextEmailAddress> for RTDRichTextEmailAddressBuilder {
    fn as_ref(&self) -> &RichTextEmailAddress {
        &self.inner
    }
}

/// A fixed-width rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextFixed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
}

impl RObject for RichTextFixed {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextFixed"
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

impl TDRichText for RichTextFixed {}

impl RichTextFixed {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextFixedBuilder {
        let mut inner = RichTextFixed::default();
        inner.td_name = "richTextFixed".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextFixedBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDRichTextFixedBuilder {
    inner: RichTextFixed,
}

impl RTDRichTextFixedBuilder {
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

impl AsRef<RichTextFixed> for RTDRichTextFixedBuilder {
    fn as_ref(&self) -> &RichTextFixed {
        &self.inner
    }
}

/// A small image inside the text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextIcon {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The image represented as a document. The image can be in GIF, JPEG or PNG format
    document: Document,
    /// Width of a bounding box in which the image should be shown; 0 if unknown
    width: i32,
    /// Height of a bounding box in which the image should be shown; 0 if unknown
    height: i32,
}

impl RObject for RichTextIcon {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextIcon"
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

impl TDRichText for RichTextIcon {}

impl RichTextIcon {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextIconBuilder {
        let mut inner = RichTextIcon::default();
        inner.td_name = "richTextIcon".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextIconBuilder { inner }
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
pub struct RTDRichTextIconBuilder {
    inner: RichTextIcon,
}

impl RTDRichTextIconBuilder {
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

impl AsRef<RichTextIcon> for RTDRichTextIconBuilder {
    fn as_ref(&self) -> &RichTextIcon {
        &self.inner
    }
}

/// An italicized rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextItalic {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
}

impl RObject for RichTextItalic {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextItalic"
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

impl TDRichText for RichTextItalic {}

impl RichTextItalic {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextItalicBuilder {
        let mut inner = RichTextItalic::default();
        inner.td_name = "richTextItalic".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextItalicBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDRichTextItalicBuilder {
    inner: RichTextItalic,
}

impl RTDRichTextItalicBuilder {
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

impl AsRef<RichTextItalic> for RTDRichTextItalicBuilder {
    fn as_ref(&self) -> &RichTextItalic {
        &self.inner
    }
}

/// A marked rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextMarked {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
}

impl RObject for RichTextMarked {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextMarked"
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

impl TDRichText for RichTextMarked {}

impl RichTextMarked {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextMarkedBuilder {
        let mut inner = RichTextMarked::default();
        inner.td_name = "richTextMarked".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextMarkedBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDRichTextMarkedBuilder {
    inner: RichTextMarked,
}

impl RTDRichTextMarkedBuilder {
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

impl AsRef<RichTextMarked> for RTDRichTextMarkedBuilder {
    fn as_ref(&self) -> &RichTextMarked {
        &self.inner
    }
}

/// A rich text phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
    /// Phone number
    phone_number: String,
}

impl RObject for RichTextPhoneNumber {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextPhoneNumber"
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

impl TDRichText for RichTextPhoneNumber {}

impl RichTextPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextPhoneNumberBuilder {
        let mut inner = RichTextPhoneNumber::default();
        inner.td_name = "richTextPhoneNumber".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextPhoneNumberBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }
}

#[doc(hidden)]
pub struct RTDRichTextPhoneNumberBuilder {
    inner: RichTextPhoneNumber,
}

impl RTDRichTextPhoneNumberBuilder {
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

impl AsRef<RichTextPhoneNumber> for RTDRichTextPhoneNumberBuilder {
    fn as_ref(&self) -> &RichTextPhoneNumber {
        &self.inner
    }
}

/// A plain text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextPlain {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
}

impl RObject for RichTextPlain {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextPlain"
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

impl TDRichText for RichTextPlain {}

impl RichTextPlain {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextPlainBuilder {
        let mut inner = RichTextPlain::default();
        inner.td_name = "richTextPlain".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextPlainBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDRichTextPlainBuilder {
    inner: RichTextPlain,
}

impl RTDRichTextPlainBuilder {
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

impl AsRef<RichTextPlain> for RTDRichTextPlainBuilder {
    fn as_ref(&self) -> &RichTextPlain {
        &self.inner
    }
}

/// A reference to a richTexts object on the same web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextReference {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The text
    text: Box<RichText>,
    /// The name of a richTextAnchor object, which is the first element of the target richTexts object
    anchor_name: String,
    /// An HTTP URL, opening the reference
    url: String,
}

impl RObject for RichTextReference {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextReference"
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

impl TDRichText for RichTextReference {}

impl RichTextReference {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextReferenceBuilder {
        let mut inner = RichTextReference::default();
        inner.td_name = "richTextReference".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextReferenceBuilder { inner }
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
pub struct RTDRichTextReferenceBuilder {
    inner: RichTextReference,
}

impl RTDRichTextReferenceBuilder {
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

impl AsRef<RichTextReference> for RTDRichTextReferenceBuilder {
    fn as_ref(&self) -> &RichTextReference {
        &self.inner
    }
}

/// A strikethrough rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextStrikethrough {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
}

impl RObject for RichTextStrikethrough {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextStrikethrough"
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

impl TDRichText for RichTextStrikethrough {}

impl RichTextStrikethrough {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextStrikethroughBuilder {
        let mut inner = RichTextStrikethrough::default();
        inner.td_name = "richTextStrikethrough".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextStrikethroughBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDRichTextStrikethroughBuilder {
    inner: RichTextStrikethrough,
}

impl RTDRichTextStrikethroughBuilder {
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

impl AsRef<RichTextStrikethrough> for RTDRichTextStrikethroughBuilder {
    fn as_ref(&self) -> &RichTextStrikethrough {
        &self.inner
    }
}

/// A subscript rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextSubscript {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
}

impl RObject for RichTextSubscript {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextSubscript"
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

impl TDRichText for RichTextSubscript {}

impl RichTextSubscript {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextSubscriptBuilder {
        let mut inner = RichTextSubscript::default();
        inner.td_name = "richTextSubscript".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextSubscriptBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDRichTextSubscriptBuilder {
    inner: RichTextSubscript,
}

impl RTDRichTextSubscriptBuilder {
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

impl AsRef<RichTextSubscript> for RTDRichTextSubscriptBuilder {
    fn as_ref(&self) -> &RichTextSubscript {
        &self.inner
    }
}

/// A superscript rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextSuperscript {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
}

impl RObject for RichTextSuperscript {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextSuperscript"
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

impl TDRichText for RichTextSuperscript {}

impl RichTextSuperscript {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextSuperscriptBuilder {
        let mut inner = RichTextSuperscript::default();
        inner.td_name = "richTextSuperscript".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextSuperscriptBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDRichTextSuperscriptBuilder {
    inner: RichTextSuperscript,
}

impl RTDRichTextSuperscriptBuilder {
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

impl AsRef<RichTextSuperscript> for RTDRichTextSuperscriptBuilder {
    fn as_ref(&self) -> &RichTextSuperscript {
        &self.inner
    }
}

/// An underlined rich text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextUnderline {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
}

impl RObject for RichTextUnderline {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextUnderline"
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

impl TDRichText for RichTextUnderline {}

impl RichTextUnderline {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextUnderlineBuilder {
        let mut inner = RichTextUnderline::default();
        inner.td_name = "richTextUnderline".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextUnderlineBuilder { inner }
    }

    pub fn text(&self) -> &Box<RichText> {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDRichTextUnderlineBuilder {
    inner: RichTextUnderline,
}

impl RTDRichTextUnderlineBuilder {
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

impl AsRef<RichTextUnderline> for RTDRichTextUnderlineBuilder {
    fn as_ref(&self) -> &RichTextUnderline {
        &self.inner
    }
}

/// A rich text URL link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text
    text: Box<RichText>,
    /// URL
    url: String,
    /// True, if the URL has cached instant view server-side
    is_cached: bool,
}

impl RObject for RichTextUrl {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTextUrl"
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

impl TDRichText for RichTextUrl {}

impl RichTextUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextUrlBuilder {
        let mut inner = RichTextUrl::default();
        inner.td_name = "richTextUrl".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextUrlBuilder { inner }
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
pub struct RTDRichTextUrlBuilder {
    inner: RichTextUrl,
}

impl RTDRichTextUrlBuilder {
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

impl AsRef<RichTextUrl> for RTDRichTextUrlBuilder {
    fn as_ref(&self) -> &RichTextUrl {
        &self.inner
    }
}

/// A concatenation of rich texts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTexts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Texts
    texts: Vec<RichText>,
}

impl RObject for RichTexts {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "richTexts"
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

impl TDRichText for RichTexts {}

impl RichTexts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRichTextsBuilder {
        let mut inner = RichTexts::default();
        inner.td_name = "richTexts".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDRichTextsBuilder { inner }
    }

    pub fn texts(&self) -> &Vec<RichText> {
        &self.texts
    }
}

#[doc(hidden)]
pub struct RTDRichTextsBuilder {
    inner: RichTexts,
}

impl RTDRichTextsBuilder {
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

impl AsRef<RichTexts> for RTDRichTextsBuilder {
    fn as_ref(&self) -> &RichTexts {
        &self.inner
    }
}
