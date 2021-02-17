use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the period of inactivity after which the account of the current user will automatically be deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAccountTtl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New account TTL
    ttl: AccountTtl,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetAccountTtl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetAccountTtl {}

impl SetAccountTtl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetAccountTtlBuilder {
        let mut inner = SetAccountTtl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setAccountTtl".to_string();

        RTDSetAccountTtlBuilder { inner }
    }

    pub fn ttl(&self) -> &AccountTtl {
        &self.ttl
    }
}

#[doc(hidden)]
pub struct RTDSetAccountTtlBuilder {
    inner: SetAccountTtl,
}

impl RTDSetAccountTtlBuilder {
    pub fn build(&self) -> SetAccountTtl {
        self.inner.clone()
    }

    pub fn ttl<T: AsRef<AccountTtl>>(&mut self, ttl: T) -> &mut Self {
        self.inner.ttl = ttl.as_ref().clone();
        self
    }
}

impl AsRef<SetAccountTtl> for SetAccountTtl {
    fn as_ref(&self) -> &SetAccountTtl {
        self
    }
}

impl AsRef<SetAccountTtl> for RTDSetAccountTtlBuilder {
    fn as_ref(&self) -> &SetAccountTtl {
        &self.inner
    }
}
