use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Venue {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Venue location; as defined by the sender
    location: Location,
    /// Venue name; as defined by the sender

    #[serde(default)]
    title: String,
    /// Venue address; as defined by the sender

    #[serde(default)]
    address: String,
    /// Provider of the venue database; as defined by the sender. Currently, only "foursquare" and "gplaces" (Google Places) need to be supported

    #[serde(default)]
    provider: String,
    /// Identifier of the venue in the provider database; as defined by the sender

    #[serde(default)]
    id: String,
    /// Type of the venue in the provider database; as defined by the sender

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(default)]
    type_: String,
}

impl RObject for Venue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Venue {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> VenueBuilder {
        let mut inner = Venue::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        VenueBuilder { inner }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn provider(&self) -> &String {
        &self.provider
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn type_(&self) -> &String {
        &self.type_
    }
}

#[doc(hidden)]
pub struct VenueBuilder {
    inner: Venue,
}

#[deprecated]
pub type RTDVenueBuilder = VenueBuilder;

impl VenueBuilder {
    pub fn build(&self) -> Venue {
        self.inner.clone()
    }

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
    fn as_ref(&self) -> &Venue {
        self
    }
}

impl AsRef<Venue> for VenueBuilder {
    fn as_ref(&self) -> &Venue {
        &self.inner
    }
}
