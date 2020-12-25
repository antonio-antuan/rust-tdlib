
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of websites the current user is logged in with Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectedWebsites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of connected websites
  websites: Vec<ConnectedWebsite>,
  
}

impl RObject for ConnectedWebsites {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectedWebsites" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ConnectedWebsites {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDConnectedWebsitesBuilder {
    let mut inner = ConnectedWebsites::default();
    inner.td_name = "connectedWebsites".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDConnectedWebsitesBuilder { inner }
  }

  pub fn websites(&self) -> &Vec<ConnectedWebsite> { &self.websites }

}

#[doc(hidden)]
pub struct RTDConnectedWebsitesBuilder {
  inner: ConnectedWebsites
}

impl RTDConnectedWebsitesBuilder {
  pub fn build(&self) -> ConnectedWebsites { self.inner.clone() }

   
  pub fn websites(&mut self, websites: Vec<ConnectedWebsite>) -> &mut Self {
    self.inner.websites = websites;
    self
  }

}

impl AsRef<ConnectedWebsites> for ConnectedWebsites {
  fn as_ref(&self) -> &ConnectedWebsites { self }
}

impl AsRef<ConnectedWebsites> for RTDConnectedWebsitesBuilder {
  fn as_ref(&self) -> &ConnectedWebsites { &self.inner }
}



