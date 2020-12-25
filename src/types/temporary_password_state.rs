
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Returns information about the availability of a temporary password, which can be used for payments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemporaryPasswordState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if a temporary password is available
  has_password: bool,
  /// Time left before the temporary password expires, in seconds
  valid_for: i64,
  
}

impl RObject for TemporaryPasswordState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "temporaryPasswordState" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TemporaryPasswordState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTemporaryPasswordStateBuilder {
    let mut inner = TemporaryPasswordState::default();
    inner.td_name = "temporaryPasswordState".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTemporaryPasswordStateBuilder { inner }
  }

  pub fn has_password(&self) -> bool { self.has_password }

  pub fn valid_for(&self) -> i64 { self.valid_for }

}

#[doc(hidden)]
pub struct RTDTemporaryPasswordStateBuilder {
  inner: TemporaryPasswordState
}

impl RTDTemporaryPasswordStateBuilder {
  pub fn build(&self) -> TemporaryPasswordState { self.inner.clone() }

   
  pub fn has_password(&mut self, has_password: bool) -> &mut Self {
    self.inner.has_password = has_password;
    self
  }

   
  pub fn valid_for(&mut self, valid_for: i64) -> &mut Self {
    self.inner.valid_for = valid_for;
    self
  }

}

impl AsRef<TemporaryPasswordState> for TemporaryPasswordState {
  fn as_ref(&self) -> &TemporaryPasswordState { self }
}

impl AsRef<TemporaryPasswordState> for RTDTemporaryPasswordStateBuilder {
  fn as_ref(&self) -> &TemporaryPasswordState { &self.inner }
}



