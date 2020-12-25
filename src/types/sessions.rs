
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of sessions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sessions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of sessions
  sessions: Vec<Session>,
  
}

impl RObject for Sessions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sessions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Sessions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSessionsBuilder {
    let mut inner = Sessions::default();
    inner.td_name = "sessions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDSessionsBuilder { inner }
  }

  pub fn sessions(&self) -> &Vec<Session> { &self.sessions }

}

#[doc(hidden)]
pub struct RTDSessionsBuilder {
  inner: Sessions
}

impl RTDSessionsBuilder {
  pub fn build(&self) -> Sessions { self.inner.clone() }

   
  pub fn sessions(&mut self, sessions: Vec<Session>) -> &mut Self {
    self.inner.sessions = sessions;
    self
  }

}

impl AsRef<Sessions> for Sessions {
  fn as_ref(&self) -> &Sessions { self }
}

impl AsRef<Sessions> for RTDSessionsBuilder {
  fn as_ref(&self) -> &Sessions { &self.inner }
}



