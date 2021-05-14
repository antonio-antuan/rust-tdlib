use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetRecoveryEmailAddressBuilder {
        let mut inner = GetRecoveryEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getRecoveryEmailAddress".to_string();

        RTDGetRecoveryEmailAddressBuilder { inner }
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[doc(hidden)]
pub struct RTDGetRecoveryEmailAddressBuilder {
    inner: GetRecoveryEmailAddress,
}

impl RTDGetRecoveryEmailAddressBuilder {
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

impl AsRef<GetRecoveryEmailAddress> for RTDGetRecoveryEmailAddressBuilder {
    fn as_ref(&self) -> &GetRecoveryEmailAddress {
        &self.inner
    }
}
