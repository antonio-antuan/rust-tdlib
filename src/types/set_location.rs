use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the location of the current user. Needs to be called if GetOption("is_location_visible") is true and location changes for more than 1 kilometer
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new location of the user
    location: Location,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetLocation {}

impl SetLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetLocationBuilder {
        let mut inner = SetLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setLocation".to_string();

        SetLocationBuilder { inner }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

#[doc(hidden)]
pub struct SetLocationBuilder {
    inner: SetLocation,
}

#[deprecated]
pub type RTDSetLocationBuilder = SetLocationBuilder;

impl SetLocationBuilder {
    pub fn build(&self) -> SetLocation {
        self.inner.clone()
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }
}

impl AsRef<SetLocation> for SetLocation {
    fn as_ref(&self) -> &SetLocation {
        self
    }
}

impl AsRef<SetLocation> for SetLocationBuilder {
    fn as_ref(&self) -> &SetLocation {
        &self.inner
    }
}
