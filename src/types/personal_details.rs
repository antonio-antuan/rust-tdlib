use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains the user's personal details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonalDetails {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// First name of the user written in English; 1-255 characters

    #[serde(default)]
    first_name: String,
    /// Middle name of the user written in English; 0-255 characters

    #[serde(default)]
    middle_name: String,
    /// Last name of the user written in English; 1-255 characters

    #[serde(default)]
    last_name: String,
    /// Native first name of the user; 1-255 characters

    #[serde(default)]
    native_first_name: String,
    /// Native middle name of the user; 0-255 characters

    #[serde(default)]
    native_middle_name: String,
    /// Native last name of the user; 1-255 characters

    #[serde(default)]
    native_last_name: String,
    /// Birthdate of the user
    birthdate: Date,
    /// Gender of the user, "male" or "female"

    #[serde(default)]
    gender: String,
    /// A two-letter ISO 3166-1 alpha-2 country code of the user's country

    #[serde(default)]
    country_code: String,
    /// A two-letter ISO 3166-1 alpha-2 country code of the user's residence country

    #[serde(default)]
    residence_country_code: String,
}

impl RObject for PersonalDetails {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PersonalDetails {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PersonalDetailsBuilder {
        let mut inner = PersonalDetails::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PersonalDetailsBuilder { inner }
    }

    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn middle_name(&self) -> &String {
        &self.middle_name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn native_first_name(&self) -> &String {
        &self.native_first_name
    }

    pub fn native_middle_name(&self) -> &String {
        &self.native_middle_name
    }

    pub fn native_last_name(&self) -> &String {
        &self.native_last_name
    }

    pub fn birthdate(&self) -> &Date {
        &self.birthdate
    }

    pub fn gender(&self) -> &String {
        &self.gender
    }

    pub fn country_code(&self) -> &String {
        &self.country_code
    }

    pub fn residence_country_code(&self) -> &String {
        &self.residence_country_code
    }
}

#[doc(hidden)]
pub struct PersonalDetailsBuilder {
    inner: PersonalDetails,
}

#[deprecated]
pub type RTDPersonalDetailsBuilder = PersonalDetailsBuilder;

impl PersonalDetailsBuilder {
    pub fn build(&self) -> PersonalDetails {
        self.inner.clone()
    }

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

    pub fn residence_country_code<T: AsRef<str>>(
        &mut self,
        residence_country_code: T,
    ) -> &mut Self {
        self.inner.residence_country_code = residence_country_code.as_ref().to_string();
        self
    }
}

impl AsRef<PersonalDetails> for PersonalDetails {
    fn as_ref(&self) -> &PersonalDetails {
        self
    }
}

impl AsRef<PersonalDetails> for PersonalDetailsBuilder {
    fn as_ref(&self) -> &PersonalDetails {
        &self.inner
    }
}
