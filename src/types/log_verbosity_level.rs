
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a TDLib internal log verbosity level
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Log verbosity level
  verbosity_level: i64,
  
}

impl RObject for LogVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logVerbosityLevel" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl LogVerbosityLevel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLogVerbosityLevelBuilder {
    let mut inner = LogVerbosityLevel::default();
    inner.td_name = "logVerbosityLevel".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLogVerbosityLevelBuilder { inner }
  }

  pub fn verbosity_level(&self) -> i64 { self.verbosity_level }

}

#[doc(hidden)]
pub struct RTDLogVerbosityLevelBuilder {
  inner: LogVerbosityLevel
}

impl RTDLogVerbosityLevelBuilder {
  pub fn build(&self) -> LogVerbosityLevel { self.inner.clone() }

   
  pub fn verbosity_level(&mut self, verbosity_level: i64) -> &mut Self {
    self.inner.verbosity_level = verbosity_level;
    self
  }

}

impl AsRef<LogVerbosityLevel> for LogVerbosityLevel {
  fn as_ref(&self) -> &LogVerbosityLevel { self }
}

impl AsRef<LogVerbosityLevel> for RTDLogVerbosityLevelBuilder {
  fn as_ref(&self) -> &LogVerbosityLevel { &self.inner }
}



