use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Resends the 2-step verification recovery email address verification code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendRecoveryEmailAddressCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResendRecoveryEmailAddressCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResendRecoveryEmailAddressCode {}

impl ResendRecoveryEmailAddressCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResendRecoveryEmailAddressCodeBuilder {
        let mut inner = ResendRecoveryEmailAddressCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resendRecoveryEmailAddressCode".to_string();

        RTDResendRecoveryEmailAddressCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDResendRecoveryEmailAddressCodeBuilder {
    inner: ResendRecoveryEmailAddressCode,
}

impl RTDResendRecoveryEmailAddressCodeBuilder {
    pub fn build(&self) -> ResendRecoveryEmailAddressCode {
        self.inner.clone()
    }
}

impl AsRef<ResendRecoveryEmailAddressCode> for ResendRecoveryEmailAddressCode {
    fn as_ref(&self) -> &ResendRecoveryEmailAddressCode {
        self
    }
}

impl AsRef<ResendRecoveryEmailAddressCode> for RTDResendRecoveryEmailAddressCodeBuilder {
    fn as_ref(&self) -> &ResendRecoveryEmailAddressCode {
        &self.inner
    }
}
