use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the email address of the user and sends an authentication code to the email address. Works only when the current authorization state is authorizationStateWaitEmailAddress
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAuthenticationEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The email address of the user

    #[serde(default)]
    email_address: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetAuthenticationEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetAuthenticationEmailAddress {}

impl SetAuthenticationEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetAuthenticationEmailAddressBuilder {
        let mut inner = SetAuthenticationEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setAuthenticationEmailAddress".to_string();

        SetAuthenticationEmailAddressBuilder { inner }
    }

    pub fn email_address(&self) -> &String {
        &self.email_address
    }
}

#[doc(hidden)]
pub struct SetAuthenticationEmailAddressBuilder {
    inner: SetAuthenticationEmailAddress,
}

#[deprecated]
pub type RTDSetAuthenticationEmailAddressBuilder = SetAuthenticationEmailAddressBuilder;

impl SetAuthenticationEmailAddressBuilder {
    pub fn build(&self) -> SetAuthenticationEmailAddress {
        self.inner.clone()
    }

    pub fn email_address<T: AsRef<str>>(&mut self, email_address: T) -> &mut Self {
        self.inner.email_address = email_address.as_ref().to_string();
        self
    }
}

impl AsRef<SetAuthenticationEmailAddress> for SetAuthenticationEmailAddress {
    fn as_ref(&self) -> &SetAuthenticationEmailAddress {
        self
    }
}

impl AsRef<SetAuthenticationEmailAddress> for SetAuthenticationEmailAddressBuilder {
    fn as_ref(&self) -> &SetAuthenticationEmailAddress {
        &self.inner
    }
}
