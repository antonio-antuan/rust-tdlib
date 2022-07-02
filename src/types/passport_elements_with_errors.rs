use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Telegram Passport elements and corresponding errors
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementsWithErrors {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Telegram Passport elements

    #[serde(default)]
    elements: Vec<PassportElement>,
    /// Errors in the elements that are already available

    #[serde(default)]
    errors: Vec<PassportElementError>,
}

impl RObject for PassportElementsWithErrors {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PassportElementsWithErrors {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementsWithErrorsBuilder {
        let mut inner = PassportElementsWithErrors::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementsWithErrorsBuilder { inner }
    }

    pub fn elements(&self) -> &Vec<PassportElement> {
        &self.elements
    }

    pub fn errors(&self) -> &Vec<PassportElementError> {
        &self.errors
    }
}

#[doc(hidden)]
pub struct PassportElementsWithErrorsBuilder {
    inner: PassportElementsWithErrors,
}

#[deprecated]
pub type RTDPassportElementsWithErrorsBuilder = PassportElementsWithErrorsBuilder;

impl PassportElementsWithErrorsBuilder {
    pub fn build(&self) -> PassportElementsWithErrors {
        self.inner.clone()
    }

    pub fn elements(&mut self, elements: Vec<PassportElement>) -> &mut Self {
        self.inner.elements = elements;
        self
    }

    pub fn errors(&mut self, errors: Vec<PassportElementError>) -> &mut Self {
        self.inner.errors = errors;
        self
    }
}

impl AsRef<PassportElementsWithErrors> for PassportElementsWithErrors {
    fn as_ref(&self) -> &PassportElementsWithErrors {
        self
    }
}

impl AsRef<PassportElementsWithErrors> for PassportElementsWithErrorsBuilder {
    fn as_ref(&self) -> &PassportElementsWithErrors {
        &self.inner
    }
}
