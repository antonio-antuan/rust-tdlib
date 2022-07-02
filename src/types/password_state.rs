use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents the current state of 2-step verification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PasswordState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if a 2-step verification password is set

    #[serde(default)]
    has_password: bool,
    /// Hint for the password; may be empty

    #[serde(default)]
    password_hint: String,
    /// True, if a recovery email is set

    #[serde(default)]
    has_recovery_email_address: bool,
    /// True, if some Telegram Passport elements were saved

    #[serde(default)]
    has_passport_data: bool,
    /// Information about the recovery email address to which the confirmation email was sent; may be null
    recovery_email_address_code_info: Option<EmailAddressAuthenticationCodeInfo>,
    /// If not 0, point in time (Unix timestamp) after which the password can be reset immediately using resetPassword

    #[serde(default)]
    pending_reset_date: i32,
}

impl RObject for PasswordState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PasswordState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PasswordStateBuilder {
        let mut inner = PasswordState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PasswordStateBuilder { inner }
    }

    pub fn has_password(&self) -> bool {
        self.has_password
    }

    pub fn password_hint(&self) -> &String {
        &self.password_hint
    }

    pub fn has_recovery_email_address(&self) -> bool {
        self.has_recovery_email_address
    }

    pub fn has_passport_data(&self) -> bool {
        self.has_passport_data
    }

    pub fn recovery_email_address_code_info(&self) -> &Option<EmailAddressAuthenticationCodeInfo> {
        &self.recovery_email_address_code_info
    }

    pub fn pending_reset_date(&self) -> i32 {
        self.pending_reset_date
    }
}

#[doc(hidden)]
pub struct PasswordStateBuilder {
    inner: PasswordState,
}

#[deprecated]
pub type RTDPasswordStateBuilder = PasswordStateBuilder;

impl PasswordStateBuilder {
    pub fn build(&self) -> PasswordState {
        self.inner.clone()
    }

    pub fn has_password(&mut self, has_password: bool) -> &mut Self {
        self.inner.has_password = has_password;
        self
    }

    pub fn password_hint<T: AsRef<str>>(&mut self, password_hint: T) -> &mut Self {
        self.inner.password_hint = password_hint.as_ref().to_string();
        self
    }

    pub fn has_recovery_email_address(&mut self, has_recovery_email_address: bool) -> &mut Self {
        self.inner.has_recovery_email_address = has_recovery_email_address;
        self
    }

    pub fn has_passport_data(&mut self, has_passport_data: bool) -> &mut Self {
        self.inner.has_passport_data = has_passport_data;
        self
    }

    pub fn recovery_email_address_code_info<T: AsRef<EmailAddressAuthenticationCodeInfo>>(
        &mut self,
        recovery_email_address_code_info: T,
    ) -> &mut Self {
        self.inner.recovery_email_address_code_info =
            Some(recovery_email_address_code_info.as_ref().clone());
        self
    }

    pub fn pending_reset_date(&mut self, pending_reset_date: i32) -> &mut Self {
        self.inner.pending_reset_date = pending_reset_date;
        self
    }
}

impl AsRef<PasswordState> for PasswordState {
    fn as_ref(&self) -> &PasswordState {
        self
    }
}

impl AsRef<PasswordState> for PasswordStateBuilder {
    fn as_ref(&self) -> &PasswordState {
        &self.inner
    }
}
