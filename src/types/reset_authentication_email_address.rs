use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Resets the login email address. May return an error with a message "TASK_ALREADY_EXISTS" if reset is still pending. Works only when the current authorization state is authorizationStateWaitEmailCode and authorization_state.can_reset_email_address == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetAuthenticationEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResetAuthenticationEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResetAuthenticationEmailAddress {}

impl ResetAuthenticationEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ResetAuthenticationEmailAddressBuilder {
        let mut inner = ResetAuthenticationEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resetAuthenticationEmailAddress".to_string();

        ResetAuthenticationEmailAddressBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ResetAuthenticationEmailAddressBuilder {
    inner: ResetAuthenticationEmailAddress,
}

#[deprecated]
pub type RTDResetAuthenticationEmailAddressBuilder = ResetAuthenticationEmailAddressBuilder;

impl ResetAuthenticationEmailAddressBuilder {
    pub fn build(&self) -> ResetAuthenticationEmailAddress {
        self.inner.clone()
    }
}

impl AsRef<ResetAuthenticationEmailAddress> for ResetAuthenticationEmailAddress {
    fn as_ref(&self) -> &ResetAuthenticationEmailAddress {
        self
    }
}

impl AsRef<ResetAuthenticationEmailAddress> for ResetAuthenticationEmailAddressBuilder {
    fn as_ref(&self) -> &ResetAuthenticationEmailAddress {
        &self.inner
    }
}
