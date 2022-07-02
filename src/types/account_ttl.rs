use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about the period of inactivity after which the current user's account will automatically be deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccountTtl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of days of inactivity before the account will be flagged for deletion; 30-366 days

    #[serde(default)]
    days: i32,
}

impl RObject for AccountTtl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AccountTtl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AccountTtlBuilder {
        let mut inner = AccountTtl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AccountTtlBuilder { inner }
    }

    pub fn days(&self) -> i32 {
        self.days
    }
}

#[doc(hidden)]
pub struct AccountTtlBuilder {
    inner: AccountTtl,
}

#[deprecated]
pub type RTDAccountTtlBuilder = AccountTtlBuilder;

impl AccountTtlBuilder {
    pub fn build(&self) -> AccountTtl {
        self.inner.clone()
    }

    pub fn days(&mut self, days: i32) -> &mut Self {
        self.inner.days = days;
        self
    }
}

impl AsRef<AccountTtl> for AccountTtl {
    fn as_ref(&self) -> &AccountTtl {
        self
    }
}

impl AsRef<AccountTtl> for AccountTtlBuilder {
    fn as_ref(&self) -> &AccountTtl {
        &self.inner
    }
}
