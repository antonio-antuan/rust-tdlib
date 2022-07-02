use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// An object of this type can be returned on every function call, in case of an error
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Error {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user

    #[serde(default)]
    code: i32,
    /// Error message; subject to future changes

    #[serde(default)]
    message: String,
}

impl RObject for Error {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Error {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ErrorBuilder {
        let mut inner = Error::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ErrorBuilder { inner }
    }

    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

#[doc(hidden)]
pub struct ErrorBuilder {
    inner: Error,
}

#[deprecated]
pub type RTDErrorBuilder = ErrorBuilder;

impl ErrorBuilder {
    pub fn build(&self) -> Error {
        self.inner.clone()
    }

    pub fn code(&mut self, code: i32) -> &mut Self {
        self.inner.code = code;
        self
    }

    pub fn message<T: AsRef<str>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().to_string();
        self
    }
}

impl AsRef<Error> for Error {
    fn as_ref(&self) -> &Error {
        self
    }
}

impl AsRef<Error> for ErrorBuilder {
    fn as_ref(&self) -> &Error {
        &self.inner
    }
}
