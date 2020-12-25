
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of chat events
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEvents {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of events
  events: Vec<ChatEvent>,
  
}

impl RObject for ChatEvents {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEvents" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatEvents {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventsBuilder {
    let mut inner = ChatEvents::default();
    inner.td_name = "chatEvents".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatEventsBuilder { inner }
  }

  pub fn events(&self) -> &Vec<ChatEvent> { &self.events }

}

#[doc(hidden)]
pub struct RTDChatEventsBuilder {
  inner: ChatEvents
}

impl RTDChatEventsBuilder {
  pub fn build(&self) -> ChatEvents { self.inner.clone() }

   
  pub fn events(&mut self, events: Vec<ChatEvent>) -> &mut Self {
    self.inner.events = events;
    self
  }

}

impl AsRef<ChatEvents> for ChatEvents {
  fn as_ref(&self) -> &ChatEvents { self }
}

impl AsRef<ChatEvents> for RTDChatEventsBuilder {
  fn as_ref(&self) -> &ChatEvents { &self.inner }
}



