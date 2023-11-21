use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks the authentication of a email address. Works only when the current authorization state is authorizationStateWaitEmailCode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckAuthenticationEmailCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Email address authentication to check

    #[serde(skip_serializing_if = "EmailAddressAuthentication::_is_default")]
    code: EmailAddressAuthentication,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckAuthenticationEmailCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckAuthenticationEmailCode {}

impl CheckAuthenticationEmailCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckAuthenticationEmailCodeBuilder {
        let mut inner = CheckAuthenticationEmailCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkAuthenticationEmailCode".to_string();

        CheckAuthenticationEmailCodeBuilder { inner }
    }

    pub fn code(&self) -> &EmailAddressAuthentication {
        &self.code
    }
}

#[doc(hidden)]
pub struct CheckAuthenticationEmailCodeBuilder {
    inner: CheckAuthenticationEmailCode,
}

#[deprecated]
pub type RTDCheckAuthenticationEmailCodeBuilder = CheckAuthenticationEmailCodeBuilder;

impl CheckAuthenticationEmailCodeBuilder {
    pub fn build(&self) -> CheckAuthenticationEmailCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<EmailAddressAuthentication>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().clone();
        self
    }
}

impl AsRef<CheckAuthenticationEmailCode> for CheckAuthenticationEmailCode {
    fn as_ref(&self) -> &CheckAuthenticationEmailCode {
        self
    }
}

impl AsRef<CheckAuthenticationEmailCode> for CheckAuthenticationEmailCodeBuilder {
    fn as_ref(&self) -> &CheckAuthenticationEmailCode {
        &self.inner
    }
}
