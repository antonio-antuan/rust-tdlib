use crate::errors::Result;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ResendPhoneNumberConfirmationCodeBuilder {
        let mut inner = ResendPhoneNumberConfirmationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resendPhoneNumberConfirmationCode".to_string();

        ResendPhoneNumberConfirmationCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ResendPhoneNumberConfirmationCodeBuilder {
    inner: ResendPhoneNumberConfirmationCode,
}

#[deprecated]
pub type RTDResendPhoneNumberConfirmationCodeBuilder = ResendPhoneNumberConfirmationCodeBuilder;

impl ResendPhoneNumberConfirmationCodeBuilder {
    pub fn build(&self) -> ResendPhoneNumberConfirmationCode {
        self.inner.clone()
    }
}

impl AsRef<ResendPhoneNumberConfirmationCode> for ResendPhoneNumberConfirmationCode {
    fn as_ref(&self) -> &ResendPhoneNumberConfirmationCode {
        self
    }
}

impl AsRef<ResendPhoneNumberConfirmationCode> for ResendPhoneNumberConfirmationCodeBuilder {
    fn as_ref(&self) -> &ResendPhoneNumberConfirmationCode {
        &self.inner
    }
}
