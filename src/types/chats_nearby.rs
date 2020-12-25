
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a list of chats located nearby
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatsNearby {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of users nearby
  users_nearby: Vec<ChatNearby>,
  /// List of location-based supergroups nearby
  supergroups_nearby: Vec<ChatNearby>,
  
}

impl RObject for ChatsNearby {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatsNearby" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatsNearby {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatsNearbyBuilder {
    let mut inner = ChatsNearby::default();
    inner.td_name = "chatsNearby".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatsNearbyBuilder { inner }
  }

  pub fn users_nearby(&self) -> &Vec<ChatNearby> { &self.users_nearby }

  pub fn supergroups_nearby(&self) -> &Vec<ChatNearby> { &self.supergroups_nearby }

}

#[doc(hidden)]
pub struct RTDChatsNearbyBuilder {
  inner: ChatsNearby
}

impl RTDChatsNearbyBuilder {
  pub fn build(&self) -> ChatsNearby { self.inner.clone() }

   
  pub fn users_nearby(&mut self, users_nearby: Vec<ChatNearby>) -> &mut Self {
    self.inner.users_nearby = users_nearby;
    self
  }

   
  pub fn supergroups_nearby(&mut self, supergroups_nearby: Vec<ChatNearby>) -> &mut Self {
    self.inner.supergroups_nearby = supergroups_nearby;
    self
  }

}

impl AsRef<ChatsNearby> for ChatsNearby {
  fn as_ref(&self) -> &ChatsNearby { self }
}

impl AsRef<ChatsNearby> for RTDChatsNearbyBuilder {
  fn as_ref(&self) -> &ChatsNearby { &self.inner }
}



