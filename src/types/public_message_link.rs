
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a public HTTPS link to a message in a supergroup or channel with a username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicMessageLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Message link
  link: String,
  /// HTML-code for embedding the message
  html: String,
  
}

impl RObject for PublicMessageLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "publicMessageLink" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PublicMessageLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPublicMessageLinkBuilder {
    let mut inner = PublicMessageLink::default();
    inner.td_name = "publicMessageLink".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPublicMessageLinkBuilder { inner }
  }

  pub fn link(&self) -> &String { &self.link }

  pub fn html(&self) -> &String { &self.html }

}

#[doc(hidden)]
pub struct RTDPublicMessageLinkBuilder {
  inner: PublicMessageLink
}

impl RTDPublicMessageLinkBuilder {
  pub fn build(&self) -> PublicMessageLink { self.inner.clone() }

   
  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }

   
  pub fn html<T: AsRef<str>>(&mut self, html: T) -> &mut Self {
    self.inner.html = html.as_ref().to_string();
    self
  }

}

impl AsRef<PublicMessageLink> for PublicMessageLink {
  fn as_ref(&self) -> &PublicMessageLink { self }
}

impl AsRef<PublicMessageLink> for RTDPublicMessageLinkBuilder {
  fn as_ref(&self) -> &PublicMessageLink { &self.inner }
}



