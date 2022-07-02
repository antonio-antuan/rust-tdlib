use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks the 2-step verification recovery email address verification code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckRecoveryEmailAddressCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Verification code to check

    #[serde(default)]
    code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckRecoveryEmailAddressCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckRecoveryEmailAddressCode {}

impl CheckRecoveryEmailAddressCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckRecoveryEmailAddressCodeBuilder {
        let mut inner = CheckRecoveryEmailAddressCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkRecoveryEmailAddressCode".to_string();

        CheckRecoveryEmailAddressCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct CheckRecoveryEmailAddressCodeBuilder {
    inner: CheckRecoveryEmailAddressCode,
}

#[deprecated]
pub type RTDCheckRecoveryEmailAddressCodeBuilder = CheckRecoveryEmailAddressCodeBuilder;

impl CheckRecoveryEmailAddressCodeBuilder {
    pub fn build(&self) -> CheckRecoveryEmailAddressCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<CheckRecoveryEmailAddressCode> for CheckRecoveryEmailAddressCode {
    fn as_ref(&self) -> &CheckRecoveryEmailAddressCode {
        self
    }
}

impl AsRef<CheckRecoveryEmailAddressCode> for CheckRecoveryEmailAddressCodeBuilder {
    fn as_ref(&self) -> &CheckRecoveryEmailAddressCode {
        &self.inner
    }
}
