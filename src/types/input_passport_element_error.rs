use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains the description of an error in a Telegram Passport element; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementError {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of Telegram Passport element that has the error

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PassportElementType::_is_default")]
    type_: PassportElementType,
    /// Error message

    #[serde(default)]
    message: String,
    /// Error source

    #[serde(skip_serializing_if = "InputPassportElementErrorSource::_is_default")]
    source: InputPassportElementErrorSource,
}

impl RObject for InputPassportElementError {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl InputPassportElementError {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementErrorBuilder {
        let mut inner = InputPassportElementError::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementErrorBuilder { inner }
    }

    pub fn type_(&self) -> &PassportElementType {
        &self.type_
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn source(&self) -> &InputPassportElementErrorSource {
        &self.source
    }
}

#[doc(hidden)]
pub struct InputPassportElementErrorBuilder {
    inner: InputPassportElementError,
}

#[deprecated]
pub type RTDInputPassportElementErrorBuilder = InputPassportElementErrorBuilder;

impl InputPassportElementErrorBuilder {
    pub fn build(&self) -> InputPassportElementError {
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

    pub fn source<T: AsRef<InputPassportElementErrorSource>>(&mut self, source: T) -> &mut Self {
        self.inner.source = source.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementError> for InputPassportElementError {
    fn as_ref(&self) -> &InputPassportElementError {
        self
    }
}

impl AsRef<InputPassportElementError> for InputPassportElementErrorBuilder {
    fn as_ref(&self) -> &InputPassportElementError {
        &self.inner
    }
}
