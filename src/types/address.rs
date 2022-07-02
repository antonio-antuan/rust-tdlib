use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes an address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Address {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A two-letter ISO 3166-1 alpha-2 country code

    #[serde(default)]
    country_code: String,
    /// State, if applicable

    #[serde(default)]
    state: String,
    /// City

    #[serde(default)]
    city: String,
    /// First line of the address

    #[serde(default)]
    street_line1: String,
    /// Second line of the address

    #[serde(default)]
    street_line2: String,
    /// Address postal code

    #[serde(default)]
    postal_code: String,
}

impl RObject for Address {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Address {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddressBuilder {
        let mut inner = Address::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AddressBuilder { inner }
    }

    pub fn country_code(&self) -> &String {
        &self.country_code
    }

    pub fn state(&self) -> &String {
        &self.state
    }

    pub fn city(&self) -> &String {
        &self.city
    }

    pub fn street_line1(&self) -> &String {
        &self.street_line1
    }

    pub fn street_line2(&self) -> &String {
        &self.street_line2
    }

    pub fn postal_code(&self) -> &String {
        &self.postal_code
    }
}

#[doc(hidden)]
pub struct AddressBuilder {
    inner: Address,
}

#[deprecated]
pub type RTDAddressBuilder = AddressBuilder;

impl AddressBuilder {
    pub fn build(&self) -> Address {
        self.inner.clone()
    }

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
    fn as_ref(&self) -> &Address {
        self
    }
}

impl AsRef<Address> for AddressBuilder {
    fn as_ref(&self) -> &Address {
        &self.inner
    }
}
