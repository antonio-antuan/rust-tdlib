use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecoveryEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The password for the current user

    #[serde(default)]
    password: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetRecoveryEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetRecoveryEmailAddress {}

impl GetRecoveryEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetRecoveryEmailAddressBuilder {
        let mut inner = GetRecoveryEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getRecoveryEmailAddress".to_string();

        GetRecoveryEmailAddressBuilder { inner }
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[doc(hidden)]
pub struct GetRecoveryEmailAddressBuilder {
    inner: GetRecoveryEmailAddress,
}

#[deprecated]
pub type RTDGetRecoveryEmailAddressBuilder = GetRecoveryEmailAddressBuilder;

impl GetRecoveryEmailAddressBuilder {
    pub fn build(&self) -> GetRecoveryEmailAddress {
        self.inner.clone()
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }
}

impl AsRef<GetRecoveryEmailAddress> for GetRecoveryEmailAddress {
    fn as_ref(&self) -> &GetRecoveryEmailAddress {
        self
    }
}

impl AsRef<GetRecoveryEmailAddress> for GetRecoveryEmailAddressBuilder {
    fn as_ref(&self) -> &GetRecoveryEmailAddress {
        &self.inner
    }
}
