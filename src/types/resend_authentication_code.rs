use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Re-sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitCode, the next_code_type of the result is not null and the server-specified timeout has passed
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ResendAuthenticationCodeBuilder {
        let mut inner = ResendAuthenticationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resendAuthenticationCode".to_string();

        ResendAuthenticationCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ResendAuthenticationCodeBuilder {
    inner: ResendAuthenticationCode,
}

#[deprecated]
pub type RTDResendAuthenticationCodeBuilder = ResendAuthenticationCodeBuilder;

impl ResendAuthenticationCodeBuilder {
    pub fn build(&self) -> ResendAuthenticationCode {
        self.inner.clone()
    }
}

impl AsRef<ResendAuthenticationCode> for ResendAuthenticationCode {
    fn as_ref(&self) -> &ResendAuthenticationCode {
        self
    }
}

impl AsRef<ResendAuthenticationCode> for ResendAuthenticationCodeBuilder {
    fn as_ref(&self) -> &ResendAuthenticationCode {
        &self.inner
    }
}
