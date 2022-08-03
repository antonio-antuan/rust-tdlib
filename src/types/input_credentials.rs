use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about the payment method chosen by the user
pub trait TDInputCredentials: Debug + RObject {}

/// Contains information about the payment method chosen by the user
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputCredentials {
    #[doc(hidden)]
    _Default,
    /// Applies if a user enters new credentials using Apple Pay
    #[serde(rename = "inputCredentialsApplePay")]
    ApplePay(InputCredentialsApplePay),
    /// Applies if a user enters new credentials using Google Pay
    #[serde(rename = "inputCredentialsGooglePay")]
    GooglePay(InputCredentialsGooglePay),
    /// Applies if a user enters new credentials on a payment provider website
    #[serde(rename = "inputCredentialsNew")]
    New(InputCredentialsNew),
    /// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password
    #[serde(rename = "inputCredentialsSaved")]
    Saved(InputCredentialsSaved),
}

impl Default for InputCredentials {
    fn default() -> Self {
        InputCredentials::_Default
    }
}

impl RObject for InputCredentials {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputCredentials::ApplePay(t) => t.extra(),
            InputCredentials::GooglePay(t) => t.extra(),
            InputCredentials::New(t) => t.extra(),
            InputCredentials::Saved(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputCredentials::ApplePay(t) => t.client_id(),
            InputCredentials::GooglePay(t) => t.client_id(),
            InputCredentials::New(t) => t.client_id(),
            InputCredentials::Saved(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputCredentials {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputCredentials::_Default)
    }
}

impl AsRef<InputCredentials> for InputCredentials {
    fn as_ref(&self) -> &InputCredentials {
        self
    }
}

/// Applies if a user enters new credentials using Apple Pay
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsApplePay {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// JSON-encoded data with the credential identifier

    #[serde(default)]
    data: String,
}

impl RObject for InputCredentialsApplePay {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputCredentials for InputCredentialsApplePay {}

impl InputCredentialsApplePay {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputCredentialsApplePayBuilder {
        let mut inner = InputCredentialsApplePay::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputCredentialsApplePayBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct InputCredentialsApplePayBuilder {
    inner: InputCredentialsApplePay,
}

#[deprecated]
pub type RTDInputCredentialsApplePayBuilder = InputCredentialsApplePayBuilder;

impl InputCredentialsApplePayBuilder {
    pub fn build(&self) -> InputCredentialsApplePay {
        self.inner.clone()
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<InputCredentialsApplePay> for InputCredentialsApplePay {
    fn as_ref(&self) -> &InputCredentialsApplePay {
        self
    }
}

impl AsRef<InputCredentialsApplePay> for InputCredentialsApplePayBuilder {
    fn as_ref(&self) -> &InputCredentialsApplePay {
        &self.inner
    }
}

/// Applies if a user enters new credentials using Google Pay
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsGooglePay {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// JSON-encoded data with the credential identifier

    #[serde(default)]
    data: String,
}

impl RObject for InputCredentialsGooglePay {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputCredentials for InputCredentialsGooglePay {}

impl InputCredentialsGooglePay {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputCredentialsGooglePayBuilder {
        let mut inner = InputCredentialsGooglePay::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputCredentialsGooglePayBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct InputCredentialsGooglePayBuilder {
    inner: InputCredentialsGooglePay,
}

#[deprecated]
pub type RTDInputCredentialsGooglePayBuilder = InputCredentialsGooglePayBuilder;

impl InputCredentialsGooglePayBuilder {
    pub fn build(&self) -> InputCredentialsGooglePay {
        self.inner.clone()
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<InputCredentialsGooglePay> for InputCredentialsGooglePay {
    fn as_ref(&self) -> &InputCredentialsGooglePay {
        self
    }
}

impl AsRef<InputCredentialsGooglePay> for InputCredentialsGooglePayBuilder {
    fn as_ref(&self) -> &InputCredentialsGooglePay {
        &self.inner
    }
}

/// Applies if a user enters new credentials on a payment provider website
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsNew {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// JSON-encoded data with the credential identifier from the payment provider

    #[serde(default)]
    data: String,
    /// True, if the credential identifier can be saved on the server side

    #[serde(default)]
    allow_save: bool,
}

impl RObject for InputCredentialsNew {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputCredentials for InputCredentialsNew {}

impl InputCredentialsNew {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputCredentialsNewBuilder {
        let mut inner = InputCredentialsNew::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputCredentialsNewBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn allow_save(&self) -> bool {
        self.allow_save
    }
}

#[doc(hidden)]
pub struct InputCredentialsNewBuilder {
    inner: InputCredentialsNew,
}

#[deprecated]
pub type RTDInputCredentialsNewBuilder = InputCredentialsNewBuilder;

impl InputCredentialsNewBuilder {
    pub fn build(&self) -> InputCredentialsNew {
        self.inner.clone()
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }

    pub fn allow_save(&mut self, allow_save: bool) -> &mut Self {
        self.inner.allow_save = allow_save;
        self
    }
}

impl AsRef<InputCredentialsNew> for InputCredentialsNew {
    fn as_ref(&self) -> &InputCredentialsNew {
        self
    }
}

impl AsRef<InputCredentialsNew> for InputCredentialsNewBuilder {
    fn as_ref(&self) -> &InputCredentialsNew {
        &self.inner
    }
}

/// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsSaved {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the saved credentials

    #[serde(default)]
    saved_credentials_id: String,
}

impl RObject for InputCredentialsSaved {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputCredentials for InputCredentialsSaved {}

impl InputCredentialsSaved {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputCredentialsSavedBuilder {
        let mut inner = InputCredentialsSaved::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputCredentialsSavedBuilder { inner }
    }

    pub fn saved_credentials_id(&self) -> &String {
        &self.saved_credentials_id
    }
}

#[doc(hidden)]
pub struct InputCredentialsSavedBuilder {
    inner: InputCredentialsSaved,
}

#[deprecated]
pub type RTDInputCredentialsSavedBuilder = InputCredentialsSavedBuilder;

impl InputCredentialsSavedBuilder {
    pub fn build(&self) -> InputCredentialsSaved {
        self.inner.clone()
    }

    pub fn saved_credentials_id<T: AsRef<str>>(&mut self, saved_credentials_id: T) -> &mut Self {
        self.inner.saved_credentials_id = saved_credentials_id.as_ref().to_string();
        self
    }
}

impl AsRef<InputCredentialsSaved> for InputCredentialsSaved {
    fn as_ref(&self) -> &InputCredentialsSaved {
        self
    }
}

impl AsRef<InputCredentialsSaved> for InputCredentialsSavedBuilder {
    fn as_ref(&self) -> &InputCredentialsSaved {
        &self.inner
    }
}
