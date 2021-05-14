use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the password for the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Previous password of the user
    old_password: String,
    /// New password of the user; may be empty to remove the password
    new_password: String,
    /// New password hint; may be empty
    new_hint: String,
    /// Pass true if the recovery email address should be changed
    set_recovery_email_address: bool,
    /// New recovery email address; may be empty
    new_recovery_email_address: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetPassword {}

impl SetPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetPasswordBuilder {
        let mut inner = SetPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setPassword".to_string();

        RTDSetPasswordBuilder { inner }
    }

    pub fn old_password(&self) -> &String {
        &self.old_password
    }

    pub fn new_password(&self) -> &String {
        &self.new_password
    }

    pub fn new_hint(&self) -> &String {
        &self.new_hint
    }

    pub fn set_recovery_email_address(&self) -> bool {
        self.set_recovery_email_address
    }

    pub fn new_recovery_email_address(&self) -> &String {
        &self.new_recovery_email_address
    }
}

#[doc(hidden)]
pub struct RTDSetPasswordBuilder {
    inner: SetPassword,
}

impl RTDSetPasswordBuilder {
    pub fn build(&self) -> SetPassword {
        self.inner.clone()
    }

    pub fn old_password<T: AsRef<str>>(&mut self, old_password: T) -> &mut Self {
        self.inner.old_password = old_password.as_ref().to_string();
        self
    }

    pub fn new_password<T: AsRef<str>>(&mut self, new_password: T) -> &mut Self {
        self.inner.new_password = new_password.as_ref().to_string();
        self
    }

    pub fn new_hint<T: AsRef<str>>(&mut self, new_hint: T) -> &mut Self {
        self.inner.new_hint = new_hint.as_ref().to_string();
        self
    }

    pub fn set_recovery_email_address(&mut self, set_recovery_email_address: bool) -> &mut Self {
        self.inner.set_recovery_email_address = set_recovery_email_address;
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

impl AsRef<SetPassword> for SetPassword {
    fn as_ref(&self) -> &SetPassword {
        self
    }
}

impl AsRef<SetPassword> for RTDSetPasswordBuilder {
    fn as_ref(&self) -> &SetPassword {
        &self.inner
    }
}
