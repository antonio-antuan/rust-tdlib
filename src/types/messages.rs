
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Messages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total count of messages found
  total_count: i64,
  /// List of messages; messages may be null
  messages: Vec<Option<Message>>,
  
}

impl RObject for Messages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messages" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Messages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessagesBuilder {
    let mut inner = Messages::default();
    inner.td_name = "messages".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessagesBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn messages(&self) -> &Vec<Option<Message>> { &self.messages }

}

#[doc(hidden)]
pub struct RTDMessagesBuilder {
  inner: Messages
}

impl RTDMessagesBuilder {
  pub fn build(&self) -> Messages { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn messages(&mut self, messages: Vec<Option<Message>>) -> &mut Self {
    self.inner.messages = messages;
    self
  }

}

impl AsRef<Messages> for Messages {
  fn as_ref(&self) -> &Messages { self }
}

impl AsRef<Messages> for RTDMessagesBuilder {
  fn as_ref(&self) -> &Messages { &self.inner }
}



