
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes an address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Address {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A two-letter ISO 3166-1 alpha-2 country code
  country_code: String,
  /// State, if applicable
  state: String,
  /// City
  city: String,
  /// First line of the address
  street_line1: String,
  /// Second line of the address
  street_line2: String,
  /// Address postal code
  postal_code: String,
  
}

impl RObject for Address {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "address" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Address {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAddressBuilder {
    let mut inner = Address::default();
    inner.td_name = "address".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAddressBuilder { inner }
  }

  pub fn country_code(&self) -> &String { &self.country_code }

  pub fn state(&self) -> &String { &self.state }

  pub fn city(&self) -> &String { &self.city }

  pub fn street_line1(&self) -> &String { &self.street_line1 }

  pub fn street_line2(&self) -> &String { &self.street_line2 }

  pub fn postal_code(&self) -> &String { &self.postal_code }

}

#[doc(hidden)]
pub struct RTDAddressBuilder {
  inner: Address
}

impl RTDAddressBuilder {
  pub fn build(&self) -> Address { self.inner.clone() }

   
  pub fn country_code<T: AsRef<str>>(&mut self, country_code: T) -> &mut Self {
    self.inner.country_code = country_code.as_ref().to_string();
    self
  }

   
  pub fn state<T: AsRef<str>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().to_string();
    self
  }

   
  pub fn city<T: AsRef<str>>(&mut self, city: T) -> &mut Self {
    self.inner.city = city.as_ref().to_string();
    self
  }

   
  pub fn street_line1<T: AsRef<str>>(&mut self, street_line1: T) -> &mut Self {
    self.inner.street_line1 = street_line1.as_ref().to_string();
    self
  }

   
  pub fn street_line2<T: AsRef<str>>(&mut self, street_line2: T) -> &mut Self {
    self.inner.street_line2 = street_line2.as_ref().to_string();
    self
  }

   
  pub fn postal_code<T: AsRef<str>>(&mut self, postal_code: T) -> &mut Self {
    self.inner.postal_code = postal_code.as_ref().to_string();
    self
  }

}

impl AsRef<Address> for Address {
  fn as_ref(&self) -> &Address { self }
}

impl AsRef<Address> for RTDAddressBuilder {
  fn as_ref(&self) -> &Address { &self.inner }
}



