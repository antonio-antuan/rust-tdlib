use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Resends the login email address verification code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResendLoginEmailAddressCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResendLoginEmailAddressCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResendLoginEmailAddressCode {}

impl ResendLoginEmailAddressCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ResendLoginEmailAddressCodeBuilder {
        let mut inner = ResendLoginEmailAddressCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resendLoginEmailAddressCode".to_string();

        ResendLoginEmailAddressCodeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ResendLoginEmailAddressCodeBuilder {
    inner: ResendLoginEmailAddressCode,
}

#[deprecated]
pub type RTDResendLoginEmailAddressCodeBuilder = ResendLoginEmailAddressCodeBuilder;

impl ResendLoginEmailAddressCodeBuilder {
    pub fn build(&self) -> ResendLoginEmailAddressCode {
        self.inner.clone()
    }
}

impl AsRef<ResendLoginEmailAddressCode> for ResendLoginEmailAddressCode {
    fn as_ref(&self) -> &ResendLoginEmailAddressCode {
        self
    }
}

impl AsRef<ResendLoginEmailAddressCode> for ResendLoginEmailAddressCodeBuilder {
    fn as_ref(&self) -> &ResendLoginEmailAddressCode {
        &self.inner
    }
}
