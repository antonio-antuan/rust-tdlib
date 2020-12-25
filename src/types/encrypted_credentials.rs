
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains encrypted Telegram Passport data credentials
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EncryptedCredentials {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The encrypted credentials
  data: String,
  /// The decrypted data hash
  hash: String,
  /// Secret for data decryption, encrypted with the service's public key
  secret: String,
  
}

impl RObject for EncryptedCredentials {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "encryptedCredentials" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl EncryptedCredentials {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEncryptedCredentialsBuilder {
    let mut inner = EncryptedCredentials::default();
    inner.td_name = "encryptedCredentials".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDEncryptedCredentialsBuilder { inner }
  }

  pub fn data(&self) -> &String { &self.data }

  pub fn hash(&self) -> &String { &self.hash }

  pub fn secret(&self) -> &String { &self.secret }

}

#[doc(hidden)]
pub struct RTDEncryptedCredentialsBuilder {
  inner: EncryptedCredentials
}

impl RTDEncryptedCredentialsBuilder {
  pub fn build(&self) -> EncryptedCredentials { self.inner.clone() }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

   
  pub fn hash<T: AsRef<str>>(&mut self, hash: T) -> &mut Self {
    self.inner.hash = hash.as_ref().to_string();
    self
  }

   
  pub fn secret<T: AsRef<str>>(&mut self, secret: T) -> &mut Self {
    self.inner.secret = secret.as_ref().to_string();
    self
  }

}

impl AsRef<EncryptedCredentials> for EncryptedCredentials {
  fn as_ref(&self) -> &EncryptedCredentials { self }
}

impl AsRef<EncryptedCredentials> for RTDEncryptedCredentialsBuilder {
  fn as_ref(&self) -> &EncryptedCredentials { &self.inner }
}



