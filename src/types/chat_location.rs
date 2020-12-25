
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a location to which a chat is connected
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The location
  location: Location,
  /// Location address; 1-64 characters, as defined by the chat owner
  address: String,
  
}

impl RObject for ChatLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatLocation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatLocationBuilder {
    let mut inner = ChatLocation::default();
    inner.td_name = "chatLocation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatLocationBuilder { inner }
  }

  pub fn location(&self) -> &Location { &self.location }

  pub fn address(&self) -> &String { &self.address }

}

#[doc(hidden)]
pub struct RTDChatLocationBuilder {
  inner: ChatLocation
}

impl RTDChatLocationBuilder {
  pub fn build(&self) -> ChatLocation { self.inner.clone() }

   
  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

   
  pub fn address<T: AsRef<str>>(&mut self, address: T) -> &mut Self {
    self.inner.address = address.as_ref().to_string();
    self
  }

}

impl AsRef<ChatLocation> for ChatLocation {
  fn as_ref(&self) -> &ChatLocation { self }
}

impl AsRef<ChatLocation> for RTDChatLocationBuilder {
  fn as_ref(&self) -> &ChatLocation { &self.inner }
}



