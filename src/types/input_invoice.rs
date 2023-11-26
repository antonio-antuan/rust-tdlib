use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes an invoice to process
pub trait TDInputInvoice: Debug + RObject {}

/// Describes an invoice to process
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum InputInvoice {
    #[doc(hidden)]
    #[default]
    _Default,
    /// An invoice from a message of the type messageInvoice
    #[serde(rename = "inputInvoiceMessage")]
    Message(InputInvoiceMessage),
    /// An invoice from a link of the type internalLinkTypeInvoice
    #[serde(rename = "inputInvoiceName")]
    Name(InputInvoiceName),
    /// An invoice for a payment toward Telegram; must not be used in the in-store apps
    #[serde(rename = "inputInvoiceTelegram")]
    Telegram(InputInvoiceTelegram),
}

impl RObject for InputInvoice {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputInvoice::Message(t) => t.extra(),
            InputInvoice::Name(t) => t.extra(),
            InputInvoice::Telegram(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputInvoice::Message(t) => t.client_id(),
            InputInvoice::Name(t) => t.client_id(),
            InputInvoice::Telegram(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputInvoice {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputInvoice::_Default)
    }
}

impl AsRef<InputInvoice> for InputInvoice {
    fn as_ref(&self) -> &InputInvoice {
        self
    }
}

/// An invoice from a message of the type messageInvoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInvoiceMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier of the message

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
}

impl RObject for InputInvoiceMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputInvoice for InputInvoiceMessage {}

impl InputInvoiceMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputInvoiceMessageBuilder {
        let mut inner = InputInvoiceMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputInvoiceMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct InputInvoiceMessageBuilder {
    inner: InputInvoiceMessage,
}

#[deprecated]
pub type RTDInputInvoiceMessageBuilder = InputInvoiceMessageBuilder;

impl InputInvoiceMessageBuilder {
    pub fn build(&self) -> InputInvoiceMessage {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }
}

impl AsRef<InputInvoiceMessage> for InputInvoiceMessage {
    fn as_ref(&self) -> &InputInvoiceMessage {
        self
    }
}

impl AsRef<InputInvoiceMessage> for InputInvoiceMessageBuilder {
    fn as_ref(&self) -> &InputInvoiceMessage {
        &self.inner
    }
}

/// An invoice from a link of the type internalLinkTypeInvoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInvoiceName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the invoice

    #[serde(default)]
    name: String,
}

impl RObject for InputInvoiceName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputInvoice for InputInvoiceName {}

impl InputInvoiceName {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputInvoiceNameBuilder {
        let mut inner = InputInvoiceName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputInvoiceNameBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct InputInvoiceNameBuilder {
    inner: InputInvoiceName,
}

#[deprecated]
pub type RTDInputInvoiceNameBuilder = InputInvoiceNameBuilder;

impl InputInvoiceNameBuilder {
    pub fn build(&self) -> InputInvoiceName {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<InputInvoiceName> for InputInvoiceName {
    fn as_ref(&self) -> &InputInvoiceName {
        self
    }
}

impl AsRef<InputInvoiceName> for InputInvoiceNameBuilder {
    fn as_ref(&self) -> &InputInvoiceName {
        &self.inner
    }
}

/// An invoice for a payment toward Telegram; must not be used in the in-store apps
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInvoiceTelegram {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Transaction purpose

    #[serde(skip_serializing_if = "TelegramPaymentPurpose::_is_default")]
    purpose: TelegramPaymentPurpose,
}

impl RObject for InputInvoiceTelegram {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputInvoice for InputInvoiceTelegram {}

impl InputInvoiceTelegram {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputInvoiceTelegramBuilder {
        let mut inner = InputInvoiceTelegram::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputInvoiceTelegramBuilder { inner }
    }

    pub fn purpose(&self) -> &TelegramPaymentPurpose {
        &self.purpose
    }
}

#[doc(hidden)]
pub struct InputInvoiceTelegramBuilder {
    inner: InputInvoiceTelegram,
}

#[deprecated]
pub type RTDInputInvoiceTelegramBuilder = InputInvoiceTelegramBuilder;

impl InputInvoiceTelegramBuilder {
    pub fn build(&self) -> InputInvoiceTelegram {
        self.inner.clone()
    }

    pub fn purpose<T: AsRef<TelegramPaymentPurpose>>(&mut self, purpose: T) -> &mut Self {
        self.inner.purpose = purpose.as_ref().clone();
        self
    }
}

impl AsRef<InputInvoiceTelegram> for InputInvoiceTelegram {
    fn as_ref(&self) -> &InputInvoiceTelegram {
        self
    }
}

impl AsRef<InputInvoiceTelegram> for InputInvoiceTelegramBuilder {
    fn as_ref(&self) -> &InputInvoiceTelegram {
        &self.inner
    }
}
