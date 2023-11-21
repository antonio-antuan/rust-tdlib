use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks the login email address authentication
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckLoginEmailAddressCode {
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

impl RObject for CheckLoginEmailAddressCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckLoginEmailAddressCode {}

impl CheckLoginEmailAddressCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckLoginEmailAddressCodeBuilder {
        let mut inner = CheckLoginEmailAddressCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkLoginEmailAddressCode".to_string();

        CheckLoginEmailAddressCodeBuilder { inner }
    }

    pub fn code(&self) -> &EmailAddressAuthentication {
        &self.code
    }
}

#[doc(hidden)]
pub struct CheckLoginEmailAddressCodeBuilder {
    inner: CheckLoginEmailAddressCode,
}

#[deprecated]
pub type RTDCheckLoginEmailAddressCodeBuilder = CheckLoginEmailAddressCodeBuilder;

impl CheckLoginEmailAddressCodeBuilder {
    pub fn build(&self) -> CheckLoginEmailAddressCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<EmailAddressAuthentication>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().clone();
        self
    }
}

impl AsRef<CheckLoginEmailAddressCode> for CheckLoginEmailAddressCode {
    fn as_ref(&self) -> &CheckLoginEmailAddressCode {
        self
    }
}

impl AsRef<CheckLoginEmailAddressCode> for CheckLoginEmailAddressCodeBuilder {
    fn as_ref(&self) -> &CheckLoginEmailAddressCode {
        &self.inner
    }
}
