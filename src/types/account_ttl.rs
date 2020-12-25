
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about the period of inactivity after which the current user's account will automatically be deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccountTtl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of days of inactivity before the account will be flagged for deletion; should range from 30-366 days
  days: i64,
  
}

impl RObject for AccountTtl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "accountTtl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl AccountTtl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAccountTtlBuilder {
    let mut inner = AccountTtl::default();
    inner.td_name = "accountTtl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAccountTtlBuilder { inner }
  }

  pub fn days(&self) -> i64 { self.days }

}

#[doc(hidden)]
pub struct RTDAccountTtlBuilder {
  inner: AccountTtl
}

impl RTDAccountTtlBuilder {
  pub fn build(&self) -> AccountTtl { self.inner.clone() }

   
  pub fn days(&mut self, days: i64) -> &mut Self {
    self.inner.days = days;
    self
  }

}

impl AsRef<AccountTtl> for AccountTtl {
  fn as_ref(&self) -> &AccountTtl { self }
}

impl AsRef<AccountTtl> for RTDAccountTtlBuilder {
  fn as_ref(&self) -> &AccountTtl { &self.inner }
}



