use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Resends phone number confirmation code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendPhoneNumberConfirmationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResendPhoneNumberConfirmationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResendPhoneNumberConfirmationCode {}

impl ResendPhoneNumberConfirmationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResendPhoneNumberConfirmationCodeBuilder {
        let mut inner = ResendPhoneNumberConfirmationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resendPhoneNumberConfirmationCode".to_string();

        RTDResendPhoneNumberConfirmationCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDResendPhoneNumberConfirmationCodeBuilder {
    inner: ResendPhoneNumberConfirmationCode,
}

impl RTDResendPhoneNumberConfirmationCodeBuilder {
    pub fn build(&self) -> ResendPhoneNumberConfirmationCode {
        self.inner.clone()
    }
}

impl AsRef<ResendPhoneNumberConfirmationCode> for ResendPhoneNumberConfirmationCode {
    fn as_ref(&self) -> &ResendPhoneNumberConfirmationCode {
        self
    }
}

impl AsRef<ResendPhoneNumberConfirmationCode> for RTDResendPhoneNumberConfirmationCodeBuilder {
    fn as_ref(&self) -> &ResendPhoneNumberConfirmationCode {
        &self.inner
    }
}
