use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the specified error and ensures that the Error object is used; for testing only. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestReturnError {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The error to be returned
    error: Error,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestReturnError {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestReturnError {}

impl TestReturnError {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestReturnErrorBuilder {
        let mut inner = TestReturnError::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testReturnError".to_string();

        TestReturnErrorBuilder { inner }
    }

    pub fn error(&self) -> &Error {
        &self.error
    }
}

#[doc(hidden)]
pub struct TestReturnErrorBuilder {
    inner: TestReturnError,
}

#[deprecated]
pub type RTDTestReturnErrorBuilder = TestReturnErrorBuilder;

impl TestReturnErrorBuilder {
    pub fn build(&self) -> TestReturnError {
        self.inner.clone()
    }

    pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
        self.inner.error = error.as_ref().clone();
        self
    }
}

impl AsRef<TestReturnError> for TestReturnError {
    fn as_ref(&self) -> &TestReturnError {
        self
    }
}

impl AsRef<TestReturnError> for TestReturnErrorBuilder {
    fn as_ref(&self) -> &TestReturnError {
        &self.inner
    }
}
