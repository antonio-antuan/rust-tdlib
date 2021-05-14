use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Checks the authentication code. Works only when the current authorization state is authorizationStateWaitCode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckAuthenticationCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The verification code received via SMS, Telegram message, phone call, or flash call
    code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckAuthenticationCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckAuthenticationCode {}

impl CheckAuthenticationCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCheckAuthenticationCodeBuilder {
        let mut inner = CheckAuthenticationCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkAuthenticationCode".to_string();

        RTDCheckAuthenticationCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct RTDCheckAuthenticationCodeBuilder {
    inner: CheckAuthenticationCode,
}

impl RTDCheckAuthenticationCodeBuilder {
    pub fn build(&self) -> CheckAuthenticationCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<CheckAuthenticationCode> for CheckAuthenticationCode {
    fn as_ref(&self) -> &CheckAuthenticationCode {
        self
    }
}

impl AsRef<CheckAuthenticationCode> for RTDCheckAuthenticationCodeBuilder {
    fn as_ref(&self) -> &CheckAuthenticationCode {
        &self.inner
    }
}
