use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed. If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetRecoveryEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Password of the current user

    #[serde(default)]
    password: String,
    /// New recovery email address

    #[serde(default)]
    new_recovery_email_address: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetRecoveryEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetRecoveryEmailAddress {}

impl SetRecoveryEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetRecoveryEmailAddressBuilder {
        let mut inner = SetRecoveryEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setRecoveryEmailAddress".to_string();

        SetRecoveryEmailAddressBuilder { inner }
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn new_recovery_email_address(&self) -> &String {
        &self.new_recovery_email_address
    }
}

#[doc(hidden)]
pub struct SetRecoveryEmailAddressBuilder {
    inner: SetRecoveryEmailAddress,
}

#[deprecated]
pub type RTDSetRecoveryEmailAddressBuilder = SetRecoveryEmailAddressBuilder;

impl SetRecoveryEmailAddressBuilder {
    pub fn build(&self) -> SetRecoveryEmailAddress {
        self.inner.clone()
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }

    pub fn new_recovery_email_address<T: AsRef<str>>(
        &mut self,
        new_recovery_email_address: T,
    ) -> &mut Self {
        self.inner.new_recovery_email_address = new_recovery_email_address.as_ref().to_string();
        self
    }
}

impl AsRef<SetRecoveryEmailAddress> for SetRecoveryEmailAddress {
    fn as_ref(&self) -> &SetRecoveryEmailAddress {
        self
    }
}

impl AsRef<SetRecoveryEmailAddress> for SetRecoveryEmailAddressBuilder {
    fn as_ref(&self) -> &SetRecoveryEmailAddress {
        &self.inner
    }
}
