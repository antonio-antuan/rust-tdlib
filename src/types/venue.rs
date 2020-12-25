
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Venue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Venue location; as defined by the sender
  location: Location,
  /// Venue name; as defined by the sender
  title: String,
  /// Venue address; as defined by the sender
  address: String,
  /// Provider of the venue database; as defined by the sender. Currently only "foursquare" needs to be supported
  provider: String,
  /// Identifier of the venue in the provider database; as defined by the sender
  id: String,
  /// Type of the venue in the provider database; as defined by the sender
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: String,
  
}

impl RObject for Venue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "venue" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Venue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDVenueBuilder {
    let mut inner = Venue::default();
    inner.td_name = "venue".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDVenueBuilder { inner }
  }

  pub fn location(&self) -> &Location { &self.location }

  pub fn title(&self) -> &String { &self.title }

  pub fn address(&self) -> &String { &self.address }

  pub fn provider(&self) -> &String { &self.provider }

  pub fn id(&self) -> &String { &self.id }

  pub fn type_(&self) -> &String { &self.type_ }

}

#[doc(hidden)]
pub struct RTDVenueBuilder {
  inner: Venue
}

impl RTDVenueBuilder {
  pub fn build(&self) -> Venue { self.inner.clone() }

   
  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn address<T: AsRef<str>>(&mut self, address: T) -> &mut Self {
    self.inner.address = address.as_ref().to_string();
    self
  }

   
  pub fn provider<T: AsRef<str>>(&mut self, provider: T) -> &mut Self {
    self.inner.provider = provider.as_ref().to_string();
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().to_string();
    self
  }

}

impl AsRef<Venue> for Venue {
  fn as_ref(&self) -> &Venue { self }
}

impl AsRef<Venue> for RTDVenueBuilder {
  fn as_ref(&self) -> &Venue { &self.inner }
}



