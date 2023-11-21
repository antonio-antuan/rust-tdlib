use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the login email address of the user. The email address can be changed only if the current user already has login email and passwordState.login_email_address_pattern is non-empty. The change will not be applied until the new login email address is confirmed with checkLoginEmailAddressCode. To use Apple ID/Google ID instead of a email address, call checkLoginEmailAddressCode directly
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetLoginEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New login email address

    #[serde(default)]
    new_login_email_address: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetLoginEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetLoginEmailAddress {}

impl SetLoginEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetLoginEmailAddressBuilder {
        let mut inner = SetLoginEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setLoginEmailAddress".to_string();

        SetLoginEmailAddressBuilder { inner }
    }

    pub fn new_login_email_address(&self) -> &String {
        &self.new_login_email_address
    }
}

#[doc(hidden)]
pub struct SetLoginEmailAddressBuilder {
    inner: SetLoginEmailAddress,
}

#[deprecated]
pub type RTDSetLoginEmailAddressBuilder = SetLoginEmailAddressBuilder;

impl SetLoginEmailAddressBuilder {
    pub fn build(&self) -> SetLoginEmailAddress {
        self.inner.clone()
    }

    pub fn new_login_email_address<T: AsRef<str>>(
        &mut self,
        new_login_email_address: T,
    ) -> &mut Self {
        self.inner.new_login_email_address = new_login_email_address.as_ref().to_string();
        self
    }
}

impl AsRef<SetLoginEmailAddress> for SetLoginEmailAddress {
    fn as_ref(&self) -> &SetLoginEmailAddress {
        self
    }
}

impl AsRef<SetLoginEmailAddress> for SetLoginEmailAddressBuilder {
    fn as_ref(&self) -> &SetLoginEmailAddress {
        &self.inner
    }
}
