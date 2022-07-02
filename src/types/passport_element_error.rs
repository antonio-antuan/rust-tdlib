use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains the description of an error in a Telegram Passport element
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementError {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the Telegram Passport element which has the error

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PassportElementType::_is_default")]
    type_: PassportElementType,
    /// Error message

    #[serde(default)]
    message: String,
    /// Error source

    #[serde(skip_serializing_if = "PassportElementErrorSource::_is_default")]
    source: PassportElementErrorSource,
}

impl RObject for PassportElementError {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PassportElementError {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementErrorBuilder {
        let mut inner = PassportElementError::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementErrorBuilder { inner }
    }

    pub fn type_(&self) -> &PassportElementType {
        &self.type_
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn source(&self) -> &PassportElementErrorSource {
        &self.source
    }
}

#[doc(hidden)]
pub struct PassportElementErrorBuilder {
    inner: PassportElementError,
}

#[deprecated]
pub type RTDPassportElementErrorBuilder = PassportElementErrorBuilder;

impl PassportElementErrorBuilder {
    pub fn build(&self) -> PassportElementError {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<PassportElementType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn message<T: AsRef<str>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().to_string();
        self
    }

    pub fn source<T: AsRef<PassportElementErrorSource>>(&mut self, source: T) -> &mut Self {
        self.inner.source = source.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementError> for PassportElementError {
    fn as_ref(&self) -> &PassportElementError {
        self
    }
}

impl AsRef<PassportElementError> for PassportElementErrorBuilder {
    fn as_ref(&self) -> &PassportElementError {
        &self.inner
    }
}
