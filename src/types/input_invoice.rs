use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describe an invoice to process
pub trait TDInputInvoice: Debug + RObject {}

/// Describe an invoice to process
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputInvoice {
    #[doc(hidden)]
    _Default,
    /// An invoice from a message of the type messageInvoice
    #[serde(rename(deserialize = "inputInvoiceMessage"))]
    Message(InputInvoiceMessage),
    /// An invoice from a link of the type internalLinkTypeInvoice
    #[serde(rename(deserialize = "inputInvoiceName"))]
    Name(InputInvoiceName),
}

impl Default for InputInvoice {
    fn default() -> Self {
        InputInvoice::_Default
    }
}

impl RObject for InputInvoice {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputInvoice::Message(t) => t.extra(),
            InputInvoice::Name(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputInvoice::Message(t) => t.client_id(),
            InputInvoice::Name(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputInvoice {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    chat_id: i64,
    /// Message identifier
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInvoiceMessageBuilder {
        let mut inner = InputInvoiceMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputInvoiceMessageBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct RTDInputInvoiceMessageBuilder {
    inner: InputInvoiceMessage,
}

impl RTDInputInvoiceMessageBuilder {
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

impl AsRef<InputInvoiceMessage> for RTDInputInvoiceMessageBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInvoiceNameBuilder {
        let mut inner = InputInvoiceName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputInvoiceNameBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct RTDInputInvoiceNameBuilder {
    inner: InputInvoiceName,
}

impl RTDInputInvoiceNameBuilder {
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

impl AsRef<InputInvoiceName> for RTDInputInvoiceNameBuilder {
    fn as_ref(&self) -> &InputInvoiceName {
        &self.inner
    }
}
