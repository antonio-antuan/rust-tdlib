use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a location on planet Earth
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Latitude of the location in degrees; as defined by the sender
    latitude: f32,
    /// Longitude of the location, in degrees; as defined by the sender
    longitude: f32,
    /// The estimated horizontal accuracy of the location, in meters; as defined by the sender. 0 if unknown
    horizontal_accuracy: f32,
}

impl RObject for Location {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Location {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLocationBuilder {
        let mut inner = Location::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDLocationBuilder { inner }
    }

    pub fn latitude(&self) -> f32 {
        self.latitude
    }

    pub fn longitude(&self) -> f32 {
        self.longitude
    }

    pub fn horizontal_accuracy(&self) -> f32 {
        self.horizontal_accuracy
    }
}

#[doc(hidden)]
pub struct RTDLocationBuilder {
    inner: Location,
}

impl RTDLocationBuilder {
    pub fn build(&self) -> Location {
        self.inner.clone()
    }

    pub fn latitude(&mut self, latitude: f32) -> &mut Self {
        self.inner.latitude = latitude;
        self
    }

    pub fn longitude(&mut self, longitude: f32) -> &mut Self {
        self.inner.longitude = longitude;
        self
    }

    pub fn horizontal_accuracy(&mut self, horizontal_accuracy: f32) -> &mut Self {
        self.inner.horizontal_accuracy = horizontal_accuracy;
        self
    }
}

impl AsRef<Location> for Location {
    fn as_ref(&self) -> &Location {
        self
    }
}

impl AsRef<Location> for RTDLocationBuilder {
    fn as_ref(&self) -> &Location {
        &self.inner
    }
}
