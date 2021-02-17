use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Re-sends the authentication code sent to confirm a new phone number for the user. Works only if the previously received authenticationCodeInfo next_code_type was not null
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendChangePhoneNumberCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResendChangePhoneNumberCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResendChangePhoneNumberCode {}

impl ResendChangePhoneNumberCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResendChangePhoneNumberCodeBuilder {
        let mut inner = ResendChangePhoneNumberCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resendChangePhoneNumberCode".to_string();

        RTDResendChangePhoneNumberCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDResendChangePhoneNumberCodeBuilder {
    inner: ResendChangePhoneNumberCode,
}

impl RTDResendChangePhoneNumberCodeBuilder {
    pub fn build(&self) -> ResendChangePhoneNumberCode {
        self.inner.clone()
    }
}

impl AsRef<ResendChangePhoneNumberCode> for ResendChangePhoneNumberCode {
    fn as_ref(&self) -> &ResendChangePhoneNumberCode {
        self
    }
}

impl AsRef<ResendChangePhoneNumberCode> for RTDResendChangePhoneNumberCodeBuilder {
    fn as_ref(&self) -> &ResendChangePhoneNumberCode {
        &self.inner
    }
}
