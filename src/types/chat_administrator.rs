
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a chat administrator
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatAdministrator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier of the administrator
  user_id: i64,
  /// Custom title of the administrator
  custom_title: String,
  /// True, if the user is the owner of the chat
  is_owner: bool,
  
}

impl RObject for ChatAdministrator {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatAdministrator" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatAdministrator {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatAdministratorBuilder {
    let mut inner = ChatAdministrator::default();
    inner.td_name = "chatAdministrator".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatAdministratorBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn custom_title(&self) -> &String { &self.custom_title }

  pub fn is_owner(&self) -> bool { self.is_owner }

}

#[doc(hidden)]
pub struct RTDChatAdministratorBuilder {
  inner: ChatAdministrator
}

impl RTDChatAdministratorBuilder {
  pub fn build(&self) -> ChatAdministrator { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn custom_title<T: AsRef<str>>(&mut self, custom_title: T) -> &mut Self {
    self.inner.custom_title = custom_title.as_ref().to_string();
    self
  }

   
  pub fn is_owner(&mut self, is_owner: bool) -> &mut Self {
    self.inner.is_owner = is_owner;
    self
  }

}

impl AsRef<ChatAdministrator> for ChatAdministrator {
  fn as_ref(&self) -> &ChatAdministrator { self }
}

impl AsRef<ChatAdministrator> for RTDChatAdministratorBuilder {
  fn as_ref(&self) -> &ChatAdministrator { &self.inner }
}



