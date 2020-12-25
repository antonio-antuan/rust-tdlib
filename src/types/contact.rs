
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a user contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Contact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Phone number of the user
  phone_number: String,
  /// First name of the user; 1-255 characters in length
  first_name: String,
  /// Last name of the user
  last_name: String,
  /// Additional data about the user in a form of vCard; 0-2048 bytes in length
  vcard: String,
  /// Identifier of the user, if known; otherwise 0
  user_id: i64,
  
}

impl RObject for Contact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "contact" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Contact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDContactBuilder {
    let mut inner = Contact::default();
    inner.td_name = "contact".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDContactBuilder { inner }
  }

  pub fn phone_number(&self) -> &String { &self.phone_number }

  pub fn first_name(&self) -> &String { &self.first_name }

  pub fn last_name(&self) -> &String { &self.last_name }

  pub fn vcard(&self) -> &String { &self.vcard }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDContactBuilder {
  inner: Contact
}

impl RTDContactBuilder {
  pub fn build(&self) -> Contact { self.inner.clone() }

   
  pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
    self.inner.phone_number = phone_number.as_ref().to_string();
    self
  }

   
  pub fn first_name<T: AsRef<str>>(&mut self, first_name: T) -> &mut Self {
    self.inner.first_name = first_name.as_ref().to_string();
    self
  }

   
  pub fn last_name<T: AsRef<str>>(&mut self, last_name: T) -> &mut Self {
    self.inner.last_name = last_name.as_ref().to_string();
    self
  }

   
  pub fn vcard<T: AsRef<str>>(&mut self, vcard: T) -> &mut Self {
    self.inner.vcard = vcard.as_ref().to_string();
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<Contact> for Contact {
  fn as_ref(&self) -> &Contact { self }
}

impl AsRef<Contact> for RTDContactBuilder {
  fn as_ref(&self) -> &Contact { &self.inner }
}



