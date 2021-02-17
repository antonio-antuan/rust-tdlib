use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Re-sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitCode and the next_code_type of the result is not null
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendAuthenticationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResendAuthenticationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResendAuthenticationCode {}

impl ResendAuthenticationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResendAuthenticationCodeBuilder {
        let mut inner = ResendAuthenticationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resendAuthenticationCode".to_string();

        RTDResendAuthenticationCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDResendAuthenticationCodeBuilder {
    inner: ResendAuthenticationCode,
}

impl RTDResendAuthenticationCodeBuilder {
    pub fn build(&self) -> ResendAuthenticationCode {
        self.inner.clone()
    }
}

impl AsRef<ResendAuthenticationCode> for ResendAuthenticationCode {
    fn as_ref(&self) -> &ResendAuthenticationCode {
        self
    }
}

impl AsRef<ResendAuthenticationCode> for RTDResendAuthenticationCodeBuilder {
    fn as_ref(&self) -> &ResendAuthenticationCode {
        &self.inner
    }
}
