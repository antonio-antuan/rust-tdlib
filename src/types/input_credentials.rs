use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about the payment method chosen by the user
pub trait TDInputCredentials: Debug + RObject {}

/// Contains information about the payment method chosen by the user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputCredentials {
    #[doc(hidden)]
    _Default(()),
    /// Applies if a user enters new credentials using Android Pay
    AndroidPay(InputCredentialsAndroidPay),
    /// Applies if a user enters new credentials using Apple Pay
    ApplePay(InputCredentialsApplePay),
    /// Applies if a user enters new credentials on a payment provider website
    New(InputCredentialsNew),
    /// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password
    Saved(InputCredentialsSaved),
}

impl Default for InputCredentials {
    fn default() -> Self {
        InputCredentials::_Default(())
    }
}

impl<'de> Deserialize<'de> for InputCredentials {
    fn deserialize<D>(deserializer: D) -> Result<InputCredentials, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          InputCredentials,
          (inputCredentialsAndroidPay, AndroidPay);
          (inputCredentialsApplePay, ApplePay);
          (inputCredentialsNew, New);
          (inputCredentialsSaved, Saved);

        )(deserializer)
    }
}

impl RObject for InputCredentials {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            InputCredentials::AndroidPay(t) => t.td_name(),
            InputCredentials::ApplePay(t) => t.td_name(),
            InputCredentials::New(t) => t.td_name(),
            InputCredentials::Saved(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            InputCredentials::AndroidPay(t) => t.extra(),
            InputCredentials::ApplePay(t) => t.extra(),
            InputCredentials::New(t) => t.extra(),
            InputCredentials::Saved(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputCredentials::AndroidPay(t) => t.client_id(),
            InputCredentials::ApplePay(t) => t.client_id(),
            InputCredentials::New(t) => t.client_id(),
            InputCredentials::Saved(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputCredentials {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputCredentials::_Default(_))
    }
}

impl AsRef<InputCredentials> for InputCredentials {
    fn as_ref(&self) -> &InputCredentials {
        self
    }
}

/// Applies if a user enters new credentials using Android Pay
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsAndroidPay {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// JSON-encoded data with the credential identifier
    data: String,
}

impl RObject for InputCredentialsAndroidPay {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputCredentialsAndroidPay"
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

impl TDInputCredentials for InputCredentialsAndroidPay {}

impl InputCredentialsAndroidPay {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputCredentialsAndroidPayBuilder {
        let mut inner = InputCredentialsAndroidPay::default();
        inner.td_name = "inputCredentialsAndroidPay".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDInputCredentialsAndroidPayBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDInputCredentialsAndroidPayBuilder {
    inner: InputCredentialsAndroidPay,
}

impl RTDInputCredentialsAndroidPayBuilder {
    pub fn build(&self) -> InputCredentialsAndroidPay {
        self.inner.clone()
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<InputCredentialsAndroidPay> for InputCredentialsAndroidPay {
    fn as_ref(&self) -> &InputCredentialsAndroidPay {
        self
    }
}

impl AsRef<InputCredentialsAndroidPay> for RTDInputCredentialsAndroidPayBuilder {
    fn as_ref(&self) -> &InputCredentialsAndroidPay {
        &self.inner
    }
}

/// Applies if a user enters new credentials using Apple Pay
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsApplePay {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// JSON-encoded data with the credential identifier
    data: String,
}

impl RObject for InputCredentialsApplePay {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputCredentialsApplePay"
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

impl TDInputCredentials for InputCredentialsApplePay {}

impl InputCredentialsApplePay {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputCredentialsApplePayBuilder {
        let mut inner = InputCredentialsApplePay::default();
        inner.td_name = "inputCredentialsApplePay".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDInputCredentialsApplePayBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDInputCredentialsApplePayBuilder {
    inner: InputCredentialsApplePay,
}

impl RTDInputCredentialsApplePayBuilder {
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

impl AsRef<InputCredentialsApplePay> for RTDInputCredentialsApplePayBuilder {
    fn as_ref(&self) -> &InputCredentialsApplePay {
        &self.inner
    }
}

/// Applies if a user enters new credentials on a payment provider website
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsNew {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Contains JSON-encoded data with a credential identifier from the payment provider
    data: String,
    /// True, if the credential identifier can be saved on the server side
    allow_save: bool,
}

impl RObject for InputCredentialsNew {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputCredentialsNew"
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

impl TDInputCredentials for InputCredentialsNew {}

impl InputCredentialsNew {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputCredentialsNewBuilder {
        let mut inner = InputCredentialsNew::default();
        inner.td_name = "inputCredentialsNew".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDInputCredentialsNewBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn allow_save(&self) -> bool {
        self.allow_save
    }
}

#[doc(hidden)]
pub struct RTDInputCredentialsNewBuilder {
    inner: InputCredentialsNew,
}

impl RTDInputCredentialsNewBuilder {
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

impl AsRef<InputCredentialsNew> for RTDInputCredentialsNewBuilder {
    fn as_ref(&self) -> &InputCredentialsNew {
        &self.inner
    }
}

/// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputCredentialsSaved {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the saved credentials
    saved_credentials_id: String,
}

impl RObject for InputCredentialsSaved {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputCredentialsSaved"
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

impl TDInputCredentials for InputCredentialsSaved {}

impl InputCredentialsSaved {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputCredentialsSavedBuilder {
        let mut inner = InputCredentialsSaved::default();
        inner.td_name = "inputCredentialsSaved".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDInputCredentialsSavedBuilder { inner }
    }

    pub fn saved_credentials_id(&self) -> &String {
        &self.saved_credentials_id
    }
}

#[doc(hidden)]
pub struct RTDInputCredentialsSavedBuilder {
    inner: InputCredentialsSaved,
}

impl RTDInputCredentialsSavedBuilder {
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

impl AsRef<InputCredentialsSaved> for RTDInputCredentialsSavedBuilder {
    fn as_ref(&self) -> &InputCredentialsSaved {
        &self.inner
    }
}
