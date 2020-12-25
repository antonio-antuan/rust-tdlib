
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of chat members
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total count of chat members found
  total_count: i64,
  /// A list of chat members
  members: Vec<ChatMember>,
  
}

impl RObject for ChatMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembers" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMembersBuilder {
    let mut inner = ChatMembers::default();
    inner.td_name = "chatMembers".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatMembersBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn members(&self) -> &Vec<ChatMember> { &self.members }

}

#[doc(hidden)]
pub struct RTDChatMembersBuilder {
  inner: ChatMembers
}

impl RTDChatMembersBuilder {
  pub fn build(&self) -> ChatMembers { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn members(&mut self, members: Vec<ChatMember>) -> &mut Self {
    self.inner.members = members;
    self
  }

}

impl AsRef<ChatMembers> for ChatMembers {
  fn as_ref(&self) -> &ChatMembers { self }
}

impl AsRef<ChatMembers> for RTDChatMembersBuilder {
  fn as_ref(&self) -> &ChatMembers { &self.inner }
}



