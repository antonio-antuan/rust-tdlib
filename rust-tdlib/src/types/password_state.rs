use crate::errors::*;
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
    has_password: bool,
    /// Hint for the password; may be empty
    password_hint: String,
    /// True, if a recovery email is set
    has_recovery_email_address: bool,
    /// True, if some Telegram Passport elements were saved
    has_passport_data: bool,
    /// Information about the recovery email address to which the confirmation email was sent; may be null
    recovery_email_address_code_info: Option<EmailAddressAuthenticationCodeInfo>,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPasswordStateBuilder {
        let mut inner = PasswordState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPasswordStateBuilder { inner }
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
}

#[doc(hidden)]
pub struct RTDPasswordStateBuilder {
    inner: PasswordState,
}

impl RTDPasswordStateBuilder {
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
}

impl AsRef<PasswordState> for PasswordState {
    fn as_ref(&self) -> &PasswordState {
        self
    }
}

impl AsRef<PasswordState> for RTDPasswordStateBuilder {
    fn as_ref(&self) -> &PasswordState {
        &self.inner
    }
}
