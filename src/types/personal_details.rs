
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains the user's personal details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonalDetails {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// First name of the user written in English; 1-255 characters
  first_name: String,
  /// Middle name of the user written in English; 0-255 characters
  middle_name: String,
  /// Last name of the user written in English; 1-255 characters
  last_name: String,
  /// Native first name of the user; 1-255 characters
  native_first_name: String,
  /// Native middle name of the user; 0-255 characters
  native_middle_name: String,
  /// Native last name of the user; 1-255 characters
  native_last_name: String,
  /// Birthdate of the user
  birthdate: Date,
  /// Gender of the user, "male" or "female"
  gender: String,
  /// A two-letter ISO 3166-1 alpha-2 country code of the user's country
  country_code: String,
  /// A two-letter ISO 3166-1 alpha-2 country code of the user's residence country
  residence_country_code: String,
  
}

impl RObject for PersonalDetails {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "personalDetails" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PersonalDetails {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPersonalDetailsBuilder {
    let mut inner = PersonalDetails::default();
    inner.td_name = "personalDetails".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPersonalDetailsBuilder { inner }
  }

  pub fn first_name(&self) -> &String { &self.first_name }

  pub fn middle_name(&self) -> &String { &self.middle_name }

  pub fn last_name(&self) -> &String { &self.last_name }

  pub fn native_first_name(&self) -> &String { &self.native_first_name }

  pub fn native_middle_name(&self) -> &String { &self.native_middle_name }

  pub fn native_last_name(&self) -> &String { &self.native_last_name }

  pub fn birthdate(&self) -> &Date { &self.birthdate }

  pub fn gender(&self) -> &String { &self.gender }

  pub fn country_code(&self) -> &String { &self.country_code }

  pub fn residence_country_code(&self) -> &String { &self.residence_country_code }

}

#[doc(hidden)]
pub struct RTDPersonalDetailsBuilder {
  inner: PersonalDetails
}

impl RTDPersonalDetailsBuilder {
  pub fn build(&self) -> PersonalDetails { self.inner.clone() }

   
  pub fn first_name<T: AsRef<str>>(&mut self, first_name: T) -> &mut Self {
    self.inner.first_name = first_name.as_ref().to_string();
    self
  }

   
  pub fn middle_name<T: AsRef<str>>(&mut self, middle_name: T) -> &mut Self {
    self.inner.middle_name = middle_name.as_ref().to_string();
    self
  }

   
  pub fn last_name<T: AsRef<str>>(&mut self, last_name: T) -> &mut Self {
    self.inner.last_name = last_name.as_ref().to_string();
    self
  }

   
  pub fn native_first_name<T: AsRef<str>>(&mut self, native_first_name: T) -> &mut Self {
    self.inner.native_first_name = native_first_name.as_ref().to_string();
    self
  }

   
  pub fn native_middle_name<T: AsRef<str>>(&mut self, native_middle_name: T) -> &mut Self {
    self.inner.native_middle_name = native_middle_name.as_ref().to_string();
    self
  }

   
  pub fn native_last_name<T: AsRef<str>>(&mut self, native_last_name: T) -> &mut Self {
    self.inner.native_last_name = native_last_name.as_ref().to_string();
    self
  }

   
  pub fn birthdate<T: AsRef<Date>>(&mut self, birthdate: T) -> &mut Self {
    self.inner.birthdate = birthdate.as_ref().clone();
    self
  }

   
  pub fn gender<T: AsRef<str>>(&mut self, gender: T) -> &mut Self {
    self.inner.gender = gender.as_ref().to_string();
    self
  }

   
  pub fn country_code<T: AsRef<str>>(&mut self, country_code: T) -> &mut Self {
    self.inner.country_code = country_code.as_ref().to_string();
    self
  }

   
  pub fn residence_country_code<T: AsRef<str>>(&mut self, residence_country_code: T) -> &mut Self {
    self.inner.residence_country_code = residence_country_code.as_ref().to_string();
    self
  }

}

impl AsRef<PersonalDetails> for PersonalDetails {
  fn as_ref(&self) -> &PersonalDetails { self }
}

impl AsRef<PersonalDetails> for RTDPersonalDetailsBuilder {
  fn as_ref(&self) -> &PersonalDetails { &self.inner }
}



