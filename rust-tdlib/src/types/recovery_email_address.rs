use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about the current recovery email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecoveryEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Recovery email address
    recovery_email_address: String,
}

impl RObject for RecoveryEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RecoveryEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRecoveryEmailAddressBuilder {
        let mut inner = RecoveryEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDRecoveryEmailAddressBuilder { inner }
    }

    pub fn recovery_email_address(&self) -> &String {
        &self.recovery_email_address
    }
}

#[doc(hidden)]
pub struct RTDRecoveryEmailAddressBuilder {
    inner: RecoveryEmailAddress,
}

impl RTDRecoveryEmailAddressBuilder {
    pub fn build(&self) -> RecoveryEmailAddress {
        self.inner.clone()
    }

    pub fn recovery_email_address<T: AsRef<str>>(
        &mut self,
        recovery_email_address: T,
    ) -> &mut Self {
        self.inner.recovery_email_address = recovery_email_address.as_ref().to_string();
        self
    }
}

impl AsRef<RecoveryEmailAddress> for RecoveryEmailAddress {
    fn as_ref(&self) -> &RecoveryEmailAddress {
        self
    }
}

impl AsRef<RecoveryEmailAddress> for RTDRecoveryEmailAddressBuilder {
    fn as_ref(&self) -> &RecoveryEmailAddress {
        &self.inner
    }
}
